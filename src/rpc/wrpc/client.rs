use crate::address::PyAddress;
use crate::rpc::model::*;
use crate::rpc::notification::PyNotification;
use crate::rpc::wrpc::resolver::PyResolver;
use ahash::AHashMap;
use futures::*;
// use kaspa_addresses::Address;
use kaspa_notify::listener::ListenerId;
use kaspa_notify::notification::Notification;
use kaspa_notify::scope::{
    BlockAddedScope, FinalityConflictResolvedScope, FinalityConflictScope, NewBlockTemplateScope,
    PruningPointUtxoSetOverrideScope, Scope, SinkBlueScoreChangedScope, UtxosChangedScope,
    VirtualChainChangedScope, VirtualDaaScoreChangedScope,
};
use kaspa_notify::{connection::ChannelType, events::EventType};
use kaspa_rpc_core::api::rpc::RpcApi;
use kaspa_rpc_core::model::*;
use kaspa_rpc_core::notify::connection::ChannelConnection;
// use kaspa_rpc_macros::{build_wrpc_python_interface, build_wrpc_python_subscriptions};
use kaspa_wrpc_client::{
    KaspaRpcClient, WrpcEncoding, client::ConnectOptions, error::Error, prelude::*, result::Result,
};
use paste::paste;
use pyo3::{
    exceptions::PyException,
    prelude::*,
    types::{PyDict, PyModule, PyTuple},
};
use std::str::FromStr;
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use workflow_core::channel::{Channel, DuplexChannel};
use workflow_log::*;
use workflow_rpc::{client::Ctl, encoding::Encoding};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum NotificationEvent {
    All,
    Notification(EventType),
    RpcCtl(Ctl),
}

impl FromStr for NotificationEvent {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s == "all" {
            Ok(NotificationEvent::All)
        } else if let Ok(ctl) = Ctl::from_str(s) {
            Ok(NotificationEvent::RpcCtl(ctl))
        } else if let Ok(event) = EventType::from_str(s) {
            Ok(NotificationEvent::Notification(event))
        } else {
            Err(Error::custom(format!(
                "Invalid notification event type: `{}`",
                s
            )))
        }
    }
}

#[derive(Clone)]
struct PyCallback {
    callback: Arc<Py<PyAny>>,
    args: Option<Arc<Py<PyTuple>>>,
    kwargs: Option<Arc<Py<PyDict>>>,
}

impl PyCallback {
    fn add_event_to_args(&self, py: Python, event: Bound<PyDict>) -> PyResult<Py<PyTuple>> {
        match &self.args {
            Some(existing_args) => {
                let tuple_ref = existing_args.bind(py);

                let mut new_args: Vec<Py<PyAny>> =
                    tuple_ref.iter().map(|arg| arg.unbind()).collect();
                new_args.push(event.into());

                Ok(Py::from(PyTuple::new(py, new_args)?))
            }
            None => Ok(Py::from(PyTuple::new(py, [event])?)),
        }
    }

    fn execute(&self, py: Python, event: Bound<PyDict>) -> PyResult<Py<PyAny>> {
        let args = self.add_event_to_args(py, event)?;
        let kwargs = self.kwargs.as_ref().map(|kw| kw.bind(py));

        let result = self
            .callback
            .call(py, args.bind(py), kwargs)
            .map_err(|err| {
                // let fn_name: String = self.callback.getattr(py, "__name__").unwrap().extract(py).unwrap();

                let traceback = PyModule::import(py, "traceback")
                    .and_then(|traceback| {
                        traceback.call_method(
                            "format_exception",
                            (err.get_type(py), err.value(py), err.traceback(py)),
                            None,
                        )
                    })
                    .map(|formatted| {
                        let trace_lines: Vec<String> = formatted
                            .extract()
                            .unwrap_or_else(|_| vec!["<Failed to retrieve traceback>".to_string()]);
                        trace_lines.join("")
                    })
                    .unwrap_or_else(|_| "<Failed to retrieve traceback>".to_string());

                PyException::new_err(traceback.to_string())
            })?;

        Ok(result)
    }
}

pub struct Inner {
    client: Arc<KaspaRpcClient>,
    resolver: Option<Resolver>,
    notification_task: Arc<AtomicBool>,
    notification_ctl: DuplexChannel,
    callbacks: Arc<Mutex<AHashMap<NotificationEvent, Vec<PyCallback>>>>,
    listener_id: Arc<Mutex<Option<ListenerId>>>,
    notification_channel: Channel<kaspa_rpc_core::Notification>,
}

impl Inner {
    fn notification_callbacks(&self, event: NotificationEvent) -> Option<Vec<PyCallback>> {
        let notification_callbacks = self.callbacks.lock().unwrap();
        let all = notification_callbacks.get(&NotificationEvent::All).cloned();
        let target = notification_callbacks.get(&event).cloned();
        match (all, target) {
            (Some(mut vec_all), Some(vec_target)) => {
                vec_all.extend(vec_target);
                Some(vec_all)
            }
            (Some(vec_all), None) => Some(vec_all),
            (None, Some(vec_target)) => Some(vec_target),
            (None, None) => None,
        }
    }
}

#[pyclass(name = "RpcClient")]
#[derive(Clone)]
pub struct PyRpcClient {
    inner: Arc<Inner>,
}

impl PyRpcClient {
    pub fn new(
        resolver: Option<Resolver>,
        url: Option<String>,
        encoding: Option<WrpcEncoding>,
        network_id: Option<NetworkId>,
    ) -> PyResult<PyRpcClient> {
        let encoding = encoding.unwrap_or(Encoding::Borsh);
        let url = url
            .map(|url| {
                if let Some(network_id) = network_id {
                    Self::parse_url(&url, encoding, network_id)
                } else {
                    Ok(url.to_string())
                }
            })
            .transpose()?;

        let client = Arc::new(
            KaspaRpcClient::new(encoding, url.as_deref(), resolver.clone(), network_id, None)
                .map_err(|err| PyException::new_err(err.to_string()))?,
        );

        let rpc_client = PyRpcClient {
            inner: Arc::new(Inner {
                client,
                resolver,
                notification_task: Arc::new(AtomicBool::new(false)),
                notification_ctl: DuplexChannel::oneshot(),
                callbacks: Arc::new(Default::default()),
                listener_id: Arc::new(Mutex::new(None)),
                notification_channel: Channel::unbounded(),
            }),
        };

        Ok(rpc_client)
    }
}

#[pymethods]
impl PyRpcClient {
    #[new]
    #[pyo3(signature = (resolver=None, url=None, encoding=None, network_id=None))]
    fn ctor(
        resolver: Option<PyResolver>,
        url: Option<String>,
        encoding: Option<String>,
        network_id: Option<String>,
    ) -> PyResult<PyRpcClient> {
        let encoding = WrpcEncoding::from_str(&encoding.unwrap_or("borsh".to_string()))
            .map_err(|err| PyException::new_err(format!("{}", err)))?;
        let network_id = NetworkId::from_str(&network_id.unwrap_or(String::from("mainnet")))
            .map_err(|err| PyException::new_err(err.to_string()))?;

        Self::new(
            resolver.map(|r| r.inner),
            url,
            Some(encoding),
            Some(network_id),
        )
    }

    #[getter]
    fn url(&self) -> Option<String> {
        self.inner.client.url()
    }

    #[getter]
    fn resolver(&self) -> Option<PyResolver> {
        self.inner.resolver.clone().map(PyResolver::new)
    }

    fn set_resolver(&self, resolver: PyResolver) -> PyResult<()> {
        self.inner
            .client
            .set_resolver(resolver.into())
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(())
    }

    fn set_network_id(&self, network_id: String) -> PyResult<()> {
        let network_id = NetworkId::from_str(&network_id)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        self.inner
            .client
            .set_network_id(&network_id)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(())
    }

    #[getter]
    fn is_connected(&self) -> bool {
        self.inner.client.is_connected()
    }

    #[getter]
    fn encoding(&self) -> String {
        self.inner.client.encoding().to_string()
    }

    #[getter]
    #[pyo3(name = "node_id")]
    fn resolver_node_id(&self) -> Option<String> {
        self.inner
            .client
            .node_descriptor()
            .map(|node| node.uid.clone())
    }

    #[pyo3(signature = (block_async_connect=None, strategy=None, url=None, timeout_duration=None, retry_interval=None))]
    pub fn connect<'py>(
        &self,
        py: Python<'py>,
        block_async_connect: Option<bool>,
        strategy: Option<String>,
        url: Option<String>,
        timeout_duration: Option<u64>,
        retry_interval: Option<u64>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let block_async_connect = block_async_connect.unwrap_or(true);
        let strategy = match strategy {
            Some(strategy) => ConnectStrategy::from_str(&strategy)
                .map_err(|err| PyException::new_err(format!("{}", err)))
                .map_err(|err| PyException::new_err(err.to_string()))?,
            None => ConnectStrategy::Retry,
        };
        let connect_timeout: Option<Duration> = timeout_duration.map(Duration::from_millis);
        let retry_interval: Option<Duration> = retry_interval.map(Duration::from_millis);

        let options = ConnectOptions {
            block_async_connect,
            strategy,
            url,
            connect_timeout,
            retry_interval,
        };

        self.start_notification_task(py)
            .map_err(|err| PyException::new_err(err.to_string()))?;

        let client = self.inner.client.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .connect(Some(options))
                .await
                .map_err(|e| PyException::new_err(e.to_string()))?;
            Ok(())
        })
    }

    fn disconnect<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let client = self.clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            client
                .inner
                .client
                .disconnect()
                .await
                .map_err(|err| PyException::new_err(err.to_string()))?;
            client
                .stop_notification_task()
                .await
                .map_err(|err| PyException::new_err(err.to_string()))?;
            Ok(())
        })
    }

    fn start<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        self.start_notification_task(py)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let inner = self.inner.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            inner
                .client
                .start()
                .await
                .map_err(|err| PyException::new_err(err.to_string()))?;
            Ok(())
        })
    }

    // fn stop() PY-TODO
    // fn trigger_abort() PY-TODO

    #[pyo3(signature = (event, callback, *args, **kwargs))]
    fn add_event_listener(
        &self,
        py: Python,
        event: String,
        callback: Py<PyAny>,
        args: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<()> {
        let event = NotificationEvent::from_str(event.as_str())
            .map_err(|err| PyException::new_err(err.to_string()))?;

        let args = args.into_pyobject(py)?.extract::<Py<PyTuple>>()?;

        let kwargs = match kwargs {
            Some(kw) => kw.into_pyobject(py)?.extract::<Py<PyDict>>()?,
            None => PyDict::new(py).into(),
        };

        let py_callback = PyCallback {
            callback: Arc::new(callback),
            args: Some(Arc::new(args)),
            kwargs: Some(Arc::new(kwargs)),
        };

        self.inner
            .callbacks
            .lock()
            .unwrap()
            .entry(event)
            .or_default()
            .push(py_callback);
        Ok(())
    }

    #[pyo3(signature = (event, callback=None))]
    fn remove_event_listener(&self, event: String, callback: Option<Py<PyAny>>) -> PyResult<()> {
        let event = NotificationEvent::from_str(event.as_str())
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let mut callbacks = self.inner.callbacks.lock().unwrap();

        match (&event, callback) {
            (NotificationEvent::All, None) => {
                // Remove all callbacks from "all" events
                callbacks.clear();
            }
            (NotificationEvent::All, Some(callback)) => {
                // Remove given callback from "all" events
                for callbacks in callbacks.values_mut() {
                    // callbacks.retain(|c| {
                    //     let cb_ref = c.callback.bind(py);
                    //     let callback_ref = callback.bind(py);
                    //     cb_ref.as_ref().ne(callback_ref.as_ref()).unwrap_or(true)
                    // });
                    callbacks.retain(|entry| entry.callback.as_ref().as_ptr() != callback.as_ptr());
                }
            }
            (_, None) => {
                // Remove all callbacks from given event
                callbacks.remove(&event);
            }
            (_, Some(callback)) => {
                // Remove given callback from given event
                if let Some(callbacks) = callbacks.get_mut(&event) {
                    // callbacks.retain(|c| {
                    //     let cb_ref = c.callback.bind(py);
                    //     let callback_ref = callback.bind(py);
                    //     cb_ref.as_ref().ne(callback_ref.as_ref()).unwrap_or(true)
                    // });
                    callbacks.retain(|entry| entry.callback.as_ref().as_ptr() != callback.as_ptr());
                }
            }
        }
        Ok(())
    }

    // fn clear_event_listener PY-TODO
    // fn default_port PY-TODO

    fn remove_all_event_listeners(&self) -> PyResult<()> {
        *self.inner.callbacks.lock().unwrap() = Default::default();
        Ok(())
    }
}

impl PyRpcClient {
    pub fn parse_url(url: &str, encoding: Encoding, network_id: NetworkId) -> PyResult<String> {
        let url_ = KaspaRpcClient::parse_url(url.to_string(), encoding, network_id.into())
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(url_)
    }
}

impl PyRpcClient {
    // fn new_with_rpc_client() PY-TODO

    pub fn listener_id(&self) -> Option<ListenerId> {
        *self.inner.listener_id.lock().unwrap()
    }

    #[allow(dead_code)]
    pub fn client(&self) -> &Arc<KaspaRpcClient> {
        &self.inner.client
    }

    async fn stop_notification_task(&self) -> Result<()> {
        if self.inner.notification_task.load(Ordering::SeqCst) {
            self.inner.notification_ctl.signal(()).await?;
            self.inner.notification_task.store(false, Ordering::SeqCst);
        }
        Ok(())
    }

    #[allow(clippy::result_large_err)]
    fn start_notification_task(&self, py: Python) -> Result<()> {
        if self.inner.notification_task.load(Ordering::SeqCst) {
            return Ok(());
        }

        self.inner.notification_task.store(true, Ordering::SeqCst);

        let ctl_receiver = self.inner.notification_ctl.request.receiver.clone();
        let ctl_sender = self.inner.notification_ctl.response.sender.clone();
        let notification_receiver = self.inner.notification_channel.receiver.clone();
        let ctl_multiplexer_channel = self
            .inner
            .client
            .rpc_client()
            .ctl_multiplexer()
            .as_ref()
            .expect("Python RpcClient ctl_multiplexer is None")
            .channel();
        let this = self.clone();

        let _ = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            loop {
                select_biased! {
                    msg = ctl_multiplexer_channel.recv().fuse() => {
                        if let Ok(ctl) = msg {

                            match ctl {
                                Ctl::Connect => {
                                    let listener_id = this.inner.client.register_new_listener(ChannelConnection::new(
                                        "kaspapy-wrpc-client-python",
                                        this.inner.notification_channel.sender.clone(),
                                        ChannelType::Persistent,
                                    ));
                                    *this.inner.listener_id.lock().unwrap() = Some(listener_id);
                                }
                                Ctl::Disconnect => {
                                    let listener_id = this.inner.listener_id.lock().unwrap().take();
                                    if let Some(listener_id) = listener_id
                                        && let Err(err) = this.inner.client.unregister_listener(listener_id).await {
                                            panic!("Error in unregister_listener: {:?}",err);
                                    }
                                }
                            }

                            let event = NotificationEvent::RpcCtl(ctl);
                            if let Some(handlers) = this.inner.notification_callbacks(event) {
                                for handler in handlers.into_iter() {
                                    Python::attach(|py| {
                                        let event = PyDict::new(py);
                                        event.set_item("type", ctl.to_string()).unwrap();
                                        event.set_item("rpc", this.url()).unwrap();

                                        handler.execute(py, event).unwrap_or_else(|err| panic!("{}", err));
                                    });
                                }
                            }
                        }
                    },
                    msg = notification_receiver.recv().fuse() => {
                        if let Ok(notification) = &msg {
                            match &notification {
                                kaspa_rpc_core::Notification::UtxosChanged(utxos_changed_notification) => {
                                    let event_type = notification.event_type();
                                    let notification_event = NotificationEvent::Notification(event_type);
                                    if let Some(handlers) = this.inner.notification_callbacks(notification_event) {
                                        let UtxosChangedNotification { added, removed } = utxos_changed_notification;

                                        for handler in handlers.into_iter() {
                                            Python::attach(|py| {
                                                let added = serde_pyobject::to_pyobject(py, added).unwrap();
                                                let removed = serde_pyobject::to_pyobject(py, removed).unwrap();

                                                let event = PyDict::new(py);
                                                event.set_item("type", event_type.to_string()).unwrap();
                                                event.set_item("added", &added).unwrap();
                                                event.set_item("removed", &removed).unwrap();

                                                handler.execute(py, event).unwrap_or_else(|err| panic!("{}", err));
                                            })
                                        }
                                    }
                                },
                                _ => {
                                    let event_type = notification.event_type();
                                    let notification_event = NotificationEvent::Notification(event_type);
                                    if let Some(handlers) = this.inner.notification_callbacks(notification_event) {
                                        for handler in handlers.into_iter() {
                                            Python::attach(|py| {
                                                let event = PyDict::new(py);
                                                event.set_item("type", event_type.to_string()).unwrap();
                                                event.set_item("data", PyNotification::from(notification.clone()).to_pyobject(py).unwrap()).unwrap();

                                                handler.execute(py, event).unwrap_or_else(|err| panic!("{}", err));
                                            });
                                        }
                                    }
                                }
                            }
                        }
                    }
                    _ = ctl_receiver.recv().fuse() => {
                        break;
                    },

                }
            }

            if let Some(listener_id) = this.listener_id() {
                this.inner.listener_id.lock().unwrap().take();
                if let Err(err) = this.inner.client.unregister_listener(listener_id).await {
                    log_error!("Error in unregister_listener: {:?}", err);
                }
            }

            ctl_sender.send(()).await.ok();

            Python::attach(|_| Ok(()))
        });

        Ok(())
    }
}

#[pymethods]
impl PyRpcClient {
    fn subscribe_utxos_changed<'py>(
        &self,
        py: Python<'py>,
        addresses: Vec<PyAddress>,
    ) -> PyResult<Bound<'py, PyAny>> {
        if let Some(listener_id) = self.listener_id() {
            let client = self.inner.client.clone();
            let addresses = addresses.iter().map(|a| a.0.clone()).collect();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                client
                    .start_notify(
                        listener_id,
                        Scope::UtxosChanged(UtxosChangedScope { addresses }),
                    )
                    .await
                    .map_err(|err| PyException::new_err(err.to_string()))?;
                Ok(())
            })
        } else {
            Err(PyException::new_err("RPC subscribe on a closed connection"))
        }
    }

    fn unsubscribe_utxos_changed<'py>(
        &self,
        py: Python<'py>,
        addresses: Vec<PyAddress>,
    ) -> PyResult<Bound<'py, PyAny>> {
        if let Some(listener_id) = self.listener_id() {
            let client = self.inner.client.clone();
            let addresses = addresses.iter().map(|a| a.0.clone()).collect();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                client
                    .stop_notify(
                        listener_id,
                        Scope::UtxosChanged(UtxosChangedScope { addresses }),
                    )
                    .await
                    .map_err(|err| PyException::new_err(err.to_string()))?;
                Ok(())
            })
        } else {
            Err(PyException::new_err(
                "RPC unsubscribe on a closed connection",
            ))
        }
    }

    fn subscribe_virtual_chain_changed<'py>(
        &self,
        py: Python<'py>,
        include_accepted_transaction_ids: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        if let Some(listener_id) = self.listener_id() {
            let client = self.inner.client.clone();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                client
                    .start_notify(
                        listener_id,
                        Scope::VirtualChainChanged(VirtualChainChangedScope {
                            include_accepted_transaction_ids,
                        }),
                    )
                    .await
                    .map_err(|err| PyException::new_err(err.to_string()))?;
                Ok(())
            })
        } else {
            Err(PyException::new_err("RPC subscribe on a closed connection"))
        }
    }

    fn unsubscribe_virtual_chain_changed<'py>(
        &self,
        py: Python<'py>,
        include_accepted_transaction_ids: bool,
    ) -> PyResult<Bound<'py, PyAny>> {
        if let Some(listener_id) = self.listener_id() {
            let client = self.inner.client.clone();
            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                client
                    .stop_notify(
                        listener_id,
                        Scope::VirtualChainChanged(VirtualChainChangedScope {
                            include_accepted_transaction_ids,
                        }),
                    )
                    .await
                    .map_err(|err| PyException::new_err(err.to_string()))?;
                Ok(())
            })
        } else {
            Err(PyException::new_err(
                "RPC unsubscribe on a closed connection",
            ))
        }
    }
}

/// Macro to generate subscribe/unsubscribe method implementations for RPC notifications.
///
/// For each scope name (e.g., `BlockAdded`), this generates:
/// - `subscribe_block_added` - Python-callable async method to start notifications
/// - `unsubscribe_block_added` - Python-callable async method to stop notifications
macro_rules! build_wrpc_python_subscriptions {
    ([$($scope:ident),* $(,)?]) => {
        paste! {
            #[pymethods]
            impl PyRpcClient {
                $(
                    fn [<subscribe_ $scope:snake>]<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                        if let Some(listener_id) = self.listener_id() {
                            let client = self.inner.client.clone();
                            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                                client.start_notify(listener_id, Scope::$scope([<$scope Scope>] {})).await
                                    .map_err(|err| PyException::new_err(err.to_string()))?;
                                Ok(())
                            })
                        } else {
                            Err(PyException::new_err("RPC subscribe on a closed connection"))
                        }
                    }

                    fn [<unsubscribe_ $scope:snake>]<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
                        if let Some(listener_id) = self.listener_id() {
                            let client = self.inner.client.clone();
                            pyo3_async_runtimes::tokio::future_into_py(py, async move {
                                client.stop_notify(listener_id, Scope::$scope([<$scope Scope>] {})).await
                                    .map_err(|err| PyException::new_err(err.to_string()))?;
                                Ok(())
                            })
                        } else {
                            Err(PyException::new_err("RPC unsubscribe on a closed connection"))
                        }
                    }
                )*
            }
        }
    };
}

build_wrpc_python_subscriptions!([
    BlockAdded,
    FinalityConflict,
    FinalityConflictResolved,
    NewBlockTemplate,
    PruningPointUtxoSetOverride,
    SinkBlueScoreChanged,
    VirtualDaaScoreChanged,
]);

/// Macro to generate RPC method implementations for RpcClient.
///
/// For each type name (e.g., `GetBlockCount`), this generates:
/// - A Python-callable async method `get_block_count`
/// - That accepts an optional `PyDict` as request parameters
/// - Calls the corresponding `get_block_count_call` method on the RPC client
/// - Returns the response as a Python object
macro_rules! build_wrpc_python_interface {
    ([$($name:ident),* $(,)?]) => {
        paste! {
            #[pymethods]
            impl PyRpcClient {
                $(
                    #[pyo3(signature = (request=None))]
                    fn [<$name:snake>]<'py>(
                        &self,
                        py: Python<'py>,
                        request: Option<Bound<'_, PyDict>>
                    ) -> PyResult<Bound<'py, PyAny>> {
                        let client = self.inner.client.clone();

                        let request: [<Py $name Request>] = request
                            .unwrap_or_else(|| PyDict::new(py))
                            .try_into()?;

                        pyo3_async_runtimes::tokio::future_into_py(py, async move {
                            let response: [<$name Response>] = client
                                .[<$name:snake _call>](None, request.0)
                                .await
                                .map_err(|err| PyException::new_err(err.to_string()))?;

                            Python::attach(|py| {
                                Ok(serde_pyobject::to_pyobject(py, &response)?.unbind())
                            })
                        })
                    }
                )*
            }
        }
    };
}

build_wrpc_python_interface!([
    GetBlockCount,
    GetBlockDagInfo,
    GetCoinSupply,
    GetConnectedPeerInfo,
    GetInfo,
    GetPeerAddresses,
    GetMetrics,
    GetConnections,
    GetSink,
    GetSinkBlueScore,
    Ping,
    Shutdown,
    GetServerInfo,
    GetSyncStatus,
    GetFeeEstimate,
    GetCurrentNetwork,
    GetSystemInfo,
]);

/// Macro to generate RPC method implementations that require request parameters.
///
/// Similar to `build_wrpc_python_interface!`, but the `request` parameter is required
/// (not optional), for RPC calls that need specific arguments.
macro_rules! build_wrpc_python_interface_with_args {
    ([$($name:ident),* $(,)?]) => {
        paste! {
            #[pymethods]
            impl PyRpcClient {
                $(
                    fn [<$name:snake>]<'py>(
                        &self,
                        py: Python<'py>,
                        request: Bound<'_, PyDict>
                    ) -> PyResult<Bound<'py, PyAny>> {
                        let client = self.inner.client.clone();

                        let request: [<Py $name Request>] = request.try_into()?;

                        pyo3_async_runtimes::tokio::future_into_py(py, async move {
                            let response: [<$name Response>] = client
                                .[<$name:snake _call>](None, request.0)
                                .await
                                .map_err(|err| PyException::new_err(err.to_string()))?;

                            Python::attach(|py| {
                                Ok(serde_pyobject::to_pyobject(py, &response)?.unbind())
                            })
                        })
                    }
                )*
            }
        }
    };
}

build_wrpc_python_interface_with_args!([
    AddPeer,
    Ban,
    EstimateNetworkHashesPerSecond,
    GetBalanceByAddress,
    GetBalancesByAddresses,
    GetBlock,
    GetBlocks,
    GetBlockTemplate,
    GetCurrentBlockColor,
    GetDaaScoreTimestampEstimate,
    GetFeeEstimateExperimental,
    GetHeaders,
    GetMempoolEntries,
    GetMempoolEntriesByAddresses,
    GetMempoolEntry,
    GetSubnetwork,
    GetUtxosByAddresses,
    GetUtxoReturnAddress,
    GetVirtualChainFromBlock,
    ResolveFinalityConflict,
    SubmitBlock,
    SubmitTransaction,
    SubmitTransactionReplacement,
    Unban,
]);

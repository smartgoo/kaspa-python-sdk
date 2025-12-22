use crate::consensus::core::network::PyNetworkId;
use kaspa_wrpc_client::{Resolver, WrpcEncoding};
use pyo3::{exceptions::PyException, prelude::*};
use std::{str::FromStr, sync::Arc};

#[pyclass(name = "Resolver")]
#[derive(Debug, Clone)]
pub struct PyResolver(Resolver);

impl PyResolver {
    pub fn new(resolver: Resolver) -> Self {
        Self(resolver)
    }

    pub fn inner(&self) -> Resolver {
        self.0.clone()
    }
}

#[pymethods]
impl PyResolver {
    #[new]
    #[pyo3(signature = (urls=None, tls=None))]
    pub fn ctor(urls: Option<Vec<String>>, tls: Option<bool>) -> PyResult<Self> {
        let tls = tls.unwrap_or(false);
        if let Some(urls) = urls {
            Ok(Self(Resolver::new(
                Some(urls.into_iter().map(Arc::new).collect::<Vec<_>>()),
                tls,
            )))
        } else {
            Ok(Self(Resolver::default()))
        }
    }
}

#[pymethods]
impl PyResolver {
    fn urls(&self) -> Vec<String> {
        self.0
            .urls()
            .unwrap_or_default()
            .into_iter()
            .map(|url| (*url).clone())
            .collect::<Vec<_>>()
    }

    fn get_node<'py>(
        &self,
        py: Python<'py>,
        encoding: &str,
        network_id: Py<PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let encoding = WrpcEncoding::from_str(encoding)
            .map_err(|err| PyException::new_err(format!("{}", err)))?;
        // let network_id = NetworkId::from_str(network_id)?;
        let network_id = network_id.extract::<PyNetworkId>(py)?;

        let resolver = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let node = resolver
                .get_node(encoding, network_id.into())
                .await
                .map_err(|err| PyException::new_err(err.to_string()))?;
            Python::attach(|py| Ok(serde_pyobject::to_pyobject(py, &node)?.unbind()))
        })
    }

    fn get_url<'py>(
        &self,
        py: Python<'py>,
        encoding: &str,
        network_id: Py<PyAny>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let encoding = WrpcEncoding::from_str(encoding)
            .map_err(|err| PyException::new_err(format!("{}", err)))?;
        // let network_id = NetworkId::from_str(network_id)?;
        let network_id = network_id.extract::<PyNetworkId>(py)?;

        let resolver = self.0.clone();
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let url = resolver
                .get_url(encoding, network_id.into())
                .await
                .map_err(|err| PyException::new_err(err.to_string()))?;
            Ok(url)
        })
    }

    // fn connect() TODO
}

impl From<PyResolver> for Resolver {
    fn from(resolver: PyResolver) -> Self {
        resolver.0
    }
}

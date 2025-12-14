use kaspa_rpc_core::api::notifications::Notification;
use pyo3::prelude::*;
use serde_pyobject::to_pyobject;

#[pyclass(name = "Notification")]
pub struct PyNotification(pub Notification);

impl PyNotification {
    pub fn to_pyobject(&self, py: Python) -> PyResult<Py<PyAny>> {
        let bound_obj = match &self.0 {
            Notification::BlockAdded(v) => to_pyobject(py, &v),
            Notification::FinalityConflict(v) => to_pyobject(py, &v),
            Notification::FinalityConflictResolved(v) => to_pyobject(py, &v),
            Notification::NewBlockTemplate(v) => to_pyobject(py, &v),
            Notification::PruningPointUtxoSetOverride(v) => to_pyobject(py, &v),
            Notification::UtxosChanged(v) => to_pyobject(py, &v),
            Notification::VirtualDaaScoreChanged(v) => to_pyobject(py, &v),
            Notification::SinkBlueScoreChanged(v) => to_pyobject(py, &v),
            Notification::VirtualChainChanged(v) => to_pyobject(py, &v),
        }?;

        Ok(bound_obj.unbind())
    }
}

impl From<Notification> for PyNotification {
    fn from(value: Notification) -> Self {
        PyNotification(value)
    }
}

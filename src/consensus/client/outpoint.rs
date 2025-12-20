use crate::consensus::core::tx::PyTransactionId;
use kaspa_consensus_client::{TransactionOutpoint, TransactionOutpointInner};
use kaspa_consensus_core::tx::TransactionIndexType;
use pyo3::{prelude::*, types::PyDict};

#[pyclass(name = "TransactionOutpoint")]
#[derive(Clone)]
pub struct PyTransactionOutpoint(pub TransactionOutpoint);

#[pymethods]
impl PyTransactionOutpoint {
    #[new]
    pub fn ctor(transaction_id: PyTransactionId, index: u32) -> PyTransactionOutpoint {
        let inner = TransactionOutpoint::new(transaction_id.into(), index);
        PyTransactionOutpoint(inner)
    }

    #[pyo3(name = "get_id")]
    pub fn id_string(&self) -> String {
        format!(
            "{}-{}",
            self.0.get_transaction_id_as_string(),
            self.get_index()
        )
    }

    #[getter]
    #[pyo3(name = "transaction_id")]
    pub fn get_transaction_id_as_string(&self) -> String {
        self.0.inner().transaction_id.to_string()
    }

    #[getter]
    #[pyo3(name = "index")]
    pub fn get_index(&self) -> TransactionIndexType {
        self.0.inner().index
    }
}

impl From<PyTransactionOutpoint> for TransactionOutpoint {
    fn from(value: PyTransactionOutpoint) -> Self {
        value.0
    }
}

impl From<TransactionOutpoint> for PyTransactionOutpoint {
    fn from(value: TransactionOutpoint) -> Self {
        PyTransactionOutpoint(value)
    }
}

impl TryFrom<&Bound<'_, PyDict>> for PyTransactionOutpoint {
    type Error = PyErr;
    fn try_from(dict: &Bound<PyDict>) -> PyResult<Self> {
        let inner: TransactionOutpointInner = serde_pyobject::from_pyobject(dict.clone())?;
        let outpoint = TransactionOutpoint::new(inner.transaction_id, inner.index);
        Ok(PyTransactionOutpoint(outpoint))
    }
}

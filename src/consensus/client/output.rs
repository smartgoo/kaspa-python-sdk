use kaspa_consensus_client::TransactionOutput;
use pyo3::prelude::*;

use crate::consensus::core::script_public_key::PyScriptPublicKey;

#[pyclass(name = "TransactionOutput")]
#[derive(Clone)]
pub struct PyTransactionOutput(pub TransactionOutput);

#[pymethods]
impl PyTransactionOutput {
    #[new]
    pub fn ctor(value: u64, script_public_key: PyScriptPublicKey) -> PyTransactionOutput {
        let inner = TransactionOutput::new(value, script_public_key.into());
        PyTransactionOutput(inner)
    }

    #[getter]
    #[pyo3(name = "value")]
    pub fn get_value(&self) -> u64 {
        self.0.inner().value
    }

    #[setter]
    #[pyo3(name = "value")]
    pub fn set_value(&self, v: u64) {
        self.0.inner().value = v;
    }

    #[getter]
    #[pyo3(name = "script_public_key")]
    pub fn get_script_public_key(&self) -> PyScriptPublicKey {
        self.0.inner().script_public_key.clone().into()
    }

    #[setter]
    #[pyo3(name = "script_public_key")]
    pub fn set_script_public_key(&self, v: PyScriptPublicKey) {
        self.0.inner().script_public_key = v.clone().into();
    }
}

impl From<TransactionOutput> for PyTransactionOutput {
    fn from(value: TransactionOutput) -> Self {
        PyTransactionOutput(value)
    }
}

impl From<PyTransactionOutput> for TransactionOutput {
    fn from(value: PyTransactionOutput) -> Self {
        value.0
    }
}

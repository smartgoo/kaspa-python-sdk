use crate::types::PyBinary;
use kaspa_consensus_core::tx::ScriptPublicKey;
use kaspa_utils::hex::FromHex;
use pyo3::{exceptions::PyException, prelude::*};
use std::str::FromStr;

#[pyclass(name = "ScriptPublicKey")]
#[derive(Clone)]
pub struct PyScriptPublicKey(ScriptPublicKey);

#[pymethods]
impl PyScriptPublicKey {
    #[new]
    pub fn constructor(version: u16, script: PyBinary) -> PyResult<PyScriptPublicKey> {
        let inner = ScriptPublicKey::new(version, script.data.into());
        Ok(PyScriptPublicKey(inner))
    }

    #[getter]
    #[pyo3(name = "script")]
    pub fn script_as_hex(&self) -> String {
        // self.0.script.to_hex()
        self.0.script_as_hex()
    }
}

impl From<PyScriptPublicKey> for ScriptPublicKey {
    fn from(value: PyScriptPublicKey) -> Self {
        value.0
    }
}

impl From<ScriptPublicKey> for PyScriptPublicKey {
    fn from(value: ScriptPublicKey) -> Self {
        PyScriptPublicKey(value)
    }
}

impl FromHex for PyScriptPublicKey {
    type Error = PyErr;

    fn from_hex(hex_str: &str) -> Result<Self, Self::Error> {
        let inner = ScriptPublicKey::from_str(hex_str)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(PyScriptPublicKey(inner))
    }
}

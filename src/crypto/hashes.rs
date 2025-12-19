use kaspa_hashes::Hash;
use pyo3::{exceptions::PyException, prelude::*};
use std::str::FromStr;

#[pyclass(name = "Hash")]
#[derive(Clone)]
pub struct PyHash(pub Hash);

#[pymethods]
impl PyHash {
    #[new]
    pub fn constructor_py(hex_str: &str) -> PyResult<Self> {
        let inner =
            Hash::from_str(hex_str).map_err(|err| PyException::new_err(format!("{}", err)))?;
        Ok(PyHash(inner))
    }

    #[pyo3(name = "to_string")]
    pub fn py_to_string(&self) -> String {
        self.0.to_string()
    }
}

impl From<PyHash> for Hash {
    fn from(value: PyHash) -> Self {
        value.0
    }
}

impl From<Hash> for PyHash {
    fn from(value: Hash) -> Self {
        PyHash(value)
    }
}

impl TryFrom<String> for PyHash {
    type Error = PyErr;

    fn try_from(value: String) -> PyResult<PyHash> {
        let inner = Hash::from_str(&value).map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(PyHash(inner))
    }
}

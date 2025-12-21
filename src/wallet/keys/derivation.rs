use kaspa_bip32::ChildNumber;
use pyo3::{exceptions::PyException, prelude::*};
use std::str::FromStr;

#[pyclass(name = "DerivationPath")]
#[derive(Clone)]
pub struct PyDerivationPath(kaspa_bip32::DerivationPath);

#[pymethods]
impl PyDerivationPath {
    #[new]
    pub fn new(path: &str) -> PyResult<PyDerivationPath> {
        let inner = kaspa_bip32::DerivationPath::from_str(path)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(PyDerivationPath(inner))
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[pyo3(name = "length")]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn parent(&self) -> Option<PyDerivationPath> {
        self.0.parent().map(|inner| PyDerivationPath(inner))
    }

    #[pyo3(signature = (child_number, hardened=None))]
    pub fn push(&mut self, child_number: u32, hardened: Option<bool>) -> PyResult<()> {
        let child = ChildNumber::new(child_number, hardened.unwrap_or(false))
            .map_err(|err| PyException::new_err(err.to_string()))?;
        self.0.push(child);
        Ok(())
    }

    #[pyo3(name = "to_string")]
    pub fn to_str(&self) -> String {
        self.0.to_string()
    }
}

impl From<PyDerivationPath> for kaspa_bip32::DerivationPath {
    fn from(value: PyDerivationPath) -> Self {
        value.0
    }
}

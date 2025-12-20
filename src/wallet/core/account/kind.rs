use std::str::FromStr;

use kaspa_wallet_core::account::kind::AccountKind;
use pyo3::{exceptions::PyException, prelude::*};

#[pyclass(name = "AccountKind")]
#[derive(Clone)]
pub struct PyAccountKind(pub AccountKind);

#[pymethods]
impl PyAccountKind {
    #[new]
    pub fn ctor(kind: &str) -> PyResult<PyAccountKind> {
        let inner =
            AccountKind::from_str(kind).map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(PyAccountKind(inner))
    }

    pub fn __str__(&self) -> String {
        self.py_to_string()
    }

    #[pyo3(name = "to_string")]
    pub fn py_to_string(&self) -> String {
        self.0.as_str().to_string()
    }
}

impl From<AccountKind> for PyAccountKind {
    fn from(value: AccountKind) -> Self {
        PyAccountKind(value)
    }
}

impl From<PyAccountKind> for AccountKind {
    fn from(value: PyAccountKind) -> Self {
        value.0
    }
}

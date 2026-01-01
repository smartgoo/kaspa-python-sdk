use kaspa_addresses::{Address, AddressError, Prefix};
use pyo3::{exceptions::PyException, prelude::*};

#[pyclass(name = "Address")]
#[derive(Clone)]
pub struct PyAddress(pub Address);

#[pymethods]
impl PyAddress {
    #[new]
    pub fn constructor(address: &str) -> PyResult<PyAddress> {
        Ok(PyAddress(address.try_into().map_err(
            |err: AddressError| PyException::new_err(err.to_string()),
        )?))
    }

    #[staticmethod]
    #[pyo3(name = "validate")]
    pub fn validate(address: &str) -> bool {
        Address::try_from(address).is_ok()
    }

    #[pyo3(name = "to_string")]
    pub fn address_to_string(&self) -> String {
        self.0.address_to_string()
    }

    #[getter]
    #[pyo3(name = "version")]
    pub fn version_to_string(&self) -> String {
        self.0.version.to_string()
    }

    #[getter]
    #[pyo3(name = "prefix")]
    pub fn prefix_to_string(&self) -> String {
        self.0.prefix.to_string()
    }

    #[setter]
    #[pyo3(name = "prefix")]
    pub fn set_prefix_from_str(&mut self, prefix: &str) -> PyResult<()> {
        self.0.prefix =
            Prefix::try_from(prefix).map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(())
    }

    // TODO Cannot expose since encode_payload is private
    // Requires reimplementation
    // #[pyo3(name = "payload")]
    // pub fn payload_to_string(&self) -> String {
    //     self.0.encode_payload()
    // }

    // TODO Cannot expose since encode_payload is private
    // Requires reimplementation
    // #[pyo3(name = "short")]
    // pub fn short(&self, n: usize) -> String {
    //     let payload = self.encode_payload();
    //     let n = std::cmp::min(n, payload.len() / 4);
    //     format!("{}:{}....{}", self.prefix, &payload[0..n], &payload[payload.len() - n..])
    // }
}

impl From<Address> for PyAddress {
    fn from(value: Address) -> Self {
        PyAddress(value)
    }
}

impl From<PyAddress> for Address {
    fn from(value: PyAddress) -> Address {
        value.0
    }
}

impl TryFrom<String> for PyAddress {
    type Error = PyErr;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let inner =
            Address::try_from(value).map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(PyAddress(inner))
    }
}

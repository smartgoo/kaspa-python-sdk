use kaspa_addresses::Address;
use pyo3::prelude::*;

#[pyclass(name = "Address")]
#[derive(Clone)]
pub struct PyAddress(pub Address);

impl From<Address> for PyAddress {
    fn from(value: Address) -> Self {
        PyAddress(value)
    }
}

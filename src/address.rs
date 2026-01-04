use kaspa_addresses::{Address, AddressError, Prefix};
use pyo3::{exceptions::PyException, prelude::*};
use pyo3_stub_gen::derive::*;

/// Represents a Kaspa blockchain address.
///
/// An Address consists of a network prefix and an encoded payload derived from
/// a public key or script hash. Kaspa uses Bech32 encoding for addresses.
#[gen_stub_pyclass]
#[pyclass(name = "Address")]
#[derive(Clone)]
pub struct PyAddress(pub Address);

#[gen_stub_pymethods]
#[pymethods]
impl PyAddress {
    /// Create a new Address from a string.
    ///
    /// Args:
    ///     address: A valid Kaspa address string.
    ///
    /// Returns:
    ///     Address: A new Address instance.
    ///
    /// Raises:
    ///     Exception: If the address string is invalid.
    #[new]
    pub fn constructor(address: &str) -> PyResult<PyAddress> {
        Ok(PyAddress(address.try_into().map_err(
            |err: AddressError| PyException::new_err(err.to_string()),
        )?))
    }

    /// Check if an address string is valid.
    ///
    /// Args:
    ///     address: A Kaspa address string to validate.
    ///
    /// Returns:
    ///     bool: True if the address is valid, False otherwise.
    #[staticmethod]
    #[pyo3(name = "validate")]
    pub fn validate(address: &str) -> bool {
        Address::try_from(address).is_ok()
    }

    /// The string representation of the Address.
    ///
    /// Returns:
    ///     str: A bech32 encoded Kaspa address string.
    #[pyo3(name = "to_string")]
    pub fn address_to_string(&self) -> String {
        self.0.address_to_string()
    }

    /// The string representation of the address version.
    /// Versions are `PubKey`, `PubKey ECDSA`, or `ScriptHash`.
    ///
    /// Returns:
    ///     str: The address version.
    #[getter]
    #[pyo3(name = "version")]
    pub fn version_to_string(&self) -> String {
        self.0.version.to_string()
    }

    /// The network prefix of the address.
    /// Common prefixes are `kaspa` (mainnet), `kaspatest` (testnet), and `kaspadev` (devnet).
    ///
    /// Returns:
    ///     str: The network prefix string.
    #[getter]
    #[pyo3(name = "prefix")]
    pub fn prefix_to_string(&self) -> String {
        self.0.prefix.to_string()
    }

    /// Set the network prefix of the address.
    ///
    /// Args:
    ///     value: The network prefix string (e.g., `kaspa`, `kaspatest`, `kaspadev`).
    ///
    /// Raises:
    ///     Exception: If the prefix string is invalid.
    #[setter]
    #[pyo3(name = "prefix")]
    pub fn set_prefix_from_str(&mut self, value: &str) -> PyResult<()> {
        self.0.prefix =
            Prefix::try_from(value).map_err(|err| PyException::new_err(err.to_string()))?;
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

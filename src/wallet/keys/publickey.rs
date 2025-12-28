use crate::{address::PyAddress, consensus::core::network::PyNetworkType};
use kaspa_addresses::{Address, Version};
use kaspa_consensus_core::network::NetworkType;
use kaspa_wallet_keys::{prelude::XOnlyPublicKey, publickey::PublicKey};
use pyo3::{exceptions::PyException, prelude::*};
use std::str::FromStr;

#[pyclass(name = "PublicKey")]
#[derive(Clone)]
pub struct PyPublicKey(pub PublicKey);

#[pymethods]
impl PyPublicKey {
    #[new]
    pub fn try_new(key: &str) -> PyResult<PyPublicKey> {
        let public_key =
            PublicKey::try_new(key).map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(PyPublicKey(public_key))
    }

    #[pyo3(name = "to_string")]
    pub fn to_string_impl(&self) -> String {
        self.0
            .public_key
            .as_ref()
            .map(|pk| pk.to_string())
            .unwrap_or_else(|| self.0.xonly_public_key.to_string())
    }

    #[pyo3(name = "to_address")]
    pub fn to_address(&self, network: PyNetworkType) -> PyResult<PyAddress> {
        let address = self
            .0
            .to_address(
                NetworkType::from(network).into(),
            )
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(PyAddress(address))
    }

    #[pyo3(name = "to_address_ecdsa")]
    pub fn to_address_ecdsa(&self, network: PyNetworkType) -> PyResult<PyAddress> {
        let address = self
            .0
            .to_address_ecdsa(
                NetworkType::from(network).into(),
            )
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(PyAddress(address))
    }

    #[pyo3(name = "to_x_only_public_key")]
    pub fn to_x_only_public_key(&self) -> PyXOnlyPublicKey {
        PyXOnlyPublicKey(self.0.xonly_public_key.into())
    }

    #[pyo3(name = "fingerprint")]
    pub fn fingerprint(&self) -> Option<String> {
        // if let Some(public_key) = self.0.public_key.as_ref() {
        //     let digest = Ripemd160::digest(Sha256::digest(public_key.serialize().as_slice()));
        //     Some(digest[..4].as_ref().to_hex().into())
        // } else {
        //     None
        // }
        self.0.fingerprint().map(|v| String::try_from(v).unwrap())
    }
}

impl From<PublicKey> for PyPublicKey {
    fn from(value: PublicKey) -> Self {
        PyPublicKey(value)
    }
}

impl From<PyPublicKey> for PublicKey {
    fn from(value: PyPublicKey) -> Self {
        value.0
    }
}

#[pyclass(name = "XOnlyPublicKey")]
pub struct PyXOnlyPublicKey(XOnlyPublicKey);

#[pymethods]
impl PyXOnlyPublicKey {
    #[new]
    pub fn try_new(key: &str) -> PyResult<PyXOnlyPublicKey> {
        let xonly_public_key =
            XOnlyPublicKey::try_new(key).map_err(|err| PyException::new_err(err.to_string()))?;
        // let xonly_public_key = secp256k1::XOnlyPublicKey::from_str(key).map_err(|err| PyException::new_err(format!("{}", err)))?;
        Ok(PyXOnlyPublicKey(xonly_public_key))
    }

    #[pyo3(name = "to_string")]
    pub fn to_string_impl(&self) -> String {
        self.0.inner.to_string()
    }

    #[pyo3(name = "to_address")]
    pub fn to_address(&self, network: PyNetworkType) -> PyResult<PyAddress> {
        let payload = &self.0.inner.serialize();
        let address = Address::new(
            NetworkType::from(network).into(),
            Version::PubKey,
            payload,
        );
        Ok(PyAddress(address))
    }

    #[pyo3(name = "to_address_ecdsa")]
    pub fn to_address_ecdsa(&self, network: PyNetworkType) -> PyResult<PyAddress> {
        let payload = &self.0.inner.serialize();
        let address = Address::new(
            NetworkType::from(network).into(),
            Version::PubKeyECDSA,
            payload,
        );
        Ok(PyAddress(address))
    }

    #[pyo3(name = "from_address")]
    #[staticmethod]
    pub fn from_address(address: PyAddress) -> PyResult<PyXOnlyPublicKey> {
        let xonly_public_key = XOnlyPublicKey::from_address(&address.into())
            .map_err(|err| PyException::new_err(err.to_string()))?;
        // let xonly_public_key = secp256k1::XOnlyPublicKey::from_slice(&address.payload)
        //     .map_err(|err| PyException::new_err(format!("{}", err)))?;
        Ok(xonly_public_key.into())
    }
}

impl From<XOnlyPublicKey> for PyXOnlyPublicKey {
    fn from(value: XOnlyPublicKey) -> Self {
        PyXOnlyPublicKey(value)
    }
}

use super::publickey::PyPublicKey;
use crate::{address::PyAddress, wallet::keys::keypair::PyKeypair};
use kaspa_addresses::{Address, Version};
use kaspa_consensus_core::network::NetworkType;
use kaspa_wallet_keys::privatekey::PrivateKey;
use pyo3::{exceptions::PyException, prelude::*};
use std::str::FromStr;

#[pyclass(name = "PrivateKey")]
#[derive(Clone)]
pub struct PyPrivateKey(PrivateKey);

impl PyPrivateKey {
    pub fn new(key: PrivateKey) -> Self {
        Self(key)
    }

    pub fn inner(&self) -> &PrivateKey {
        &self.0
    }
}

#[pymethods]
impl PyPrivateKey {
    #[new]
    pub fn try_new(key: &str) -> PyResult<PyPrivateKey> {
        let private_key =
            PrivateKey::try_new(key).map_err(|err| PyException::new_err(format!("{}", err)))?;
        Ok(PyPrivateKey(private_key))
    }

    #[pyo3(name = "to_string")]
    pub fn to_hex(&self) -> String {
        self.0.to_hex()
    }

    pub fn to_public_key(&self) -> PyResult<PyPublicKey> {
        let public_key = self
            .0
            .to_public_key()
            .map_err(|_| PyException::new_err("Failed to derive public key"))?;

        Ok(public_key.into())
    }

    pub fn to_address(&self, network: &str) -> PyResult<PyAddress> {
        let public_key = self
            .0
            .to_public_key()
            .map_err(|_| PyException::new_err("Failed to derive public key"))?;
        let (x_only_public_key, _) = public_key.public_key.unwrap().x_only_public_key();
        let payload = x_only_public_key.serialize();
        let address = Address::new(
            NetworkType::from_str(network)
                .map_err(|err| PyException::new_err(err.to_string()))?
                .into(),
            Version::PubKey,
            &payload,
        );
        Ok(address.into())
    }

    pub fn to_address_ecdsa(&self, network: &str) -> PyResult<PyAddress> {
        let public_key = self
            .0
            .to_public_key()
            .map_err(|_| PyException::new_err("Failed to derive public key"))?;
        let payload = public_key.public_key.unwrap().serialize();
        let address = Address::new(
            NetworkType::from_str(network)
                .map_err(|err| PyException::new_err(err.to_string()))?
                .into(),
            Version::PubKeyECDSA,
            &payload,
        );
        Ok(address.into())
    }

    pub fn to_keypair(&self) -> PyResult<PyKeypair> {
        PyKeypair::from_private_key(self).map_err(|err| PyException::new_err(err.to_string()))
    }
}

impl From<PyPrivateKey> for PrivateKey {
    fn from(value: PyPrivateKey) -> Self {
        value.0
    }
}

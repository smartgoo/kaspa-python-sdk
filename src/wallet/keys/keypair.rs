use super::privatekey::PyPrivateKey;
use crate::{address::PyAddress, consensus::core::network::PyNetworkType};
use kaspa_addresses::{Address, Version};
use kaspa_consensus_core::network::NetworkType;
use kaspa_wallet_keys::{privatekey::PrivateKey, publickey::PublicKey};
use pyo3::{exceptions::PyException, prelude::*};
use std::str::FromStr;
use zeroize::Zeroize;

#[pyclass(name = "Keypair")]
pub struct PyKeypair {
    secret_key: secp256k1::SecretKey,
    public_key: secp256k1::PublicKey,
    xonly_public_key: secp256k1::XOnlyPublicKey,
}

#[pymethods]
impl PyKeypair {
    #[new]
    pub fn new(secret_key: &str, public_key: &str, xonly_public_key: &str) -> PyResult<Self> {
        let secret_key = secp256k1::SecretKey::from_str(secret_key)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let public_key = secp256k1::PublicKey::from_str(public_key)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let xonly_public_key = secp256k1::XOnlyPublicKey::from_str(xonly_public_key)
            .map_err(|err| PyException::new_err(err.to_string()))?;

        Ok(PyKeypair {
            secret_key,
            public_key,
            xonly_public_key,
        })
    }

    #[getter]
    #[pyo3(name = "xonly_public_key")]
    pub fn get_xonly_public_key(&self) -> String {
        self.xonly_public_key.to_string()
    }

    #[getter]
    #[pyo3(name = "public_key")]
    pub fn get_public_key(&self) -> String {
        PublicKey::from(&self.public_key).to_string()
    }

    #[getter]
    #[pyo3(name = "private_key")]
    pub fn get_private_key(&self) -> String {
        PrivateKey::from(&self.secret_key).to_hex()
    }

    #[pyo3(name = "to_address")]
    pub fn to_address(&self, network: PyNetworkType) -> PyResult<PyAddress> {
        let payload = &self.xonly_public_key.serialize();
        let address = Address::new(NetworkType::from(network).into(), Version::PubKey, payload);
        Ok(address.into())
    }

    #[pyo3(name = "to_address_ecdsa")]
    pub fn to_address_ecdsa(&self, network: PyNetworkType) -> PyResult<PyAddress> {
        let payload = &self.public_key.serialize();
        let address = Address::new(
            NetworkType::from(network).into(),
            Version::PubKeyECDSA,
            payload,
        );
        Ok(address.into())
    }

    #[staticmethod]
    #[pyo3(name = "random")]
    pub fn random() -> PyResult<PyKeypair> {
        let secp = secp256k1::Secp256k1::new();
        let (secret_key, public_key) = secp.generate_keypair(&mut rand::thread_rng());
        let (xonly_public_key, _) = public_key.x_only_public_key();
        Ok(PyKeypair {
            secret_key,
            public_key,
            xonly_public_key,
        })
    }

    #[staticmethod]
    #[pyo3(name = "from_private_key")]
    pub fn from_private_key(private_key: &PyPrivateKey) -> PyResult<PyKeypair> {
        let secp = secp256k1::Secp256k1::new();
        let mut key_bytes = private_key.secret_bytes();
        let secret_key = secp256k1::SecretKey::from_slice(&key_bytes)
            .map_err(|e| PyException::new_err(format!("{e}")))?;
        key_bytes.zeroize();
        let public_key = secp256k1::PublicKey::from_secret_key(&secp, &secret_key);
        let (xonly_public_key, _) = public_key.x_only_public_key();
        Ok(PyKeypair {
            secret_key,
            public_key,
            xonly_public_key,
        })
    }
}

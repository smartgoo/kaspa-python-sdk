use crate::wallet::keys::derivation::PyDerivationPath;
use crate::wallet::keys::{privatekey::PyPrivateKey, xpub::PyXPub};
use kaspa_bip32::Error;
use kaspa_bip32::{ChildNumber, ExtendedPrivateKey};
use kaspa_utils::hex::FromHex;
use kaspa_wallet_keys::prelude::PrivateKey;
use kaspa_wallet_keys::xpub::XPub;
use pyo3::{exceptions::PyException, prelude::*};
use secp256k1::SecretKey;
use std::str::FromStr;
use workflow_core::hex::ToHex;

#[pyclass(name = "XPrv")]
#[derive(Clone)]
pub struct PyXPrv {
    inner: ExtendedPrivateKey<SecretKey>,
}

#[pymethods]
impl PyXPrv {
    #[new]
    fn try_new_py(seed: &str) -> PyResult<PyXPrv> {
        let seed_bytes = Vec::<u8>::from_hex(seed)
            .map_err(|e| PyErr::new::<PyException, _>(format!("{}", e)))?;

        let inner = ExtendedPrivateKey::<SecretKey>::new(seed_bytes)
            .map_err(|err: Error| PyException::new_err(err.to_string()))?;
        Ok(Self { inner })
    }

    #[staticmethod]
    #[pyo3(name = "from_xprv")]
    pub fn from_xprv_str_py(xprv: &str) -> PyResult<PyXPrv> {
        Ok(Self {
            inner: ExtendedPrivateKey::<SecretKey>::from_str(xprv)
                .map_err(|err| PyException::new_err(err.to_string()))?,
        })
    }

    #[pyo3(name = "derive_child")]
    #[pyo3(signature = (child_number, hardened=None))]
    pub fn derive_child_py(&self, child_number: u32, hardened: Option<bool>) -> PyResult<PyXPrv> {
        let child_number = ChildNumber::new(child_number, hardened.unwrap_or(false))
            .map_err(|err: Error| PyException::new_err(err.to_string()))?;
        let inner = self
            .inner
            .derive_child(child_number)
            .map_err(|err: Error| PyException::new_err(err.to_string()))?;
        Ok(Self { inner })
    }

    #[pyo3(name = "derive_path")]
    pub fn derive_path_py(&self, path: &Bound<PyAny>) -> PyResult<PyXPrv> {
        let path = if let Ok(path_str) = path.extract::<String>() {
            Ok(PyDerivationPath::new_py(path_str.as_str())?)
        } else if let Ok(path_obj) = path.extract::<PyDerivationPath>() {
            Ok(path_obj)
        } else {
            Err(PyException::new_err(
                "`path` must be of type `str` or `DerivationPath`",
            ))
        }?;

        let inner = self
            .inner
            .clone()
            .derive_path(&(path).into())
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(Self { inner })
    }

    #[allow(clippy::wrong_self_convention)]
    #[pyo3(name = "into_string")]
    pub fn into_string_py(&self, prefix: &str) -> PyResult<String> {
        let str = self
            .inner
            .to_extended_key(
                prefix
                    .try_into()
                    .map_err(|err: Error| PyException::new_err(err.to_string()))?,
            )
            .to_string();
        Ok(str)
    }

    #[pyo3(name = "to_string")]
    pub fn to_string_py(&self) -> PyResult<String> {
        let str = self
            .inner
            .to_extended_key(
                "kprv"
                    .try_into()
                    .map_err(|err: Error| PyException::new_err(err.to_string()))?,
            )
            .to_string();
        Ok(str)
    }

    #[pyo3(name = "to_xpub")]
    pub fn to_xpub_py(&self) -> PyResult<PyXPub> {
        let public_key = self.inner.public_key();
        let inner = XPub::from(public_key);
        Ok(PyXPub(inner))
    }

    #[pyo3(name = "to_private_key")]
    pub fn to_private_key_py(&self) -> PyResult<PyPrivateKey> {
        let private_key = self.inner.private_key();
        let inner = PrivateKey::from(private_key);
        Ok(PyPrivateKey(inner))
    }

    #[getter]
    #[pyo3(name = "xprv")]
    pub fn xprv_py(&self) -> PyResult<String> {
        let str = self
            .inner
            .to_extended_key(
                "kprv"
                    .try_into()
                    .map_err(|err: Error| PyException::new_err(err.to_string()))?,
            )
            .to_string();
        Ok(str)
    }

    #[getter]
    #[pyo3(name = "private_key")]
    pub fn private_key_as_hex_string_py(&self) -> String {
        use kaspa_bip32::PrivateKey;
        self.inner.private_key().to_bytes().to_vec().to_hex()
    }

    #[getter]
    #[pyo3(name = "depth")]
    pub fn depth_py(&self) -> u8 {
        self.inner.attrs().depth
    }

    #[getter]
    #[pyo3(name = "parent_fingerprint")]
    pub fn parent_fingerprint_as_hex_string_py(&self) -> String {
        self.inner.attrs().parent_fingerprint.to_vec().to_hex()
    }

    #[getter]
    #[pyo3(name = "child_number")]
    pub fn child_number_py(&self) -> u32 {
        self.inner.attrs().child_number.into()
    }

    #[getter]
    #[pyo3(name = "chain_code")]
    pub fn chain_code_as_hex_string_py(&self) -> String {
        self.inner.attrs().chain_code.to_vec().to_hex()
    }
}

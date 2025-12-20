use crate::wallet::keys::publickey::PyPublicKey;
use kaspa_bip32::Error as Bip32Error;
use kaspa_bip32::{ChildNumber, ExtendedPublicKey};
use kaspa_wallet_keys::prelude::DerivationPath;
use kaspa_wallet_keys::{prelude::PublicKey, xpub::XPub};
use pyo3::{exceptions::PyException, prelude::*};
use std::str::FromStr;
use workflow_core::hex::ToHex;

#[pyclass(name = "XPub")]
#[derive(Clone)]
pub struct PyXPub(pub XPub);

#[pymethods]
impl PyXPub {
    #[new]
    pub fn try_new(xpub: &str) -> PyResult<PyXPub> {
        let inner = XPub::from(
            ExtendedPublicKey::<secp256k1::PublicKey>::from_str(xpub)
                .map_err(|err| PyException::new_err(err.to_string()))?,
        );
        Ok(PyXPub(inner))
    }

    #[pyo3(name = "derive_child")]
    #[pyo3(signature = (child_number, hardened = None))]
    pub fn derive_child(&self, child_number: u32, hardened: Option<bool>) -> PyResult<PyXPub> {
        let child_number = ChildNumber::new(child_number, hardened.unwrap_or(false))
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let inner = XPub::from(
            self.0
                .inner()
                .derive_child(child_number)
                .map_err(|err| PyException::new_err(err.to_string()))?,
        );
        Ok(PyXPub(inner))
    }

    #[pyo3(name = "derive_path")]
    pub fn derive_path(&self, path: &str) -> PyResult<PyXPub> {
        let path =
            DerivationPath::new(path).map_err(|err| PyException::new_err(err.to_string()))?;
        // let inner = self.0.inner().clone().derive_path((&path).into())
        //     .map_err(|err| PyException::new_err(err.to_string()))?;
        let inner = XPub::from(
            self.0
                .inner()
                .clone()
                .derive_path((&path).into())
                .map_err(|err| PyException::new_err(err.to_string()))?,
        );
        Ok(PyXPub(inner))
    }

    #[pyo3(name = "into_string")]
    pub fn to_str(&self, prefix: &str) -> PyResult<String> {
        Ok(self.0.inner().to_string(Some(
            prefix
                .try_into()
                .map_err(|err: Bip32Error| PyException::new_err(err.to_string()))?,
        )))
    }

    #[pyo3(name = "to_public_key")]
    pub fn public_key(&self) -> PyPublicKey {
        let inner: PublicKey = self.0.inner().public_key().into();
        PyPublicKey(inner)
    }

    #[getter]
    #[pyo3(name = "xpub")]
    pub fn xpub(&self) -> PyResult<String> {
        let str = self
            .0
            .inner()
            .to_extended_key("kpub".try_into().unwrap())
            .to_string();
        Ok(str)
    }

    #[getter]
    #[pyo3(name = "depth")]
    pub fn depth(&self) -> u8 {
        self.0.inner().attrs().depth
    }

    #[getter]
    #[pyo3(name = "parent_fingerprint")]
    pub fn parent_fingerprint_as_hex_string(&self) -> String {
        self.0.inner().attrs().parent_fingerprint.to_vec().to_hex()
    }

    #[getter]
    #[pyo3(name = "child_number")]
    pub fn child_number(&self) -> u32 {
        self.0.inner().attrs().child_number.into()
    }

    #[getter]
    #[pyo3(name = "chain_code")]
    pub fn chain_code_as_hex_string(&self) -> String {
        self.0.inner().attrs().chain_code.to_vec().to_hex()
    }
}

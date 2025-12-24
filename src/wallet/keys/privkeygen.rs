use kaspa_bip32::{ChildNumber, ExtendedPrivateKey};
use kaspa_wallet_keys::{
    derivation::gen1::WalletDerivationManager, prelude::PrivateKey, xprv::XPrv,
};
use pyo3::{exceptions::PyException, prelude::*};
use secp256k1::SecretKey;

use crate::wallet::keys::privatekey::PyPrivateKey;

#[pyclass(name = "PrivateKeyGenerator")]
#[derive(Clone)]
pub struct PyPrivateKeyGenerator {
    receive: ExtendedPrivateKey<SecretKey>,
    change: ExtendedPrivateKey<SecretKey>,
}

#[pymethods]
impl PyPrivateKeyGenerator {
    #[new]
    #[pyo3(signature = (xprv, is_multisig, account_index, cosigner_index=None))]
    pub fn new(
        xprv: String,
        is_multisig: bool,
        account_index: u64,
        cosigner_index: Option<u32>,
    ) -> PyResult<PyPrivateKeyGenerator> {
        let xprv =
            XPrv::from_xprv_str(xprv).map_err(|err| PyException::new_err(err.to_string()))?;
        let xprv = xprv.inner();
        let receive = xprv
            .clone()
            .derive_path(
                &WalletDerivationManager::build_derivate_path(
                    is_multisig,
                    account_index,
                    cosigner_index,
                    Some(kaspa_bip32::AddressType::Receive),
                )
                .map_err(|err| PyException::new_err(err.to_string()))?,
            )
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let change = xprv
            .clone()
            .derive_path(
                &WalletDerivationManager::build_derivate_path(
                    is_multisig,
                    account_index,
                    cosigner_index,
                    Some(kaspa_bip32::AddressType::Change),
                )
                .map_err(|err| PyException::new_err(err.to_string()))?,
            )
            .map_err(|err| PyException::new_err(err.to_string()))?;

        Ok(Self { receive, change })
    }

    pub fn receive_key(&self, index: u32) -> PyResult<PyPrivateKey> {
        let xkey = self
            .receive
            .derive_child(
                ChildNumber::new(index, false)
                    .map_err(|err| PyException::new_err(err.to_string()))?,
            )
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let inner = PrivateKey::from(xkey.private_key());
        Ok(PyPrivateKey::new(inner))
    }

    pub fn change_key(&self, index: u32) -> PyResult<PyPrivateKey> {
        let xkey = self
            .change
            .derive_child(
                ChildNumber::new(index, false)
                    .map_err(|err| PyException::new_err(err.to_string()))?,
            )
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let inner = PrivateKey::from(xkey.private_key());
        Ok(PyPrivateKey::new(inner))
    }
}

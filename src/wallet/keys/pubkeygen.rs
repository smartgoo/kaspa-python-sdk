use kaspa_addresses::Address;
use kaspa_consensus_core::network::NetworkType;
use kaspa_wallet_core::derivation::WalletDerivationManagerTrait;
use kaspa_wallet_keys::publickey::PublicKey;
use kaspa_wallet_keys::result::Result;
use kaspa_wallet_keys::{derivation::gen1::WalletDerivationManager, xprv::XPrv, xpub::XPub};
use pyo3::{exceptions::PyException, prelude::*};

use crate::consensus::core::network::PyNetworkType;
use crate::{address::PyAddress, wallet::keys::publickey::PyPublicKey};

#[pyclass(name = "PublicKeyGenerator")]
#[derive(Clone)]
pub struct PyPublicKeyGenerator {
    hd_wallet: WalletDerivationManager,
}

#[pymethods]
impl PyPublicKeyGenerator {
    #[staticmethod]
    #[pyo3(name = "from_xpub")]
    #[pyo3(signature = (kpub, cosigner_index=None))]
    fn from_xpub(kpub: &str, cosigner_index: Option<u32>) -> PyResult<PyPublicKeyGenerator> {
        let kpub = XPub::try_new(kpub).map_err(|err| PyException::new_err(err.to_string()))?;
        let xpub = kpub.inner();
        let hd_wallet =
            WalletDerivationManager::from_extended_public_key(xpub.clone(), cosigner_index)
                .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(Self { hd_wallet })
    }

    #[staticmethod]
    #[pyo3(name = "from_master_xprv")]
    #[pyo3(signature = (xprv, is_multisig, account_index, cosigner_index=None))]
    fn from_master_xprv(
        xprv: &str,
        is_multisig: bool,
        account_index: u64,
        cosigner_index: Option<u32>,
    ) -> PyResult<PyPublicKeyGenerator> {
        let path =
            WalletDerivationManager::build_derivate_path(is_multisig, account_index, None, None)
                .map_err(|err| PyException::new_err(err.to_string()))?;
        let xprv = XPrv::from_xprv_str(xprv)
            .map_err(|err| PyException::new_err(err.to_string()))?
            .inner()
            .clone()
            .derive_path(&path)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let xpub = xprv.public_key();
        let hd_wallet = WalletDerivationManager::from_extended_public_key(xpub, cosigner_index)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(Self { hd_wallet })
    }

    #[pyo3(name = "receive_pubkeys")]
    fn receive_pubkeys(&self, mut start: u32, mut end: u32) -> PyResult<Vec<PyPublicKey>> {
        if start > end {
            (start, end) = (end, start)
        }
        let pubkeys = self
            .hd_wallet
            .receive_pubkey_manager()
            .derive_pubkey_range(start..end)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(pubkeys
            .into_iter()
            .map(|pk| PyPublicKey(PublicKey::from(pk)))
            .collect())
    }

    #[pyo3(name = "receive_pubkey")]
    pub fn receive_pubkey(&self, index: u32) -> PyResult<PyPublicKey> {
        let inner = self
            .hd_wallet
            .receive_pubkey_manager()
            .derive_pubkey(index)
            .map_err(|err| PyException::new_err(err.to_string()))?
            .into();

        Ok(PyPublicKey(inner))
    }

    #[pyo3(name = "receive_pubkeys_as_strings")]
    fn receive_pubkeys_as_strings(&self, mut start: u32, mut end: u32) -> PyResult<Vec<String>> {
        if start > end {
            (start, end) = (end, start);
        }
        let pubkeys = self
            .hd_wallet
            .receive_pubkey_manager()
            .derive_pubkey_range(start..end)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(pubkeys
            .into_iter()
            .map(|pk| PublicKey::from(pk).to_string())
            .collect())
    }

    #[pyo3(name = "receive_pubkey_as_string")]
    pub fn receive_pubkey_as_string(&self, index: u32) -> PyResult<String> {
        Ok(self
            .hd_wallet
            .receive_pubkey_manager()
            .derive_pubkey(index)
            .map_err(|err| PyException::new_err(err.to_string()))?
            .to_string())
    }

    #[pyo3(name = "receive_addresses")]
    fn receive_addresses(
        &self,
        network_type: PyNetworkType,
        mut start: u32,
        mut end: u32,
    ) -> PyResult<Vec<PyAddress>> {
        if start > end {
            (start, end) = (end, start);
        }
        let network_type: NetworkType = network_type.into();
        let pubkeys = self
            .hd_wallet
            .receive_pubkey_manager()
            .derive_pubkey_range(start..end)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let addresses = pubkeys
            .into_iter()
            .map(|pk| PublicKey::from(pk).to_address(network_type))
            .collect::<Result<Vec<Address>>>()
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let addresses = addresses.into_iter().map(PyAddress::from).collect();
        Ok(addresses)
    }

    #[pyo3(name = "receive_address")]
    fn receive_address(&self, network_type: PyNetworkType, index: u32) -> PyResult<PyAddress> {
        let inner = PublicKey::from(
            self.hd_wallet
                .receive_pubkey_manager()
                .derive_pubkey(index)
                .map_err(|err| PyException::new_err(err.to_string()))?,
        )
        .to_address(NetworkType::from(network_type))
        .map_err(|err| PyException::new_err(err.to_string()))?;

        Ok(PyAddress(inner))
    }

    #[pyo3(name = "receive_addresses_as_strings")]
    fn receive_addresses_as_strings(
        &self,
        network_type: PyNetworkType,
        mut start: u32,
        mut end: u32,
    ) -> PyResult<Vec<String>> {
        if start > end {
            (start, end) = (end, start);
        }
        let network_type: NetworkType = network_type.into();
        let pubkeys = self
            .hd_wallet
            .receive_pubkey_manager()
            .derive_pubkey_range(start..end)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let addresses = pubkeys
            .into_iter()
            .map(|pk| PublicKey::from(pk).to_address(network_type))
            .collect::<Result<Vec<Address>>>()
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(addresses
            .into_iter()
            .map(|a| a.address_to_string())
            .collect())
    }

    #[pyo3(name = "receive_address_as_string")]
    fn receive_address_as_string(
        &self,
        network_type: PyNetworkType,
        index: u32,
    ) -> PyResult<String> {
        Ok(PublicKey::from(
            self.hd_wallet
                .receive_pubkey_manager()
                .derive_pubkey(index)
                .map_err(|err| PyException::new_err(err.to_string()))?,
        )
        .to_address(NetworkType::from(network_type))
        .map_err(|err| PyException::new_err(err.to_string()))?
        .to_string())
    }

    #[pyo3(name = "change_pubkeys")]
    pub fn change_pubkeys(&self, mut start: u32, mut end: u32) -> PyResult<Vec<PyPublicKey>> {
        if start > end {
            (start, end) = (end, start);
        }
        let pubkeys = self
            .hd_wallet
            .change_pubkey_manager()
            .derive_pubkey_range(start..end)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let pubkeys = pubkeys
            .into_iter()
            .map(PublicKey::from)
            .collect::<Vec<PublicKey>>();
        let pubkeys = pubkeys.into_iter().map(PyPublicKey::from).collect();
        Ok(pubkeys)
    }

    #[pyo3(name = "change_pubkey")]
    pub fn change_pubkey(&self, index: u32) -> PyResult<PyPublicKey> {
        let inner: PublicKey = self
            .hd_wallet
            .change_pubkey_manager()
            .derive_pubkey(index)
            .map_err(|err| PyException::new_err(err.to_string()))?
            .into();

        Ok(PyPublicKey(inner))
    }

    #[pyo3(name = "change_pubkeys_as_strings")]
    pub fn change_pubkeys_as_strings(&self, mut start: u32, mut end: u32) -> PyResult<Vec<String>> {
        if start > end {
            (start, end) = (end, start);
        }
        let pubkeys = self
            .hd_wallet
            .change_pubkey_manager()
            .derive_pubkey_range(start..end)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(pubkeys
            .into_iter()
            .map(|pk| PublicKey::from(pk).to_string())
            .collect())
    }

    #[pyo3(name = "change_pubkey_as_string")]
    pub fn change_pubkey_as_string(&self, index: u32) -> PyResult<String> {
        Ok(self
            .hd_wallet
            .change_pubkey_manager()
            .derive_pubkey(index)
            .map_err(|err| PyException::new_err(err.to_string()))?
            .to_string())
    }

    #[pyo3(name = "change_addresses")]
    pub fn change_addresses(
        &self,
        network_type: PyNetworkType,
        mut start: u32,
        mut end: u32,
    ) -> PyResult<Vec<PyAddress>> {
        if start > end {
            (start, end) = (end, start);
        }
        let network_type: NetworkType = network_type.into();
        let pubkeys = self
            .hd_wallet
            .receive_pubkey_manager()
            .derive_pubkey_range(start..end)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let addresses = pubkeys
            .into_iter()
            .map(|pk| PublicKey::from(pk).to_address(network_type))
            .collect::<Result<Vec<Address>>>()
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let addresses = addresses.into_iter().map(PyAddress::from).collect();
        Ok(addresses)
    }

    #[pyo3(name = "change_address")]
    pub fn change_address(&self, network_type: PyNetworkType, index: u32) -> PyResult<PyAddress> {
        let inner = PublicKey::from(
            self.hd_wallet
                .change_pubkey_manager()
                .derive_pubkey(index)
                .map_err(|err| PyException::new_err(err.to_string()))?,
        )
        .to_address(NetworkType::from(network_type))
        .map_err(|err| PyException::new_err(err.to_string()))?;

        Ok(PyAddress(inner))
    }

    #[pyo3(name = "change_addresses_as_strings")]
    pub fn change_addresses_as_strings(
        &self,
        network_type: PyNetworkType,
        mut start: u32,
        mut end: u32,
    ) -> PyResult<Vec<String>> {
        if start > end {
            (start, end) = (end, start);
        }
        let network_type: NetworkType = network_type.into();
        let pubkeys = self
            .hd_wallet
            .change_pubkey_manager()
            .derive_pubkey_range(start..end)
            .map_err(|err| PyException::new_err(err.to_string()))?;
        let addresses = pubkeys
            .into_iter()
            .map(|pk| PublicKey::from(pk).to_address(network_type))
            .collect::<Result<Vec<Address>>>()
            .map_err(|err| PyException::new_err(err.to_string()))?;
        Ok(addresses
            .into_iter()
            .map(|a| a.address_to_string())
            .collect())
    }

    #[pyo3(name = "change_address_as_string")]
    pub fn change_address_as_string(
        &self,
        network_type: PyNetworkType,
        index: u32,
    ) -> PyResult<String> {
        Ok(PublicKey::from(
            self.hd_wallet
                .receive_pubkey_manager()
                .derive_pubkey(index)
                .map_err(|err| PyException::new_err(err.to_string()))?,
        )
        .to_address(NetworkType::from(network_type))
        .map_err(|err| PyException::new_err(err.to_string()))?
        .to_string())
    }

    #[pyo3(name = "to_string")]
    pub fn to_string(&self) -> PyResult<String> {
        Ok(self.hd_wallet.to_string(None).to_string())
    }
}

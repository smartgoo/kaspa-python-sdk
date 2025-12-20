use kaspa_consensus_core::network::NetworkType;
use kaspa_wallet_core::{derivation::create_address, prelude::AccountKind};
use kaspa_wallet_keys::publickey::PublicKey;
use pyo3::{exceptions::PyException, prelude::*};
use std::str::FromStr;

use crate::{
    address::PyAddress,
    wallet::{core::account::kind::PyAccountKind, keys::publickey::PyPublicKey},
};

#[pyfunction]
#[pyo3(name = "create_multisig_address")]
#[pyo3(signature = (minimum_signatures, keys, network_type, ecdsa=false, account_kind=None))]
pub fn py_create_multisig_address(
    minimum_signatures: usize,
    keys: Vec<PyPublicKey>,
    network_type: &str,
    ecdsa: Option<bool>,
    account_kind: Option<PyAccountKind>,
) -> PyResult<PyAddress> {
    let network_type =
        NetworkType::from_str(network_type).map_err(|err| PyException::new_err(err.to_string()))?;
    let keys = keys
        .into_iter()
        .map(|pk| PublicKey::from(pk).try_into())
        .collect::<Result<Vec<_>, kaspa_wallet_keys::error::Error>>()
        .map_err(|err| PyException::new_err(err.to_string()))?;
    Ok(create_address(
        minimum_signatures,
        keys,
        network_type.into(),
        ecdsa.unwrap_or(false),
        account_kind.map(AccountKind::from),
    )
    .map_err(|err| PyException::new_err(err.to_string()))?
    .into())
}

use crate::{
    address::PyAddress, consensus::core::script_public_key::PyScriptPublicKey, types::PyBinary,
};
use kaspa_consensus_core::network::NetworkType;
use kaspa_txscript::{script_class::ScriptClass, standard};
use pyo3::{exceptions::PyException, prelude::*};
use std::str::FromStr;
use workflow_core::hex::ToHex;

#[pyfunction]
#[pyo3(name = "pay_to_address_script")]
pub fn py_pay_to_address_script(address: PyAddress) -> PyResult<PyScriptPublicKey> {
    Ok(standard::pay_to_address_script(&address.into()).into())
}

#[pyfunction]
#[pyo3(name = "pay_to_script_hash_script")]
pub fn py_pay_to_script_hash_script(redeem_script: PyBinary) -> PyResult<PyScriptPublicKey> {
    Ok(standard::pay_to_script_hash_script(redeem_script.data.as_slice()).into())
}

#[pyfunction]
#[pyo3(name = "pay_to_script_hash_signature_script")]
pub fn py_pay_to_script_hash_signature_script(
    redeem_script: PyBinary,
    signature: PyBinary,
) -> PyResult<String> {
    let script = standard::pay_to_script_hash_signature_script(redeem_script.data, signature.data)
        .map_err(|err| PyException::new_err(err.to_string()))?;
    Ok(script.to_hex())
}

#[pyfunction]
#[pyo3(name = "address_from_script_public_key")]
pub fn py_address_from_script_public_key(
    script_public_key: PyScriptPublicKey,
    network: &str,
) -> PyResult<PyAddress> {
    match standard::extract_script_pub_key_address(
        &script_public_key.into(),
        NetworkType::from_str(network)
            .map_err(|err| PyException::new_err(err.to_string()))?
            .into(),
    ) {
        Ok(address) => Ok(address.into()),
        Err(err) => Err(pyo3::exceptions::PyException::new_err(format!("{}", err))),
    }
}

#[pyfunction]
#[pyo3(name = "is_script_pay_to_pubkey")]
pub fn py_is_script_pay_to_pubkey(script: PyBinary) -> PyResult<bool> {
    Ok(ScriptClass::is_pay_to_pubkey(script.data.as_slice()))
}

#[pyfunction]
#[pyo3(name = "is_script_pay_to_pubkey_ecdsa")]
pub fn py_is_script_pay_to_pubkey_ecdsa(script: PyBinary) -> PyResult<bool> {
    Ok(ScriptClass::is_pay_to_pubkey_ecdsa(script.data.as_slice()))
}

#[pyfunction]
#[pyo3(name = "is_script_pay_to_script_hash")]
pub fn py_is_script_pay_to_script_hash(script: PyBinary) -> PyResult<bool> {
    Ok(ScriptClass::is_pay_to_script_hash(script.data.as_slice()))
}

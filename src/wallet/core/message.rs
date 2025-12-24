use crate::wallet::keys::{privatekey::PyPrivateKey, publickey::PyPublicKey};
// use kaspa_wallet_core::imports::*;
use kaspa_wallet_core::message::*;
use pyo3::{exceptions::PyException, prelude::*};
use zeroize::Zeroize;

#[pyfunction]
#[pyo3(name = "sign_message")]
#[pyo3(signature = (message, private_key, no_aux_rand=false))]
pub fn py_sign_message(
    message: String,
    private_key: &PyPrivateKey,
    no_aux_rand: bool,
) -> PyResult<String> {
    let mut privkey_bytes = [0u8; 32];
    privkey_bytes.copy_from_slice(&private_key.secret_bytes());
    let pm = PersonalMessage(&message);
    let sign_options = SignMessageOptions { no_aux_rand };
    let sig_vec = sign_message(&pm, &privkey_bytes, &sign_options)
        .map_err(|err| PyException::new_err(format!("{}", err)))?;
    privkey_bytes.zeroize();
    Ok(faster_hex::hex_string(sig_vec.as_slice()))
}

#[pyfunction]
#[pyo3(name = "verify_message")]
pub fn py_verify_message(
    message: String,
    signature: String,
    public_key: PyPublicKey,
) -> PyResult<bool> {
    let pm = PersonalMessage(&message);
    let mut signature_bytes = [0u8; 64];
    faster_hex::hex_decode(signature.as_bytes(), &mut signature_bytes)
        .map_err(|err| PyException::new_err(format!("{}", err)))?;

    Ok(verify_message(
        &pm,
        &signature_bytes.to_vec(),
        &public_key.0.xonly_public_key,
    )
    .is_ok())
}

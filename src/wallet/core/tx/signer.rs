use crate::{
    consensus::{client::transaction::PyTransaction, core::hashing::PySighashType},
    crypto::hashes::PyHash,
    wallet::keys::privatekey::PyPrivateKey,
};
use kaspa_consensus_client::{Transaction, sign_with_multiple_v3};
use kaspa_consensus_core::{
    hashing::{sighash_type::SIG_HASH_ALL, wasm::SighashType},
    sign::{sign_input, verify},
    tx::PopulatedTransaction,
};
use kaspa_hashes::Hash;
use kaspa_wallet_core::result::Result;
use kaspa_wallet_keys::prelude::PrivateKey;
use pyo3::{exceptions::PyException, prelude::*};
use workflow_core::hex::ToHex;
use zeroize::Zeroize;

#[pyfunction(name = "sign_transaction")]
pub fn py_sign_transaction(
    tx: PyTransaction,
    signer: Vec<PyPrivateKey>,
    verify_sig: bool,
) -> PyResult<PyTransaction> {
    let mut private_keys: Vec<[u8; 32]> = vec![];
    for key in signer.iter() {
        private_keys.push(key.0.secret_bytes());
    }

    let transaction: Transaction = tx.into();
    let tx = sign_transaction(&transaction, &private_keys, verify_sig)
        .map_err(|err| PyException::new_err(format!("Unable to sign: {err:?}")))?;
    private_keys.zeroize();
    Ok(tx.clone().into())
}

#[pyfunction]
#[pyo3(name = "create_input_signature")]
#[pyo3(signature = (tx, input_index, private_key, sighash_type=None))]
pub fn py_create_input_signature(
    tx: &PyTransaction,
    input_index: u8,
    private_key: &PyPrivateKey,
    sighash_type: Option<PySighashType>,
) -> PyResult<String> {
    let (cctx, utxos) = tx
        .inner()
        .tx_and_utxos()
        .map_err(|err| PyException::new_err(err.to_string()))?;
    let populated_transaction = PopulatedTransaction::new(&cctx, utxos);

    let sighash_type: SighashType = sighash_type.unwrap_or(PySighashType::All).into();

    let signature = sign_input(
        &populated_transaction,
        input_index.into(),
        &private_key.0.secret_bytes(),
        sighash_type.into(),
    );

    Ok(signature.to_hex())
}

#[pyfunction]
#[pyo3(name = "create_input_signature")]
pub fn py_sign_script_hash(script_hash: String, privkey: &PyPrivateKey) -> PyResult<String> {
    let script_hash = PyHash::try_from(script_hash)?;
    let privkey: PrivateKey = privkey.clone().into();
    let result = sign_hash(script_hash.into(), &(&(privkey)).into())
        .map_err(|err| PyException::new_err(err.to_string()))?;
    Ok(result.to_hex())
}

fn sign_transaction<'a>(
    tx: &'a Transaction,
    private_keys: &[[u8; 32]],
    verify_sig: bool,
) -> Result<&'a Transaction> {
    let tx = sign(tx, private_keys)?;
    if verify_sig {
        let (cctx, utxos) = tx.tx_and_utxos()?;
        let populated_transaction = PopulatedTransaction::new(&cctx, utxos);
        verify(&populated_transaction)?;
    }
    Ok(tx)
}

/// Sign a transaction using schnorr, returns a new transaction with the signatures added.
/// The resulting transaction may be partially signed if the supplied keys are not sufficient
/// to sign all of its inputs.
fn sign<'a>(tx: &'a Transaction, privkeys: &[[u8; 32]]) -> Result<&'a Transaction> {
    Ok(sign_with_multiple_v3(tx, privkeys)?.unwrap())
}

fn sign_hash(sig_hash: Hash, privkey: &[u8; 32]) -> Result<Vec<u8>> {
    let msg = secp256k1::Message::from_digest_slice(sig_hash.as_bytes().as_slice())?;
    let schnorr_key = secp256k1::Keypair::from_seckey_slice(secp256k1::SECP256K1, privkey)?;
    let sig: [u8; 64] = *schnorr_key.sign_schnorr(msg).as_ref();
    let signature = std::iter::once(65u8)
        .chain(sig)
        .chain([SIG_HASH_ALL.to_u8()])
        .collect();
    Ok(signature)
}

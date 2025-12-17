use crate::{
    consensus::client::transaction::PyTransaction, wallet::keys::privatekey::PyPrivateKey,
};
use kaspa_consensus_client::{Transaction, sign_with_multiple_v3};
use kaspa_consensus_core::{sign::verify, tx::PopulatedTransaction};
use kaspa_wallet_core::result::Result;
use pyo3::{exceptions::PyException, prelude::*};
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

pub fn sign_transaction<'a>(
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
pub fn sign<'a>(tx: &'a Transaction, privkeys: &[[u8; 32]]) -> Result<&'a Transaction> {
    Ok(sign_with_multiple_v3(tx, privkeys)?.unwrap())
}

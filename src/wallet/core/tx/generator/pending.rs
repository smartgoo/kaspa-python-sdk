use super::super::super::imports::*;
use crate::{
    consensus::{
        client::{transaction::PyTransaction, utxo::PyUtxoEntryReference},
        core::hashing::PySighashType,
    },
    rpc::wrpc::client::PyRpcClient,
    wallet::keys::privatekey::PyPrivateKey,
};
use kaspa_consensus_client::Transaction;
use kaspa_consensus_core::hashing::wasm::SighashType;
use kaspa_wallet_core::tx::generator as native;
use workflow_core::hex::ToHex;
use zeroize::Zeroize;

#[pyclass]
pub struct PendingTransaction(native::PendingTransaction);

#[pymethods]
impl PendingTransaction {
    #[getter]
    fn id(&self) -> String {
        self.0.id().to_string()
    }

    #[getter]
    #[pyo3(name = "payment_amount")]
    fn payment_value(&self) -> Option<u64> {
        self.0.payment_value()
    }

    #[getter]
    #[pyo3(name = "change_amount")]
    fn change_value(&self) -> u64 {
        self.0.change_value()
    }

    #[getter]
    #[pyo3(name = "fee_amount")]
    fn fees(&self) -> u64 {
        self.0.fees()
    }

    #[getter]
    fn mass(&self) -> u64 {
        self.0.mass()
    }

    #[getter]
    fn minimum_signatures(&self) -> u16 {
        self.0.minimum_signatures()
    }

    #[getter]
    #[pyo3(name = "aggregate_input_amount")]
    fn aggregate_input_value(&self) -> u64 {
        self.0.aggregate_input_value()
    }

    #[getter]
    #[pyo3(name = "aggregate_output_amount")]
    fn aggregate_output_value(&self) -> u64 {
        self.0.aggregate_output_value()
    }

    #[getter]
    #[pyo3(name = "transaction_type")]
    fn kind(&self) -> String {
        if self.0.is_batch() {
            "batch".to_string()
        } else {
            "final".to_string()
        }
    }

    fn addresses(&self) -> Vec<PyAddress> {
        self.0
            .addresses()
            .clone()
            .into_iter()
            .map(PyAddress::from)
            .collect()
    }

    fn get_utxo_entries(&self) -> Vec<PyUtxoEntryReference> {
        self.0
            .utxo_entries()
            .values()
            .map(|utxo_entry| PyUtxoEntryReference::from(utxo_entry.clone()))
            .collect()
    }

    #[pyo3(signature = (input_index, private_key, sighash_type=None))]
    fn create_input_signature(
        &self,
        input_index: u8,
        private_key: &PyPrivateKey,
        sighash_type: Option<PySighashType>,
    ) -> PyResult<String> {
        let sighash_type: SighashType = sighash_type.unwrap_or(PySighashType::All).into();

        let signature = self
            .0
            .create_input_signature(
                input_index.into(),
                &private_key.inner().secret_bytes(),
                sighash_type.into(),
            )
            .map_err(|err| PyException::new_err(format!("{}", err)))?;

        Ok(signature.to_hex())
    }

    fn fill_input(&self, input_index: u8, signature_script: PyBinary) -> PyResult<()> {
        self.0
            .fill_input(input_index.into(), signature_script.into())
            .map_err(|err| PyException::new_err(err.to_string()))?;

        Ok(())
    }

    #[pyo3(signature = (input_index, private_key, sighash_type=None))]
    fn sign_input(
        &self,
        input_index: u8,
        private_key: &PyPrivateKey,
        sighash_type: Option<PySighashType>,
    ) -> PyResult<()> {
        let sighash_type: SighashType = sighash_type.unwrap_or(PySighashType::All).into();

        self.0
            .sign_input(
                input_index.into(),
                &private_key.inner().secret_bytes(),
                sighash_type.into(),
            )
            .map_err(|err| PyException::new_err(format!("{}", err)))?;

        Ok(())
    }

    #[pyo3(signature = (private_keys, check_fully_signed=None))]
    fn sign(
        &self,
        private_keys: Vec<PyPrivateKey>,
        check_fully_signed: Option<bool>,
    ) -> PyResult<()> {
        let mut keys = private_keys
            .iter()
            .map(|key| key.inner().secret_bytes())
            .collect::<Vec<_>>();
        self.0
            .try_sign_with_keys(&keys, check_fully_signed)
            .map_err(|err| PyException::new_err(format!("{}", err)))?;
        keys.zeroize();
        Ok(())
    }

    fn submit<'py>(
        &self,
        py: Python<'py>,
        rpc_client: &PyRpcClient,
    ) -> PyResult<Bound<'py, PyAny>> {
        let inner = self.0.clone();
        let rpc: Arc<DynRpcApi> = rpc_client.client().clone();

        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let txid = inner
                .try_submit(&rpc)
                .await
                .map_err(|err| PyException::new_err(format!("{}", err)))?;
            Ok(txid.to_string())
        })
    }

    #[getter]
    fn transaction(&self) -> PyResult<PyTransaction> {
        Ok(Transaction::from_cctx_transaction(&self.0.transaction(), self.0.utxo_entries()).into())
    }
}

impl From<native::PendingTransaction> for PendingTransaction {
    fn from(pending_transaction: native::PendingTransaction) -> Self {
        Self(pending_transaction)
    }
}

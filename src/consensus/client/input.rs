use crate::{
    consensus::client::{outpoint::PyTransactionOutpoint, utxo::PyUtxoEntryReference},
    types::PyBinary,
};
use kaspa_consensus_client::{TransactionInput, UtxoEntryReference};
use pyo3::prelude::*;
use workflow_core::hex::ToHex;

#[pyclass(name = "TransactionInput")]
#[derive(Clone)]
pub struct PyTransactionInput(TransactionInput);

#[pymethods]
impl PyTransactionInput {
    #[new]
    #[pyo3(signature = (previous_outpoint, signature_script, sequence, sig_op_count, utxo=None))]
    pub fn constructor(
        previous_outpoint: PyTransactionOutpoint,
        signature_script: PyBinary,
        sequence: u64,
        sig_op_count: u8,
        utxo: Option<PyUtxoEntryReference>,
    ) -> PyResult<Self> {
        let inner = TransactionInput::new(
            previous_outpoint.into(),
            Some(signature_script.into()),
            sequence,
            sig_op_count,
            utxo.map(UtxoEntryReference::from),
        );
        Ok(PyTransactionInput(inner))
    }

    #[getter]
    #[pyo3(name = "previous_outpoint")]
    pub fn get_previous_outpoint(&self) -> PyTransactionOutpoint {
        self.0.inner().previous_outpoint.clone().into()
    }

    #[setter]
    #[pyo3(name = "previous_outpoint")]
    pub fn set_previous_outpoint(&mut self, outpoint: PyTransactionOutpoint) -> PyResult<()> {
        self.0.inner().previous_outpoint = outpoint.into();
        Ok(())
    }

    #[getter]
    #[pyo3(name = "signature_script")]
    pub fn get_signature_script_as_hex(&self) -> Option<String> {
        self.0
            .inner()
            .signature_script
            .as_ref()
            .map(|script| script.to_hex())
    }

    #[setter]
    #[pyo3(name = "signature_script")]
    pub fn set_signature_script_as_hex(&mut self, signature_script: PyBinary) -> PyResult<()> {
        self.0.set_signature_script(signature_script.into());
        Ok(())
    }

    #[getter]
    #[pyo3(name = "sequence")]
    pub fn get_sequence(&self) -> u64 {
        self.0.inner().sequence
    }

    #[setter]
    #[pyo3(name = "sequence")]
    pub fn set_sequence(&mut self, sequence: u64) {
        self.0.inner().sequence = sequence;
    }

    #[getter]
    #[pyo3(name = "sig_op_count")]
    pub fn get_sig_op_count(&self) -> u8 {
        self.0.inner().sig_op_count
    }

    #[setter]
    #[pyo3(name = "sig_op_count")]
    pub fn set_sig_op_count(&mut self, sig_op_count: u8) {
        self.0.inner().sig_op_count = sig_op_count;
    }

    #[getter]
    #[pyo3(name = "utxo")]
    pub fn get_utxo(&self) -> Option<PyUtxoEntryReference> {
        self.0.inner().utxo.clone().map(PyUtxoEntryReference::from)
    }
}

impl From<TransactionInput> for PyTransactionInput {
    fn from(value: TransactionInput) -> Self {
        PyTransactionInput(value)
    }
}

impl From<PyTransactionInput> for TransactionInput {
    fn from(value: PyTransactionInput) -> Self {
        value.0
    }
}

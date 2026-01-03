use crate::{
    consensus::client::{outpoint::PyTransactionOutpoint, utxo::PyUtxoEntryReference},
    types::PyBinary,
};
use kaspa_consensus_client::{TransactionInput, UtxoEntryReference};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};
use workflow_core::hex::ToHex;

/// A transaction input referencing a previous output.
///
/// Inputs reference UTXOs (unspent transaction outputs) that are being spent.
#[gen_stub_pyclass]
#[pyclass(name = "TransactionInput")]
#[derive(Clone)]
pub struct PyTransactionInput(TransactionInput);

#[gen_stub_pymethods]
#[pymethods]
impl PyTransactionInput {
    /// Create a new transaction input.
    ///
    /// Args:
    ///     previous_outpoint: Reference to the UTXO being spent.
    ///     signature_script: The unlocking script (signature).
    ///     sequence: Sequence number for relative time locks.
    ///     sig_op_count: Number of signature operations.
    ///     utxo: Optional UTXO entry reference for signing.
    ///
    /// Returns:
    ///     TransactionInput: A new TransactionInput instance.
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
        Ok(Self(inner))
    }

    /// The outpoint referencing the UTXO being spent.
    ///
    /// Returns:
    ///     TransactionOutpoint: The previous output reference.
    #[getter]
    #[pyo3(name = "previous_outpoint")]
    pub fn get_previous_outpoint(&self) -> PyTransactionOutpoint {
        self.0.inner().previous_outpoint.clone().into()
    }

    /// Set the outpoint referencing the UTXO being spent.
    ///
    /// Args:
    ///     outpoint: The previous output reference.
    #[setter]
    #[pyo3(name = "previous_outpoint")]
    pub fn set_previous_outpoint(&mut self, outpoint: PyTransactionOutpoint) -> PyResult<()> {
        self.0.inner().previous_outpoint = outpoint.into();
        Ok(())
    }

    /// The unlocking script (signature) that proves ownership of the UTXO.
    ///
    /// Returns:
    ///     str | None: The signature script as a hex string, or None if not set.
    #[getter]
    #[pyo3(name = "signature_script")]
    pub fn get_signature_script_as_hex(&self) -> Option<String> {
        self.0
            .inner()
            .signature_script
            .as_ref()
            .map(|script| script.to_hex())
    }

    /// Set the unlocking script (signature).
    ///
    /// Args:
    ///     signature_script: The signature script as bytes or hex string.
    #[setter]
    #[pyo3(name = "signature_script")]
    pub fn set_signature_script_as_hex(&mut self, signature_script: PyBinary) -> PyResult<()> {
        self.0.set_signature_script(signature_script.into());
        Ok(())
    }

    /// The sequence number used for relative time locks.
    ///
    /// Returns:
    ///     int: The sequence number.
    #[getter]
    #[pyo3(name = "sequence")]
    pub fn get_sequence(&self) -> u64 {
        self.0.inner().sequence
    }

    /// Set the sequence number.
    ///
    /// Args:
    ///     sequence: The sequence number for relative time locks.
    #[setter]
    #[pyo3(name = "sequence")]
    pub fn set_sequence(&mut self, sequence: u64) {
        self.0.inner().sequence = sequence;
    }

    /// The number of signature operations in this input.
    ///
    /// Returns:
    ///     int: The signature operation count.
    #[getter]
    #[pyo3(name = "sig_op_count")]
    pub fn get_sig_op_count(&self) -> u8 {
        self.0.inner().sig_op_count
    }

    /// Set the signature operation count.
    ///
    /// Args:
    ///     sig_op_count: The number of signature operations.
    #[setter]
    #[pyo3(name = "sig_op_count")]
    pub fn set_sig_op_count(&mut self, sig_op_count: u8) {
        self.0.inner().sig_op_count = sig_op_count;
    }

    /// The UTXO entry reference for transaction signing.
    ///
    /// Returns:
    ///     UtxoEntryReference | None: The UTXO reference, or None if not set.
    #[getter]
    #[pyo3(name = "utxo")]
    pub fn get_utxo(&self) -> Option<PyUtxoEntryReference> {
        self.0.inner().utxo.clone().map(PyUtxoEntryReference::from)
    }
}

impl From<TransactionInput> for PyTransactionInput {
    fn from(value: TransactionInput) -> Self {
        Self(value)
    }
}

impl From<PyTransactionInput> for TransactionInput {
    fn from(value: PyTransactionInput) -> Self {
        value.0
    }
}

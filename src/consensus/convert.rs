use kaspa_consensus_client::{
    Transaction, TransactionInput, TransactionOutpoint, TransactionOutput, UtxoEntry,
    UtxoEntryReference,
};
use kaspa_utils::hex::ToHex;
use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Trait for converting Rust types to Python dictionaries.
///
/// This trait provides a standardized way to convert wrapped SDK types
/// to Python dicts with a flat structure (no unnecessary nesting).
///
/// A custom trait is required as `py: Python` is required fn arg so
/// that dict can be created on the Python heap.
pub trait TryToPyDict {
    /// Convert this value to a Python dictionary.
    ///
    /// # Arguments
    /// * `py` - Python interpreter token
    ///
    /// # Returns
    /// A Python dictionary representation of the value.
    fn try_to_pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>>;
}

// **********************************************
// Trait impls for rusty-kaspa native types
// **********************************************

impl TryToPyDict for TransactionOutpoint {
    fn try_to_pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = serde_pyobject::to_pyobject(py, self.inner())?;
        Ok(dict.cast_into::<PyDict>()?)
    }
}

impl TryToPyDict for UtxoEntryReference {
    fn try_to_pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);

        // Set `address` key
        if let Some(addr) = self.address() {
            dict.set_item("address", addr.to_string())?;
        } else {
            dict.set_item("address", py.None())?;
        }

        // Set `outpoint` key
        dict.set_item(
            "outpoint",
            serde_pyobject::to_pyobject(py, self.outpoint().inner())?,
        )?;

        // Set `amount` key
        dict.set_item("amount", self.amount())?;

        // Set `scriptPublicKey` key
        dict.set_item(
            "scriptPublicKey",
            format!(
                "{:02x}{}",
                self.script_public_key().version(),
                self.script_public_key().script().to_hex()
            ),
        )?;

        // Set `blockDaaScore` key
        dict.set_item("blockDaaScore", self.block_daa_score())?;

        // Set `isCoinbase` key
        dict.set_item("isCoinbase", self.is_coinbase())?;

        Ok(dict)
    }
}

impl TryToPyDict for UtxoEntry {
    fn try_to_pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);

        // Set `address` key
        if let Some(addr) = &self.address {
            dict.set_item("address", addr.to_string())?;
        } else {
            dict.set_item("address", py.None())?;
        }

        // Set `outpoint` key
        dict.set_item(
            "outpoint",
            serde_pyobject::to_pyobject(py, self.outpoint.inner())?,
        )?;

        // Set `amount` key
        dict.set_item("amount", self.amount())?;

        // Set `scriptPublicKey` key
        dict.set_item(
            "scriptPublicKey",
            format!(
                "{:02x}{}",
                self.script_public_key.version(),
                self.script_public_key.script().to_hex()
            ),
        )?;

        // Set `blockDaaScore` key
        dict.set_item("blockDaaScore", self.block_daa_score())?;

        // Set `isCoinbase` key
        dict.set_item("isCoinbase", self.is_coinbase())?;

        Ok(dict)
    }
}

impl TryToPyDict for TransactionInput {
    fn try_to_pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);

        // Set `previousOutpoint` key
        dict.set_item(
            "previousOutpoint",
            self.get_previous_outpoint().try_to_pydict(py)?,
        )?;

        // Set `signatureScript` key
        dict.set_item("signatureScript", self.get_signature_script_as_hex())?;

        // Set `sequence` key
        dict.set_item("sequence", self.get_sequence())?;

        // Set `sigOpCount` key
        dict.set_item("sigOpCount", self.get_sig_op_count())?;

        // Set `utxo` key
        let utxo_dict = self
            .get_utxo()
            .map(|utxo_ref| utxo_ref.try_to_pydict(py))
            .transpose()?;
        dict.set_item("utxo", utxo_dict)?;

        Ok(dict)
    }
}

impl TryToPyDict for TransactionOutput {
    fn try_to_pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let inner = self.inner();
        let dict = PyDict::new(py);

        dict.set_item("value", inner.value)?;
        dict.set_item("scriptPublicKey", inner.script_public_key.script_as_hex())?;

        Ok(dict)
    }
}

impl TryToPyDict for Transaction {
    fn try_to_pydict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let inner = self.inner();
        let dict = PyDict::new(py);

        // Set `id` key
        dict.set_item("id", self.id().to_hex())?;

        // Set `inputs` key
        dict.set_item(
            "inputs",
            inner
                .inputs
                .iter()
                .map(|input| input.try_to_pydict(py))
                .collect::<PyResult<Vec<Bound<'_, PyDict>>>>()?,
        )?;

        // Set `outputs` key
        dict.set_item(
            "outputs",
            inner
                .outputs
                .iter()
                .map(|output| output.try_to_pydict(py))
                .collect::<PyResult<Vec<Bound<'_, PyDict>>>>()?,
        )?;

        // Set `locktime` key
        dict.set_item("lockTime", inner.lock_time)?;

        // Set `subnetworkId` key
        dict.set_item("subnetworkId", inner.subnetwork_id.to_hex())?;

        // Set `gas` key
        dict.set_item("gas", inner.gas)?;

        // Set `payload` key
        dict.set_item("payload", inner.payload.to_hex())?;

        // Set `mass`
        dict.set_item("mass", inner.mass)?;

        Ok(dict)
    }
}

use super::outpoint::PyTransactionOutpoint;
use crate::{address::PyAddress, consensus::core::script_public_key::PyScriptPublicKey};
use kaspa_consensus_client::{UtxoEntries, UtxoEntry, UtxoEntryReference};
use kaspa_utils::hex::FromHex;
use pyo3::{
    exceptions::{PyException, PyKeyError},
    prelude::*,
    types::PyDict,
};
use std::sync::Arc;

#[pyclass(name = "UtxoEntry")]
#[derive(Clone)]
pub struct PyUtxoEntry(pub UtxoEntry);

#[pymethods]
impl PyUtxoEntry {
    #[getter]
    pub fn address(&self) -> Option<PyAddress> {
        self.0.address.clone().map(PyAddress::from)
    }

    #[getter]
    pub fn outpoint(&self) -> PyTransactionOutpoint {
        self.0.outpoint.clone().into()
    }

    #[getter]
    pub fn amount(&self) -> u64 {
        self.0.amount
    }

    #[getter]
    pub fn script_public_key(&self) -> PyScriptPublicKey {
        self.0.script_public_key.clone().into()
    }

    #[getter]
    pub fn block_daa_score(&self) -> u64 {
        self.0.block_daa_score
    }

    #[getter]
    pub fn is_coinbase(&self) -> bool {
        self.0.is_coinbase
    }
}

impl From<PyUtxoEntry> for UtxoEntry {
    fn from(value: PyUtxoEntry) -> Self {
        value.0
    }
}

impl From<UtxoEntry> for PyUtxoEntry {
    fn from(value: UtxoEntry) -> Self {
        PyUtxoEntry(value)
    }
}

#[pyclass(name = "UtxoEntries")]
#[derive(Clone)]
pub struct PyUtxoEntries(Arc<Vec<UtxoEntryReference>>);

#[pymethods]
impl PyUtxoEntries {
    #[getter]
    #[pyo3(name = "items")]
    pub fn get_items_as_py_list(&self) -> Vec<PyUtxoEntryReference> {
        self.0
            .as_ref()
            .clone()
            .into_iter()
            .map(PyUtxoEntryReference::from)
            .collect()
    }

    #[setter]
    #[pyo3(name = "items")]
    pub fn set_items_from_py_list(&mut self, v: Vec<PyUtxoEntryReference>) {
        self.0 = Arc::new(v.iter().map(UtxoEntryReference::from).collect());
    }

    #[pyo3(name = "sort")]
    pub fn sort_py(&mut self) {
        let mut items = (*self.0).clone();
        items.sort_by_key(|e| e.amount());
        self.0 = Arc::new(items);
    }

    #[pyo3(name = "amount")]
    pub fn amount_py(&self) -> u64 {
        self.0.iter().map(|e| e.amount()).sum()
    }
}

#[pyclass(name = "UtxoEntryReference")]
#[derive(Clone)]
pub struct PyUtxoEntryReference(pub UtxoEntryReference);

#[pymethods]
impl PyUtxoEntryReference {
    #[getter]
    pub fn entry(&self) -> PyUtxoEntry {
        self.0.as_ref().clone().into()
    }

    #[getter]
    pub fn outpoint(&self) -> PyTransactionOutpoint {
        self.0.utxo.outpoint.clone().into()
    }

    #[getter]
    pub fn address(&self) -> Option<PyAddress> {
        self.0.utxo.address.clone().map(PyAddress::from)
    }

    #[getter]
    pub fn amount(&self) -> u64 {
        self.0.utxo.amount
    }

    #[getter]
    pub fn is_coinbase(&self) -> bool {
        self.0.utxo.is_coinbase
    }

    #[getter]
    pub fn block_daa_score(&self) -> u64 {
        self.0.utxo.block_daa_score
    }

    #[getter]
    pub fn script_public_key(&self) -> PyScriptPublicKey {
        self.0.utxo.script_public_key.clone().into()
    }
}

impl From<PyUtxoEntryReference> for UtxoEntryReference {
    fn from(value: PyUtxoEntryReference) -> Self {
        value.0
    }
}

impl From<&PyUtxoEntryReference> for UtxoEntryReference {
    fn from(value: &PyUtxoEntryReference) -> Self {
        value.0.clone()
    }
}

impl From<UtxoEntryReference> for PyUtxoEntryReference {
    fn from(value: UtxoEntryReference) -> Self {
        PyUtxoEntryReference(value)
    }
}

impl TryFrom<&Bound<'_, PyDict>> for PyUtxoEntryReference {
    type Error = PyErr;
    fn try_from(dict: &Bound<PyDict>) -> PyResult<Self> {
        let address = PyAddress::try_from(
            dict.get_item("address")?
                .ok_or_else(|| PyKeyError::new_err("Key `address` not present"))?
                .extract::<String>()?,
        )?;

        let outpoint = PyTransactionOutpoint::try_from(
            dict.get_item("outpoint")?
                .ok_or_else(|| PyKeyError::new_err("Key `outpoint` not present"))?
                .cast::<PyDict>()?,
        )?;

        let utxo_entry_value = dict
            .get_item("utxoEntry")?
            .ok_or_else(|| PyKeyError::new_err("Key `utxoEntry` not present"))?;
        let utxo_entry = utxo_entry_value.cast::<PyDict>()?;

        let amount: u64 = utxo_entry
            .get_item("amount")?
            .ok_or_else(|| PyKeyError::new_err("Key `amount` not present"))?
            .extract()?;

        let script_public_key = PyScriptPublicKey::from_hex(
            utxo_entry
                .get_item("scriptPublicKey")?
                .ok_or_else(|| PyKeyError::new_err("Key `scriptPublicKey` not present"))?
                .extract::<&str>()?,
        )
        .map_err(|err| PyException::new_err(format!("{}", err)))?;

        let block_daa_score: u64 = utxo_entry
            .get_item("blockDaaScore")?
            .ok_or_else(|| PyKeyError::new_err("Key `blockDaaScore` not present"))?
            .extract()?;

        let is_coinbase: bool = utxo_entry
            .get_item("isCoinbase")?
            .ok_or_else(|| PyKeyError::new_err("Key `is_coinbase` not present"))?
            .extract()?;

        let utxo = UtxoEntry {
            address: Some(address.into()),
            outpoint: outpoint.into(),
            amount,
            script_public_key: script_public_key.into(),
            block_daa_score,
            is_coinbase,
        };

        let inner = UtxoEntryReference {
            utxo: Arc::new(utxo),
        };

        Ok(PyUtxoEntryReference(inner))
    }
}

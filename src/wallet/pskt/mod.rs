use std::sync::{Arc, Mutex};

use kaspa_consensus_client::Transaction;
use kaspa_wallet_pskt::{pskt::Inner, wasm::pskt::State};
use pyo3::prelude::*;
use pyo3_stub_gen::derive::*;

use crate::consensus::client::transaction::PyTransaction;

/// Partially Signed Kaspa Transaction
#[gen_stub_pyclass]
#[pyclass(name = "PSKT")]
#[derive(Clone)]
pub struct PyPSKT {
    state: Arc<Mutex<Option<State>>>,
}

#[gen_stub_pymethods]
#[pymethods]
impl PyPSKT {
    #[new]
    pub fn new(payload: Bound<'_, PyAny>) -> PyResult<Self> {
        let payload = if let Ok(p) = payload.cast::<String>() {
            Inner::from(s)
        } else if let Ok(p) = payload.cast::<PyTransaction>() {
            let tx: Transaction = p.into();
            let inner: Inner = tx.into();
            inner
        };

        Ok(())
    }
}
// use crate::imports::*;
use kaspa_wallet_core::tx::generator as core;
use pyo3::prelude::*;

///
/// A class containing a summary produced by transaction {@link Generator}.
/// This class contains the number of transactions, the aggregated fees,
/// the aggregated UTXOs and the final transaction amount that includes
/// both network and QoS (priority) fees.
///
/// @see {@link createTransactions}, {@link IGeneratorSettingsObject}, {@link Generator}
/// @category Wallet SDK
///
#[pyclass]
pub struct PyGeneratorSummary(core::GeneratorSummary);

#[pymethods]
impl PyGeneratorSummary {
    #[getter]
    pub fn network_type(&self) -> String {
        self.0.network_type().to_string()
    }

    #[getter]
    #[pyo3(name = "utxos")]
    pub fn aggregated_utxos(&self) -> usize {
        self.0.aggregated_utxos()
    }

    #[getter]
    #[pyo3(name = "fees")]
    pub fn aggregate_fees(&self) -> u64 {
        self.0.aggregate_fees()
    }

    #[getter]
    #[pyo3(name = "transactions")]
    pub fn number_of_generated_transactions(&self) -> usize {
        self.0.number_of_generated_transactions()
    }

    #[getter]
    #[pyo3(name = "final_amount")]
    pub fn final_transaction_amount(&self) -> Option<u64> {
        self.0.final_transaction_amount()
    }

    #[getter]
    #[pyo3(name = "final_transaction_id")]
    pub fn final_transaction_id(&self) -> Option<String> {
        self.0.final_transaction_id().map(|id| id.to_string())
    }
}

impl From<core::GeneratorSummary> for PyGeneratorSummary {
    fn from(inner: core::GeneratorSummary) -> Self {
        Self(inner)
    }
}

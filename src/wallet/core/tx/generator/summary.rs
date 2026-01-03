// use crate::imports::*;
use kaspa_wallet_core::tx::generator as core;
use pyo3::prelude::*;
use pyo3_stub_gen::derive::{gen_stub_pyclass, gen_stub_pymethods};

/// A class containing a summary produced by transaction Generator.
///
/// This class contains the number of transactions, the aggregated fees,
/// the aggregated UTXOs and the final transaction amount that includes
/// both network and QoS (priority) fees.
///
/// See Also:
///     create_transactions, Generator
#[gen_stub_pyclass]
#[pyclass(name = "GeneratorSummary")]
pub struct PyGeneratorSummary(core::GeneratorSummary);

#[gen_stub_pymethods]
#[pymethods]
impl PyGeneratorSummary {
    /// The network type used for generation.
    ///
    /// Returns:
    ///     str: The network type string.
    #[getter]
    pub fn network_type(&self) -> String {
        self.0.network_type().to_string()
    }

    /// The total number of UTXOs consumed.
    ///
    /// Returns:
    ///     int: The UTXO count.
    #[getter]
    #[pyo3(name = "utxos")]
    pub fn aggregated_utxos(&self) -> usize {
        self.0.aggregated_utxos()
    }

    /// The total fees across all generated transactions in sompi.
    ///
    /// Returns:
    ///     int: The aggregate fee amount.
    #[getter]
    #[pyo3(name = "fees")]
    pub fn aggregate_fees(&self) -> u64 {
        self.0.aggregate_fees()
    }

    /// The number of transactions generated.
    ///
    /// Returns:
    ///     int: The transaction count.
    #[getter]
    #[pyo3(name = "transactions")]
    pub fn number_of_generated_transactions(&self) -> usize {
        self.0.number_of_generated_transactions()
    }

    /// The final transaction amount in sompi.
    ///
    /// Returns:
    ///     int | None: The final amount, or None if not applicable.
    #[getter]
    #[pyo3(name = "final_amount")]
    pub fn final_transaction_amount(&self) -> Option<u64> {
        self.0.final_transaction_amount()
    }

    /// The ID of the final transaction.
    ///
    /// Returns:
    ///     str | None: The transaction ID, or None if not yet generated.
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

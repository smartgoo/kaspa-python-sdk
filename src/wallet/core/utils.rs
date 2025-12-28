use kaspa_wrpc_client::prelude::NetworkType;
use pyo3::{exceptions::PyException, prelude::*};
use std::str::FromStr;

use crate::consensus::core::network::PyNetworkType;

#[pyfunction]
#[pyo3(name = "kaspa_to_sompi")]
pub fn py_kaspa_to_sompi(kaspa: f64) -> u64 {
    kaspa_wallet_core::utils::kaspa_to_sompi(kaspa)
}

#[pyfunction]
#[pyo3(name = "sompi_to_kaspa")]
pub fn py_sompi_to_kaspa(sompi: u64) -> f64 {
    kaspa_wallet_core::utils::sompi_to_kaspa(sompi)
}

#[pyfunction]
#[pyo3(name = "sompi_to_kaspa_string_with_suffix")]
pub fn py_sompi_to_kaspa_string_with_suffix(sompi: u64, network: PyNetworkType) -> PyResult<String> {
    Ok(kaspa_wallet_core::utils::sompi_to_kaspa_string_with_suffix(
        sompi,
        &network.into(),
    ))
}

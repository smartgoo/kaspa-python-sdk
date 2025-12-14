mod address;
mod network;
mod rpc;

use pyo3::prelude::*;

#[pymodule]
fn kaspa(m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    m.add_class::<address::PyAddress>()?;

    m.add_class::<network::PyNetworkId>()?;
    m.add_class::<network::PyNetworkType>()?;

    m.add_class::<rpc::wrpc::resolver::PyResolver>()?;
    m.add_class::<rpc::wrpc::client::PyRpcClient>()?;

    Ok(())
}

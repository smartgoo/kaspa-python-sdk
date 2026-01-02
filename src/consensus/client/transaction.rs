use crate::address::PyAddress;
use crate::consensus::client::input::PyTransactionInput;
use crate::consensus::client::output::PyTransactionOutput;
use crate::consensus::core::network::PyNetworkType;
use crate::{consensus::core::tx::PyTransactionId, types::PyBinary};
use kaspa_consensus_client::{Transaction, TransactionInput, TransactionOutput};
use kaspa_consensus_core::network::NetworkType;
use kaspa_consensus_core::subnets;
use kaspa_consensus_core::subnets::SubnetworkId;
use kaspa_consensus_core::tx as cctx;
use kaspa_txscript::extract_script_pub_key_address;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;
use workflow_core::hex::{FromHex, ToHex};

#[pyclass(name = "Transaction")]
#[derive(Clone)]
pub struct PyTransaction(Transaction);

impl PyTransaction {
    pub fn inner(&self) -> &Transaction {
        &self.0
    }
}

#[pymethods]
impl PyTransaction {
    #[pyo3(name = "is_coinbase")]
    pub fn is_coinbase(&self) -> bool {
        self.0.inner().subnetwork_id == subnets::SUBNETWORK_ID_COINBASE
    }

    #[pyo3(name = "finalize")]
    pub fn finalize(&self) -> PyResult<PyTransactionId> {
        let tx: cctx::Transaction = self.into();
        self.0.inner().id = tx.id();
        Ok(self.0.inner().id.into())
    }

    #[getter]
    #[pyo3(name = "id")]
    pub fn id_string(&self) -> String {
        self.0.inner().id.to_string()
    }

    #[new]
    pub fn constructor(
        version: u16,
        inputs: Vec<PyTransactionInput>,
        outputs: Vec<PyTransactionOutput>,
        lock_time: u64,
        subnetwork_id: PyBinary,
        gas: u64,
        payload: PyBinary,
        mass: u64,
    ) -> PyResult<Self> {
        let subnetwork_id: SubnetworkId =
            subnetwork_id.data.as_slice().try_into().map_err(|err| {
                PyException::new_err(format!("subnetwork_id conversion error: {}", err))
            })?;

        let inner = Transaction::new(
            None,
            version,
            inputs.into_iter().map(TransactionInput::from).collect(),
            outputs.into_iter().map(TransactionOutput::from).collect(),
            lock_time,
            subnetwork_id,
            gas,
            payload.into(),
            mass,
        )
        .map_err(|err| PyException::new_err(err.to_string()))?;

        Ok(Self(inner))
    }

    #[getter]
    #[pyo3(name = "inputs")]
    pub fn get_inputs_as_list(&self) -> PyResult<Vec<PyTransactionInput>> {
        Ok(self
            .0
            .inner()
            .inputs
            .clone()
            .into_iter()
            .map(PyTransactionInput::from)
            .collect())
    }

    #[setter]
    #[pyo3(name = "inputs")]
    pub fn set_inputs_from_list(&mut self, v: Vec<PyTransactionInput>) {
        self.0.inner().inputs = v.into_iter().map(TransactionInput::from).collect();
    }

    #[pyo3(name = "addresses")]
    pub fn addresses(&self, network_type: PyNetworkType) -> PyResult<Vec<PyAddress>> {
        let network_type: NetworkType = network_type.into();
        let mut list = std::collections::HashSet::new();
        for input in &self.0.inner().inputs {
            if let Some(utxo) = input.get_utxo() {
                if let Some(address) = &utxo.utxo.address {
                    list.insert(address.clone());
                } else if let Ok(address) = extract_script_pub_key_address(
                    &utxo.utxo.script_public_key,
                    network_type.into(),
                ) {
                    list.insert(address);
                }
            }
        }
        Ok(list.into_iter().map(PyAddress::from).collect())
    }

    #[getter]
    #[pyo3(name = "outputs")]
    pub fn get_outputs_as_list(&self) -> PyResult<Vec<PyTransactionOutput>> {
        Ok(self
            .0
            .inner()
            .outputs
            .clone()
            .into_iter()
            .map(PyTransactionOutput::from)
            .collect())
    }

    #[setter]
    #[pyo3(name = "outputs")]
    pub fn set_outputs_from_list(&mut self, v: Vec<PyTransactionOutput>) {
        self.0.inner().outputs = v.into_iter().map(TransactionOutput::from).collect();
    }

    #[getter]
    #[pyo3(name = "version")]
    pub fn get_version(&self) -> u16 {
        self.0.inner().version
    }

    #[setter]
    #[pyo3(name = "version")]
    pub fn set_version(&mut self, v: u16) {
        self.0.inner().version = v;
    }

    #[getter]
    #[pyo3(name = "lock_time")]
    pub fn get_lock_time(&self) -> u64 {
        self.0.inner().lock_time
    }

    #[setter]
    #[pyo3(name = "lock_time")]
    pub fn set_lock_time(&mut self, v: u64) {
        self.0.inner().lock_time = v;
    }

    #[getter]
    #[pyo3(name = "gas")]
    pub fn get_gas(&self) -> u64 {
        self.0.inner().gas
    }

    #[setter]
    #[pyo3(name = "gas")]
    pub fn set_gas(&mut self, v: u64) {
        self.0.inner().gas = v;
    }

    #[getter]
    #[pyo3(name = "subnetwork_id")]
    pub fn get_subnetwork_id_as_hex(&self) -> String {
        self.0.inner().subnetwork_id.to_string()
    }

    #[setter]
    #[pyo3(name = "subnetwork_id")]
    pub fn set_subnetwork_id_from_value(&mut self, v: &str) -> PyResult<()> {
        let subnetwork_id = Vec::from_hex(v)
            .map_err(|err| PyException::new_err(err.to_string()))?
            .as_slice()
            .try_into()
            .map_err(|err| {
                PyException::new_err(format!("subnetwork_id conversion error: {}", err))
            })?;
        self.0.inner().subnetwork_id = subnetwork_id;
        Ok(())
    }

    #[getter]
    #[pyo3(name = "payload")]
    pub fn get_payload_as_hex_string(&self) -> String {
        self.0.inner().payload.to_hex()
    }

    #[setter]
    #[pyo3(name = "payload")]
    pub fn set_payload_from_value(&mut self, v: PyBinary) {
        self.0.inner().payload = v.into();
    }

    #[getter]
    #[pyo3(name = "mass")]
    pub fn get_mass(&self) -> u64 {
        self.0.inner().mass
    }

    #[setter]
    #[pyo3(name = "mass")]
    pub fn set_mass(&mut self, v: u64) {
        self.0.inner().mass = v;
    }
}

impl From<Transaction> for PyTransaction {
    fn from(value: Transaction) -> Self {
        PyTransaction(value)
    }
}

impl From<PyTransaction> for Transaction {
    fn from(value: PyTransaction) -> Self {
        value.0
    }
}

impl From<&PyTransaction> for cctx::Transaction {
    fn from(value: &PyTransaction) -> Self {
        cctx::Transaction::from(&value.0)
    }
}

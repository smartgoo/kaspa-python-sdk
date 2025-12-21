use kaspa_addresses::Prefix;
use kaspa_consensus_core::network::{NetworkId, NetworkType, NetworkTypeError};
use pyo3::{exceptions::PyException, prelude::*};
use std::str::FromStr;

crate::wrap_unit_enum_for_py!(PyNetworkType, "NetworkType", NetworkType, {
    Mainnet,
    Testnet,
    Devnet,
    Simnet,
});

#[pymethods]
impl PyNetworkType {
    pub fn default_rpc_port(&self) -> u16 {
        NetworkType::from(self).default_rpc_port()
    }

    pub fn default_borsh_rpc_port(&self) -> u16 {
        NetworkType::from(self).default_borsh_rpc_port()
    }

    pub fn default_json_rpc_port(&self) -> u16 {
        NetworkType::from(self).default_json_rpc_port()
    }
}

impl From<&PyNetworkType> for NetworkType {
    fn from(value: &PyNetworkType) -> Self {
        value.clone().into()
    }
}

impl FromStr for PyNetworkType {
    type Err = NetworkTypeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let network_type = NetworkType::from_str(s)?;
        Ok(PyNetworkType::from(network_type))
    }
}

#[derive(Clone)]
#[pyclass(name = "NetworkId")]
pub struct PyNetworkId {
    inner: NetworkId,
}

#[pymethods]
impl PyNetworkId {
    #[new]
    pub fn new(network_id: Bound<PyAny>) -> PyResult<Self> {
        if let Ok(network_id) = network_id.extract::<String>() {
            let inner = NetworkId::from_str(&network_id).unwrap();
            Ok(Self { inner })
        } else if let Ok(network_type) = network_id.extract::<PyNetworkType>() {
            let inner = NetworkId::new(network_type.into());
            Ok(Self { inner })
        } else {
            Err(PyException::new_err(
                "`network_id` must be of type NetworkType or String representation (mainnet, testnet-10, etc)",
            ))
        }
    }

    #[staticmethod]
    pub fn with_suffix(network_type: PyNetworkType, suffix: u32) -> Self {
        let inner = NetworkId::with_suffix(network_type.into(), suffix);
        Self { inner }
    }

    #[getter]
    pub fn network_type(&self) -> PyNetworkType {
        self.inner.network_type.into()
    }

    pub fn is_mainnet(&self) -> bool {
        self.inner.is_mainnet()
    }

    #[getter]
    pub fn suffix(&self) -> Option<u32> {
        self.inner.suffix()
    }

    #[getter]
    pub fn default_p2p_port(&self) -> u16 {
        self.inner.default_p2p_port()
    }

    pub fn to_prefixed(&self) -> String {
        self.inner.to_prefixed()
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.inner.to_string()
    }

    pub fn address_prefix(&self) -> String {
        Prefix::from(self.inner.network_type).to_string()
    }
}

impl From<PyNetworkId> for NetworkId {
    fn from(value: PyNetworkId) -> Self {
        NetworkId {
            network_type: value.inner.network_type,
            suffix: value.inner.suffix,
        }
    }
}

// impl Into<NetworkId> for PyNetworkId {
//     fn into(self) -> NetworkId {
//         NetworkId {
//             network_type: self.inner.network_type,
//             suffix: self.inner.suffix,
//         }
//     }
// }

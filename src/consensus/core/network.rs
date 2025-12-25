use kaspa_addresses::Prefix;
use kaspa_consensus_core::network::{self, NetworkId, NetworkType, NetworkTypeError};
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
        Ok(Self::from(network_type))
    }
}

#[derive(Clone)]
#[pyclass(name = "NetworkId", skip_from_py_object)]
pub struct PyNetworkId(NetworkId);

#[pymethods]
impl PyNetworkId {
    #[new]
    pub fn new(network_id: Bound<PyAny>) -> PyResult<Self> {
        if let Ok(network_id) = network_id.extract::<String>() {
            PyNetworkId::from_str(&network_id)
        } else if let Ok(network_type) = network_id.extract::<PyNetworkType>() {
            let inner = NetworkId::new(network_type.into());
            Ok(Self(inner))
        } else {
            Err(PyException::new_err(
                "`network_id` must be of type NetworkType or String representation (mainnet, testnet-10, etc)",
            ))
        }
    }

    #[staticmethod]
    pub fn with_suffix(network_type: PyNetworkType, suffix: u32) -> Self {
        let inner = NetworkId::with_suffix(network_type.into(), suffix);
        Self(inner)
    }

    #[getter]
    pub fn network_type(&self) -> PyNetworkType {
        self.0.network_type.into()
    }

    pub fn is_mainnet(&self) -> bool {
        self.0.is_mainnet()
    }

    #[getter]
    pub fn suffix(&self) -> Option<u32> {
        self.0.suffix()
    }

    #[getter]
    pub fn default_p2p_port(&self) -> u16 {
        self.0.default_p2p_port()
    }

    pub fn to_prefixed(&self) -> String {
        self.0.to_prefixed()
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        self.0.to_string()
    }

    pub fn address_prefix(&self) -> String {
        Prefix::from(self.0.network_type).to_string()
    }
}

impl From<PyNetworkId> for NetworkId {
    fn from(value: PyNetworkId) -> Self {
        Self {
            network_type: value.0.network_type,
            suffix: value.0.suffix,
        }
    }
}

impl FromStr for PyNetworkId {
    type Err = PyErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inner = NetworkId::from_str(s)
            .map_err(|err| PyException::new_err(err.to_string()))?;

        Ok(Self(inner))
    }
}

impl<'py> FromPyObject<'_, 'py> for PyNetworkId {
    type Error = PyErr;

    fn extract(obj: Borrowed<'_, 'py, PyAny>) -> Result<Self, Self::Error> {
        if let Ok(s) = obj.extract::<String>() {
            PyNetworkId::from_str(&s)
        } else if let Ok(network_type) = obj.extract::<PyNetworkType>() {
            let inner = NetworkId::new(network_type.into());
            Ok(Self(inner))
        } else if let Ok(network_id) = obj.extract::<Self>() {
            Ok(network_id)
        } else {
            Err(PyException::new_err(
                "`network_id` must be a String or NetworkId instance",
            ))
        }
    }
}
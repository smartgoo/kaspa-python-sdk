use crate::rpc::model::*;
use paste::paste;
use pyo3::{prelude::*, types::PyDict};
use serde_pyobject::from_pyobject;

/// Macro to implement `TryFrom<Bound<'_, PyDict>>` for multiple request wrapper types.
///
/// For each name (e.g., `GetBlockCount`), generates an impl for `PyGetBlockCountRequest`
/// that deserializes from a Python dict using serde_pyobject.
macro_rules! impl_try_from_pydict {
    ([$($name:ident),* $(,)?]) => {
        paste! {
            $(
                impl TryFrom<Bound<'_, PyDict>> for [<Py $name Request>] {
                    type Error = PyErr;
                    fn try_from(dict: Bound<'_, PyDict>) -> PyResult<Self> {
                        Ok(from_pyobject(dict)?)
                    }
                }
            )*
        }
    };
}

impl_try_from_pydict!([
    // Optional request parameter (no args required)
    GetBlockCount,
    GetBlockDagInfo,
    GetCoinSupply,
    GetConnectedPeerInfo,
    GetInfo,
    GetPeerAddresses,
    GetMetrics,
    GetConnections,
    GetSink,
    GetSinkBlueScore,
    Ping,
    Shutdown,
    GetServerInfo,
    GetSyncStatus,
    GetFeeEstimate,
    GetCurrentNetwork,
    GetSystemInfo,
    // Required request parameter (args needed)
    AddPeer,
    Ban,
    EstimateNetworkHashesPerSecond,
    GetBalanceByAddress,
    GetBalancesByAddresses,
    GetBlock,
    GetBlocks,
    GetBlockTemplate,
    GetCurrentBlockColor,
    GetDaaScoreTimestampEstimate,
    GetFeeEstimateExperimental,
    GetHeaders,
    GetMempoolEntries,
    GetMempoolEntriesByAddresses,
    GetMempoolEntry,
    GetSubnetwork,
    GetUtxosByAddresses,
    GetUtxoReturnAddress,
    GetVirtualChainFromBlock,
    ResolveFinalityConflict,
    SubmitBlock,
    SubmitTransaction,
    SubmitTransactionReplacement,
    Unban,
]);

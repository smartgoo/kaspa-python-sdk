use kaspa_consensus_core::hashing::wasm::SighashType;
use pyo3::prelude::*;

crate::wrap_unit_enum_for_py!(PySighashType, "SighashType", SighashType, {
    All,
    None,
    Single,
    AllAnyOneCanPay,
    NoneAnyOneCanPay,
    SingleAnyOneCanPay,
});

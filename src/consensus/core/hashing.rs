use kaspa_consensus_core::hashing::wasm::SighashType;
use pyo3::prelude::*;

#[pyclass(name = "SighashType")]
#[derive(Clone)]
pub enum PySighashType {
    All,
    None,
    Single,
    AllAnyOneCanPay,
    NoneAnyOneCanPay,
    SingleAnyOneCanPay,
}

impl From<SighashType> for PySighashType {
    fn from(value: SighashType) -> Self {
        match value {
            SighashType::All => PySighashType::All,
            SighashType::None => PySighashType::None,
            SighashType::Single => PySighashType::Single,
            SighashType::AllAnyOneCanPay => PySighashType::AllAnyOneCanPay,
            SighashType::NoneAnyOneCanPay => PySighashType::NoneAnyOneCanPay,
            SighashType::SingleAnyOneCanPay => PySighashType::SingleAnyOneCanPay,
        }
    }
}

impl From<PySighashType> for SighashType {
    fn from(value: PySighashType) -> Self {
        match value {
            PySighashType::All => SighashType::All,
            PySighashType::None => SighashType::None,
            PySighashType::Single => SighashType::Single,
            PySighashType::AllAnyOneCanPay => SighashType::AllAnyOneCanPay,
            PySighashType::NoneAnyOneCanPay => SighashType::NoneAnyOneCanPay,
            PySighashType::SingleAnyOneCanPay => SighashType::SingleAnyOneCanPay,
        }
    }
}

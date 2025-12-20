use kaspa_bip32::Language;
use pyo3::prelude::*;

crate::wrap_unit_enum_for_py!(PyLanguage, "Language", Language, { English });

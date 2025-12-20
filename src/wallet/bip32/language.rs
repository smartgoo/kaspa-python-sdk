use kaspa_bip32::Language;
use pyo3::prelude::*;

#[pyclass(name = "Language")]
#[derive(Clone, Default)]
pub enum PyLanguage {
    #[default]
    English,
}

impl From<Language> for PyLanguage {
    fn from(value: Language) -> Self {
        match value {
            Language::English => PyLanguage::English,
        }
    }
}

impl From<PyLanguage> for Language {
    fn from(value: PyLanguage) -> Self {
        match value {
            PyLanguage::English => Language::English,
        }
    }
}

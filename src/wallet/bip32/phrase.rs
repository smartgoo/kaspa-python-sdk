use crate::wallet::bip32::language::PyLanguage;
use kaspa_bip32::{Error, Language, Mnemonic};
use pyo3::{exceptions::PyException, prelude::*};
use workflow_core::hex::ToHex;

#[pyclass(name = "Mnemonic")]
#[derive(Clone)]
pub struct PyMnemonic(Mnemonic);

#[pymethods]
impl PyMnemonic {
    #[new]
    #[pyo3(signature = (phrase, language=None))]
    pub fn constructor(phrase: &str, language: Option<PyLanguage>) -> PyResult<Self> {
        let inner = Mnemonic::new(
            phrase,
            language.map(Language::from).unwrap_or(Language::English),
        )
        .map_err(|err| PyException::new_err(err.to_string()))?;

        Ok(Self(inner))
    }

    #[staticmethod]
    #[pyo3(name = "validate")]
    #[pyo3(signature = (phrase, language=None))]
    pub fn validate(phrase: &str, language: Option<PyLanguage>) -> bool {
        Mnemonic::new(
            phrase,
            language.map(Language::from).unwrap_or(Language::English),
        )
        .is_ok()
    }

    #[getter]
    #[pyo3(name = "entropy")]
    pub fn get_entropy(&self) -> String {
        self.0.get_entropy()
    }

    #[setter]
    #[pyo3(name = "entropy")]
    pub fn set_entropy(&mut self, entropy: &str) {
        // let vec = Vec::<u8>::from_hex(entropy)
        //     .unwrap_or_else(|err| panic!("invalid entropy `{entropy}`: {err}"));
        // let len = vec.len();
        // if len != 16 && len != 32 {
        //     panic!("Invalid entropy: `{entropy}`")
        // }
        self.0.set_entropy(entropy.to_string());
        // self.entropy = vec;
    }

    #[staticmethod]
    #[pyo3(name = "random")]
    #[pyo3(signature = (word_count=None))]
    pub fn create_random(word_count: Option<u32>) -> PyResult<Self> {
        let word_count = word_count.unwrap_or(24) as usize;
        let inner = Mnemonic::random(
            word_count
                .try_into()
                .map_err(|err: Error| PyException::new_err(err.to_string()))?,
            Default::default(),
        )
        .map_err(|err: Error| PyException::new_err(err.to_string()))?;
        Ok(Self(inner))
    }

    #[getter]
    #[pyo3(name = "phrase")]
    pub fn phrase_string(&self) -> String {
        self.0.phrase().to_string()
        // self.phrase.clone()
    }

    #[setter]
    #[pyo3(name = "phrase")]
    pub fn set_phrase(&mut self, phrase: String) {
        // self.phrase = phrase;
        self.0.set_phrase(&phrase);
    }

    #[pyo3(name = "to_seed")]
    #[pyo3(signature = (password=None))]
    pub fn create_seed(&self, password: Option<&str>) -> String {
        let password = password.unwrap_or_default();
        self.0.to_seed(password).as_bytes().to_vec().to_hex()
    }
}

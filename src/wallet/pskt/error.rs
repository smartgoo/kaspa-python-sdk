use kaspa_wallet_pskt::wasm::error::Error;
use pyo3::create_exception;
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

pub struct PyPsktError(pub Error);

create_exception!("kaspa", PsktCustomError, PyException);
create_exception!("kaspa", PsktStateError, PyException);
create_exception!("kaspa", PsktExpectedStateError, PyException);
create_exception!("kaspa", PsktCtorError, PyException);
create_exception!("kaspa", PsktInvalidPayloadError, PyException);
create_exception!("kaspa", PsktTxNotFinalizedError, PyException);
create_exception!("kaspa", PsktCreateNotAllowedError, PyException);
create_exception!("kaspa", PsktNotInitializedError, PyException);
create_exception!("kaspa", PsktConsensusClientError, PyException);
create_exception!("kaspa", PsktError, PyException);

impl From<PyPsktError> for PyErr {
    fn from(value: PyPsktError) -> Self {
        match value.0 {
            Error::Custom(msg) => PsktCustomError::new_err(msg),
            Error::State(msg) => PsktStateError::new_err(msg),
            Error::ExpectedState(msg) => PsktExpectedStateError::new_err(msg),
            Error::Ctor(msg) => PsktCtorError::new_err(msg),
            Error::InvalidPayload => PsktInvalidPayloadError::new_err("Invalid payload"),
            Error::TxNotFinalized(inner) => PsktTxNotFinalizedError::new_err(inner.to_string()),
            Error::CreateNotAllowed => PsktCreateNotAllowedError::new_err(
                "Create state is not allowed for PSKT initialized from transaction or a payload",
            ),
            Error::NotInitialized => PsktNotInitializedError::new_err(
                "PSKT must be initialized with a payload or CREATE role",
            ),
            Error::ConsensusClient(inner) => PsktConsensusClientError::new_err(inner.to_string()),
            Error::Pskt(inner) => PsktError::new_err(inner.to_string()),
            _ => PyException::new_err("Unhandled error type"),
        }
    }
}

impl From<Error> for PyPsktError {
    fn from(value: Error) -> Self {
        PyPsktError(value)
    }
}

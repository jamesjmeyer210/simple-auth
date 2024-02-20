mod runtime_error;
mod error_kind;

pub type Error = runtime_error::RuntimeError;
pub type ErrorKind = error_kind::ErrorKind;
pub type Result<T> = std::result::Result<T,Error>;
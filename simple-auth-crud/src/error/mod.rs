pub mod encryption_error;
mod crud_error;

pub type EncryptionError = encryption_error::EncryptionError;
pub type DecryptionError = encryption_error::DecryptionError;
pub type CrudError = crud_error::CrudError;
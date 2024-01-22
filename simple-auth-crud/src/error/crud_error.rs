use crate::error::{DecryptionError, EncryptionError};

#[derive(Debug)]
pub enum CrudError {
    SqlxError(sqlx::Error),
    ValueIsNone,
    PasswordMismatch,
    EncryptionFailed(EncryptionError),
    DecryptionFailed(DecryptionError),
    Argon2Error(argon2::Error)
}

impl From<sqlx::Error> for CrudError {
    fn from(value: sqlx::Error) -> Self {
        CrudError::SqlxError(value)
    }
}

impl From<DecryptionError> for CrudError {
    fn from(value: DecryptionError) -> Self {
        CrudError::DecryptionFailed(value)
    }
}

impl From<EncryptionError> for CrudError {
    fn from(value: EncryptionError) -> Self {
        CrudError::EncryptionFailed(value)
    }
}

impl From<argon2::Error> for CrudError {
    fn from(value: argon2::Error) -> Self {
        CrudError::Argon2Error(value)
    }
}
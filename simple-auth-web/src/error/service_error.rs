use std::error::Error;
use std::fmt::{Display, Formatter};

use simple_auth_crud::error::CrudError;
use simple_auth_crud::sqlx;

#[derive(Debug)]
pub enum ServiceError {
    DbError(sqlx::Error),
    InvalidArgument,
    InternalAppError,
    NotImplemented,
}

impl From<sqlx::Error> for ServiceError {
    fn from(value: sqlx::Error) -> Self {
        Self::DbError(value)
    }
}

impl From<CrudError> for ServiceError {
    fn from(value: CrudError) -> Self {
        match value {
            CrudError::SqlxError(e) => Self::DbError(e),
            CrudError::PasswordMismatch => Self::InvalidArgument,
            CrudError::ValueIsNone => Self::InvalidArgument,
            CrudError::EncryptionFailed(_) => Self::InternalAppError,
            CrudError::DecryptionFailed(_) => Self::InternalAppError,
            _ => Self::NotImplemented
        }
    }
}

impl Display for ServiceError {
    fn fmt(&self, _f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ServiceError {

}
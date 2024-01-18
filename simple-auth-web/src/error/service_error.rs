use std::error::Error;
use std::fmt::{Display, Formatter};
use simple_auth_crud::sqlx;

#[derive(Debug)]
pub enum ServiceError {
    DbError(sqlx::Error),
    InvalidArgument,
    NotImplemented,
}

impl From<sqlx::Error> for ServiceError {
    fn from(value: sqlx::Error) -> Self {
        Self::DbError(value)
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for ServiceError {

}
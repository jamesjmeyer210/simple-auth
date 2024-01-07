use simple_auth_crud::sqlx;

#[derive(Debug)]
pub enum ServiceError {
    DbError(sqlx::Error)
}

impl From<sqlx::Error> for ServiceError {
    fn from(value: sqlx::Error) -> Self {
        Self::DbError(value)
    }
}
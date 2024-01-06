use simple_auth_crud::sqlx;

pub enum ServiceError {
    DbError(sqlx::Error)
}

impl From<sqlx::Error> for ServiceError {
    fn from(value: sqlx::Error) -> Self {
        Self::DbError(value)
    }
}
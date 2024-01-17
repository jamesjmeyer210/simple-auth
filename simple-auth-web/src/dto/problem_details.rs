use actix_web::http::{StatusCode};
use serde::Serialize;
use simple_auth_crud::sqlx::Error;
use simple_auth_crud::sqlx::error::ErrorKind;
use simple_auth_model::uuid::Uuid;
use crate::error::ServiceError;

#[derive(Debug, Serialize)]
pub struct ProblemDetails<'p> {
    pub title: &'p str,
    pub detail: Option<&'p str>,
    pub id: Uuid,
    pub status: u16,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    pub instance: Option<String>
}

impl <'p>ProblemDetails<'p> {
    pub fn new(status: StatusCode, title: &'p str) -> Self {
        Self {
            status: status.as_u16(),
            title,
            id: Uuid::new_v4(),
            detail: None,
            _type: None,
            instance: None,
        }
    }

    pub fn with_detail(mut self, detail: &'p str) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn with_type(mut self, value: String) -> Self {
        self._type = Some(value);
        self
    }

    pub fn with_instance(mut self, instance: String) -> Self {
        self.instance = Some(instance);
        self
    }

    pub fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl Default for ProblemDetails<'_> {
    fn default() -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
    }
}

impl From<ServiceError> for ProblemDetails<'_> {
    fn from(value: ServiceError) -> Self {
        match value {
            ServiceError::DbError(e) => {
                match e {
                    Error::Configuration(_) => todo!(),
                    Error::Database(db_error) => {
                        match db_error.kind() {
                            ErrorKind::Other => ProblemDetails::default(),
                            ErrorKind::UniqueViolation => ProblemDetails::new(StatusCode::CONFLICT, "Unique Violation"),
                            _ => ProblemDetails::new(StatusCode::BAD_REQUEST, "Bad Request")
                        }
                    },
                    Error::Io(_) => todo!(),
                    Error::Tls(_) => todo!(),
                    Error::Protocol(_) => todo!(),
                    Error::RowNotFound => ProblemDetails::new(StatusCode::NOT_FOUND, "Not Found"),
                    Error::TypeNotFound { .. } => todo!(),
                    Error::ColumnIndexOutOfBounds { .. } => ProblemDetails::new(StatusCode::RANGE_NOT_SATISFIABLE, "Range Not Satisfiable"),
                    Error::ColumnNotFound(_) => todo!(),
                    Error::ColumnDecode { .. } => todo!(),
                    Error::Decode(_) => todo!(),
                    Error::AnyDriverError(_) => todo!(),
                    Error::PoolTimedOut => todo!(),
                    Error::PoolClosed => todo!(),
                    Error::WorkerCrashed => todo!(),
                    Error::Migrate(_) => todo!(),
                    _ => ProblemDetails::new(StatusCode::INTERNAL_SERVER_ERROR, "An unknown error occurred"),
                }
            }
            ServiceError::InvalidArgument => ProblemDetails::new(StatusCode::BAD_REQUEST, "Bad Request")
        }
    }
}
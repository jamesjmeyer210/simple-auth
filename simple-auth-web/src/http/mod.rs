use actix_web::HttpResponse;
use serde::Serialize;
use crate::dto::ProblemDetails;
use crate::error::ServiceError;

pub(crate) struct HttpContext;

impl HttpContext {
    
    pub fn ok<T>(result: Result<T,ServiceError>) -> HttpResponse
    where T: Serialize
    {
        match result {
            Ok(model) => HttpResponse::Ok().json(model),
            Err(e) => Self::error_response(e)
        }
    }

    pub fn accepted<T>(result: Result<T,ServiceError>) -> HttpResponse
    where T: Serialize
    {
        match result {
            Ok(model) => HttpResponse::Accepted().json(model),
            Err(e) => Self::error_response(e)
        }
    }

    pub fn no_content<T>(result: Result<T,ServiceError>) -> HttpResponse
    where T: Serialize
    {
        match result {
            Ok(_) => HttpResponse::NoContent().finish(),
            Err(e) => Self::error_response(e)
        }
    }

    pub fn error_response(error: ServiceError) -> HttpResponse {
        log::error!("{:?}", error);
        let e: ProblemDetails = error.into();
        HttpResponse::build(e.status_code()).json(e)
    }
}
use actix_web::{HttpResponse, Responder, web};
use actix_web::web::ServiceConfig;
use serde::Serialize;
use crate::dto::ProblemDetails;
use crate::error::ServiceError;

mod realm_api;
mod role_api;

type RealmApi = realm_api::RealmApi;
type RoleApi = role_api::RoleApi;

pub trait WebApi {
    fn register(cfg: &mut web::ServiceConfig);
}

pub struct SimpleAuthApi;

impl WebApi for SimpleAuthApi {
    fn register(cfg: &mut ServiceConfig) {
        RealmApi::register(cfg);
        RoleApi::register(cfg);
    }
}

struct HttpContext;

impl HttpContext {
    fn ok<T>(result: Result<T,ServiceError>) -> impl Responder
        where T: Serialize
    {
        match result {
            Ok(model) => HttpResponse::Ok().json(model),
            Err(e) => {
                log::error!("{:?}", e);
                let e: ProblemDetails = e.into();
                HttpResponse::build(e.status_code()).json(e)
            }
        }
    }

    fn accepted<T>(result: Result<T,ServiceError>) -> HttpResponse
        where T: Serialize
    {
        match result {
            Ok(model) => HttpResponse::Accepted().json(model),
            Err(e) => {
                log::error!("{:?}", e);
                let e: ProblemDetails = e.into();
                HttpResponse::build(e.status_code()).json(e)
            }
        }
    }
}
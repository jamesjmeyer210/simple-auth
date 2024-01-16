use actix_web::{HttpResponse, Responder, web};
use actix_web::web::ServiceConfig;
use serde::Serialize;
use crate::di::{ServiceFactory, TransientFactory};
use crate::dto::ProblemDetails;
use crate::error::ServiceError;
use crate::service::{Service};

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
            Ok(models) => HttpResponse::Ok().json(models),
            Err(e) => {
                log::error!("{:?}", e);
                let e: ProblemDetails = e.into();
                HttpResponse::build(e.status_code()).json(e)
            }
        }
    }
}

pub(crate) struct DefaultCrudApi;

impl DefaultCrudApi {
    pub(crate) async fn get_all<'r,S,M>(factory: &ServiceFactory<'r>) -> impl Responder
        where
            S: Service<M> + for<'t> From<&'t ServiceFactory<'r>>,
            M: Serialize
    {
        let service: S = factory.get_transient();

        match service.get_all().await {
            Ok(models) => HttpResponse::Ok().json(models),
            Err(e) => {
                log::error!("{:?}", e);
                let e: ProblemDetails = e.into();
                HttpResponse::build(e.status_code()).json(e)
            }
        }
    }
}
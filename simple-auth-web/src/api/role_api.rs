use actix_web::{get, Responder, web};
use actix_web::web::{service, ServiceConfig};
use simple_auth_model::Role;
use crate::api::{DefaultCrudApi, HttpContext, WebApi};
use crate::di::{ServiceFactory, TransientFactory};
use crate::service::{RoleService, Service};

pub struct RoleApi;

impl WebApi for RoleApi {
    fn register(cfg: &mut ServiceConfig) {
        cfg.service(get_all);
    }
}

#[get("/role")]
async fn get_all(factory: web::Data<ServiceFactory<'_>>) -> impl Responder + '_ {
    let service: RoleService = factory.get_transient();
    let result = service.get_all().await;
    HttpContext::ok::<Vec<Role>>(result)
}
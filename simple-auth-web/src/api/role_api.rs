use actix_web::{get, Responder, web};
use actix_web::web::{ServiceConfig};
use simple_auth_model::Role;
use crate::api::{DefaultCrudApi, WebApi};
use crate::di::{ServiceFactory};
use crate::service::{RoleService, Service};

pub struct RoleApi;

impl WebApi for RoleApi {
    fn register(cfg: &mut ServiceConfig) {
        cfg.service(get_all);
    }
}

#[get("/role")]
async fn get_all(factory: web::Data<ServiceFactory<'_>>) -> impl Responder + '_ {
    DefaultCrudApi::get_all::<RoleService,Role>(factory.get_ref()).await
}
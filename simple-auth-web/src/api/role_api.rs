use actix_web::{delete, get, patch, post, Responder, web};
use actix_web::web::{ServiceConfig};
use simple_auth_model::role::RoleUpdate;
use crate::api::{HttpContext, WebApi};
use crate::di::{ServiceFactory, TransientFactory};
use crate::dto::{AddRoleDto};
use crate::service::{RoleService, Service};

pub struct RoleApi;

impl WebApi for RoleApi {
    fn register(cfg: &mut ServiceConfig) {
        cfg.service(get_all);
        cfg.service(get_by_id);
        cfg.service(add);
        cfg.service(update);
        cfg.service(soft_delete_by_id);
    }
}

#[get("/role")]
async fn get_all(factory: web::Data<ServiceFactory<'_>>) -> impl Responder + '_ {
    let service: RoleService = factory.get_transient();
    let result = service.get_all().await;
    HttpContext::ok(result)
}

#[get("/role/{id}")]
async fn get_by_id(id: web::Path<String>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    let service: RoleService = factory.get_transient();
    let result = service.get_by_id(&id).await;
    HttpContext::ok(result)
}

#[post("/role")]
pub async fn add(role: web::Json<AddRoleDto>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    let role = role.into_inner();

    let service: RoleService = factory.get_transient();
    let result = service.add(role.name, role.max, role.realm).await;
    HttpContext::accepted(result)
}

#[patch("/role")]
pub async fn update(role: web::Json<RoleUpdate>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    let service: RoleService = factory.get_transient();
    let result = service.update(role.into_inner()).await;
    HttpContext::accepted(result)
}

#[delete("/role/{id}")]
pub async fn soft_delete_by_id(id: web::Path<String>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    let service: RoleService = factory.get_transient();
    let result = service.soft_delete_by_id(id.as_str()).await;
    HttpContext::no_content(result)
}
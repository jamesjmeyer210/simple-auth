use actix_web::{get, HttpResponse, post, Responder, web};
use actix_web::http::StatusCode;
use actix_web::web::{ServiceConfig};
use crate::api::{HttpContext, WebApi};
use crate::di::{ServiceFactory, TransientFactory};
use crate::dto::{AddRoleDto, ProblemDetails};
use crate::service::{RoleService, Service};

pub struct RoleApi;

impl WebApi for RoleApi {
    fn register(cfg: &mut ServiceConfig) {
        cfg.service(get_all);
        cfg.service(get_by_id);
        cfg.service(add);
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
use actix_web::{patch, web, Responder};
use simple_auth_model::role::RoleUpdate;
use crate::di::{ServiceFactory, TransientFactory};
use crate::http::HttpContext;
use crate::service::RoleService;

#[patch("/role")]
pub async fn update(role: web::Json<RoleUpdate>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    let service: RoleService = factory.get_transient();
    let result = service.update(role.into_inner()).await;
    HttpContext::accepted(result)
}
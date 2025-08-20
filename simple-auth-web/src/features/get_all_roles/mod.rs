use actix_web::{get, web, Responder};
use crate::di::{ServiceFactory, TransientFactory};
use crate::http::HttpContext;
use crate::service::{RoleService, Service};

#[get("/role")]
async fn get_all(factory: web::Data<ServiceFactory<'_>>) -> impl Responder + '_ {
    let service: RoleService = factory.get_transient();
    let result = service.get_all().await;
    HttpContext::ok(result)
}
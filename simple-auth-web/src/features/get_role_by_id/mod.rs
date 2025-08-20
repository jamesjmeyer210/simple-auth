use actix_web::{get, web, Responder};
use crate::di::{ServiceFactory, TransientFactory};
use crate::http::HttpContext;
use crate::service::RoleService;

#[get("/role/{id}")]
async fn get_by_id(id: web::Path<String>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    let service: RoleService = factory.get_transient();
    let result = service.get_by_id(&id).await;
    HttpContext::ok(result)
}
use actix_web::{delete, web, Responder};
use crate::di::{ServiceFactory, TransientFactory};
use crate::http::HttpContext;
use crate::service::RoleService;

#[delete("/role/{id}")]
pub async fn soft_delete_by_id(id: web::Path<String>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    let service: RoleService = factory.get_transient();
    let result = service.soft_delete_by_id(id.as_str()).await;
    HttpContext::no_content(result)
}
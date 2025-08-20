use actix_web::{get, web, Responder};
use crate::di::{ServiceFactory, TransientFactory};
use crate::http::HttpContext;
use crate::service::RealmService;

#[get("/realm/{id}")]
async fn get_by_id(id: web::Path<String>, service_provider: web::Data<ServiceFactory<'_>>) -> impl Responder
{
    let realm_service: RealmService = service_provider.get_transient();
    let result =  realm_service.get_by_id(id.as_str()).await;
    HttpContext::ok(result)
}
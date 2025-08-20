use actix_web::{get, web, Responder};
use crate::di::{ServiceFactory, TransientFactory};
use crate::http::HttpContext;
use crate::service::{RealmService, Service};

#[get("/realm")]
async fn get_all(factory: web::Data<ServiceFactory<'_>>) -> impl Responder
{
    let realm_service: RealmService = factory.get_transient();
    let result =  realm_service.get_all().await;
    HttpContext::ok(result)
}
use actix_web::{patch, web, Responder};
use simple_auth_model::realm::UpdateRealm;
use crate::di::{ServiceFactory, TransientFactory};
use crate::http::HttpContext;
use crate::service::RealmService;

#[patch("/realm")]
async fn update(update: web::Json<UpdateRealm>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    let service: RealmService = factory.get_transient();
    let result = service.update(update.into_inner()).await;
    HttpContext::accepted(result)
}
use actix_web::{get, HttpResponse, patch, post, Responder, web};
use actix_web::http::StatusCode;
use actix_web::web::ServiceConfig;
use simple_auth_model::realm::UpdateRealm;
use crate::api::{HttpContext, WebApi};
use crate::di::{ServiceFactory, TransientFactory};
use crate::dto::ProblemDetails;
use crate::service::{RealmService, Service};

pub struct RealmApi;

impl WebApi for RealmApi {
    fn register(cfg: &mut ServiceConfig) {
        cfg.service(get_all);
        cfg.service(get_by_id);
        cfg.service(add);
        cfg.service(update);
    }
}

#[get("/realm")]
async fn get_all(factory: web::Data<ServiceFactory<'_>>) -> impl Responder
{
    let realm_service: RealmService = factory.get_transient();
    let result =  realm_service.get_all().await;
    HttpContext::ok(result)
}

#[get("/realm/{id}")]
async fn get_by_id(id: web::Path<String>, service_provider: web::Data<ServiceFactory<'_>>) -> impl Responder
{
    let realm_service: RealmService = service_provider.get_transient();
    let result =  realm_service.get_by_id(id.as_str()).await;
    HttpContext::ok(result)
}

#[post("/realm")]
async fn add(realm: web::Json<String>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    let service: RealmService = factory.get_transient();
    match service.add(realm.as_str()).await {
        Ok(realm) => HttpResponse::Ok().json(realm),
        Err(e) => {
            log::error!("{:?}", e);
            let mut e: ProblemDetails = e.into();
            match e.status_code() {
                StatusCode::CONFLICT => {
                    e = e.with_detail("A realm with that name already exists");
                    HttpResponse::build(e.status_code()).json(e)
                },
                _ => HttpResponse::build(e.status_code()).json(e)
            }
        }
    }
}

#[patch("/realm")]
async fn update(update: web::Json<UpdateRealm>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    let service: RealmService = factory.get_transient();
    let result = service.update(update.into_inner()).await;
    HttpContext::accepted(result)
}
use actix_web::{post, web, HttpResponse, Responder};
use actix_web::http::StatusCode;
use crate::di::{ServiceFactory, TransientFactory};
use crate::dto::ProblemDetails;
use crate::service::RealmService;

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
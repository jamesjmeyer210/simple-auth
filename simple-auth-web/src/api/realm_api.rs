use actix_web::{get, HttpResponse, Responder, web};
use actix_web::web::ServiceConfig;
use crate::api::RegisterApi;
use crate::di::{ServiceFactory, TransientFactory};
use crate::service::RealmService;

pub struct RealmApi;

impl RegisterApi for RealmApi {
    fn register(cfg: &mut ServiceConfig) {
        cfg.service(get_all);
        cfg.service(get_by_id);
    }
}

#[get("/realm")]
async fn get_all(factory: web::Data<ServiceFactory<'_>>) -> impl Responder
{
    let realm_service: RealmService = factory.get_transient();
    realm_service.get_all()
        .await
        .map(|x|HttpResponse::Ok().json(x))
        .map_err(|_|HttpResponse::InternalServerError().finish())
        .unwrap()
}

#[get("/realm/{id}")]
async fn get_by_id(id: web::Path<String>, service_provider: web::Data<ServiceFactory<'_>>) -> impl Responder
{
    let realm_service: RealmService = service_provider.get_transient();
    match realm_service.get_by_id(id.as_str()).await {
        Ok(realm) => HttpResponse::Ok().json(realm),
        Err(e) => {
            log::error!("{:?}", e);
            HttpResponse::NotFound().finish()
        }
    }
}
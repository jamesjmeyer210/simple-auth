use actix_web::{get, HttpResponse, Responder, web};
use actix_web::web::ServiceConfig;
use crate::api::RegisterApi;
use crate::di::{ServiceFactory, TransientFactory};
use crate::service::RealmService;

/*pub fn register(cfg: &mut web::ServiceConfig) {
//    cfg.service(get_by_id);
    cfg.service(get_all);
}*/

pub struct RealmApi;

impl RegisterApi for RealmApi {
    fn register(cfg: &mut ServiceConfig) {
        cfg.service(get_all);
    }
}

#[get("/realm/{id}")]
async fn get_all(realm_id: web::Path<String>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder
{
    let realm_service: RealmService = factory.get_transient();
    match realm_service
        .get_all()
        .await
        .map(|x|HttpResponse::Ok().json(x))
        .map_err(|e|HttpResponse::InternalServerError().finish())
    {
        Ok(x) => x,
        Err(x) => x
    }
}

/*#[get("/realm/{id}")]
async fn get_by_id(realm_id: web::Path<String>, service_provider: web::Data<ServiceProvider>)
    -> impl Responder
{
    let realm_service = service_provider.get_transient::<RealmService>();

}*/
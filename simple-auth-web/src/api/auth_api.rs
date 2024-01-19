use actix_web::web::ServiceConfig;
use crate::api::WebApi;

pub(crate) struct AuthApi;

impl WebApi for AuthApi {
    fn register(cfg: &mut ServiceConfig) {

    }
}
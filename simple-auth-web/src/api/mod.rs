use actix_web::web;

mod realm_api;

pub trait RegisterApi {
    fn register(cfg: &mut web::ServiceConfig);
}

pub type RealmApi = realm_api::RealmApi;
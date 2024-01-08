use simple_auth_crud::DbContext;
use simple_auth_model::log4rs;
use simple_auth_web::di::ServiceCollection;
use simple_auth_web::service::{RealmService, RoleService};

#[actix_rt::main]
async fn main() {
    log4rs::init_file("logcfg.yaml", Default::default()).unwrap();

    let db = DbContext::in_memory().await.unwrap();

    let mut services = ServiceCollection::new();
    services.add(db);

    let provider = services.build_provider();
    let realm_service = provider.get_transient::<RealmService>();

    let realm = realm_service.add_default().await;
    if realm.is_err() {
        log::error!("{:?}", realm.unwrap_err());
        return;
    }

    let role_service = provider.get_transient::<RoleService>();
    let role = role_service.add_default(realm.unwrap()).await;
    if role.is_err() {
        log::error!("{:?}", role.unwrap_err());
        return;
    }

    log::info!("Pre-server start complete!");
}
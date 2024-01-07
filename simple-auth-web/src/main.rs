use simple_auth_crud::DbContext;
use simple_auth_model::log4rs;
use simple_auth_web::di::ServiceCollection;
use simple_auth_web::service::RealmService;

#[actix_rt::main]
async fn main() {
    log4rs::init_file("logcfg.yaml", Default::default()).unwrap();

    let db = DbContext::in_memory().await.unwrap();

    let mut services = ServiceCollection::new();
    services.add(db);

    let provider = services.build_provider();
    let service = provider.get_transient::<RealmService>();

    let x = service.add_default().await;
    if x.is_err() {
        log::error!("{:?}", x.unwrap_err());
        return;
    }

    log::info!("Pre-server start complete!");
}
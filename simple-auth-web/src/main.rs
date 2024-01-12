use std::sync::Arc;
use simple_auth_crud::DbContext;
use simple_auth_model::log4rs;
use simple_auth_web::di::{ServiceCollection, ServiceProvider};
use simple_auth_web::error::ServiceError;
use simple_auth_web::service::{RealmService, RoleService, UserService};

async fn init_defaults(provider: &ServiceProvider) -> Result<(),ServiceError> {
    let realm_service = provider.get_transient::<RealmService>();

    let realm = realm_service.add_default().await?;

    let role_service = provider.get_transient::<RoleService>();
    let mut role = role_service.add_default(realm).await?;
    let realm = role.realms.pop().unwrap();

    let user_service = provider.get_transient::<UserService>();

    let _ = user_service.add_default(realm, role).await?;
    Ok(())
}

#[actix_rt::main]
async fn main() {
    log4rs::init_file("logcfg.yaml", Default::default()).unwrap();

    let db = DbContext::in_memory().await.unwrap();

    let secret_store = (&db).get_secret_store().await;
    if secret_store.is_err() {
        log::error!("Failed to load secrets");
        return;
    }
    let secret_store = secret_store.unwrap();
    log::info!("Loaded secrets");

    let mut services = ServiceCollection::new();
    services.add(db);
    services.add(Arc::new(secret_store));

    let provider = services.build_provider();

    let op = init_defaults(&provider).await;
    if op.is_err() {
        log::error!("{:?}", op.unwrap_err());
        return;
    }

    log::info!("Pre-server start complete!");
}
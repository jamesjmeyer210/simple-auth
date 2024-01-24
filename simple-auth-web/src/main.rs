use actix_web::{App, HttpServer, web};
use actix_web_httpauth::middleware::HttpAuthentication;
use simple_auth_crud::DbContext;
use simple_auth_crud::sqlx::Error::Database;
use simple_auth_model::config::{Config, DatabaseConfig, SqliteConfig};
use simple_auth_model::log4rs;
use simple_auth_web::api::{OAuthApiV1, SimpleAuthApiV1, WebApi};
use simple_auth_web::di::{ServiceFactory, TransientFactory};
use simple_auth_web::error::ServiceError;
use simple_auth_web::middleware::SimpleAuthMiddleware;
use simple_auth_web::service::{RealmService, RoleService, UserService};

// TODO: move this to a workflow
async fn init_defaults(provider: &ServiceFactory<'_>) -> Result<(),ServiceError> {
    let realm_service: RealmService = provider.get_transient();

    let realm = realm_service.add_default().await?;

    let role_service: RoleService = provider.get_transient();
    let mut role = role_service.add_default(realm).await?;
    let realm = role.realms.pop().unwrap();

    let user_service: UserService = provider.get_transient();

    let _ = user_service.add_default(realm, role).await?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::load("appconfig.dev.json")?;
    config.print_content()?;

    log4rs::init_file(&config.log_file, Default::default()).unwrap();

    let db = match &config.database {
        DatabaseConfig::Sqlite(sqlite) => match sqlite {
            SqliteConfig::InMemory => DbContext::in_memory().await.unwrap(),
            SqliteConfig::Path(path) => DbContext::new(path.as_str()).await.unwrap()
        }
    };

    db.migrate()
        .await
        .expect("ERROR: Migration failed");

    let secret_store = (&db).get_secret_store().await;
    if secret_store.is_err() {
        log::error!("Failed to load secrets");
        return Err(std::io::Error::other(secret_store.unwrap_err()));
    }
    let secret_store = secret_store.unwrap();
    log::info!("Loaded secrets");

    let factory = ServiceFactory::new()
        .add_singleton(db)
        .add_singleton(secret_store);

    let op = init_defaults(&factory).await;
    if op.is_err() {
        return Err(std::io::Error::other(op.unwrap_err()));
    }

    log::info!("Pre-server start complete!");

    let provider = web::Data::new(factory);

    HttpServer::new(move || {
        let authentication_middleware = HttpAuthentication::bearer(SimpleAuthMiddleware::authenticate_bearer);
        App::new()
            .app_data(provider.clone())
            .service(
                web::scope("/v1/api")
                    .wrap(authentication_middleware)
                    .configure(SimpleAuthApiV1::register)
            )
            .service(
                web::scope("/v1/oauth")
                    .configure(OAuthApiV1::register)
            )
    }).bind(("127.0.0.1", 7777))?
        .run()
        .await
}
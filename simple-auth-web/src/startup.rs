use std::io;
use std::rc::Rc;
use std::sync::Arc;
use actix_cors::Cors;




use simple_auth_crud::crypto::SecretStore;
use simple_auth_crud::DbContext;
use simple_auth_model::config::{Config, DatabaseConfig, ServerConfig, SqliteConfig};
use simple_auth_model::log4rs;

use crate::di::{ServiceFactory, TransientFactory};
use crate::error::ServiceError;

use crate::service::{RealmService, RoleService, UserService};

pub struct Startup;

impl Startup {
    pub fn load_config(path: &str) -> Result<Config, io::Error> {
        let config = Config::load(path)?;
        config.print_content()?;
        Ok(config)
    }

    pub async fn configure_services(config: Rc<Config>) -> Result<ServiceFactory<'static>, io::Error> {
        log4rs::init_file(&config.log_file, Default::default()).unwrap();

        let db = Self::configure_database(config).await;
        let secret_store = Self::configure_secret_store(&db).await?;

        let factory = ServiceFactory::new()
            .add_singleton(db)
            .add_singleton(secret_store);

        let op = Self::init_defaults(&factory).await;
        if op.is_err() {
            return Err(io::Error::other(op.unwrap_err()));
        }

        Ok(factory)
    }

    pub fn configure_cors(config: &ServerConfig) -> Cors {
        let origin = config.domain.clone();
        let allowed_origins: Arc<Vec<String>> = Arc::new(config.allowed_origins.clone());

        Cors::default()
            .allow_any_origin()
            .allowed_origin(origin.as_str())
            .allowed_origin_fn(move |origin, _req_head|{
                allowed_origins.iter().any(|x|x.as_bytes().eq(origin.as_bytes()))
            })
            .allow_any_method()
            .allow_any_header()
    }

    async fn configure_database(config: Rc<Config>) -> DbContext<'static> {
        let db = match &config.database {
            DatabaseConfig::Sqlite(sqlite) => match sqlite {
                SqliteConfig::InMemory => DbContext::in_memory().await.unwrap(),
                SqliteConfig::Path(path) => DbContext::new(path.as_str()).await.unwrap()
            }
        };

        db.migrate().await.expect("ERROR: Migration failed");
        db
    }

    async fn configure_secret_store(db: &DbContext<'_>) -> Result<SecretStore,io::Error> {
        let secret_store = db.get_secret_store().await;
        if secret_store.is_err() {
            log::error!("Failed to load secrets");
            return Err(io::Error::other(secret_store.unwrap_err()));
        }

        let secret_store = secret_store.unwrap();
        log::info!("Loaded secrets");
        Ok(secret_store)
    }

    async fn init_defaults(provider: &ServiceFactory<'_>) -> Result<(),ServiceError> {
        let realm_service: RealmService = provider.get_transient();

        let realm = realm_service.add_default().await?;

        let role_service: RoleService = provider.get_transient();
        let role = role_service.add_default(realm.name.clone()).await?;

        let user_service: UserService = provider.get_transient();

        let _ = user_service.add_default(realm, role).await?;
        Ok(())
    }
}
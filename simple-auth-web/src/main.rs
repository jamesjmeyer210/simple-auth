use std::rc::Rc;
use std::sync::Arc;
use actix_cors::Cors;
use actix_web::{App, http, HttpServer, web};
use actix_web::middleware::Logger;
use actix_web_httpauth::middleware::HttpAuthentication;
use simple_auth_crud::DbContext;
use simple_auth_crud::sqlx::Error::Database;
use simple_auth_model::log4rs;
use simple_auth_web::api::{OAuthApiV1, SimpleAuthApiV1, WebApi};
use simple_auth_web::di::{ServiceFactory, TransientFactory};
use simple_auth_web::error::ServiceError;
use simple_auth_web::middleware::SimpleAuthMiddleware;
use simple_auth_web::service::{RealmService, RoleService, UserService};
use simple_auth_web::startup::Startup;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let config = Startup::load_config("appconfig.dev.json")?;
    let server_config = config.server.clone();
    let cfg = Rc::new(config);
    let factory = Startup::configure_services(cfg.clone()).await?;

    log::info!("Pre-server start complete!");

    let provider = web::Data::new(factory);

    let mut server = HttpServer::new(move || {

        let authentication_middleware = HttpAuthentication::bearer(SimpleAuthMiddleware::authenticate_bearer);
        let cors_middleware = Startup::configure_cors(&server_config);

        App::new()
            .app_data(provider.clone())
            .wrap(Logger::default())
            .wrap(cors_middleware)
            .service(
                web::scope("/v1/api")
                    .wrap(authentication_middleware)
                    .configure(SimpleAuthApiV1::register)
            )
            .service(
                web::scope("/v1/oauth")
                    .configure(OAuthApiV1::register)
            )
    });

    if cfg.server.workers.is_some() {
        server = server.workers(cfg.server.workers.clone().unwrap());
    }

    server.bind((cfg.server.domain.as_str(), cfg.server.port))?
        .run()
        .await
}
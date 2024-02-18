use std::rc::Rc;
use actix_web::{App, test, web};
use actix_web::middleware::Logger;
use simple_auth_web::api::{WebApi, OAuthApiV1};
use simple_auth_web::startup::Startup;
use simple_auth_model::auth::PasswordLogin;

#[actix_web::test]
async fn login_returns_tokens() {
    let config = Startup::load_config("appconfig.test.json").unwrap();
    let cfg = Rc::new(config);
    let factory = Startup::configure_services(cfg.clone()).await.unwrap();
    let provider = web::Data::new(factory);

    let app = test::init_service(App::new()
        .app_data(provider.clone())
        .wrap(Logger::default())
        .service(
            web::scope("/v1/oauth")
                .configure(OAuthApiV1::register)
        )).await;

    let req = test::TestRequest::post()
        .uri("/v1/oauth/token")
        .set_json(PasswordLogin {
            user_name: "root".to_string(),
            password: "password123".to_string()
        })
        .to_request();

    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success());
}
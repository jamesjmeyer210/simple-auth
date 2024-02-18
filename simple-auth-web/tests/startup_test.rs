use simple_auth_web::startup::Startup;

#[test]
fn load_returns_ok() {
    let ld = Startup::load_config("appconfig.test.json");
    assert!(ld.is_ok());
}
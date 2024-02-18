use actix_web::{HttpResponse, post, Responder, web};
use actix_web::web::{ServiceConfig};
use simple_auth_model::auth::PasswordLogin;
use simple_auth_model::Password;
use crate::api::{HttpContext, WebApi};
use crate::di::{ServiceFactory, TransientFactory};
use crate::dto::{ProblemDetails};
use crate::dto::oauth::ResourceOwnerPasswordResponse;
use crate::service::AuthService;

pub(crate) struct AuthApi;

impl WebApi for AuthApi {
    fn register(cfg: &mut ServiceConfig) {
        cfg.service(login);
    }
}

#[post("token")]
async fn login(dto: web::Json<PasswordLogin>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    let dto = dto.into_inner();

    let pass = Password::try_from(dto.password.as_str());
    if pass.is_err() {
        let e = ProblemDetails::bad_request().with_detail(pass.unwrap_err());
        return HttpResponse::BadRequest().json(e);
    }
    let pass = pass.unwrap();

    let service: AuthService = factory.get_transient();
    let tokens = service.get_resource_owner_tokens(dto.user_name, pass).await;

    if tokens.is_err() {
        return HttpContext::error_response(tokens.unwrap_err());
    }

    HttpResponse::Ok().json(
        ResourceOwnerPasswordResponse::bearer()
            .with_resource_owner_tokens(tokens.unwrap())
    )
}
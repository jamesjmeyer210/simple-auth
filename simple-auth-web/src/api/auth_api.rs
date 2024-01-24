use actix_web::{HttpResponse, post, Responder, web};
use actix_web::web::{ServiceConfig};
use simple_auth_model::Password;
use crate::api::{HttpContext, WebApi};
use crate::di::{ServiceFactory, TransientFactory};
use crate::dto::{PasswordLoginDto, ProblemDetails};
use crate::dto::oauth::ResourceOwnerPasswordResponse;
use crate::service::AuthService;

pub(crate) struct AuthApi;

impl WebApi for AuthApi {
    fn register(cfg: &mut ServiceConfig) {
        cfg.service(login);
    }
}

#[post("token")]
async fn login(dto: web::Json<PasswordLoginDto>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    let dto = dto.into_inner();

    let pass = Password::try_from(dto.password.as_str());
    if pass.is_err() {
        let e = ProblemDetails::bad_request().with_detail(pass.unwrap_err());
        return HttpResponse::BadRequest().json(e);
    }
    let pass = pass.unwrap();

    let service: AuthService = factory.get_transient();
    let access_token = service.get_jwt(dto.user_name, pass)
        .await
        .map(|x|x.to_base64_string());

    if access_token.is_err() {
        return HttpContext::error_response(access_token.unwrap_err());
    }

    HttpResponse::Ok().json(
        ResourceOwnerPasswordResponse::bearer()
            .with_access_token(access_token.unwrap())
    )
}
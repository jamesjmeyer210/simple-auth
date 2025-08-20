use actix_web::{post, web, HttpResponse, Responder};
use simple_auth_model::auth::PasswordLogin;
use simple_auth_model::Password;
use crate::di::{ServiceFactory, TransientFactory};
use crate::dto::oauth::ResourceOwnerPasswordResponse;
use crate::dto::ProblemDetails;
use crate::http::HttpContext;
use crate::service::AuthService;

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
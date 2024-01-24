use actix_web::dev::ServiceRequest;
use actix_web::web::Data;
use actix_web_httpauth::extractors::{AuthenticationError, bearer};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::di::{ServiceFactory, TransientFactory};
use crate::dto::ProblemDetails;
use crate::service::AuthService;

pub struct SimpleAuthMiddleware;

impl SimpleAuthMiddleware {
    pub async fn authenticate_bearer(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)>
    {
        //TODO: use `credentials.token()` instead of manually getting the header
        let header = req.headers().get("Authorization").unwrap();

        let factory: Option<&Data<ServiceFactory>> = req.app_data::<Data<ServiceFactory>>();
        if factory.is_none() {
            log::info!("Factory is none");
            return Err((
                actix_web::Error::from(ProblemDetails::bad_request()),
                req));
        }
        log::info!("Factory exists!");

        let factory = factory.unwrap();
        let auth_service: AuthService = factory.get_transient();
        match auth_service.validate_jwt(header) {
            true => Ok(req),
            _ => {
                let config = req.app_data::<bearer::Config>()
                    .cloned()
                    .unwrap_or_default();

                Err((AuthenticationError::from(config).into(), req))
            },
        }
    }
}
use actix_web::{get, HttpResponse, Responder, web};
use actix_web::web::{service, ServiceConfig};
use serde::Deserialize;
use simple_auth_crud::sqlx::query;
use simple_auth_model::uuid::Uuid;
use crate::api::{HttpContext, WebApi};
use crate::di::{ServiceFactory, TransientFactory};
use crate::dto::ProblemDetails;
use crate::error::ServiceError;
use crate::service::UserService;

pub struct UserApi;

impl WebApi for UserApi {
    fn register(cfg: &mut ServiceConfig) {
        cfg.service(get_all);
        cfg.service(get_by_query);
    }
}

#[get("/user/{page}")]
async fn get_all(page: web::Path<u32>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    let service: UserService = factory.get_transient();
    match service.get_page(page.into_inner()).await {
        Ok(users) => HttpResponse::Ok()
            .append_header(("X-Total", users.total))
            .json(users.data),
        Err(e) => {
            log::error!("{:?}", e);
            let e: ProblemDetails = e.into();
            HttpResponse::build(e.status_code()).json(e)
        }
    }
}

#[derive(Debug, Deserialize)]
struct UserQuery {
    id: Option<Uuid>,
    name: Option<String>,
    contact: Option<String>,
}

impl UserQuery {
    fn is_none(&self) -> bool {
        self.id.is_none() && self.name.is_none() && self.contact.is_none()
    }
}

#[get("/user")]
async fn get_by_query(query: web::Query<UserQuery>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    if query.is_none() {
        return HttpResponse::BadRequest().json(
            ProblemDetails::bad_request().with_detail("Query must not be empty"))
    }

    let service: UserService = factory.get_transient();
    let result = if query.id.is_some() {
        service.get_by_id(&query.id.unwrap()).await
    }
    else if query.name.is_some() {
        Err(ServiceError::NotImplemented)
    }
    else if query.contact.is_some() {
        Err(ServiceError::NotImplemented)
    }
    else {
        unreachable!()
    };

    HttpContext::ok(result)
}
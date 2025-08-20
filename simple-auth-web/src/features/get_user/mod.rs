use actix_web::{get, web, HttpResponse, Responder};
use serde::Deserialize;
use simple_auth_crud::sqlx::types::Uuid;
use crate::di::{ServiceFactory, TransientFactory};
use crate::dto::ProblemDetails;
use crate::http::HttpContext;
use crate::service::UserService;

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
            ProblemDetails::bad_request().with_detail("Query must not be empty"));
    }

    let service: UserService = factory.get_transient();
    if query.id.is_some() {
        let result = service.get_by_id(query.id.as_ref().unwrap()).await;
        return HttpContext::ok(result);
    }
    if query.name.is_some() {
        let result = service.get_by_name(query.name.as_ref().unwrap()).await;
        return HttpContext::ok(result);
    }
    if query.contact.is_some() {
        let result = service.get_by_contact(query.contact.as_ref().unwrap()).await;
        return HttpContext::ok(result);
    }

    HttpResponse::NotImplemented().finish()
}
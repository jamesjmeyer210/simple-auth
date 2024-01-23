use actix_web::{get, HttpResponse, post, Responder, web};
use actix_web::web::{service, ServiceConfig};
use serde::Deserialize;
use simple_auth_model::{ContactInfo, Password, User};
use simple_auth_model::uuid::Uuid;
use crate::api::{HttpContext, WebApi};
use crate::di::{ServiceFactory, TransientFactory};
use crate::dto::{AddUserDto, ProblemDetails};
use crate::service::{RealmService, RoleService, UserService};

pub struct UserApi;

impl WebApi for UserApi {
    fn register(cfg: &mut ServiceConfig) {
        cfg.service(get_all);
        cfg.service(get_by_query);
        cfg.service(add_user);
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
            HttpContext::error_response(e)
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

#[post("/user")]
async fn add_user(user: web::Json<AddUserDto>, factory: web::Data<ServiceFactory<'_>>) -> impl Responder {
    if user.password1 != user.password2 {
        return HttpResponse::BadRequest()
            .json(ProblemDetails::bad_request()
                .with_detail("Passwords do not match"));
    }

    let password = Password::try_from(user.password1.as_str());
    if password.is_err() {
        return HttpResponse::BadRequest()
            .json(ProblemDetails::bad_request()
                .with_detail(password.unwrap_err()))
    }

    let contact = ContactInfo::try_new(&user.contact.label, &user.contact.value);
    if contact.is_err() {
        return HttpResponse::BadRequest()
            .json(ProblemDetails::bad_request()
                .with_detail(contact.unwrap_err()));
    }

    let service: RealmService = factory.get_transient();
    let realm = service.get_by_id(&user.realm).await;
    if realm.is_err() {
        return HttpContext::error_response(realm.unwrap_err())
    }

    let service: RoleService = factory.get_transient();
    let role = service.get_by_id(&user.role).await;
    if role.is_err() {
        return HttpContext::error_response(role.unwrap_err());
    }

    let user = User::new(user.into_inner().name, password.unwrap())
        .with_contact_info(contact.unwrap())
        .with_realm(realm.unwrap())
        .with_role(role.unwrap());

    let service: UserService = factory.get_transient();
    let user = service.add(user).await;
    HttpContext::accepted(user)
}
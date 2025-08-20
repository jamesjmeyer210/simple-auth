use actix_web::{post, web, HttpResponse, Responder};
use simple_auth_model::{ContactInfo, Password, User};
use crate::di::{ServiceFactory, TransientFactory};
use crate::dto::{AddUserDto, ProblemDetails};
use crate::http::HttpContext;
use crate::service::{RealmService, RoleService, UserService};

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
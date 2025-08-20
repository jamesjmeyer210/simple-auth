use actix_web::{get, web, HttpResponse, Responder};
use crate::di::{ServiceFactory, TransientFactory};
use crate::http::HttpContext;
use crate::service::UserService;

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
use actix_web::{ web, get, HttpResponse, Responder };
use crate::routes::user;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
}

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg
    .service(index)
    .service(
        web::scope("/user").configure(user::routes)
    );
}

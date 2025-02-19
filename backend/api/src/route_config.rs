use actix_web::web;
use crate::routes::user;

pub fn setup(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user").configure(user::routes)
    );
}

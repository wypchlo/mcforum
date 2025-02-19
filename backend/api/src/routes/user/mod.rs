mod handlers;
use actix_web::web;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(handlers::get_users);
}

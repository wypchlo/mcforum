mod handlers;
use actix_web::web;

use handlers as user;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(user::get_all)
    .service(user::create);
}

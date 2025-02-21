mod handlers;
use actix_web::web;

use handlers as user_handlers;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(user_handlers::get_all)
    .service(user_handlers::create)
    .service(user_handlers::delete);
}

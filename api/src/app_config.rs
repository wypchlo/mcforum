use actix_web::web;
use crate::handlers::{ user, category, comment, post };

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(user::get_users)
    );
    cfg.service(
        web::scope("/category")
            .service(category::get_categories)
    );
    cfg.service(
        web::scope("/post")   
            .service(post::get_posts)
    );
    cfg.service(
        web::scope("/comment")
            .service(comment::get_comments)
    );
}

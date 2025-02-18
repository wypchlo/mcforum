use actix_web::{HttpServer, App};
use actix_web::middleware::{NormalizePath, TrailingSlash};
use crate::app_config::config_app;

#[actix_web::main]
async fn start_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .configure(config_app)
    }) 
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

pub fn main() {
    let result = start_server();
}

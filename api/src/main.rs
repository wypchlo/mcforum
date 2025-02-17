use actix_web::{HttpServer, App};
use mcforum_api::app_config::config_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(config_app)
    }) 
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

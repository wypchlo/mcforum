use actix_web::{HttpServer, App};
use actix_web::middleware::{NormalizePath, TrailingSlash};
use crate::app_config::config_app;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    const DB_URL: &str = std::env().expect("Couldn't find DATABASE_URL in the .env file");

    const HOST: &str = "127.0.0.1"; //localhost
    const PORT: u16 = 3000;

    let server = HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .configure(config_app)
    }) 
    .bind((HOST, PORT))?; 

    println!("Server running at url: http://{HOST}:{PORT}");

    server.run().await
}

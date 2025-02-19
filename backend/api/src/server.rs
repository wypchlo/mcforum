use actix_web::{HttpServer, App};
use actix_web::middleware::{NormalizePath, TrailingSlash};
use crate::route_config;
use dotenv::dotenv;

#[actix_web::main]
pub async fn start() -> std::io::Result<()> {
    dotenv().ok();
    let db_url: String = std::env::var("DATABASE_URL").expect("Couldn't find DATABASE_URL in the .env file");

    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 3000;

    let server = HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .configure(route_config::setup)
    }) 
    .bind((HOST, PORT))?; 

    println!("Server running at url: http://{HOST}:{PORT}");

    server.run().await
}

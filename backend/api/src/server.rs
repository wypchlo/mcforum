use actix_web::{HttpServer, App};
use actix_web::middleware::{NormalizePath, TrailingSlash, Logger};
use crate::route_config;
use dotenv::dotenv;
use service::sea_orm::{DatabaseConnection, Database, ConnectOptions};
use std::time::Duration;
use std::io::Result;
use log::error;

#[actix_web::main]
pub async fn start() -> Result<()> {
    dotenv().ok();

    let db_url: String = std::env::var("DATABASE_URL").unwrap_or_else(|error| {
        error!("Couldn't find DATABASE_URL in the .env file");
        std::process::exit(1);
    });
   
    let mut opt = ConnectOptions::new(&db_url);

    opt.acquire_timeout(Duration::from_secs(2));

    let conn: DatabaseConnection = Database::connect(opt).await.unwrap_or_else(|error| {
        error!("Couldn't connect to the database at url {}.\n{error}", &db_url);
        std::process::exit(1);
    });

    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 3000;

    let server = HttpServer::new(|| {
        App::new()
            .wrap(NormalizePath::new(TrailingSlash::Always))
            .wrap(Logger::default())
            .configure(route_config::setup)
    }) 
    .bind((HOST, PORT))?; 

    server.run().await
}

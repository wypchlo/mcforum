use actix_web::{get, post, Responder, HttpResponse, web};
use crate::utils::app_state::AppState;
use entity::user;
use service::sea_orm::{ActiveValue};

#[get("/")]
pub async fn get_all() -> impl Responder {
    HttpResponse::Ok().body("A list of users will be returned here")
}

#[post("/")]
pub async fn create(data: web::Data<AppState>) -> impl Responder {
    let conn = &data.db_conn;

    service::user::mutation::Mutation::create_post(conn, user::Model {
        username: "jakub".to_string(),
        email: "jakub.rymanowski@spoko.pl".to_string(),
        ..Default::default()
    })
    .await
    .expect("Zjebalo sie cos");

    HttpResponse::Ok()
}

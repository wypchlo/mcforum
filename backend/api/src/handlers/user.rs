use actix_web::{get, Responder, HttpResponse};

#[get("/")]
async fn get_users() -> impl Responder {
    HttpResponse::Ok().body("A list of users will be returned here")
}

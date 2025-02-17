use actix_web::{ get, Responder, HttpResponse };

#[get("/")]
async fn get_categories() -> impl Responder {
    HttpResponse::Ok().body("categories.")
}

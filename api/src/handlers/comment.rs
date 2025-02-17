use actix_web::{ get, Responder, HttpResponse };

#[get("/")]
async fn get_comments() -> impl Responder {
    HttpResponse::Ok().body("Comments here")
}

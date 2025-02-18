use actix_web::{ get, Responder, HttpResponse };

#[get("/")]
async fn get_posts() -> impl Responder {
    HttpResponse::Ok().body("Posts will be here in the future ")
}

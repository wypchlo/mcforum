use actix_web::{get, post, Responder, HttpResponse, web};
use crate::utils::app_state::AppState;
use entity::user;
use service::user::{
    mutation::Mutation as user_mutation,
    query::Query as user_query
};
use log::error;

#[get("/")]
pub async fn get_all(
    app_state: web::Data<AppState>
) -> impl Responder {
    let conn = &app_state.db_conn;
    
    let result = user_query::get_all(conn).await;

    match result {
        Ok(users) => return HttpResponse::Ok().json(users),
        Err(error) => {
            let message = format!("Encountered an error while retrieving users: {error}");
            error!("{message}");
            return HttpResponse::InternalServerError().json(message);
        }
    }
}

#[post("/")]
pub async fn create(
    app_state: web::Data<AppState>,
    data: web::Json<user::Model>
) -> impl Responder {
    let conn = &app_state.db_conn;

    let user::Model { username, email, profile_picture, .. } = data.0;
    
    let result = user_mutation::create_post(conn, user::Model {
        username, 
        email, 
        profile_picture, 
        ..Default::default()
    })
    .await;

    match result {
        Ok(model) => model,
        Err(error) => { 
            let message = format!("Encountered an error while creating a new post: {error}");
            error!("{message}");
            return HttpResponse::InternalServerError().json(message)
        }
    };

    HttpResponse::Ok().json("Successfully created a new post")
}

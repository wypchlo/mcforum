use sea_orm::*;
use ::entity::{user, user::Entity as User};

pub struct Query;

impl Query {
    pub async fn get_all(conn: &DbConn) -> Result<Vec<user::Model>, DbErr> {
        User::find().all(conn).await
    }
}

use ::entity::user;
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create_post(
        conn: &DbConn,
        data: user::Model
    ) -> Result<user::ActiveModel, DbErr> {
        user::ActiveModel {
            username: Set(data.username.to_owned()),
            email: Set(data.email.to_owned()),
            profile_picture: Set(data.profile_picture.to_owned()),
            ..Default::default()
        }
        .save(conn)
        .await
    }
}

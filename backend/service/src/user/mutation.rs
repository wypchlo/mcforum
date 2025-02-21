use ::entity::{user, user::Entity as User};
use sea_orm::*;

pub struct Mutation;

impl Mutation {
    pub async fn create(
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

    pub async fn delete(  
        conn: &DbConn,
        id: u64
    ) -> Result<DeleteResult, DbErr> {
        let user: user::ActiveModel = User::find_by_id(id)
            .one(conn)
            .await?
            .ok_or(DbErr::Custom("Encountered an error while deleting the user. An user with the given id does not exist".to_owned()))
            .map(Into::into)?;

        user.delete(conn).await
    }
}

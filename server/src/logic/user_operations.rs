use super::error::UserError;
use crate::routes::dto::NewUserRequest;
use entity::user::{self, Entity as User};
use pwhash::bcrypt;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn find_user_by_username(
    conn: &DatabaseConnection,
    username: String,
) -> Result<user::Model, UserError> {
    let opt = User::find()
        .filter(user::Column::Username.eq(username.clone()))
        .one(conn)
        .await?;
    match opt {
        Some(user) => Ok(user),
        None => Err(UserError::UserNotFound(username)),
    }
}

pub async fn find_user_by_id(conn: &DatabaseConnection, id: i32) -> Result<user::Model, UserError> {
    match User::find_by_id(id).one(conn).await? {
        Some(user) => Ok(user),
        None => Err(UserError::UserNotFound(id.to_string())),
    }
}

pub async fn save_user(
    conn: &DatabaseConnection,
    req: NewUserRequest,
) -> Result<user::Model, UserError> {
    match User::find()
        .filter(user::Column::Email.eq(req.email.clone()))
        .one(conn)
        .await?
    {
        None => {
            let pw_hash = bcrypt::hash(req.password)?;
            let user = user::ActiveModel {
                id: NotSet,
                email: Set(req.email),
                username: Set(req.username),
                pw_hash: Set(pw_hash),
            };
            let user = user.insert(conn).await?;
            // TODO activation logic and email sending
            Ok(user)
        }
        Some(_) => Err(UserError::EmailAlreadyExists(req.email)),
    }
}

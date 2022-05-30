use entity::user::{self, Entity as User};

use pwhash::{bcrypt, error::Error as PwHashError};
use sea_orm::error::DbErr;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize)]
pub struct NewUser {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Error, Debug)]
pub enum UserError {
    #[error("database error occured: {0}")]
    DatabaseError(#[from] DbErr),
    #[error("the user with key {0} could not be found")]
    NotFound(String),
    #[error("the password could not be hashed: {0}")]
    PasswordHashError(#[from] PwHashError),
}

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
        None => Err(UserError::NotFound(username)),
    }
}

pub async fn find_user_by_id(conn: &DatabaseConnection, id: i32) -> Result<user::Model, UserError> {
    match User::find_by_id(id).one(conn).await? {
        Some(user) => Ok(user),
        None => Err(UserError::NotFound(id.to_string())),
    }
}

pub async fn save_user(
    conn: &DatabaseConnection,
    new_user: NewUser,
) -> Result<user::Model, UserError> {
    let pw_hash = bcrypt::hash(new_user.password)?;
    let user = user::ActiveModel {
        id: NotSet,
        email: Set(new_user.email),
        username: Set(new_user.username),
        pw_hash: Set(pw_hash),
    };
    let user = user.insert(conn).await?;
    // TODO activation logic and email sending
    Ok(user)
}

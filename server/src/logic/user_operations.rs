use entity::user::{self, Entity as User};

use sea_orm::error::DbErr;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("database error occured: {0:?}")]
    DatabaseError(DbErr),
    #[error("the user with key {0} could not be found")]
    NotFound(String),
}

impl From<DbErr> for UserError {
    fn from(e: DbErr) -> Self {
        UserError::DatabaseError(e)
    }
}

pub async fn find_user_by_username(
    conn: &DatabaseConnection,
    username: String,
) -> Result<user::Model, UserError> {
    let result = User::find()
        .filter(user::Column::Username.eq(username.clone()))
        .one(conn)
        .await;

    match result {
        Ok(opt) => match opt {
            Some(user) => Ok(user),
            None => Err(UserError::NotFound(username)),
        },
        Err(e) => Err(e.into()),
    }
}

pub async fn find_user_by_id(conn: &DatabaseConnection, id: i32) -> Result<user::Model, UserError> {
    let result = User::find_by_id(id).one(conn).await;
    match result {
        Ok(opt) => match opt {
            Some(user) => Ok(user),
            None => Err(UserError::NotFound(id.to_string())),
        },
        Err(e) => Err(e.into()),
    }
}

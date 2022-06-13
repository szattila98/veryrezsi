use pwhash::error::Error as PwHashError;
use sea_orm::error::DbErr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("user with identifier '{0}' not found")]
    UserNotFound(String),
    #[error("user with this email '{0}' already exists")]
    EmailAlreadyExists(String),
    #[error("password could not be hashed")]
    PasswordHashError(#[from] PwHashError),
    #[error("database error")]
    DatabaseError(#[from] DbErr),
}

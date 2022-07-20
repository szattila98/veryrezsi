use pwhash::error::Error as PwHashError;
use sea_orm::{error::DbErr, TransactionError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("user with identifier '{0}' not found")]
    UserNotFound(String),
    #[error("user with this email '{0}' already exists")]
    EmailAlreadyExists(String),
    #[error("password could not be hashed")]
    PasswordCannotBeHashed(#[from] PwHashError),
    #[error("activation token '{0}' not found")]
    ActivationTokenNotFound(String),
    #[error("database error: '{0}'")]
    DatabaseError(#[from] DbErr),
}

impl From<TransactionError<UserError>> for UserError {
    fn from(e: TransactionError<UserError>) -> Self {
        match e {
            TransactionError::Connection(e) => e.into(),
            TransactionError::Transaction(e) => e,
        }
    }
}

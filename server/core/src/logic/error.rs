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
    #[error("activation token expired")]
    ActivationTokenExpired,
    #[error("user has no right to complete action")]
    UserHasNoRightForAction,
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

#[derive(Error, Debug)]
pub enum ExpenseError {
    #[error("some data provided for expense creation is invalid or cannot be parsed: '{0}'")]
    InvalidExpenseData(String),
    #[error("database error: '{0}'")]
    DatabaseError(#[from] DbErr),
}

impl From<TransactionError<ExpenseError>> for ExpenseError {
    fn from(e: TransactionError<ExpenseError>) -> Self {
        match e {
            TransactionError::Connection(e) => e.into(),
            TransactionError::Transaction(e) => e,
        }
    }
}

impl From<chrono::ParseError> for ExpenseError {
    fn from(e: chrono::ParseError) -> Self {
        ExpenseError::InvalidExpenseData(e.to_string())
    }
}

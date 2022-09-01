#![allow(missing_docs)]

use pwhash::error::Error as PwHashError;
use sea_orm::{error::DbErr, TransactionError};
use thiserror::Error;

/// Errors that can happen during the execution of user logic.
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
    #[error("database error: '{0}'")]
    DatabaseError(#[from] DbErr),
}

#[derive(Error, Debug)]
pub enum ExpenseError {
    #[error("No expense found for user '{0}'")]
    NoExpenseFoundForUser(String),
}

impl From<TransactionError<UserError>> for UserError {
    fn from(e: TransactionError<UserError>) -> Self {
        match e {
            TransactionError::Connection(e) => e.into(),
            TransactionError::Transaction(e) => e,
        }
    }
}

impl From<TransactionError<ExpenseError>> for ExpenseError {
    fn from(e: TransactionError<ExpenseError>) -> Self {
        match e {
            TransactionError::Connection(e) => e.into(),
            TransactionError::Transaction(e) => e,
        }
    }
}
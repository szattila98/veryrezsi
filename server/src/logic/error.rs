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

impl From<TransactionError<UserError>> for UserError {
    fn from(e: TransactionError<UserError>) -> Self {
        match e {
            TransactionError::Connection(e) => e.into(),
            TransactionError::Transaction(e) => e,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_connection_err_converts_to_database_user_err() {
        let dbe = DbErr::Custom("test".to_string());
        let tre = TransactionError::<UserError>::Connection(dbe);
        let ue = UserError::from(tre);
        let res = match ue {
            UserError::DatabaseError(e) => match e {
                DbErr::Custom(s) => s == "test".to_string(),
                _ => false,
            },
            _ => false,
        };
        assert!(res);
    }

    #[test]
    fn transaction_err_returns_user_err() {
        let ue = UserError::UserNotFound("test".to_string());
        let tre = TransactionError::<UserError>::Transaction(ue);
        let ue = UserError::from(tre);
        let res = match ue {
            UserError::UserNotFound(e) => e == "test".to_string(),
            _ => false,
        };
        assert!(res);

        let ue = UserError::EmailAlreadyExists("test".to_string());
        let tre = TransactionError::<UserError>::Transaction(ue);
        let ue = UserError::from(tre);
        let res = match ue {
            UserError::EmailAlreadyExists(e) => e == "test".to_string(),
            _ => false,
        };
        assert!(res);

        let ue = UserError::PasswordCannotBeHashed(PwHashError::RandomError("test".to_string()));
        let tre = TransactionError::<UserError>::Transaction(ue);
        let ue = UserError::from(tre);
        let res = match ue {
            UserError::PasswordCannotBeHashed(e) => e.to_string() == "test".to_string(),
            _ => false,
        };
        assert!(res);

        let ue = UserError::ActivationTokenNotFound("test".to_string());
        let tre = TransactionError::<UserError>::Transaction(ue);
        let ue = UserError::from(tre);
        let res = match ue {
            UserError::ActivationTokenNotFound(e) => e == "test".to_string(),
            _ => false,
        };
        assert!(res);

        let ue = UserError::ActivationTokenExpired;
        let tre = TransactionError::<UserError>::Transaction(ue);
        let ue = UserError::from(tre);
        let res = match ue {
            UserError::ActivationTokenExpired => true,
            _ => false,
        };
        assert!(res)
    }
}

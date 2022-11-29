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

#[derive(Error, Debug)]
pub enum ExpenseTransactionError {
    #[error("some data provided for transaction creation is invalid or cannot be parsed: '{0}'")]
    InvalidTransactionData(String),
    #[error("the transaction requested to be deleted does not exist")]
    TransactionToDeleteDoesNotExist,
    #[error("you do not have permission for the parent expense: '{0}'")]
    ParentExpenseIsNotOwnedByTheUser(String),
    #[error("database error: '{0}'")]
    DatabaseError(#[from] DbErr),
}

impl From<UserError> for ExpenseTransactionError {
    fn from(e: UserError) -> Self {
        ExpenseTransactionError::ParentExpenseIsNotOwnedByTheUser(e.to_string())
    }
}

impl From<chrono::ParseError> for ExpenseTransactionError {
    fn from(e: chrono::ParseError) -> Self {
        ExpenseTransactionError::InvalidTransactionData(e.to_string())
    }
}

#[derive(Error, Debug)]
pub enum CurrencyError {
    #[error("database error: '{0}'")]
    DatabaseError(#[from] DbErr),
}

#[derive(Error, Debug)]
pub enum RecurrenceError {
    #[error("database error: '{0}'")]
    DatabaseError(#[from] DbErr),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transaction_connection_err_converts_to_database_user_err() {
        let dbe = DbErr::Custom("SqMPqEYywurqXiDEKzNU".to_string());
        let tre = TransactionError::<UserError>::Connection(dbe);
        let ue = UserError::from(tre);
        let res = match ue {
            UserError::DatabaseError(e) => match e {
                DbErr::Custom(s) => s == "SqMPqEYywurqXiDEKzNU".to_string(),
                _ => false,
            },
            _ => false,
        };
        assert!(res);
    }

    #[test]
    fn transaction_err_returns_user_err() {
        let ue = UserError::UserNotFound("LMMOVfLkzaTedkqPnyam".to_string());
        let tre = TransactionError::<UserError>::Transaction(ue);
        let ue = UserError::from(tre);
        let res = match ue {
            UserError::UserNotFound(e) => e == "LMMOVfLkzaTedkqPnyam".to_string(),
            _ => false,
        };
        assert!(res);

        let ue = UserError::EmailAlreadyExists("fTsobToWEkJETIpefNLN".to_string());
        let tre = TransactionError::<UserError>::Transaction(ue);
        let ue = UserError::from(tre);
        let res = match ue {
            UserError::EmailAlreadyExists(e) => e == "fTsobToWEkJETIpefNLN".to_string(),
            _ => false,
        };
        assert!(res);

        let ue = UserError::PasswordCannotBeHashed(PwHashError::RandomError(
            "temtwhEzvTaLnbWLGXmZJWst".to_string(),
        ));
        let tre = TransactionError::<UserError>::Transaction(ue);
        let ue = UserError::from(tre);
        let res = match ue {
            UserError::PasswordCannotBeHashed(e) => {
                e.to_string() == "temtwhEzvTaLnbWLGXmZJWst".to_string()
            }
            _ => false,
        };
        assert!(res);

        let ue = UserError::ActivationTokenNotFound("teBlwJsmuUsAmiogKmdsecst".to_string());
        let tre = TransactionError::<UserError>::Transaction(ue);
        let ue = UserError::from(tre);
        let res = match ue {
            UserError::ActivationTokenNotFound(e) => e == "teBlwJsmuUsAmiogKmdsecst".to_string(),
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

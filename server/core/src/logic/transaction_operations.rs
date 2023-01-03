use self::errors::{CreateTransactionError, DeleteTransactionByIdError};

use super::common;
use super::user_operations::authorize_user_by_id;
use crate::dto::transactions::NewTransactionRequest;
use crate::logic::currency_operations::find_currency_type_by_id;
use crate::logic::expense_operations::find_expense_by_id;

use entity::transaction::{self, Entity as Transaction};
use entity::Id;

use chrono::NaiveDate;
use migration::DbErr;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

pub async fn create_transaction(
    conn: &DatabaseConnection,
    user_id: Id,
    req: NewTransactionRequest,
) -> Result<Id, CreateTransactionError> {
    let (expense_result, currency_result) = tokio::join!(
        find_expense_by_id(conn, req.expense_id),
        find_currency_type_by_id(conn, req.currency_type_id)
    );
    let Some(expense) = expense_result? else {
        return Err(CreateTransactionError::InvalidExpenseId);
    };
    let Some(_) = currency_result? else {
        return Err(CreateTransactionError::InvalidCurrency);
    };
    authorize_user_by_id(user_id, expense.user_id)?;

    let parsed_date = NaiveDate::parse_from_str(&req.date, common::DATE_FORMAT)?;
    let transaction = transaction::ActiveModel {
        id: NotSet,
        donor_name: Set(req.donor_name),
        currency_type_id: Set(req.currency_type_id),
        value: Set(req.value),
        date: Set(parsed_date),
        expense_id: Set(req.expense_id),
    };
    let transaction = transaction.insert(conn).await?;
    Ok(transaction.id)
}

pub async fn delete_transaction_by_id(
    conn: &DatabaseConnection,
    user_id: Id,
    transaction_id: Id,
) -> Result<(), DeleteTransactionByIdError> {
    let Some(transaction) = find_transaction_by_id(conn, transaction_id).await? else {
        return Err(DeleteTransactionByIdError::InvalidTransaction);
    };
    let Some(expense) = find_expense_by_id(conn, transaction.expense_id).await? else {
        return Err(DeleteTransactionByIdError::InvalidExpenseId);
    };
    authorize_user_by_id(user_id, expense.user_id)?;

    Transaction::delete_by_id(transaction_id).exec(conn).await?;
    Ok(())
}

async fn find_transaction_by_id(
    conn: &DatabaseConnection,
    transaction_id: Id,
) -> Result<Option<transaction::Model>, DbErr> {
    let transaction = Transaction::find_by_id(transaction_id).one(conn).await?;
    Ok(transaction)
}

pub mod errors {
    use migration::DbErr;
    use thiserror::Error;

    use crate::logic::user_operations::errors::AuthorizeUserError;

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum CreateTransactionError {
        #[error("expense id is invalid")]
        InvalidExpenseId,
        #[error("currency type is invalid")]
        InvalidCurrency,
        #[error("user is not authorized")]
        UserUnauthorized(#[from] AuthorizeUserError),
        #[error("start_date could not be parsed")]
        InvalidStartDate(#[from] chrono::ParseError),
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum DeleteTransactionByIdError {
        #[error("transaction id is invalid")]
        InvalidTransaction,
        #[error("expense id is invalid")]
        InvalidExpenseId,
        #[error("{0}")]
        UserUnauthorized(#[from] AuthorizeUserError),
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::DbErr;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[tokio::test]
    async fn create_transaction_happy_path() {}

    #[tokio::test]
    async fn create_transaction_error_cases() {}

    #[tokio::test]
    async fn create_transaction_db_error_cases() {}

    #[tokio::test]
    async fn delete_transaction_happy_path() {}

    #[tokio::test]
    async fn delete_transaction_error_cases() {}

    #[tokio::test]
    async fn delete_transaction_db_error_cases() {}

    #[tokio::test]
    async fn get_transaction_by_id_if_exists_all_cases() {}
}

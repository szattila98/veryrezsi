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
        return Err(CreateTransactionError::InvalidCurrencyType);
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
        InvalidCurrencyType,
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
    use std::vec;

    use crate::logic::user_operations::errors::AuthorizeUserError;

    use super::*;
    use assert2::check;
    use entity::{currency_type, expense};
    use migration::DbErr;
    use sea_orm::{prelude::Decimal, DatabaseBackend, MockDatabase, MockExecResult};

    const TEST_STR: &str = "test";
    const TEST_ID: u64 = 1;
    const TEST_DATE: &str = "06-08-1998";

    fn test_decimal() -> Decimal {
        Decimal::new(1, 2)
    }

    #[tokio::test]
    async fn create_transaction_happy_path() {
        let mock_expense = expense::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: test_decimal(),
            start_date: NaiveDate::MIN,
            user_id: TEST_ID,
            currency_type_id: TEST_ID,
            recurrence_type_id: TEST_ID,
            predefined_expense_id: None,
        };
        let mock_currency_type = currency_type::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        };
        let mock_transaction = transaction::Model {
            id: TEST_ID,
            donor_name: TEST_STR.to_string(),
            value: test_decimal(),
            date: NaiveDate::MIN,
            currency_type_id: TEST_ID,
            expense_id: TEST_ID,
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![mock_expense.clone()]])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: TEST_ID,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![mock_transaction.clone()]])
            .into_connection();

        let saved_transaction_id = create_transaction(
            &conn,
            TEST_ID,
            NewTransactionRequest {
                donor_name: TEST_STR.to_string(),
                currency_type_id: TEST_ID,
                value: test_decimal(),
                date: TEST_DATE.to_string(),
                expense_id: TEST_ID,
            },
        )
        .await;

        check!(saved_transaction_id == Ok(TEST_ID));
    }

    #[tokio::test]
    async fn create_transaction_error_cases() {
        let mock_expense = expense::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: test_decimal(),
            start_date: NaiveDate::MIN,
            user_id: TEST_ID,
            currency_type_id: TEST_ID,
            recurrence_type_id: TEST_ID,
            predefined_expense_id: None,
        };
        let mock_currency_type = currency_type::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // invalid expense id
            .append_query_results::<expense::Model>(vec![vec![]])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            // invalid currency type id
            .append_query_results(vec![vec![mock_expense.clone()]])
            .append_query_results::<currency_type::Model>(vec![vec![]])
            // unauthorized
            .append_query_results(vec![vec![mock_expense.clone()]])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            // date cannot be parsed
            .append_query_results(vec![vec![mock_expense.clone()]])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            .into_connection();

        let req = NewTransactionRequest {
            donor_name: TEST_STR.to_string(),
            currency_type_id: TEST_ID,
            value: test_decimal(),
            date: "wrong_date".to_string(),
            expense_id: TEST_ID,
        };
        let (invalid_expense_id, invalid_currency_type_id, user_unauthorized, invalid_start_date) = tokio::join!(
            create_transaction(&conn, TEST_ID, req.clone()),
            create_transaction(&conn, TEST_ID, req.clone()),
            create_transaction(&conn, TEST_ID - 1, req.clone()),
            create_transaction(&conn, TEST_ID, req),
        );

        check!(invalid_expense_id == Err(CreateTransactionError::InvalidExpenseId));
        check!(invalid_currency_type_id == Err(CreateTransactionError::InvalidCurrencyType));
        check!(
            user_unauthorized == Err(CreateTransactionError::UserUnauthorized(AuthorizeUserError))
        );
        check!(let Err(CreateTransactionError::InvalidStartDate(_)) = invalid_start_date);
    }

    #[tokio::test]
    async fn create_transaction_db_error_cases() {
        let mock_expense = expense::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: test_decimal(),
            start_date: NaiveDate::MIN,
            user_id: TEST_ID,
            currency_type_id: TEST_ID,
            recurrence_type_id: TEST_ID,
            predefined_expense_id: None,
        };
        let mock_currency_type = currency_type::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // expense query db error
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            // currency type query db error
            .append_query_results(vec![vec![mock_expense.clone()]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            // transaction insert db error
            .append_query_results(vec![vec![mock_expense.clone()]])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            .append_exec_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let req = NewTransactionRequest {
            donor_name: TEST_STR.to_string(),
            currency_type_id: TEST_ID,
            value: test_decimal(),
            date: TEST_DATE.to_string(),
            expense_id: TEST_ID,
        };
        let (expense_db_error, currency_type_db_error, transaction_insert_db_error) = tokio::join!(
            create_transaction(&conn, TEST_ID, req.clone()),
            create_transaction(&conn, TEST_ID, req.clone()),
            create_transaction(&conn, TEST_ID, req),
        );

        let db_error = Err(CreateTransactionError::DatabaseError(DbErr::Custom(
            TEST_STR.to_string(),
        )));
        check!(expense_db_error == db_error);
        check!(currency_type_db_error == db_error);
        check!(transaction_insert_db_error == db_error);
    }

    #[tokio::test]
    async fn delete_transaction_happy_path() {}

    #[tokio::test]
    async fn delete_transaction_error_cases() {}

    #[tokio::test]
    async fn delete_transaction_db_error_cases() {}

    #[tokio::test]
    async fn get_transaction_by_id_if_exists_all_cases() {
        let mock_transaction = transaction::Model {
            id: TEST_ID,
            donor_name: TEST_STR.to_string(),
            value: test_decimal(),
            date: NaiveDate::MIN,
            currency_type_id: TEST_ID,
            expense_id: TEST_ID,
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![mock_transaction.clone()]])
            .append_query_results::<transaction::Model>(vec![vec![]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let (found, not_found, db_error) = tokio::join!(
            find_transaction_by_id(&conn, TEST_ID),
            find_transaction_by_id(&conn, TEST_ID),
            find_transaction_by_id(&conn, TEST_ID),
        );

        check!(found == Ok(Some(mock_transaction)));
        check!(not_found == Ok(None));
        check!(db_error == Err(DbErr::Custom(TEST_STR.to_string())));
    }
}

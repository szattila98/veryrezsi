use self::errors::{CreateTransactionError, DeleteTransactionByIdError};

use super::common;
use super::user_operations::authorize_user;
use crate::dto::transactions::NewTransactionRequest;
use crate::logic::common::find_entity_by_id;

use entity::transaction::{self, Entity as Transaction};
use entity::{currency, expense, Id};

use chrono::NaiveDate;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};

pub async fn create_transaction(
    conn: &DatabaseConnection,
    user_id: Id,
    req: NewTransactionRequest,
) -> Result<Id, CreateTransactionError> {
    let (expense_result, currency_result) = tokio::join!(
        find_entity_by_id::<expense::Entity>(conn, req.expense_id),
        find_entity_by_id::<currency::Entity>(conn, req.currency_id)
    );
    let Some(expense) = expense_result? else {
        return Err(CreateTransactionError::InvalidExpenseId);
    };
    let Some(_) = currency_result? else {
        return Err(CreateTransactionError::InvalidCurrency);
    };
    authorize_user(user_id, expense.user_id)?;

    let parsed_date = NaiveDate::parse_from_str(&req.date, common::DATE_FORMAT)?;
    let transaction = transaction::ActiveModel {
        id: NotSet,
        donor_name: Set(req.donor_name),
        currency_id: Set(req.currency_id),
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
    let Some(transaction) = find_entity_by_id::<transaction::Entity>(conn, transaction_id).await? else {
        return Err(DeleteTransactionByIdError::InvalidTransaction);
    };
    let Some(expense) = find_entity_by_id::<expense::Entity>(conn, transaction.expense_id).await? else {
        return Err(DeleteTransactionByIdError::InvalidTransaction);
    };
    authorize_user(user_id, expense.user_id)?;

    Transaction::delete_by_id(transaction_id).exec(conn).await?;
    Ok(())
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
        #[error("{0}")]
        UserUnauthorized(#[from] AuthorizeUserError),
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use crate::logic::{
        user_operations::errors::AuthorizeUserError,
        common::tests::{
            TEST_ID,
            TEST_STR,
            TEST_DATE,
            TEST_DECIMAL,
            TEST_DB_ERROR,
            TEST_RECURRENCE,
        }
    };

    use super::*;
    use assert2::check;
    use entity::{currency, expense};
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

    #[tokio::test]
    async fn create_transaction_happy_path() {
        let mock_expense = expense::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: TEST_DECIMAL(),
            start_date: NaiveDate::MIN,
            user_id: TEST_ID,
            currency_id: TEST_ID,
            recurrence_id: TEST_ID,
            predefined_expense_id: None,
        };
        let mock_currency = currency::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        };
        let mock_transaction = transaction::Model {
            id: TEST_ID,
            donor_name: TEST_STR.to_string(),
            value: TEST_DECIMAL(),
            date: NaiveDate::MIN,
            currency_id: TEST_ID,
            expense_id: TEST_ID,
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![mock_expense.clone()]])
            .append_query_results(vec![vec![mock_currency.clone()]])
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
                currency_id: TEST_ID,
                value: TEST_DECIMAL(),
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
            value: TEST_DECIMAL(),
            start_date: NaiveDate::MIN,
            user_id: TEST_ID,
            currency_id: TEST_ID,
            recurrence_id: TEST_ID,
            predefined_expense_id: None,
        };
        let mock_currency = currency::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // invalid expense id
            .append_query_results(vec![Vec::<expense::Model>::new()])
            .append_query_results(vec![vec![mock_currency.clone()]])
            // invalid currency type id
            .append_query_results(vec![vec![mock_expense.clone()]])
            .append_query_results(vec![Vec::<currency::Model>::new()])
            // unauthorized
            .append_query_results(vec![vec![mock_expense.clone()]])
            .append_query_results(vec![vec![mock_currency.clone()]])
            // date cannot be parsed
            .append_query_results(vec![vec![mock_expense.clone()]])
            .append_query_results(vec![vec![mock_currency.clone()]])
            .into_connection();

        let req = NewTransactionRequest {
            donor_name: TEST_STR.to_string(),
            currency_id: TEST_ID,
            value: TEST_DECIMAL(),
            date: "wrong_date".to_string(),
            expense_id: TEST_ID,
        };
        let (invalid_expense_id, invalid_currency_id, user_unauthorized, invalid_start_date) = tokio::join!(
            create_transaction(&conn, TEST_ID, req.clone()),
            create_transaction(&conn, TEST_ID, req.clone()),
            create_transaction(&conn, TEST_ID - 1, req.clone()),
            create_transaction(&conn, TEST_ID, req),
        );

        check!(invalid_expense_id == Err(CreateTransactionError::InvalidExpenseId));
        check!(invalid_currency_id == Err(CreateTransactionError::InvalidCurrency));
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
            value: TEST_DECIMAL(),
            start_date: NaiveDate::MIN,
            user_id: TEST_ID,
            currency_id: TEST_ID,
            recurrence_id: TEST_ID,
            predefined_expense_id: None,
        };
        let mock_currency = currency::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // expense query db error
            .append_query_errors(vec![TEST_DB_ERROR()])
            .append_query_results(vec![vec![mock_currency.clone()]])
            // currency type query db error
            .append_query_results(vec![vec![mock_expense.clone()]])
            .append_query_errors(vec![TEST_DB_ERROR()])
            // transaction insert db error
            .append_query_results(vec![vec![mock_expense.clone()]])
            .append_query_results(vec![vec![mock_currency.clone()]])
            .append_exec_errors(vec![TEST_DB_ERROR()])
            .into_connection();

        let req = NewTransactionRequest {
            donor_name: TEST_STR.to_string(),
            currency_id: TEST_ID,
            value: TEST_DECIMAL(),
            date: TEST_DATE.to_string(),
            expense_id: TEST_ID,
        };
        let (expense_db_error, currency_db_error, transaction_insert_db_error) = tokio::join!(
            create_transaction(&conn, TEST_ID, req.clone()),
            create_transaction(&conn, TEST_ID, req.clone()),
            create_transaction(&conn, TEST_ID, req),
        );

        let db_error = Err(CreateTransactionError::DatabaseError(TEST_DB_ERROR()));
        check!(expense_db_error == db_error);
        check!(currency_db_error == db_error);
        check!(transaction_insert_db_error == db_error);
    }

    #[tokio::test]
    async fn delete_transaction_all_cases() {
        let mock_transaction = transaction::Model {
            id: TEST_ID,
            value: TEST_DECIMAL(),
            currency_id: TEST_ID,
            expense_id: TEST_ID,
            date: NaiveDate::MIN,
            donor_name: TEST_STR.to_string(),
        };
        let mock_expense = expense::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: TEST_DECIMAL(),
            start_date: NaiveDate::MIN,
            user_id: TEST_ID,
            currency_id: TEST_ID,
            recurrence_id: TEST_ID,
            predefined_expense_id: None,
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // happy case
            .append_query_results(vec![vec![mock_transaction.clone()]])
            .append_query_results(vec![vec![mock_expense.clone()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: TEST_ID,
                rows_affected: 1,
            }])
            // transaction not found
            .append_query_results(vec![Vec::<transaction::Model>::new()])
            // expense not found
            .append_query_results(vec![vec![mock_transaction.clone()]])
            .append_query_results(vec![Vec::<expense::Model>::new()])
            // user unauthorized
            .append_query_results(vec![vec![mock_transaction.clone()]])
            .append_query_results(vec![vec![mock_expense.clone()]])
            // transaction query db error
            .append_query_errors(vec![TEST_DB_ERROR()])
            // expense query db error
            .append_query_results(vec![vec![mock_transaction.clone()]])
            .append_query_errors(vec![TEST_DB_ERROR()])
            // transaction delete db error
            .append_query_results(vec![vec![mock_transaction.clone()]])
            .append_query_results(vec![vec![mock_expense]])
            .append_exec_errors(vec![TEST_DB_ERROR()])
            .into_connection();

        let (
            happy_case,
            transaction_not_found,
            expense_not_found,
            user_unauthorized,
            transaction_query_db_error,
            expense_query_db_error,
            transaction_delete_db_error,
        ) = tokio::join!(
            delete_transaction_by_id(&conn, TEST_ID, TEST_ID),
            delete_transaction_by_id(&conn, TEST_ID, TEST_ID),
            delete_transaction_by_id(&conn, TEST_ID, TEST_ID),
            delete_transaction_by_id(&conn, TEST_ID + 1, TEST_ID),
            delete_transaction_by_id(&conn, TEST_ID, TEST_ID),
            delete_transaction_by_id(&conn, TEST_ID, TEST_ID),
            delete_transaction_by_id(&conn, TEST_ID, TEST_ID),
        );

        let db_error = Err(DeleteTransactionByIdError::DatabaseError(TEST_DB_ERROR()));
        check!(happy_case == Ok(()));
        check!(transaction_not_found == Err(DeleteTransactionByIdError::InvalidTransaction));
        check!(expense_not_found == Err(DeleteTransactionByIdError::InvalidTransaction));
        check!(
            user_unauthorized
                == Err(DeleteTransactionByIdError::UserUnauthorized(
                    AuthorizeUserError
                ))
        );
        check!(transaction_query_db_error == db_error);
        check!(expense_query_db_error == db_error);
        check!(transaction_delete_db_error == db_error);
    }
}

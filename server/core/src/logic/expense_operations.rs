use std::collections::VecDeque;

use self::errors::{
    CreateExpenseError, CreatePredefinedExpenseError, FindExpensesWithTransactionsByUserIdError,
    ValidateRecurrenceAndCurrencyError,
};

use super::common;
use super::user_operations::authorize_user;
use crate::dto::expenses::{
    ExpenseResponse, NewExpenseRequest, NewPredefinedExpenseRequest, PredefinedExpenseResponse,
};
use crate::logic::common::find_entity_by_id;

use entity::expense::{self, Entity as Expense};
use entity::predefined_expense::{self, Entity as PredefinedExpense};
use entity::transaction;
use entity::{currency, recurrence, Id};

use chrono::NaiveDate;
use migration::DbErr;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, LoaderTrait, QueryFilter, Set,
};

pub async fn find_expenses_by_user_id(
    conn: &DatabaseConnection,
    authenticated_user_id: Id,
    user_id: Id,
) -> Result<Vec<ExpenseResponse>, FindExpensesWithTransactionsByUserIdError> {
    authorize_user(authenticated_user_id, user_id)?;
    let expenses = Expense::find()
        .filter(expense::Column::UserId.eq(user_id))
        .all(conn)
        .await?;
    let (predefined_expenses, grouped_transactions, currencies, recurrences) = tokio::join!(
        expenses.load_one(predefined_expense::Entity, conn),
        expenses.load_many(transaction::Entity, conn),
        currency::Entity::find().all(conn),
        recurrence::Entity::find().all(conn)
    );
    let mut predefined_expenses: VecDeque<Option<predefined_expense::Model>> =
        predefined_expenses?.into();
    let mut grouped_transactions: VecDeque<Vec<transaction::Model>> =
        grouped_transactions?.into_iter().collect();
    let currencies = currencies?;
    let recurrences = recurrences?;

    assert!(
        vec![predefined_expenses.len(), grouped_transactions.len()]
            .iter()
            .all(|&x| x == expenses.len()),
        "the lengths of the fetched expense related lists should always be equal"
    );

    let expense_parts = expenses.into_iter().map(|expense| {
        let currency = find_currency(&currencies, expense.currency_id);
        let recurrence = find_recurrence(&recurrences, expense.recurrence_id);

        let predefined_expense = match predefined_expenses
            .pop_front()
            .expect("predefined expense queue should not be empty")
        {
            Some(predefined_expense) => {
                let currency = find_currency(&currencies, predefined_expense.currency_id);
                let recurrence = find_recurrence(&recurrences, predefined_expense.recurrence_id);
                Some((predefined_expense, currency, recurrence))
            }
            None => None,
        };

        let transactions = grouped_transactions
            .pop_front()
            .expect("grouped transactions queue should not be empty");
        let transaction_parts = transactions
            .into_iter()
            .map(|transaction| {
                let transaction_currency = find_currency(&currencies, transaction.currency_id);
                (transaction, transaction_currency)
            })
            .collect();

        (
            expense,
            currency,
            recurrence,
            predefined_expense,
            transaction_parts,
        )
    });

    let expense_responses = expense_parts
        .into_iter()
        .map(|parts| parts.into())
        .collect();

    Ok(expense_responses)
}

pub async fn create_expense(
    conn: &DatabaseConnection,
    user_id: Id,
    req: NewExpenseRequest,
) -> Result<Id, CreateExpenseError> {
    if let Some(predefined_expense_id) = req.predefined_expense_id {
        let opt = PredefinedExpense::find()
            .filter(predefined_expense::Column::Id.eq(predefined_expense_id))
            .one(conn)
            .await?;
        let Some(_) = opt else {
            return Err(CreateExpenseError::InvalidPredefinedExpense);
        };
    }
    validate_recurrence_and_currency(conn, req.currency_id, req.recurrence_id).await?;
    let parsed_date = NaiveDate::parse_from_str(&req.start_date, common::DATE_FORMAT)?;
    let expense = expense::ActiveModel {
        id: NotSet,
        name: Set(req.name),
        description: Set(req.description),
        recurrence_id: Set(req.recurrence_id),
        currency_id: Set(req.currency_id),
        predefined_expense_id: Set(req.predefined_expense_id),
        start_date: Set(parsed_date),
        user_id: Set(user_id),
        value: Set(req.value),
    };
    let expense = expense.insert(conn).await?;
    Ok(expense.id)
}

pub async fn find_predefined_expenses(
    conn: &DatabaseConnection,
) -> Result<Vec<PredefinedExpenseResponse>, DbErr> {
    let predefined_expenses = PredefinedExpense::find().all(conn).await?;
    let (currencies, recurrences) = tokio::join!(
        predefined_expenses.load_one(currency::Entity, conn),
        predefined_expenses.load_one(recurrence::Entity, conn)
    );
    let mut currencies: VecDeque<currency::Model> = currencies?.into_iter().flatten().collect();
    let mut recurrences: VecDeque<recurrence::Model> = recurrences?.into_iter().flatten().collect();

    assert!(
        vec![currencies.len(), recurrences.len()]
            .iter()
            .all(|&x| x == predefined_expenses.len()),
        "the lengths of the fetched predefined expense related lists should be equal"
    );

    let predefined_expense_response = predefined_expenses
        .into_iter()
        .map(|predefined_expense| {
            (
                predefined_expense,
                currencies
                    .pop_front()
                    .expect("currency queue should not be empty"),
                recurrences
                    .pop_front()
                    .expect("recurrence queue should not be empty"),
            )
        })
        .map(|parts| parts.into())
        .collect();

    Ok(predefined_expense_response)
}

pub async fn create_predefined_expense(
    conn: &DatabaseConnection,
    req: NewPredefinedExpenseRequest,
) -> Result<Id, CreatePredefinedExpenseError> {
    validate_recurrence_and_currency(conn, req.currency_id, req.recurrence_id).await?;
    let predefined_expense = predefined_expense::ActiveModel {
        id: NotSet,
        name: Set(req.name),
        description: Set(req.description),
        value: Set(req.value),
        currency_id: Set(req.currency_id),
        recurrence_id: Set(req.recurrence_id),
    };
    let predefined_expense = predefined_expense.insert(conn).await?;
    Ok(predefined_expense.id)
}

async fn validate_recurrence_and_currency(
    conn: &DatabaseConnection,
    currency_id: Id,
    recurrence_id: Id,
) -> Result<(), ValidateRecurrenceAndCurrencyError> {
    let (referred_recurrence, referred_currency) = tokio::join!(
        find_entity_by_id::<recurrence::Entity>(conn, recurrence_id),
        find_entity_by_id::<currency::Entity>(conn, currency_id)
    );
    let Some(_) = referred_currency? else {
        return Err(ValidateRecurrenceAndCurrencyError::InvalidCurrency)
    };
    let Some(_) = referred_recurrence? else {
        return Err(ValidateRecurrenceAndCurrencyError::InvalidRecurrence)
    };
    Ok(())
}

fn find_currency(currencies: &Vec<currency::Model>, id: Id) -> currency::Model {
    currencies
        .iter()
        .find(|currency| currency.id == id)
        .expect("the currency should be in the database")
        .clone()
}

fn find_recurrence(recurrences: &Vec<recurrence::Model>, id: Id) -> recurrence::Model {
    recurrences
        .iter()
        .find(|recurrence| recurrence.id == id)
        .expect("the currency should be in the database")
        .clone()
}

pub mod errors {
    use migration::DbErr;
    use thiserror::Error;

    use crate::logic::user_operations::errors::AuthorizeUserError;

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum FindExpensesWithTransactionsByUserIdError {
        #[error("{0}")]
        UnauthorizedUser(#[from] AuthorizeUserError),
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum CreateExpenseError {
        #[error("predefined expense is invalid")]
        InvalidPredefinedExpense,
        #[error("start_date could not be parsed")]
        InvalidStartDate(#[from] chrono::ParseError),
        #[error("invalid related type: '{0}'")]
        InvalidRelatedType(#[from] ValidateRecurrenceAndCurrencyError),
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum CreatePredefinedExpenseError {
        #[error("invalid related type: '{0}'")]
        InvalidRelatedType(#[from] ValidateRecurrenceAndCurrencyError),
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum ValidateRecurrenceAndCurrencyError {
        #[error("currency type is invalid")]
        InvalidCurrency,
        #[error("recurrence type is invalid")]
        InvalidRecurrence,
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        logic::user_operations::errors::AuthorizeUserError,
        dto::{
            currencies::CurrencyResponse,
            recurrences::RecurrenceResponse,
            expenses::PredefinedExpenseResponse,
            transactions::TransactionResponse
        }
    };

    use super::*;
    use assert2::check;
    use entity::{currency, recurrence, transaction};
    use migration::DbErr;
    use sea_orm::{prelude::Decimal, DatabaseBackend, MockDatabase, MockExecResult};

    const TEST_STR: &str = "test";
    const TEST_ID: u64 = 1;
    const TEST_FLOAT: f64 = 1.0;
    const TEST_DATE: &str = "06-08-1998";

    const TEST_EXPENSE: expense::Model = expense::Model {
        id: TEST_ID,
        name: TEST_STR.to_string(),
        description: TEST_STR.to_string(),
        value: TEST_DECIMAL(),
        start_date: NaiveDate::MIN,
        user_id: TEST_ID,
        currency_id: TEST_ID,
        recurrence_id: TEST_ID,
        predefined_expense_id: Some(TEST_ID),
    };
    const TEST_PREDEFINED_EXPENSE: predefined_expense::Model = predefined_expense::Model {
        id: TEST_ID,
        name: TEST_STR.to_string(),
        description: TEST_STR.to_string(),
        value: TEST_DECIMAL(),
        currency_id: TEST_ID,
        recurrence_id: TEST_ID,
    };
    const TEST_CURRENCY: currency::Model = currency::Model {
        id: TEST_ID,
        name: TEST_STR.to_string(),
        abbreviation: TEST_STR.to_string(),
    };
    const TEST_RECURRENCE: recurrence::Model = recurrence::Model {
        id: TEST_ID,
        name: TEST_STR.to_string(),
        per_year: TEST_FLOAT,
    };
    const TEST_TRANSACTION: transaction::Model = transaction::Model {
        id: TEST_ID,
        donor_name: TEST_STR.to_string(),
        value: TEST_DECIMAL(),
        date: NaiveDate::MIN,
        currency_id: TEST_ID,
        expense_id: TEST_ID,
    };
    const TEST_TRANSACTION_2: transaction::Model = transaction::Model {
        id: TEST_TRANSACTION.id + 1,
        ..TEST_TRANSACTION
    };

    const fn TEST_DECIMAL() -> Decimal {
        Decimal::new(1, 2)
    }

    const fn TEST_DB_ERROR() -> DbErr {
        DbErr::Custom(TEST_STR.to_string())
    }

    #[tokio::test]
    async fn find_expenses_by_user_id_all_cases() {
        let expected_currency: CurrencyResponse = TEST_CURRENCY.into();
        let expected_recurrence: RecurrenceResponse = TEST_RECURRENCE.into();
        let expected_predefined_expense: PredefinedExpenseResponse =
            (TEST_PREDEFINED_EXPENSE, TEST_CURRENCY, TEST_RECURRENCE).into();
        let expected_transaction: TransactionResponse = (TEST_TRANSACTION, TEST_CURRENCY).into();
        let expected_transaction_2: TransactionResponse = (TEST_TRANSACTION_2, TEST_CURRENCY).into();
        let expected_expenses = vec![ExpenseResponse {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: TEST_DECIMAL(),
            start_date: NaiveDate::MIN.to_string(),
            user_id: TEST_ID,
            currency: expected_currency,
            recurrence: expected_recurrence,
            predefined_expense: Some(expected_predefined_expense),
            transactions: vec![expected_transaction, expected_transaction_2],
        }];

        let expenses_with_transaction_stub = vec![
            (TEST_EXPENSE.clone(), TEST_TRANSACTION.clone()),
            (TEST_EXPENSE.clone(), TEST_TRANSACTION_2.clone()),
        ];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![expenses_with_transaction_stub, vec![]])
            .append_query_errors(vec![TEST_DB_ERROR()])
            .into_connection();

        let (
            expenses,
            empty_expenses,
            unauthorized_error,
            db_error
        ) = tokio::join!(
            find_expenses_by_user_id(&conn, TEST_ID, TEST_ID),
            find_expenses_by_user_id(&conn, TEST_ID, TEST_ID),
            find_expenses_by_user_id(&conn, TEST_ID, TEST_ID + 1),
            find_expenses_by_user_id(&conn, TEST_ID, TEST_ID)
        );

        check!(expenses == Ok(expected_expenses));
        check!(empty_expenses == Ok(vec![]));
        check!(
            unauthorized_error
                == Err(FindExpensesWithTransactionsByUserIdError::UnauthorizedUser(
                    AuthorizeUserError
                ))
        );
        check!(
            db_error
                == Err(FindExpensesWithTransactionsByUserIdError::DatabaseError(
                    TEST_DB_ERROR()
                ))
        );
    }

    #[tokio::test]
    async fn create_expense_happy_path() {
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![TEST_RECURRENCE]])
            .append_query_results(vec![vec![TEST_CURRENCY]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: TEST_ID,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![TEST_EXPENSE]])
            .into_connection();

        let saved_expense_id = create_expense(
            &conn,
            TEST_ID,
            NewExpenseRequest {
                name: TEST_STR.to_string(),
                description: TEST_STR.to_string(),
                value: TEST_DECIMAL(),
                start_date: TEST_DATE.to_string(),
                currency_id: TEST_ID,
                recurrence_id: TEST_ID,
                predefined_expense_id: None,
            },
        )
        .await;

        check!(saved_expense_id == Ok(TEST_ID));
    }

    #[tokio::test]
    async fn create_expense_happy_path_with_predefined_expense_id() {
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![TEST_PREDEFINED_EXPENSE]])
            .append_query_results(vec![vec![TEST_RECURRENCE]])
            .append_query_results(vec![vec![TEST_CURRENCY]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: TEST_ID,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![TEST_EXPENSE]])
            .into_connection();

        let saved_expense_id = create_expense(
            &conn,
            TEST_ID,
            NewExpenseRequest {
                name: TEST_STR.to_string(),
                description: TEST_STR.to_string(),
                value: TEST_DECIMAL(),
                start_date: TEST_DATE.to_string(),
                currency_id: TEST_ID,
                recurrence_id: TEST_ID,
                predefined_expense_id: Some(TEST_ID),
            },
        )
        .await;

        check!(saved_expense_id == Ok(TEST_ID));
    }

    #[tokio::test]
    async fn create_expense_db_error_cases() {
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // predefined expense not found
            .append_query_results(vec![Vec::<predefined_expense::Model>::new()])
            // recurrence type not found
            .append_query_results(vec![vec![TEST_PREDEFINED_EXPENSE.clone()]])
            .append_query_results(vec![Vec::<recurrence::Model>::new()])
            .append_query_results(vec![vec![TEST_CURRENCY.clone()]])
            // currency type not found
            .append_query_results(vec![vec![TEST_PREDEFINED_EXPENSE.clone()]])
            .append_query_results(vec![vec![TEST_RECURRENCE.clone()]])
            .append_query_results(vec![Vec::<currency::Model>::new()])
            // db error on predefined expense query
            .append_query_errors(vec![TEST_DB_ERROR()])
            // db error on recurrence type query
            .append_query_results(vec![vec![TEST_PREDEFINED_EXPENSE.clone()]])
            .append_query_errors(vec![TEST_DB_ERROR()])
            .append_query_results(vec![vec![TEST_CURRENCY.clone()]])
            // db error on currency type query
            .append_query_results(vec![vec![TEST_PREDEFINED_EXPENSE.clone()]])
            .append_query_results(vec![vec![TEST_RECURRENCE.clone()]])
            .append_query_errors(vec![TEST_DB_ERROR()])
            // db error on expense insertion
            .append_query_results(vec![vec![TEST_PREDEFINED_EXPENSE.clone()]])
            .append_query_results(vec![vec![TEST_RECURRENCE.clone()]])
            .append_query_results(vec![vec![TEST_CURRENCY.clone()]])
            .append_exec_errors(vec![TEST_DB_ERROR()])
            // date cannot be parsed
            .append_query_results(vec![vec![TEST_PREDEFINED_EXPENSE]])
            .append_query_results(vec![vec![TEST_RECURRENCE]])
            .append_query_results(vec![vec![TEST_CURRENCY]])
            .into_connection();
        let mut req = NewExpenseRequest {
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: TEST_DECIMAL(),
            start_date: TEST_DATE.to_string(),
            currency_id: TEST_ID,
            recurrence_id: TEST_ID,
            predefined_expense_id: Some(TEST_ID),
        };

        let (
            predefined_expense_not_found,
            recurrence_not_found,
            currency_not_found,
            predefined_expense_db_error,
            recurrence_db_error,
            currency_db_error,
            expense_insert_db_error,
        ) = tokio::join!(
            create_expense(&conn, TEST_ID, req.clone()),
            create_expense(&conn, TEST_ID, req.clone()),
            create_expense(&conn, TEST_ID, req.clone()),
            create_expense(&conn, TEST_ID, req.clone()),
            create_expense(&conn, TEST_ID, req.clone()),
            create_expense(&conn, TEST_ID, req.clone()),
            create_expense(&conn, TEST_ID, req.clone()),
        );
        req.start_date = "wrong_date".to_string();
        let parse_date_error = create_expense(&conn, TEST_ID, req).await;

        check!(predefined_expense_not_found == Err(CreateExpenseError::InvalidPredefinedExpense));
        check!(
            recurrence_not_found
                == Err(CreateExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyError::InvalidRecurrence
                ))
        );
        check!(
            currency_not_found
                == Err(CreateExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyError::InvalidCurrency
                ))
        );
        check!(let Err(CreateExpenseError::InvalidStartDate(_)) = parse_date_error);
        check!(
            predefined_expense_db_error == Err(CreateExpenseError::DatabaseError(TEST_DB_ERROR()))
        );
        check!(
            currency_db_error
                == Err(CreateExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyError::DatabaseError(TEST_DB_ERROR())
                ))
        );
        check!(
            recurrence_db_error
                == Err(CreateExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyError::DatabaseError(TEST_DB_ERROR())
                ))
        );
        check!(expense_insert_db_error == Err(CreateExpenseError::DatabaseError(TEST_DB_ERROR())));
    }

    #[tokio::test]
    async fn find_predefined_expenses_all_cases() {
        let expected_predefined_expenses: Vec<PredefinedExpenseResponse> = vec![
            (TEST_PREDEFINED_EXPENSE, TEST_CURRENCY, TEST_RECURRENCE).into(),
            (TEST_PREDEFINED_EXPENSE, TEST_CURRENCY, TEST_RECURRENCE).into()
        ];

        let predefined_expenses_stub = vec![TEST_PREDEFINED_EXPENSE, TEST_PREDEFINED_EXPENSE];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![predefined_expenses_stub.clone(), vec![]])
            .append_query_errors(vec![TEST_DB_ERROR()])
            .into_connection();

        let (
            predefined_expenses,
            empty_predefined_expenses,
            db_error
        ) = tokio::join!(
            find_predefined_expenses(&conn),
            find_predefined_expenses(&conn),
            find_predefined_expenses(&conn)
        );

        check!(predefined_expenses == Ok(expected_predefined_expenses));
        check!(empty_predefined_expenses == Ok(vec![]));
        check!(db_error == Err(TEST_DB_ERROR()));
    }

    #[tokio::test]
    async fn create_predefined_expense_happy_path() {
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![TEST_RECURRENCE]])
            .append_query_results(vec![vec![TEST_CURRENCY]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: TEST_ID,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![TEST_PREDEFINED_EXPENSE]])
            .into_connection();

        let saved_predefined_expense_id = create_predefined_expense(
            &conn,
            NewPredefinedExpenseRequest {
                name: TEST_STR.to_string(),
                description: TEST_STR.to_string(),
                value: TEST_DECIMAL(),
                currency_id: TEST_ID,
                recurrence_id: TEST_ID,
            },
        )
        .await;

        check!(saved_predefined_expense_id == Ok(TEST_ID));
    }

    #[tokio::test]
    async fn create_predefined_expense_error_cases() {
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // recurrence type not found
            .append_query_results(vec![Vec::<recurrence::Model>::new()])
            .append_query_results(vec![vec![TEST_CURRENCY.clone()]])
            // currency type not found
            .append_query_results(vec![vec![TEST_RECURRENCE.clone()]])
            .append_query_results(vec![Vec::<currency::Model>::new()])
            // recurrence type db error
            .append_query_errors(vec![TEST_DB_ERROR()])
            .append_query_results(vec![vec![TEST_CURRENCY.clone()]])
            // currency type db error
            .append_query_results(vec![vec![TEST_RECURRENCE.clone()]])
            .append_query_errors(vec![TEST_DB_ERROR()])
            // insertion db error
            .append_query_results(vec![vec![TEST_RECURRENCE.clone()]])
            .append_query_results(vec![vec![TEST_CURRENCY.clone()]])
            .append_exec_errors(vec![TEST_DB_ERROR()])
            .into_connection();
        let req = NewPredefinedExpenseRequest {
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: TEST_DECIMAL(),
            currency_id: TEST_ID,
            recurrence_id: TEST_ID,
        };

        let (
            recurrence_not_found,
            currency_not_found,
            recurrence_db_error,
            currency_db_error,
            insertion_db_error,
        ) = tokio::join!(
            create_predefined_expense(&conn, req.clone()),
            create_predefined_expense(&conn, req.clone()),
            create_predefined_expense(&conn, req.clone()),
            create_predefined_expense(&conn, req.clone()),
            create_predefined_expense(&conn, req)
        );

        check!(
            recurrence_not_found
                == Err(CreatePredefinedExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyError::InvalidRecurrence
                ))
        );
        check!(
            currency_not_found
                == Err(CreatePredefinedExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyError::InvalidCurrency
                ))
        );
        check!(
            recurrence_db_error
                == Err(CreatePredefinedExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyError::DatabaseError(TEST_DB_ERROR())
                ))
        );
        check!(
            currency_db_error
                == Err(CreatePredefinedExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyError::DatabaseError(TEST_DB_ERROR())
                ))
        );
        check!(
            insertion_db_error == Err(CreatePredefinedExpenseError::DatabaseError(TEST_DB_ERROR()))
        );
    }

    #[tokio::test]
    async fn validate_recurrence_and_currency_all_cases() {
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // happy path
            .append_query_results(vec![vec![TEST_RECURRENCE.clone()]])
            .append_query_results(vec![vec![TEST_CURRENCY.clone()]])
            // recurrence type not found
            .append_query_results(vec![Vec::<recurrence::Model>::new()])
            .append_query_results(vec![vec![TEST_CURRENCY.clone()]])
            // currency type not found
            .append_query_results(vec![vec![TEST_RECURRENCE.clone()]])
            .append_query_results(vec![Vec::<currency::Model>::new()])
            // recurrence type db error
            .append_query_errors(vec![TEST_DB_ERROR()])
            .append_query_results(vec![vec![TEST_CURRENCY.clone()]])
            // currency type db error
            .append_query_results(vec![vec![TEST_RECURRENCE.clone()]])
            .append_query_errors(vec![TEST_DB_ERROR()])
            .into_connection();

        let (
            happy_path,
            recurrence_not_found,
            currency_not_found,
            recurrence_db_error,
            currency_db_error,
        ) = tokio::join!(
            validate_recurrence_and_currency(&conn, TEST_ID, TEST_ID),
            validate_recurrence_and_currency(&conn, TEST_ID, TEST_ID),
            validate_recurrence_and_currency(&conn, TEST_ID, TEST_ID),
            validate_recurrence_and_currency(&conn, TEST_ID, TEST_ID),
            validate_recurrence_and_currency(&conn, TEST_ID, TEST_ID),
        );

        check!(happy_path == Ok(()));
        check!(recurrence_not_found == Err(ValidateRecurrenceAndCurrencyError::InvalidRecurrence));
        check!(currency_not_found == Err(ValidateRecurrenceAndCurrencyError::InvalidCurrency));
        check!(
            recurrence_db_error
                == Err(ValidateRecurrenceAndCurrencyError::DatabaseError(
                    TEST_DB_ERROR()
                ))
        );
        check!(
            currency_db_error
                == Err(ValidateRecurrenceAndCurrencyError::DatabaseError(
                    TEST_DB_ERROR()
                ))
        );
    }
}

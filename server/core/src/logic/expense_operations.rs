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
        logic::{
            user_operations::errors::AuthorizeUserError,
            common::tests::{
                TEST_ID,
                TEST_STR,
                TEST_DATE,
                test_decimal,
                test_db_error,
                test_currency,
                test_recurrence,
                test_expense,
                test_predefined_expense,
                test_transaction,
                test_transaction_2
            }
        },
        dto::{
            currencies::CurrencyResponse,
            recurrences::RecurrenceResponse,
            expenses::PredefinedExpenseResponse,
            transactions::TransactionResponse
        }
    };

    use super::*;
    use assert2::check;
    use entity::{currency, recurrence};
    use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};

    #[tokio::test]
    async fn find_expenses_by_user_id_all_cases() {
        let expected_currency: CurrencyResponse = test_currency().into();
        let expected_recurrence: RecurrenceResponse = test_recurrence().into();
        let expected_predefined_expense: PredefinedExpenseResponse =
            (test_predefined_expense(), test_currency(), test_recurrence()).into();
        let expected_transaction: TransactionResponse = (test_transaction(), test_currency()).into();
        let expected_transaction_2: TransactionResponse = (test_transaction_2(), test_currency()).into();
        let expected_expenses = vec![ExpenseResponse {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: test_decimal(),
            start_date: NaiveDate::MIN.to_string(),
            user_id: TEST_ID,
            currency: expected_currency,
            recurrence: expected_recurrence,
            predefined_expense: Some(expected_predefined_expense),
            transactions: vec![expected_transaction, expected_transaction_2],
        }];

        let expenses_stub = vec![test_expense()];
        let predefined_expenses_stub = vec![test_predefined_expense()];
        let transactions_stub = vec![test_transaction(), test_transaction_2()];
        let currencies_stub = vec![test_currency()];
        let recurrences_stub = vec![test_recurrence()];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // expenses
            .append_query_results(vec![expenses_stub])
            .append_query_results(vec![predefined_expenses_stub])
            .append_query_results(vec![transactions_stub])
            .append_query_results(vec![currencies_stub.clone()])
            .append_query_results(vec![recurrences_stub.clone()])
            // empty_expenses
            .append_query_results(vec![Vec::<expense::Model>::new()])
            .append_query_results(vec![Vec::<predefined_expense::Model>::new()])
            .append_query_results(vec![Vec::<transaction::Model>::new()])
            .append_query_results(vec![currencies_stub])
            .append_query_results(vec![recurrences_stub])
            // db_error
            .append_query_errors(vec![test_db_error()])
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
                    test_db_error()
                ))
        );
    }

    #[tokio::test]
    async fn create_expense_happy_path() {
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![test_recurrence()]])
            .append_query_results(vec![vec![test_currency()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: TEST_ID,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![test_expense()]])
            .into_connection();

        let saved_expense_id = create_expense(
            &conn,
            TEST_ID,
            NewExpenseRequest {
                name: TEST_STR.to_string(),
                description: TEST_STR.to_string(),
                value: test_decimal(),
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
            .append_query_results(vec![vec![test_predefined_expense()]])
            .append_query_results(vec![vec![test_recurrence()]])
            .append_query_results(vec![vec![test_currency()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: TEST_ID,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![test_expense()]])
            .into_connection();

        let saved_expense_id = create_expense(
            &conn,
            TEST_ID,
            NewExpenseRequest {
                name: TEST_STR.to_string(),
                description: TEST_STR.to_string(),
                value: test_decimal(),
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
            .append_query_results(vec![vec![test_predefined_expense()]])
            .append_query_results(vec![Vec::<recurrence::Model>::new()])
            .append_query_results(vec![vec![test_currency()]])
            // currency type not found
            .append_query_results(vec![vec![test_predefined_expense()]])
            .append_query_results(vec![vec![test_recurrence()]])
            .append_query_results(vec![Vec::<currency::Model>::new()])
            // db error on predefined expense query
            .append_query_errors(vec![test_db_error()])
            // db error on recurrence type query
            .append_query_results(vec![vec![test_predefined_expense()]])
            .append_query_errors(vec![test_db_error()])
            .append_query_results(vec![vec![test_currency()]])
            // db error on currency type query
            .append_query_results(vec![vec![test_predefined_expense()]])
            .append_query_results(vec![vec![test_recurrence()]])
            .append_query_errors(vec![test_db_error()])
            // db error on expense insertion
            .append_query_results(vec![vec![test_predefined_expense()]])
            .append_query_results(vec![vec![test_recurrence()]])
            .append_query_results(vec![vec![test_currency()]])
            .append_exec_errors(vec![test_db_error()])
            // date cannot be parsed
            .append_query_results(vec![vec![test_predefined_expense()]])
            .append_query_results(vec![vec![test_recurrence()]])
            .append_query_results(vec![vec![test_currency()]])
            .into_connection();
        let mut req = NewExpenseRequest {
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: test_decimal(),
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
            predefined_expense_db_error == Err(CreateExpenseError::DatabaseError(test_db_error()))
        );
        check!(
            currency_db_error
                == Err(CreateExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyError::DatabaseError(test_db_error())
                ))
        );
        check!(
            recurrence_db_error
                == Err(CreateExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyError::DatabaseError(test_db_error())
                ))
        );
        check!(expense_insert_db_error == Err(CreateExpenseError::DatabaseError(test_db_error())));
    }

    #[tokio::test]
    async fn find_predefined_expenses_all_cases() {
        let expected_predefined_expenses: Vec<PredefinedExpenseResponse> = vec![
            (test_predefined_expense(), test_currency(), test_recurrence()).into(),
            (test_predefined_expense(), test_currency(), test_recurrence()).into()
        ];

        let predefined_expenses_stub = vec![test_predefined_expense(), test_predefined_expense()];
        let currencies_stub = vec![test_currency()];
        let recurrences_stub = vec![test_recurrence()];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // predefined_expenses
            .append_query_results(vec![predefined_expenses_stub])
            .append_query_results(vec![currencies_stub])
            .append_query_results(vec![recurrences_stub])
            // empty_predefined_expenses
            .append_query_results(vec![Vec::<predefined_expense::Model>::new()])
            .append_query_results(vec![Vec::<currency::Model>::new()])
            .append_query_results(vec![Vec::<recurrence::Model>::new()])
            // db_error
            .append_query_errors(vec![test_db_error()])
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
        check!(db_error == Err(test_db_error()));
    }

    #[tokio::test]
    async fn create_predefined_expense_happy_path() {
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![test_recurrence()]])
            .append_query_results(vec![vec![test_currency()]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: TEST_ID,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![test_predefined_expense()]])
            .into_connection();

        let saved_predefined_expense_id = create_predefined_expense(
            &conn,
            NewPredefinedExpenseRequest {
                name: TEST_STR.to_string(),
                description: TEST_STR.to_string(),
                value: test_decimal(),
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
            .append_query_results(vec![vec![test_currency()]])
            // currency type not found
            .append_query_results(vec![vec![test_recurrence()]])
            .append_query_results(vec![Vec::<currency::Model>::new()])
            // recurrence type db error
            .append_query_errors(vec![test_db_error()])
            .append_query_results(vec![vec![test_currency()]])
            // currency type db error
            .append_query_results(vec![vec![test_recurrence()]])
            .append_query_errors(vec![test_db_error()])
            // insertion db error
            .append_query_results(vec![vec![test_recurrence()]])
            .append_query_results(vec![vec![test_currency()]])
            .append_exec_errors(vec![test_db_error()])
            .into_connection();
        let req = NewPredefinedExpenseRequest {
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: test_decimal(),
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
                    ValidateRecurrenceAndCurrencyError::DatabaseError(test_db_error())
                ))
        );
        check!(
            currency_db_error
                == Err(CreatePredefinedExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyError::DatabaseError(test_db_error())
                ))
        );
        check!(
            insertion_db_error == Err(CreatePredefinedExpenseError::DatabaseError(test_db_error()))
        );
    }

    #[tokio::test]
    async fn validate_recurrence_and_currency_all_cases() {
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // happy path
            .append_query_results(vec![vec![test_recurrence()]])
            .append_query_results(vec![vec![test_currency()]])
            // recurrence type not found
            .append_query_results(vec![Vec::<recurrence::Model>::new()])
            .append_query_results(vec![vec![test_currency()]])
            // currency type not found
            .append_query_results(vec![vec![test_recurrence()]])
            .append_query_results(vec![Vec::<currency::Model>::new()])
            // recurrence type db error
            .append_query_errors(vec![test_db_error()])
            .append_query_results(vec![vec![test_currency()]])
            // currency type db error
            .append_query_results(vec![vec![test_recurrence()]])
            .append_query_errors(vec![test_db_error()])
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
                    test_db_error()
                ))
        );
        check!(
            currency_db_error
                == Err(ValidateRecurrenceAndCurrencyError::DatabaseError(
                    test_db_error()
                ))
        );
    }
}

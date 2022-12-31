use self::errors::{
    CreateExpenseError, CreatePredefinedExpenseError, ValidateRecurrenceAndCurrencyTypesError,
};

use super::common;
use crate::dto::expenses::{NewExpenseRequest, NewPredefinedExpenseRequest};

use entity::currency_type::{self, Entity as CurrencyType};
use entity::expense::{self, Entity as Expense};
use entity::predefined_expense::{self, Entity as PredefinedExpense};
use entity::recurrence_type::{self, Entity as RecurrenceType};
use entity::Id;

use chrono::NaiveDate;
use migration::DbErr;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn find_expenses_by_user_id(
    conn: &DatabaseConnection,
    user_id: Id,
) -> Result<Vec<expense::Model>, DbErr> {
    let expenses = Expense::find()
        .filter(expense::Column::UserId.eq(user_id))
        .all(conn)
        .await?;
    Ok(expenses)
}

pub async fn create_expense(
    conn: &DatabaseConnection,
    user_id: Id,
    req: NewExpenseRequest,
) -> Result<Id, CreateExpenseError> {
    if let Some(predefined_expense_id)= req.predefined_expense_id {
        let Some(_) = PredefinedExpense::find()
            .filter(predefined_expense::Column::Id.eq(predefined_expense_id))
            .one(conn)
            .await? 
        else {
            return Err(CreateExpenseError::InvalidPredefinedExpense);
        };
    }
    validate_recurrence_and_currency_types(conn, req.currency_type_id, req.recurrence_type_id)
        .await?;
    let parsed_date = NaiveDate::parse_from_str(&req.start_date, common::DATE_FORMAT)?;
    let expense = expense::ActiveModel {
        id: NotSet,
        name: Set(req.name),
        description: Set(req.description),
        recurrence_type_id: Set(req.recurrence_type_id),
        currency_type_id: Set(req.currency_type_id),
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
) -> Result<Vec<predefined_expense::Model>, DbErr> {
    let predefined_expenses = PredefinedExpense::find().all(conn).await?;
    Ok(predefined_expenses)
}

pub async fn create_predefined_expense(
    conn: &DatabaseConnection,
    req: NewPredefinedExpenseRequest,
) -> Result<Id, CreatePredefinedExpenseError> {
    validate_recurrence_and_currency_types(conn, req.currency_type_id, req.recurrence_type_id)
        .await?;
    let predefined_expense = predefined_expense::ActiveModel {
        id: NotSet,
        name: Set(req.name),
        description: Set(req.description),
        value: Set(req.value),
        currency_type_id: Set(req.currency_type_id),
        recurrence_type_id: Set(req.recurrence_type_id),
    };
    let predefined_expense = predefined_expense.insert(conn).await?;
    Ok(predefined_expense.id)
}

async fn validate_recurrence_and_currency_types(
    conn: &DatabaseConnection,
    currency_type_id: Id,
    recurrence_type_id: Id,
) -> Result<(), ValidateRecurrenceAndCurrencyTypesError> {
    let (referred_recurrence_type, referred_currency_type) = tokio::join!(
        RecurrenceType::find()
            .filter(recurrence_type::Column::Id.eq(recurrence_type_id))
            .one(conn),
        CurrencyType::find()
            .filter(currency_type::Column::Id.eq(currency_type_id))
            .one(conn)
    );
    let Some(_) = referred_currency_type? else {
        return Err(ValidateRecurrenceAndCurrencyTypesError::InvalidCurrencyType)
    };
    let Some(_) = referred_recurrence_type? else {
        return Err(ValidateRecurrenceAndCurrencyTypesError::InvalidRecurrenceType)
    };
    Ok(())
}

pub mod errors {
    use migration::DbErr;
    use thiserror::Error;

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum CreateExpenseError {
        #[error("predefined expense does not exist with the given id")]
        InvalidPredefinedExpense,
        #[error("start_date could not be parsed as a date")]
        InvalidExpenseStartDate(#[from] chrono::ParseError),
        #[error("invalid related type: '{0}'")]
        InvalidRelatedType(#[from] ValidateRecurrenceAndCurrencyTypesError),
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum CreatePredefinedExpenseError {
        #[error("invalid related type: '{0}'")]
        InvalidRelatedType(#[from] ValidateRecurrenceAndCurrencyTypesError),
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum ValidateRecurrenceAndCurrencyTypesError {
        #[error("provided currency type is non existent")]
        InvalidCurrencyType,
        #[error("provided recurrence type is non existent")]
        InvalidRecurrenceType,
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::DbErr;
    use sea_orm::{prelude::Decimal, DatabaseBackend, MockDatabase, MockExecResult};

    const TEST_STR: &str = "test";
    const TEST_ID: u64 = 1;
    const TEST_FLOAT: f64 = 1.0;
    const TEST_DATE: &str = "06-08-1998";

    fn test_decimal() -> Decimal {
        Decimal::new(1, 2)
    }

    #[tokio::test]
    async fn find_expenses_by_user_id_all_cases() {
        let mock_expenses = vec![
            expense::Model {
                id: TEST_ID,
                name: TEST_STR.to_string(),
                description: TEST_STR.to_string(),
                value: test_decimal(),
                start_date: NaiveDate::MIN,
                user_id: TEST_ID,
                currency_type_id: TEST_ID,
                recurrence_type_id: TEST_ID,
                predefined_expense_id: None,
            },
            expense::Model {
                id: TEST_ID,
                name: TEST_STR.to_string(),
                description: TEST_STR.to_string(),
                value: test_decimal(),
                start_date: NaiveDate::MIN,
                user_id: TEST_ID,
                currency_type_id: TEST_ID,
                recurrence_type_id: TEST_ID,
                predefined_expense_id: Some(TEST_ID),
            },
        ];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![mock_expenses.clone(), vec![]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let expenses = find_expenses_by_user_id(&conn, TEST_ID)
            .await
            .expect("not ok");
        let empty_expenses = find_expenses_by_user_id(&conn, TEST_ID)
            .await
            .expect("not an error");
        let db_error = find_expenses_by_user_id(&conn, TEST_ID)
            .await
            .expect_err("not an error");

        assert_eq!(
            expenses, mock_expenses,
            "returned expenses are not the same as the ones supplied to the mock database"
        );
        assert!(empty_expenses.is_empty(), "returned expenses are not empty");
        assert_eq!(
            db_error,
            DbErr::Custom(TEST_STR.to_string()),
            "mock was supplied with a database error but the function did not return it"
        );
    }

    #[tokio::test]
    async fn create_expense_happy_path() {
        let mock_currency_type = currency_type::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        };
        let mock_recurrence_type = recurrence_type::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            per_year: TEST_FLOAT,
        };
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
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![mock_recurrence_type]])
            .append_query_results(vec![vec![mock_currency_type]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: TEST_ID,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![mock_expense]])
            .into_connection();

        let saved_expense_id = create_expense(
            &conn,
            TEST_ID,
            NewExpenseRequest {
                name: TEST_STR.to_string(),
                description: TEST_STR.to_string(),
                value: test_decimal(),
                start_date: TEST_DATE.to_string(),
                currency_type_id: TEST_ID,
                recurrence_type_id: TEST_ID,
                predefined_expense_id: None,
            },
        )
        .await
        .expect("not ok");

        assert_eq!(saved_expense_id, TEST_ID);
    }

    #[tokio::test]
    async fn create_expense_happy_path_with_predefined_expense_id() {
        let mock_currency_type = currency_type::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        };
        let mock_recurrence_type = recurrence_type::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            per_year: TEST_FLOAT,
        };
        let mock_predefined_expense = predefined_expense::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: test_decimal(),
            currency_type_id: TEST_ID,
            recurrence_type_id: TEST_ID,
        };
        let mock_expense = expense::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: test_decimal(),
            start_date: NaiveDate::MIN,
            user_id: TEST_ID,
            currency_type_id: TEST_ID,
            recurrence_type_id: TEST_ID,
            predefined_expense_id: Some(TEST_ID),
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![mock_predefined_expense]])
            .append_query_results(vec![vec![mock_recurrence_type]])
            .append_query_results(vec![vec![mock_currency_type]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: TEST_ID,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![mock_expense]])
            .into_connection();

        let saved_expense_id = create_expense(
            &conn,
            TEST_ID,
            NewExpenseRequest {
                name: TEST_STR.to_string(),
                description: TEST_STR.to_string(),
                value: test_decimal(),
                start_date: TEST_DATE.to_string(),
                currency_type_id: TEST_ID,
                recurrence_type_id: TEST_ID,
                predefined_expense_id: Some(TEST_ID),
            },
        )
        .await
        .expect("not ok");

        assert_eq!(saved_expense_id, TEST_ID);
    }

    #[tokio::test]
    async fn create_expense_db_error_cases() {
        let mock_currency_type = currency_type::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        };
        let mock_recurrence_type = recurrence_type::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            per_year: TEST_FLOAT,
        };
        let mock_predefined_expense = predefined_expense::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: test_decimal(),
            currency_type_id: TEST_ID,
            recurrence_type_id: TEST_ID,
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // predefined expense not found
            .append_query_results(vec![Vec::<predefined_expense::Model>::new()])
            // recurrence type not found
            .append_query_results(vec![vec![mock_predefined_expense.clone()]])
            .append_query_results(vec![Vec::<recurrence_type::Model>::new()])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            // currency type not found
            .append_query_results(vec![vec![mock_predefined_expense.clone()]])
            .append_query_results(vec![vec![mock_recurrence_type.clone()]])
            .append_query_results(vec![Vec::<currency_type::Model>::new()])
            // db error on predefined expense query
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            // db error on recurrence type query
            .append_query_results(vec![vec![mock_predefined_expense.clone()]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            // db error on currency type query
            .append_query_results(vec![vec![mock_predefined_expense.clone()]])
            .append_query_results(vec![vec![mock_recurrence_type.clone()]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            // db error on expense insertion
            .append_query_results(vec![vec![mock_predefined_expense.clone()]])
            .append_query_results(vec![vec![mock_recurrence_type.clone()]])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            .append_exec_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            // date cannot be parsed
            .append_query_results(vec![vec![mock_predefined_expense]])
            .append_query_results(vec![vec![mock_recurrence_type]])
            .append_query_results(vec![vec![mock_currency_type]])
            .into_connection();
        let mut req = NewExpenseRequest {
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: test_decimal(),
            start_date: TEST_DATE.to_string(),
            currency_type_id: TEST_ID,
            recurrence_type_id: TEST_ID,
            predefined_expense_id: Some(TEST_ID),
        };

        let (
            predefined_expense_not_found_err,
            recurrence_type_not_found_err,
            currency_type_not_found_err,
            predefined_expense_db_err,
            recurrence_type_db_err,
            currency_type_db_err,
            expense_insert_db_err,
        ) = tokio::join!(
            create_expense(&conn, TEST_ID, req.clone()),
            create_expense(&conn, TEST_ID, req.clone()),
            create_expense(&conn, TEST_ID, req.clone()),
            create_expense(&conn, TEST_ID, req.clone()),
            create_expense(&conn, TEST_ID, req.clone()),
            create_expense(&conn, TEST_ID, req.clone()),
            create_expense(&conn, TEST_ID, req.clone()),
        );
        req.start_date = "wrond_date".to_string();
        let parse_date_error = create_expense(&conn, TEST_ID, req).await;

        assert_eq!(
            predefined_expense_not_found_err.expect_err("not an error"),
            CreateExpenseError::InvalidPredefinedExpense
        );
        assert_eq!(
            recurrence_type_not_found_err.expect_err("not an error"),
            CreateExpenseError::InvalidRelatedType(
                ValidateRecurrenceAndCurrencyTypesError::InvalidRecurrenceType
            )
        );
        assert_eq!(
            currency_type_not_found_err.expect_err("not an error"),
            CreateExpenseError::InvalidRelatedType(
                ValidateRecurrenceAndCurrencyTypesError::InvalidCurrencyType
            )
        );
        assert!(
            matches!(
                parse_date_error.expect_err("not an error"),
                CreateExpenseError::InvalidExpenseStartDate(_)
            ),
            "date parse error is different from expected"
        );
        assert_eq!(
            predefined_expense_db_err.expect_err("not an error"),
            CreateExpenseError::DatabaseError(DbErr::Custom(TEST_STR.to_string()))
        );
        assert_eq!(
            currency_type_db_err.expect_err("not an error"),
            CreateExpenseError::InvalidRelatedType(
                ValidateRecurrenceAndCurrencyTypesError::DatabaseError(DbErr::Custom(
                    TEST_STR.to_string()
                ))
            )
        );
        assert_eq!(
            recurrence_type_db_err.expect_err("not an error"),
            CreateExpenseError::InvalidRelatedType(
                ValidateRecurrenceAndCurrencyTypesError::DatabaseError(DbErr::Custom(
                    TEST_STR.to_string()
                ))
            )
        );
        assert_eq!(
            expense_insert_db_err.expect_err("not an error"),
            CreateExpenseError::DatabaseError(DbErr::Custom(TEST_STR.to_string()))
        );
    }

    #[tokio::test]
    async fn find_predefined_expenses_all_cases() {
        let mock_predefined_expenses = vec![
            predefined_expense::Model {
                id: TEST_ID,
                name: TEST_STR.to_string(),
                description: TEST_STR.to_string(),
                value: test_decimal(),
                currency_type_id: TEST_ID,
                recurrence_type_id: TEST_ID,
            },
            predefined_expense::Model {
                id: TEST_ID,
                name: TEST_STR.to_string(),
                description: TEST_STR.to_string(),
                value: test_decimal(),
                currency_type_id: TEST_ID,
                recurrence_type_id: TEST_ID,
            },
        ];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![mock_predefined_expenses.clone(), vec![]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let (predefined_expenses, empty_predefined_expenses, db_error) = tokio::join!(
            find_predefined_expenses(&conn),
            find_predefined_expenses(&conn),
            find_predefined_expenses(&conn)
        );

        assert_eq!(
            predefined_expenses.expect("not ok"), mock_predefined_expenses,
            "returned predefined expenses are not the same as the ones supplied to the mock database"
        );
        assert!(
            empty_predefined_expenses.expect("not ok").is_empty(),
            "returned predefined expenses are not empty"
        );
        assert!(
            db_error.is_err(),
            "mock was supplied with a database error but the function did not return it"
        );
    }

    #[tokio::test]
    async fn create_predefined_expense_happy_path() {
        let mock_currency_type = currency_type::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        };
        let mock_recurrence_type = recurrence_type::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            per_year: TEST_FLOAT,
        };
        let mock_predefined_expense = predefined_expense::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: test_decimal(),
            currency_type_id: TEST_ID,
            recurrence_type_id: TEST_ID,
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![mock_recurrence_type]])
            .append_query_results(vec![vec![mock_currency_type]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: TEST_ID,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![mock_predefined_expense]])
            .into_connection();

        let saved_predefined_expense_id = create_predefined_expense(
            &conn,
            NewPredefinedExpenseRequest {
                name: TEST_STR.to_string(),
                description: TEST_STR.to_string(),
                value: test_decimal(),
                currency_type_id: TEST_ID,
                recurrence_type_id: TEST_ID,
            },
        )
        .await
        .expect("not ok");

        assert_eq!(saved_predefined_expense_id, TEST_ID);
    }

    #[tokio::test]
    async fn create_predefined_expense_error_cases() {
        let mock_currency_type = currency_type::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        };
        let mock_recurrence_type = recurrence_type::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            per_year: TEST_FLOAT,
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // recurrence type not found
            .append_query_results(vec![Vec::<recurrence_type::Model>::new()])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            // currency type not found
            .append_query_results(vec![vec![mock_recurrence_type.clone()]])
            .append_query_results(vec![Vec::<currency_type::Model>::new()])
            // recurrence type db error
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            // currency type db error
            .append_query_results(vec![vec![mock_recurrence_type.clone()]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            // insertion db error
            .append_query_results(vec![vec![mock_recurrence_type.clone()]])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            .append_exec_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();
        let req = NewPredefinedExpenseRequest {
            name: TEST_STR.to_string(),
            description: TEST_STR.to_string(),
            value: test_decimal(),
            currency_type_id: TEST_ID,
            recurrence_type_id: TEST_ID,
        };

        let (
            recurrence_type_not_found_err,
            currency_type_not_found_err,
            recurrence_type_db_err,
            currency_type_db_err,
            insertion_db_error,
        ) = tokio::join!(
            create_predefined_expense(&conn, req.clone()),
            create_predefined_expense(&conn, req.clone()),
            create_predefined_expense(&conn, req.clone()),
            create_predefined_expense(&conn, req.clone()),
            create_predefined_expense(&conn, req)
        );

        assert_eq!(
            recurrence_type_not_found_err.expect_err("not an error"),
            CreatePredefinedExpenseError::InvalidRelatedType(
                ValidateRecurrenceAndCurrencyTypesError::InvalidRecurrenceType
            )
        );
        assert_eq!(
            currency_type_not_found_err.expect_err("not an error"),
            CreatePredefinedExpenseError::InvalidRelatedType(
                ValidateRecurrenceAndCurrencyTypesError::InvalidCurrencyType
            )
        );
        assert_eq!(
            recurrence_type_db_err.expect_err("not an error"),
            CreatePredefinedExpenseError::InvalidRelatedType(
                ValidateRecurrenceAndCurrencyTypesError::DatabaseError(DbErr::Custom(
                    TEST_STR.to_string()
                ))
            )
        );
        assert_eq!(
            currency_type_db_err.expect_err("not an error"),
            CreatePredefinedExpenseError::InvalidRelatedType(
                ValidateRecurrenceAndCurrencyTypesError::DatabaseError(DbErr::Custom(
                    TEST_STR.to_string()
                ))
            )
        );
        assert_eq!(
            insertion_db_error.expect_err("not an error"),
            CreatePredefinedExpenseError::DatabaseError(DbErr::Custom(TEST_STR.to_string()))
        );
    }

    #[tokio::test]
    async fn validate_recurrence_and_currency_types_all_cases() {
        let mock_currency_type = currency_type::Model {
            id: TEST_ID,
            abbreviation: TEST_STR.to_string(),
            name: TEST_STR.to_string(),
        };
        let mock_recurrence_type = recurrence_type::Model {
            id: TEST_ID,
            name: TEST_STR.to_string(),
            per_year: TEST_FLOAT,
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // happy path
            .append_query_results(vec![vec![mock_recurrence_type.clone()]])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            // recurrence type not found
            .append_query_results(vec![Vec::<recurrence_type::Model>::new()])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            // currency type not found
            .append_query_results(vec![vec![mock_recurrence_type.clone()]])
            .append_query_results(vec![Vec::<currency_type::Model>::new()])
            // recurrence type db error
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            // currency type db error
            .append_query_results(vec![vec![mock_recurrence_type.clone()]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let (
            happy_path,
            recurrence_type_not_found_err,
            currency_type_not_found_err,
            recurrence_type_db_err,
            currency_type_db_err,
        ) = tokio::join!(
            validate_recurrence_and_currency_types(&conn, TEST_ID, TEST_ID),
            validate_recurrence_and_currency_types(&conn, TEST_ID, TEST_ID),
            validate_recurrence_and_currency_types(&conn, TEST_ID, TEST_ID),
            validate_recurrence_and_currency_types(&conn, TEST_ID, TEST_ID),
            validate_recurrence_and_currency_types(&conn, TEST_ID, TEST_ID),
        );

        assert!(happy_path.is_ok());
        assert_eq!(
            recurrence_type_not_found_err.expect_err("not an error"),
            ValidateRecurrenceAndCurrencyTypesError::InvalidRecurrenceType
        );
        assert_eq!(
            currency_type_not_found_err.expect_err("not an error"),
            ValidateRecurrenceAndCurrencyTypesError::InvalidCurrencyType
        );
        assert_eq!(
            recurrence_type_db_err.expect_err("not an error"),
            ValidateRecurrenceAndCurrencyTypesError::DatabaseError(DbErr::Custom(
                TEST_STR.to_string()
            ))
        );
        assert_eq!(
            currency_type_db_err.expect_err("not an error"),
            ValidateRecurrenceAndCurrencyTypesError::DatabaseError(DbErr::Custom(
                TEST_STR.to_string()
            ))
        );
    }
}

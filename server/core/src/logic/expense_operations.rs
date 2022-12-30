use self::errors::{
    CreateExpenseError, CreatePredefinedExpenseError, FindExpensesByUserIdError,
    FindPredefinedExpensesError, ValidateRecurrenceAndCurrencyTypesError,
};

use super::common;
use crate::dto::expenses::{NewExpenseRequest, NewPredefinedExpenseRequest};

use entity::currency_type::{self, Entity as CurrencyType};
use entity::expense::{self, Entity as Expense};
use entity::predefined_expense::{self, Entity as PredefinedExpense};
use entity::recurrence_type::{self, Entity as RecurrenceType};
use entity::Id;

use chrono::NaiveDate;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn find_expenses_by_user_id(
    conn: &DatabaseConnection,
    user_id: Id,
) -> Result<Vec<expense::Model>, FindExpensesByUserIdError> {
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
    if req.predefined_expense_id.is_some()
        && PredefinedExpense::find()
            .filter(predefined_expense::Column::Id.eq(req.predefined_expense_id))
            .one(conn)
            .await?
            .is_none()
    {
        return Err(CreateExpenseError::InvalidExpenseData(String::from(
            "Predefined expense id is defined but we have no expense with this id.",
        )));
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
) -> Result<Vec<predefined_expense::Model>, FindPredefinedExpensesError> {
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
    if referred_recurrence_type?.is_none() || referred_currency_type?.is_none() {
        return Err(ValidateRecurrenceAndCurrencyTypesError::InvalidExpenseData(
            String::from("We have no recurrence or currency type with the given id."),
        ));
    };
    Ok(())
}

pub mod errors {
    use migration::DbErr;
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum FindExpensesByUserIdError {
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }

    #[derive(Error, Debug)]
    pub enum CreateExpenseError {
        #[error("some data provided for expense creation is invalid or cannot be parsed: '{0}'")]
        InvalidExpenseData(String),
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }

    impl From<chrono::ParseError> for CreateExpenseError {
        fn from(e: chrono::ParseError) -> Self {
            CreateExpenseError::InvalidExpenseData(e.to_string())
        }
    }

    #[derive(Error, Debug)]
    pub enum FindPredefinedExpensesError {
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }

    #[derive(Error, Debug)]
    pub enum CreatePredefinedExpenseError {
        #[error("some data provided for expense creation is invalid or cannot be parsed: '{0}'")]
        InvalidExpenseData(String),
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }

    #[derive(Error, Debug)]
    pub enum ValidateRecurrenceAndCurrencyTypesError {
        #[error("some data provided for expense creation is invalid or cannot be parsed: '{0}'")]
        InvalidExpenseData(String),
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }

    impl From<ValidateRecurrenceAndCurrencyTypesError> for CreateExpenseError {
        fn from(error: ValidateRecurrenceAndCurrencyTypesError) -> Self {
            match error {
                ValidateRecurrenceAndCurrencyTypesError::InvalidExpenseData(msg) => {
                    CreateExpenseError::InvalidExpenseData(msg)
                }
                ValidateRecurrenceAndCurrencyTypesError::DatabaseError(db_error) => {
                    CreateExpenseError::DatabaseError(db_error)
                }
            }
        }
    }

    impl From<ValidateRecurrenceAndCurrencyTypesError> for CreatePredefinedExpenseError {
        fn from(error: ValidateRecurrenceAndCurrencyTypesError) -> Self {
            match error {
                ValidateRecurrenceAndCurrencyTypesError::InvalidExpenseData(msg) => {
                    CreatePredefinedExpenseError::InvalidExpenseData(msg)
                }
                ValidateRecurrenceAndCurrencyTypesError::DatabaseError(db_error) => {
                    CreatePredefinedExpenseError::DatabaseError(db_error)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::DbErr;
    use sea_orm::{prelude::Decimal, DatabaseBackend, MockDatabase, MockExecResult};

    #[tokio::test]
    async fn find_expenses_by_user_id_all_cases() {
        let user_id = 3171731820;
        let mock_expenses = vec![
            expense::Model {
                id: 1630341601,
                name: "Cambodia".to_string(),
                description: "Jordan".to_string(),
                value: Decimal::new(2394761845, 2),
                start_date: NaiveDate::MIN,
                user_id,
                currency_type_id: 4034035371,
                recurrence_type_id: 3413654293,
                predefined_expense_id: None,
            },
            expense::Model {
                id: 3992012396,
                name: "Taiwan".to_string(),
                description: "France".to_string(),
                value: Decimal::new(3766262178, 2),
                start_date: NaiveDate::MIN,
                user_id,
                currency_type_id: 1724888834,
                recurrence_type_id: 1153356612,
                predefined_expense_id: Some(2427040649),
            },
        ];
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![mock_expenses.clone(), vec![]])
            .append_query_errors(vec![DbErr::Custom("Mozsojuc".to_string())])
            .into_connection();

        let expenses = find_expenses_by_user_id(&conn, user_id).await.unwrap();
        let empty_expenses = find_expenses_by_user_id(&conn, 3236857222).await.unwrap();
        let db_error = find_expenses_by_user_id(&conn, user_id).await;

        assert_eq!(
            expenses, mock_expenses,
            "returned expenses are not the same as the ones supplied to the mock database"
        );
        assert!(empty_expenses.is_empty(), "returned expenses are not empty");
        assert!(
            db_error.is_err(),
            "mock was supplied with a database error but the function did not return it"
        );
    }

    #[tokio::test]
    async fn create_expense_happy_path() {
        let expense_id = 2856022774;
        let currency_type_id = 3826426650;
        let recurrence_type_id = 2682431782;
        let user_id = 2040834566;
        let mock_currency_type = currency_type::Model {
            id: currency_type_id,
            abbreviation: "Monaco".to_string(),
            name: "Turkey".to_string(),
        };
        let mock_recurrence_type = recurrence_type::Model {
            id: recurrence_type_id,
            name: "Burundi".to_string(),
            per_year: 3946985683.0,
        };
        let mock_expense = expense::Model {
            id: expense_id,
            name: "Iraq".to_string(),
            description: "Bosnia & Herzegovina".to_string(),
            value: Decimal::new(3147479767, 2),
            start_date: NaiveDate::MIN,
            user_id,
            currency_type_id: currency_type_id,
            recurrence_type_id: recurrence_type_id,
            predefined_expense_id: None,
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![mock_recurrence_type]])
            .append_query_results(vec![vec![mock_currency_type]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: expense_id,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![mock_expense]])
            .into_connection();

        let saved_expense_id = create_expense(
            &conn,
            user_id,
            NewExpenseRequest {
                name: "Samoa".to_string(),
                description: "Guinea-Bissau".to_string(),
                value: Decimal::new(3820587564, 2),
                start_date: "17-04-2022".to_string(),
                currency_type_id: 40194903,
                recurrence_type_id: 2134051446,
                predefined_expense_id: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(saved_expense_id, expense_id);
    }

    #[tokio::test]
    async fn create_expense_happy_path_with_predefined_expense_id() {
        let predefined_expense_id = 1630341601;
        let expense_id = 2856022774;
        let currency_type_id = 3826426650;
        let recurrence_type_id = 2682431782;
        let user_id = 2040834566;
        let mock_currency_type = currency_type::Model {
            id: currency_type_id,
            abbreviation: "Monaco".to_string(),
            name: "Turkey".to_string(),
        };
        let mock_recurrence_type = recurrence_type::Model {
            id: recurrence_type_id,
            name: "Burundi".to_string(),
            per_year: 3946985683.0,
        };
        let mock_predefined_expense = predefined_expense::Model {
            id: predefined_expense_id,
            name: "Costa Rica".to_string(),
            description: "Côte d'Ivoire".to_string(),
            value: Decimal::new(2394761845, 2),
            currency_type_id: currency_type_id,
            recurrence_type_id: recurrence_type_id,
        };
        let mock_expense = expense::Model {
            id: expense_id,
            name: "Iraq".to_string(),
            description: "Bosnia & Herzegovina".to_string(),
            value: Decimal::new(3147479767, 2),
            start_date: NaiveDate::MIN,
            user_id,
            currency_type_id: currency_type_id,
            recurrence_type_id: recurrence_type_id,
            predefined_expense_id: Some(predefined_expense_id),
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            .append_query_results(vec![vec![mock_predefined_expense]])
            .append_query_results(vec![vec![mock_recurrence_type]])
            .append_query_results(vec![vec![mock_currency_type]])
            .append_exec_results(vec![MockExecResult {
                last_insert_id: expense_id,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![mock_expense]])
            .into_connection();

        let saved_expense_id = create_expense(
            &conn,
            user_id,
            NewExpenseRequest {
                name: "Samoa".to_string(),
                description: "Guinea-Bissau".to_string(),
                value: Decimal::new(3820587564, 2),
                start_date: "17-04-2022".to_string(),
                currency_type_id: 40194903,
                recurrence_type_id: 2134051446,
                predefined_expense_id: Some(predefined_expense_id),
            },
        )
        .await
        .unwrap();

        assert_eq!(saved_expense_id, expense_id);
    }

    #[tokio::test]
    async fn create_expense_db_error_cases() {
        let predefined_expense_id = 2462185108;
        let currency_type_id = 110264993;
        let recurrence_type_id = 3987499508;
        let user_id = 1621035542;
        let mock_currency_type = currency_type::Model {
            id: currency_type_id,
            abbreviation: "Netherlands".to_string(),
            name: "Afghanistan".to_string(),
        };
        let mock_recurrence_type = recurrence_type::Model {
            id: recurrence_type_id,
            name: "Mayotte".to_string(),
            per_year: 3719769771.0,
        };
        let mock_predefined_expense = predefined_expense::Model {
            id: predefined_expense_id,
            name: "Panama".to_string(),
            description: "Togo".to_string(),
            value: Decimal::new(1664784468, 2),
            currency_type_id: currency_type_id,
            recurrence_type_id: recurrence_type_id,
        };
        let conn = MockDatabase::new(DatabaseBackend::MySql)
            // db error on predefined expense query
            .append_query_errors(vec![DbErr::Custom("Singapore".to_string())])
            // db error on recurrence type validation query
            .append_query_results(vec![vec![mock_predefined_expense.clone()]])
            .append_query_errors(vec![DbErr::Custom("Estonia".to_string())])
            .append_query_results(vec![vec![mock_currency_type.clone()]])
            // db error on currency type validation query
            .append_query_results(vec![vec![mock_predefined_expense.clone()]])
            .append_query_results(vec![vec![mock_recurrence_type.clone()]])
            .append_query_errors(vec![DbErr::Custom("Trinidad & Tobago".to_string())])
            // db error on expense insertion
            .append_query_results(vec![vec![mock_predefined_expense.clone()]])
            .append_query_results(vec![vec![mock_recurrence_type]])
            .append_query_results(vec![vec![mock_currency_type]])
            .append_exec_errors(vec![DbErr::Custom("Sierra Leone".to_string())])
            .into_connection();
        let mut req = NewExpenseRequest {
            name: "Samoa".to_string(),
            description: "Guinea-Bissau".to_string(),
            value: Decimal::new(3820587564, 2),
            start_date: "2012-04-17".to_string(),
            currency_type_id,
            recurrence_type_id,
            predefined_expense_id: Some(predefined_expense_id),
        };

        let (
            predefined_expense_db_err,
            currency_type_db_err,
            recurrence_type_db_err,
            expense_insert_db_err,
        ) = tokio::join!(
            create_expense(&conn, user_id, req.clone()),
            create_expense(&conn, user_id, req.clone()),
            create_expense(&conn, user_id, req.clone()),
            create_expense(&conn, user_id, req.clone()),
        );
        req.start_date = "Jesus Montoya".to_string();
        let parse_date_error = create_expense(&conn, user_id, req).await;

        assert!(predefined_expense_db_err.is_err());
        assert!(currency_type_db_err.is_err());
        assert!(recurrence_type_db_err.is_err());
        assert!(expense_insert_db_err.is_err());
        assert!(parse_date_error.is_err());
    }

    #[tokio::test]
    async fn find_predefined_on_correct_types_returns_ok() {}

    #[tokio::test]
    async fn find_predefined_on_dberr_returns_err() {}

    #[tokio::test]
    async fn create_predefined_on_correct_types_returns_ok() {}

    #[tokio::test]
    async fn create_predefined_on_incorrect_data_returns_err() {}

    #[tokio::test]
    async fn create_predefined_on_dberr_returns_err() {}
}

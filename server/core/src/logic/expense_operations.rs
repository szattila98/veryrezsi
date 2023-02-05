use self::errors::{
    CreateExpenseError, CreatePredefinedExpenseError, FindExpensesByUserIdError,
    ValidateRecurrenceAndCurrencyTypesError,
};

use super::common;
use super::user_operations::authorize_user;
use crate::dto::expenses::{NewExpenseRequest, NewPredefinedExpenseRequest};
use crate::logic::currency_operations::find_currency_type_by_id;
use crate::logic::recurrence_operations::find_recurrence_type_by_id;

use entity::expense::{self, Entity as Expense};
use entity::predefined_expense::{self, Entity as PredefinedExpense};
use entity::Id;

use chrono::NaiveDate;
use migration::DbErr;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn find_expenses_by_user_id(
    conn: &DatabaseConnection,
    authenticated_user_id: Id,
    user_id: Id,
) -> Result<Vec<expense::Model>, FindExpensesByUserIdError> {
    authorize_user(authenticated_user_id, user_id)?;
    let expenses = Expense::find()
        .filter(expense::Column::UserId.eq(user_id))
        .all(conn)
        .await?;
    Ok(expenses)
}

pub async fn find_expense_by_id(
    conn: &DatabaseConnection,
    id: Id,
) -> Result<Option<expense::Model>, DbErr> {
    let expense = Expense::find_by_id(id).one(conn).await?;
    Ok(expense)
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
        find_recurrence_type_by_id(conn, recurrence_type_id),
        find_currency_type_by_id(conn, currency_type_id)
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

    use crate::logic::user_operations::errors::AuthorizeUserError;

    #[derive(Error, Debug, PartialEq, Eq)]
    pub enum FindExpensesByUserIdError {
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
        #[error("currency type is invalid")]
        InvalidCurrencyType,
        #[error("recurrence type is invalid")]
        InvalidRecurrenceType,
        #[error("database error: '{0}'")]
        DatabaseError(#[from] DbErr),
    }
}

#[cfg(test)]
mod tests {
    use crate::logic::user_operations::errors::AuthorizeUserError;

    use super::*;
    use assert2::check;
    use entity::{currency_type, recurrence_type};
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

        let (expenses, empty_expenses, unauthorized_error, db_error) = tokio::join!(
            find_expenses_by_user_id(&conn, TEST_ID, TEST_ID),
            find_expenses_by_user_id(&conn, TEST_ID, TEST_ID),
            find_expenses_by_user_id(&conn, TEST_ID, TEST_ID + 1),
            find_expenses_by_user_id(&conn, TEST_ID, TEST_ID)
        );

        check!(expenses == Ok(mock_expenses));
        check!(empty_expenses == Ok(vec![]));
        check!(
            unauthorized_error
                == Err(FindExpensesByUserIdError::UnauthorizedUser(
                    AuthorizeUserError
                ))
        );
        check!(
            db_error
                == Err(FindExpensesByUserIdError::DatabaseError(DbErr::Custom(
                    TEST_STR.to_string()
                )))
        );
    }

    #[tokio::test]
    async fn find_expense_by_id_all_cases() {
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
            .append_query_results(vec![vec![mock_expense.clone()], vec![]])
            .append_query_errors(vec![DbErr::Custom(TEST_STR.to_string())])
            .into_connection();

        let (expense, none, db_error) = tokio::join!(
            find_expense_by_id(&conn, TEST_ID),
            find_expense_by_id(&conn, TEST_ID),
            find_expense_by_id(&conn, TEST_ID)
        );

        check!(expense == Ok(Some(mock_expense)));
        check!(none == Ok(None));
        check!(db_error == Err(DbErr::Custom(TEST_STR.to_string())));
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
            .append_exec_results(vec![MockExecResult::default()])
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
        .await;

        check!(saved_expense_id == Ok(TEST_ID));
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
            .append_exec_results(vec![MockExecResult::default()])
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
        .await;

        check!(saved_expense_id == Ok(TEST_ID));
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
            predefined_expense_not_found,
            recurrence_type_not_found,
            currency_type_not_found,
            predefined_expense_db_error,
            recurrence_type_db_error,
            currency_type_db_error,
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
        req.start_date = "wrond_date".to_string();
        let parse_date_error = create_expense(&conn, TEST_ID, req).await;

        check!(predefined_expense_not_found == Err(CreateExpenseError::InvalidPredefinedExpense));
        check!(
            recurrence_type_not_found
                == Err(CreateExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyTypesError::InvalidRecurrenceType
                ))
        );
        check!(
            currency_type_not_found
                == Err(CreateExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyTypesError::InvalidCurrencyType
                ))
        );
        check!(let Err(CreateExpenseError::InvalidStartDate(_)) = parse_date_error);
        check!(
            predefined_expense_db_error
                == Err(CreateExpenseError::DatabaseError(DbErr::Custom(
                    TEST_STR.to_string()
                )))
        );
        check!(
            currency_type_db_error
                == Err(CreateExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyTypesError::DatabaseError(DbErr::Custom(
                        TEST_STR.to_string()
                    ))
                ))
        );
        check!(
            recurrence_type_db_error
                == Err(CreateExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyTypesError::DatabaseError(DbErr::Custom(
                        TEST_STR.to_string()
                    ))
                ))
        );
        check!(
            expense_insert_db_error
                == Err(CreateExpenseError::DatabaseError(DbErr::Custom(
                    TEST_STR.to_string()
                )))
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

        check!(predefined_expenses == Ok(mock_predefined_expenses));
        check!(empty_predefined_expenses == Ok(vec![]));
        check!(db_error == Err(DbErr::Custom(TEST_STR.to_string())));
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
            .append_exec_results(vec![MockExecResult::default()])
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
        .await;

        check!(saved_predefined_expense_id == Ok(TEST_ID));
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
            recurrence_type_not_found,
            currency_type_not_found,
            recurrence_type_db_error,
            currency_type_db_error,
            insertion_db_error,
        ) = tokio::join!(
            create_predefined_expense(&conn, req.clone()),
            create_predefined_expense(&conn, req.clone()),
            create_predefined_expense(&conn, req.clone()),
            create_predefined_expense(&conn, req.clone()),
            create_predefined_expense(&conn, req)
        );

        check!(
            recurrence_type_not_found
                == Err(CreatePredefinedExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyTypesError::InvalidRecurrenceType
                ))
        );
        check!(
            currency_type_not_found
                == Err(CreatePredefinedExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyTypesError::InvalidCurrencyType
                ))
        );
        check!(
            recurrence_type_db_error
                == Err(CreatePredefinedExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyTypesError::DatabaseError(DbErr::Custom(
                        TEST_STR.to_string()
                    ))
                ))
        );
        check!(
            currency_type_db_error
                == Err(CreatePredefinedExpenseError::InvalidRelatedType(
                    ValidateRecurrenceAndCurrencyTypesError::DatabaseError(DbErr::Custom(
                        TEST_STR.to_string()
                    ))
                ))
        );
        check!(
            insertion_db_error
                == Err(CreatePredefinedExpenseError::DatabaseError(DbErr::Custom(
                    TEST_STR.to_string()
                )))
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
            recurrence_type_not_found,
            currency_type_not_found,
            recurrence_type_db_error,
            currency_type_db_error,
        ) = tokio::join!(
            validate_recurrence_and_currency_types(&conn, TEST_ID, TEST_ID),
            validate_recurrence_and_currency_types(&conn, TEST_ID, TEST_ID),
            validate_recurrence_and_currency_types(&conn, TEST_ID, TEST_ID),
            validate_recurrence_and_currency_types(&conn, TEST_ID, TEST_ID),
            validate_recurrence_and_currency_types(&conn, TEST_ID, TEST_ID),
        );

        check!(happy_path == Ok(()));
        check!(
            recurrence_type_not_found
                == Err(ValidateRecurrenceAndCurrencyTypesError::InvalidRecurrenceType)
        );
        check!(
            currency_type_not_found
                == Err(ValidateRecurrenceAndCurrencyTypesError::InvalidCurrencyType)
        );
        check!(
            recurrence_type_db_error
                == Err(ValidateRecurrenceAndCurrencyTypesError::DatabaseError(
                    DbErr::Custom(TEST_STR.to_string())
                ))
        );
        check!(
            currency_type_db_error
                == Err(ValidateRecurrenceAndCurrencyTypesError::DatabaseError(
                    DbErr::Custom(TEST_STR.to_string())
                ))
        );
    }
}
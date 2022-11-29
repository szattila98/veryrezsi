use super::common;
use super::error::ExpenseError;
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
) -> Result<Vec<expense::Model>, ExpenseError> {
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
) -> Result<Id, ExpenseError> {
    if req.predefined_expense_id.is_some()
        && PredefinedExpense::find()
            .filter(predefined_expense::Column::Id.eq(req.predefined_expense_id))
            .one(conn)
            .await?
            .is_none()
    {
        return Err(ExpenseError::InvalidExpenseData(String::from(
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
) -> Result<Vec<predefined_expense::Model>, ExpenseError> {
    let predefined_expenses = PredefinedExpense::find().all(conn).await?;
    Ok(predefined_expenses)
}

pub async fn create_predefined_expense(
    conn: &DatabaseConnection,
    req: NewPredefinedExpenseRequest,
) -> Result<Id, ExpenseError> {
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
) -> Result<(), ExpenseError> {
    let (referred_recurrence_type, referred_currency_type) = tokio::join!(
        RecurrenceType::find()
            .filter(recurrence_type::Column::Id.eq(recurrence_type_id))
            .one(conn),
        CurrencyType::find()
            .filter(currency_type::Column::Id.eq(currency_type_id))
            .one(conn)
    );
    if referred_recurrence_type?.is_none() || referred_currency_type?.is_none() {
        return Err(ExpenseError::InvalidExpenseData(String::from(
            "We have no recurrence or currency type with the given id.",
        )));
    };
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use migration::DbErr;
    use sea_orm::{DatabaseBackend, MockDatabase};

    #[tokio::test]
    async fn find_expense_on_correct_types_returns_ok() {}

    #[tokio::test]
    async fn find_expense_on_dberr_returns_err() {}

    #[tokio::test]
    async fn create_expense_on_correct_types_returns_ok() {}

    #[tokio::test]
    async fn create_expense_on_incorrect_data_returns_err() {}

    #[tokio::test]
    async fn create_expense_on_dberr_returns_err() {}

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

    #[tokio::test]
    async fn validate_on_correct_types_returns_ok() {}

    #[tokio::test]
    async fn validate_on_incorrect_data_returns_err() {}

    #[tokio::test]
    async fn validate_on_dberr_returns_err() {}
}

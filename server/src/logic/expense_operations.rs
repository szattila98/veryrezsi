use super::common;
use super::error::ExpenseError;

use crate::routes::dto::expenses::NewExpenseRequest;

use chrono::NaiveDate;
use entity::currency_type::{self, Entity as CurrencyType};
use entity::expense::{self, Entity as Expense};
use entity::predefined_expense::{self, Entity as PredefinedExpense};
use entity::recurrence_type::{self, Entity as RecurrenceType};

use sea_orm::ActiveValue::NotSet;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

pub async fn find_expenses_by_user_id(
    conn: &DatabaseConnection,
    user_id: i64,
) -> Result<Vec<expense::Model>, ExpenseError> {
    let expenses = Expense::find()
        .filter(expense::Column::UserId.eq(user_id))
        .all(conn)
        .await?;

    if expenses.is_empty() {
        return Err(ExpenseError::NoExpenseFoundForUser(user_id.to_string()));
    }
    Ok(expenses)
}

pub async fn create_expense(
    conn: &DatabaseConnection,
    user_id: i64,
    req: NewExpenseRequest,
) -> Result<i64, ExpenseError> {
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

    let referred_recurrence_query = RecurrenceType::find()
        .filter(recurrence_type::Column::Id.eq(req.recurrence_type_id))
        .one(conn);

    let referred_currency_query = CurrencyType::find()
        .filter(currency_type::Column::Id.eq(req.currency_type_id))
        .one(conn);

    let (referred_recurrence_type, referred_currency_type) =
        tokio::join!(referred_currency_query, referred_recurrence_query);

    if referred_recurrence_type.unwrap().is_none() || referred_currency_type.unwrap().is_none() {
        return Err(ExpenseError::InvalidExpenseData(String::from(
            "We have no recurrence or currency type with the given id.",
        )));
    }

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

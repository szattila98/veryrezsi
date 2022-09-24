use super::error::ExpenseError;

use crate::routes::dto::expenses::NewExpenseRequest;

use chrono::NaiveDate;
use entity::expense::{self, Entity as Expense};

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
    let parsed_date = NaiveDate::parse_from_str(&req.start_date, "%d-%m-%Y")?;

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

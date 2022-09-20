use super::error::ExpenseError;

use entity::expense::{self, Entity as Expense};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};


pub async fn find_expenses_by_user_id(
    conn: &DatabaseConnection,
    user_id: i64,
) -> Result<Vec<expense::Model>, ExpenseError> {
    let expenses = Expense::find()
        .filter(expense::Column::UserId.eq(user_id.clone()))
        .all(conn)
        .await?;

    if expenses.is_empty() {
        return Err(ExpenseError::NoExpenseFoundForUser(user_id.to_string()))
    }
    Ok(expenses)
}
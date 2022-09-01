use super::error::ExpenseError;

use entity::expense::{self, Entity as Expense};
use entity::Id;
use sea_orm::prelude::Uuid;
use sea_orm::ActiveValue::NotSet;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set, TransactionTrait,
};


pub async fn find_expense_by_user_id(
    conn: &DatabaseConnection,
    user_id: i64,
) -> Result<Vec<expense::Model>, ExpenseError> {
    let opt = Expense::find()
        .filter(expense::Column::UserId.eq(user_id.clone()))
        .all(conn)
        .await?;
    match opt {
        Some(expenses) => Ok(expenses),
        None => Err(ExpenseError::NoExpenseFoundForUser(user_id.to_string())),
    }
}
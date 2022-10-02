use super::common::ValidatedJson;
use super::{dto::expenses::NewExpenseRequest, error::ErrorMsg};
use crate::auth;
use crate::logic::expense_operations;

use axum::{Extension, Json};
use entity::{expense, predefined_expense, Id};
use sea_orm::DatabaseConnection;

/// Handles all expenses by current user query route
// FIXME: Not a good idea to return with domain model
pub async fn get_expenses(
    Extension(ref conn): Extension<DatabaseConnection>,
    user: auth::AuthenticatedUser,
) -> Result<Json<Vec<expense::Model>>, ErrorMsg<()>> {
    match expense_operations::find_expenses_by_user_id(conn, user.id).await {
        Ok(expenses) => Ok(Json(expenses)),
        Err(e) => Err(e.into()),
    }
}

/// Handles new expense creation route
pub async fn create_expense(
    ValidatedJson(new_expense_data): ValidatedJson<NewExpenseRequest>,
    user: auth::AuthenticatedUser,
    Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Json<Id>, ErrorMsg<()>> {
    match expense_operations::create_expense(conn, user.id, new_expense_data).await {
        Ok(expense_id) => Ok(Json(expense_id)),
        Err(e) => Err(e.into()),
    }
}

/// Handles predefined expense listing
pub async fn get_predefined_expenses(
    Extension(ref conn): Extension<DatabaseConnection>,
    _: auth::AuthenticatedUser,
) -> Result<Json<Vec<predefined_expense::Model>>, ErrorMsg<()>> {
    match expense_operations::find_predefined_expenses(conn).await {
        Ok(expenses) => Ok(Json(expenses)),
        Err(e) => Err(e.into()),
    }
}

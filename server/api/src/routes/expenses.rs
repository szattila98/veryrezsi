use sea_orm::DatabaseConnection;
use veryrezsi_core::dto::expenses::{NewExpenseRequest, NewPredefinedExpenseRequest};
use veryrezsi_core::logic::{expense_operations, user_operations};

use super::common::ValidatedJson;
use super::error::ErrorMsg;
use crate::auth;

use axum::extract::{Path, State};
use axum::Json;
use entity::{expense, predefined_expense, Id};

// FIXME: Not a good idea to return with domain model
pub async fn get_expenses(
    user: auth::AuthenticatedUser,
    State(ref conn): State<DatabaseConnection>,
    Path(user_id): Path<Id>,
) -> Result<Json<Vec<expense::Model>>, ErrorMsg<()>> {
    user_operations::authorize_user_by_id(user_id, user.id)?;
    match expense_operations::find_expenses_by_user_id(conn, user_id).await {
        Ok(expenses) => Ok(Json(expenses)),
        Err(e) => Err(e.into()),
    }
}

/// Handles new expense creation route
pub async fn create_expense(
    user: auth::AuthenticatedUser,
    State(ref conn): State<DatabaseConnection>,
    ValidatedJson(new_expense_data): ValidatedJson<NewExpenseRequest>,
) -> Result<Json<Id>, ErrorMsg<()>> {
    match expense_operations::create_expense(conn, user.id, new_expense_data).await {
        Ok(expense_id) => Ok(Json(expense_id)),
        Err(e) => Err(e.into()),
    }
}

/// Handles predefined expense listing
pub async fn get_predefined_expenses(
    _: auth::AuthenticatedUser,
    State(ref conn): State<DatabaseConnection>,
) -> Result<Json<Vec<predefined_expense::Model>>, ErrorMsg<()>> {
    match expense_operations::find_predefined_expenses(conn).await {
        Ok(expenses) => Ok(Json(expenses)),
        Err(e) => Err(e.into()),
    }
}

/// Handles new predefined expense creation route
pub async fn create_predefined_expense(
    _: auth::AuthenticatedUser,
    State(ref conn): State<DatabaseConnection>,
    ValidatedJson(req): ValidatedJson<NewPredefinedExpenseRequest>,
) -> Result<Json<Id>, ErrorMsg<()>> {
    match expense_operations::create_predefined_expense(conn, req).await {
        Ok(predefined_expense_id) => Ok(Json(predefined_expense_id)),
        Err(e) => Err(e.into()),
    }
}

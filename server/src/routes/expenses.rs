use super::common::ValidatedJson;
use super::{dto::expenses::NewExpenseRequest, error::ErrorMsg};
use crate::auth;
use crate::logic::expense_operations;

use axum::extract::Path;
use axum::{Extension, Json};
use axum_extra::extract::PrivateCookieJar;
use entity::{expense, Id};
use sea_orm::DatabaseConnection;

/// Handles all expenses by current user query route
// Not a good idea to return with domain model
pub async fn get_expenses(
	Extension(ref conn): Extension<DatabaseConnection>,
    user: auth::AuthenticatedUser
) -> Result<Json<Vec<expense::Model>>, ErrorMsg<()>> {
	match expense_operations::find_expenses_by_user_id(conn, user.id).await {
		Ok(expenses) => Ok(Json(expenses)),
		Err(e) => Err(e.into()),
	}
}

/// Handles new expense creation route
pub async fn create_expense(
	ValidatedJson(new_expense_data): ValidatedJson<NewExpenseRequest>,
	Path(userid): Path<String>,
	Extension(ref conn): Extension<DatabaseConnection>,
    cookies: PrivateCookieJar
) -> Result<Json<Id>, ErrorMsg<()>> {
	todo!()
}
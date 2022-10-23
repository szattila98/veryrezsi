use super::common::ValidatedJson;
use crate::auth;
use crate::logic::transaction_operations;
use super::{dto::transactions::NewTransactionRequest, error::ErrorMsg};
use entity::Id;

use axum::{http::StatusCode, Extension, Json};
use axum::extract::Path;
use sea_orm::DatabaseConnection;

/// Handles new transaction creation route
pub async fn create_transaction(
	ValidatedJson(new_transaction_data): ValidatedJson<NewTransactionRequest>,
	user: auth::AuthenticatedUser,
	Extension(ref conn): Extension<DatabaseConnection>,
) -> Result<Json<Id>, ErrorMsg<()>> {

	match transaction_operations::create_transaction(conn, user.id, new_transaction_data).await {
			Ok(transaction_id) => Ok(Json(transaction_id)),
			Err(e) => Err(e.into()),
	}
}

/// Handles transaction deletion route
pub async fn delete_transaction(
	Extension(ref conn): Extension<DatabaseConnection>,
	Path(transaction_id): Path<i64>,
	user: auth::AuthenticatedUser,
) -> Result<&'static str, ErrorMsg<()>> {

	match transaction_operations::delete_transaction_by_id(conn, user.id, transaction_id).await {
			Ok(_) => Ok("Transaction was deleted"),
			Err(e) => Err(e.into()),
	}
}
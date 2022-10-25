use super::common::ValidatedJson;
use super::{dto::transactions::NewTransactionRequest, error::ErrorMsg};
use crate::auth;
use crate::logic::transaction_operations;
use entity::Id;

use axum::extract::Path;
use axum::{http::StatusCode, Extension, Json};
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
) -> Result<StatusCode, ErrorMsg<()>> {
    match transaction_operations::delete_transaction_by_id(conn, user.id, transaction_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e.into()),
    }
}

use sea_orm::DatabaseConnection;
use veryrezsi_core::dto::transactions::NewTransactionRequest;
use veryrezsi_core::logic::transaction_operations;

use super::common::ValidatedJson;
use super::error::ErrorMsg;
use crate::auth;
use entity::Id;

use axum::extract::{Path, State};
use axum::{http::StatusCode, Json};

pub async fn create_transaction(
    user: auth::AuthenticatedUser,
    State(ref conn): State<DatabaseConnection>,
    ValidatedJson(new_transaction_data): ValidatedJson<NewTransactionRequest>,
) -> Result<Json<Id>, ErrorMsg<()>> {
    match transaction_operations::create_transaction(conn, user.id, new_transaction_data).await {
        Ok(transaction_id) => Ok(Json(transaction_id)),
        Err(e) => Err(e.into()),
    }
}

pub async fn delete_transaction(
    user: auth::AuthenticatedUser,
    State(ref conn): State<DatabaseConnection>,
    Path(transaction_id): Path<i64>,
) -> Result<StatusCode, ErrorMsg<()>> {
    match transaction_operations::delete_transaction_by_id(conn, user.id, transaction_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e.into()),
    }
}

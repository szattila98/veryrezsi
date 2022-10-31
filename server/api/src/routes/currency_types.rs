use crate::auth;

use super::error::ErrorMsg;
use axum::{Extension, Json};
use entity::currency_type;
use sea_orm::DatabaseConnection;
use veryrezsi_core::logic::currency_operations;

/// Handles currency type listing
pub async fn get_currency_types(
    Extension(ref conn): Extension<DatabaseConnection>,
    _: auth::AuthenticatedUser,
) -> Result<Json<Vec<currency_type::Model>>, ErrorMsg<()>> {
    match currency_operations::find_currency_types(conn).await {
        Ok(currency_types) => Ok(Json(currency_types)),
        Err(e) => Err(e.into()),
    }
}
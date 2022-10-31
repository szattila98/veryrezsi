use axum::{Extension, Json};
use entity::recurrence_type;
use sea_orm::DatabaseConnection;
use veryrezsi_core::logic::recurrence_operations;

use crate::auth;

use super::error::ErrorMsg;

/// Handles recurrence type listing
pub async fn get_recurrence_types(
    Extension(ref conn): Extension<DatabaseConnection>,
    _: auth::AuthenticatedUser,
) -> Result<Json<Vec<recurrence_type::Model>>, ErrorMsg<()>> {
    match recurrence_operations::find_recurrence_types(conn).await {
        Ok(recurrence_types) => Ok(Json(recurrence_types)),
        Err(e) => Err(e.into()),
    }
}

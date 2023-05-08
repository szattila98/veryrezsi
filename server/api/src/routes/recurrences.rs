use axum::{extract::State, Json};
use entity::recurrence;
use sea_orm::DatabaseConnection;
use veryrezsi_core::logic::recurrence_operations;

use crate::auth;

use super::error::ErrorMsg;

pub async fn get_recurrences(
    _: auth::AuthenticatedUser,
    State(ref conn): State<DatabaseConnection>,
) -> Result<Json<Vec<recurrence::Model>>, ErrorMsg<()>> {
    match recurrence_operations::find_recurrences(conn).await {
        Ok(recurrences) => Ok(Json(recurrences)),
        Err(e) => Err(e.into()),
    }
}

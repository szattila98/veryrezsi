use axum::{extract::State, Json};
use entity::recurrence_type;
use sea_orm::DatabaseConnection;
use veryrezsi_core::logic::recurrence_operations;

use crate::auth;

use super::error::ErrorMsg;

pub async fn get_recurrence_types(
    _: auth::AuthenticatedUser,
    State(ref conn): State<DatabaseConnection>,
) -> Result<Json<Vec<recurrence_type::Model>>, ErrorMsg<()>> {
    match recurrence_operations::find_recurrence_types(conn).await {
        Ok(recurrence_types) => Ok(Json(recurrence_types)),
        Err(e) => Err(e.into()),
    }
}

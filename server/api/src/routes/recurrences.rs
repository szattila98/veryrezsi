use axum::{extract::State, Json};
use sea_orm::DatabaseConnection;
use veryrezsi_core::{dto::recurrences::RecurrenceResponse, logic::recurrence_operations};

use crate::auth;

use super::error::ErrorMsg;

pub async fn get_recurrences(
    _: auth::AuthenticatedUser,
    State(ref conn): State<DatabaseConnection>,
) -> Result<Json<Vec<RecurrenceResponse>>, ErrorMsg<()>> {
    match recurrence_operations::find_recurrences(conn).await {
        Ok(recurrences) => Ok(Json(recurrences)),
        Err(e) => Err(e.into()),
    }
}

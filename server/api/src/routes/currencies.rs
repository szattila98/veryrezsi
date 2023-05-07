use crate::auth;

use super::error::ErrorMsg;
use axum::{extract::State, Json};
use entity::currencies;
use sea_orm::DatabaseConnection;
use veryrezsi_core::logic::currency_operations;

pub async fn get_currencies(
    _: auth::AuthenticatedUser,
    State(ref conn): State<DatabaseConnection>,
) -> Result<Json<Vec<currencies::Model>>, ErrorMsg<()>> {
    match currency_operations::find_currencies(conn).await {
        Ok(currencies) => Ok(Json(currencies)),
        Err(e) => Err(e.into()),
    }
}

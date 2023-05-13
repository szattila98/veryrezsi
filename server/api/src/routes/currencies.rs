use crate::auth;

use super::error::ErrorMsg;
use axum::{extract::State, Json};
use entity::currency;
use sea_orm::DatabaseConnection;
use veryrezsi_core::logic::currency_operations;

pub async fn get_currencies(
    _: auth::AuthenticatedUser,
    State(ref conn): State<DatabaseConnection>,
) -> Result<Json<Vec<CurrencyResponse>>, ErrorMsg<()>> {
    match currency_operations::find_currencies(conn).await {
        Ok(currencies) => Ok(Json(currencies)),
        Err(e) => Err(e.into()),
    }
}

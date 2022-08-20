use super::common::ValidatedJson;
use super::dto::expenses::NewExpenseRequest;
use super::{dto::expenses::LoginRequest, error::ErrorMsg};
use crate::logic::expense_operations;

use axum::{http::StatusCode, Extension, Json};
use axum_extra::extract::PrivateCookieJar;
use entity::{expense, Id};
use sea_orm::DatabaseConnection;

// Not a good idea to return with domain model
pub async fn get_expenses(
	Extension(ref conn): Extension<DatabaseConnection>,
    cookies: PrivateCookieJar,
) -> Result<Json<[expense::Model]>, ErrorMsg<()>> {
	todo!()
}


pub async fn create_expense(
	ValidatedJson(new_request_data): ValidatedJson<NewExpenseRequest>,
	Path(userid): Path<String>
	Extension(ref conn): Extension<DatabaseConnection>,
    cookies: PrivateCookieJar
) -> Result<Json<Id>, ErrorMsg<()>> {
	todo!()
}
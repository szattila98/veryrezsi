use super::common::ValidatedJson;
use super::dto::expenses::NewUserRequest;
use super::{dto::expenses::LoginRequest, error::ErrorMsg};
use crate::auth::{self, AUTH_COOKIE_NAME};
use crate::config::AppConfig;
use crate::logic::expense_operation;
use axum::extract::Path;
use axum::{http::StatusCode, Extension, Json};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};
use entity::user;
use pwhash::bcrypt;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub async fn getExpenses(
	ValidatedJson(request_data): ValidatedJson<>
)
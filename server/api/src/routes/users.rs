use super::common::ValidatedJson;
use super::error::ErrorMsg;
use super::AppState;
use crate::auth::{self, AUTH_COOKIE_NAME};
use axum::extract::{Path, State};
use axum::{http::StatusCode, Json};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};
use veryrezsi_core::dto::users::{LoginRequest, NewUserRequest, UserResponse};
use veryrezsi_core::logic::user_operations;
use veryrezsi_core::DatabaseConnection;

pub async fn login(
    cookies: PrivateCookieJar,
    State(ref conn): State<DatabaseConnection>,
    ValidatedJson(req): ValidatedJson<LoginRequest>,
) -> Result<PrivateCookieJar, ErrorMsg<()>> {
    let user_id = user_operations::verify_login(conn, req).await?;
    let mut cookie = Cookie::new(auth::AUTH_COOKIE_NAME, user_id.to_string());
    cookie.set_path("/");
    Ok(cookies.add(cookie))
}

pub async fn me(
    user: auth::AuthenticatedUser,
    State(ref conn): State<DatabaseConnection>,
) -> Result<Json<UserResponse>, ErrorMsg<()>> {
    match user_operations::find_user_by_id(conn, user.id).await? {
        Some(user) => Ok(Json(user)),
        None => Err(ErrorMsg::new(StatusCode::NOT_FOUND, "user not found")),
    }
}

pub async fn logout(cookies: PrivateCookieJar) -> Result<PrivateCookieJar, ErrorMsg<()>> {
    match cookies.get(AUTH_COOKIE_NAME) {
        Some(cookie) => Ok(cookies.remove(cookie)),
        None => Err(ErrorMsg::new(StatusCode::BAD_REQUEST, "not logged in")),
    }
}

pub async fn register(
    State(app_state): State<AppState>,
    ValidatedJson(new_user): ValidatedJson<NewUserRequest>,
) -> Result<Json<UserResponse>, ErrorMsg<()>> {
    match user_operations::save_user(
        &app_state.config,
        &app_state.conn,
        app_state.mail_transport,
        new_user,
    )
    .await
    {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e.into()),
    }
}

pub async fn activate_account(
    State(ref conn): State<DatabaseConnection>,
    Path(token): Path<String>,
) -> Result<(), ErrorMsg<()>> {
    match user_operations::activate_account(conn, token).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}

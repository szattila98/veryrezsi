use super::dto::NewUserRequest;
use super::common::ValidatedJson;
use super::{dto::LoginRequest, error::ErrorMsg};
use crate::auth::{self, AUTH_COOKIE_NAME};
use crate::logic::user_operations;
use axum::{http::StatusCode, Extension, Json};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};
use entity::user;
use pwhash::bcrypt;
use sea_orm::DatabaseConnection;

pub async fn login(
    ValidatedJson(login_data): ValidatedJson<LoginRequest>,
    Extension(ref conn): Extension<DatabaseConnection>,
    cookies: PrivateCookieJar,
) -> Result<PrivateCookieJar, ErrorMsg<()>> {
    match user_operations::find_user_by_username(conn, login_data.username.to_string()).await {
        Ok(user) => {
            if bcrypt::verify(login_data.password, &user.pw_hash) {
                return Ok(cookies.add(Cookie::new(auth::AUTH_COOKIE_NAME, user.id.to_string())));
            }
            Err(ErrorMsg::new(
                StatusCode::UNAUTHORIZED,
                "incorrect credentials",
            ))
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn me(
    Extension(ref conn): Extension<DatabaseConnection>,
    user: auth::AuthenticatedUser,
) -> Result<Json<user::Model>, ErrorMsg<()>> {
    // TODO maybe query user from db in the guard and then there is even less repetition with always finding the user by id
    match user_operations::find_user_by_id(conn, user.id).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e.into()),
    }
}

pub async fn logout(cookies: PrivateCookieJar) -> Result<PrivateCookieJar, ErrorMsg<()>> {
    match cookies.get(AUTH_COOKIE_NAME) {
        Some(cookie) => Ok(cookies.remove(cookie)),
        None => Err(ErrorMsg::new(StatusCode::BAD_REQUEST, "not logged in")),
    }
}

pub async fn register(
    Extension(ref conn): Extension<DatabaseConnection>,
    ValidatedJson(new_user): ValidatedJson<NewUserRequest>,
) -> Result<Json<user::Model>, ErrorMsg<()>> {
    match user_operations::save_user(conn, new_user).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e.into()),
    }
}

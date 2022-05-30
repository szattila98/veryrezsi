use axum::{http, response::IntoResponse, Extension, Json};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};
use pwhash::bcrypt;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{self, AUTH_COOKIE_NAME},
    logic::user_operations::NewUser,
};
use entity::user;

use crate::logic::user_operations;

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginData {
    pub username: String,
    pub password: String,
}

pub async fn login(
    Json(login_data): Json<LoginData>,
    Extension(ref conn): Extension<DatabaseConnection>,
    cookies: PrivateCookieJar,
) -> Result<PrivateCookieJar, http::StatusCode> {
    if let Ok(user) =
        user_operations::find_user_by_username(conn, login_data.username.to_string()).await
    {
        if bcrypt::verify(login_data.password, &user.pw_hash) {
            return Ok(cookies.add(Cookie::new(auth::AUTH_COOKIE_NAME, user.id.to_string())));
        }
    }
    Err(http::StatusCode::UNAUTHORIZED)
}

pub async fn me(
    Extension(ref conn): Extension<DatabaseConnection>,
    user: auth::AuthenticatedUser,
) -> Result<Json<user::Model>, http::StatusCode> {
    // TODO maybe query user from db in the guard and then there is even less repetition with always finding the user by id
    match user_operations::find_user_by_id(conn, user.id).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(http::StatusCode::UNAUTHORIZED),
    }
}

pub async fn logout(cookies: PrivateCookieJar) -> impl IntoResponse {
    cookies.remove(Cookie::named(AUTH_COOKIE_NAME))
}

pub async fn register(
    Extension(ref conn): Extension<DatabaseConnection>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<user::Model>, http::StatusCode> {
    match user_operations::save_user(conn, new_user).await {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(http::StatusCode::BAD_REQUEST),
    }
}

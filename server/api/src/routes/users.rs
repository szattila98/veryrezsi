use super::common::ValidatedJson;
use super::error::ErrorMsg;
use crate::auth::{self, AUTH_COOKIE_NAME};
use axum::extract::Path;
use axum::{http::StatusCode, Extension, Json};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};
use entity::user;
use pwhash::bcrypt;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use veryrezsi_core::config::AppConfig;
use veryrezsi_core::dto::users::{LoginRequest, NewUserRequest};
use veryrezsi_core::email::MailTransport;
use veryrezsi_core::logic::user_operations;

pub async fn login(
    ValidatedJson(login_data): ValidatedJson<LoginRequest>,
    Extension(ref conn): Extension<DatabaseConnection>,
    cookies: PrivateCookieJar,
) -> Result<PrivateCookieJar, ErrorMsg<()>> {
    match user_operations::find_user_by_email(conn, login_data.email.to_string()).await {
        Ok(user) => {
            return if user.activated {
                return if bcrypt::verify(login_data.password, &user.pw_hash) {
                    let mut cookie = Cookie::new(auth::AUTH_COOKIE_NAME, user.id.to_string());
                    cookie.set_path("/api");
                    Ok(cookies.add(cookie))
                } else {
                    Err(ErrorMsg::new(
                        StatusCode::UNAUTHORIZED,
                        "incorrect credentials",
                    ))
                };
            } else {
                Err(ErrorMsg::new(
                    StatusCode::BAD_REQUEST,
                    "account not activated",
                ))
            }
        }
        Err(e) => Err(e.into()),
    }
}

pub async fn me(
    Extension(ref conn): Extension<DatabaseConnection>,
    user: auth::AuthenticatedUser,
) -> Result<Json<user::Model>, ErrorMsg<()>> {
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
    Extension(ref config): Extension<AppConfig>,
    Extension(ref conn): Extension<DatabaseConnection>,
    Extension(mail_transport): Extension<Arc<MailTransport>>,
    ValidatedJson(new_user): ValidatedJson<NewUserRequest>,
) -> Result<Json<user::Model>, ErrorMsg<()>> {
    match user_operations::save_user(config, conn, mail_transport, new_user).await {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(e.into()),
    }
}

pub async fn activate_account(
    Extension(ref conn): Extension<DatabaseConnection>,
    Path(token): Path<String>,
) -> Result<&'static str, ErrorMsg<()>> {
    match user_operations::activate_account(conn, token).await {
        Ok(_) => Ok("Account activated!"),
        Err(e) => Err(e.into()),
    }
}

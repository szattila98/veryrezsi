use super::common::ValidatedJson;
use super::error::ErrorMsg;
use super::AppState;
use crate::auth::{self, AUTH_COOKIE_NAME};
use axum::extract::{Path, State};
use axum::{http::StatusCode, Json};
use axum_extra::extract::{cookie::Cookie, PrivateCookieJar};
use entity::user;
use pwhash::bcrypt;
use sea_orm::DatabaseConnection;
use veryrezsi_core::dto::users::{LoginRequest, NewUserRequest};
use veryrezsi_core::logic::{find_entity_by_id, user_operations};

pub async fn login(
    cookies: PrivateCookieJar,
    State(ref conn): State<DatabaseConnection>,
    ValidatedJson(login_data): ValidatedJson<LoginRequest>,
) -> Result<PrivateCookieJar, ErrorMsg<()>> {
    let Some(user) = user_operations::find_user_by_email(conn, login_data.email).await? else {
        return Err(ErrorMsg::new(
            StatusCode::UNAUTHORIZED,
            "invalid credentials",
        ));
    };
    if !user.activated {
        return Err(ErrorMsg::new(
            StatusCode::BAD_REQUEST,
            "account not activated",
        ));
    };
    if !bcrypt::verify(login_data.password, &user.pw_hash) {
        return Err(ErrorMsg::new(
            StatusCode::UNAUTHORIZED,
            "incorrect credentials",
        ));
    };
    let mut cookie = Cookie::new(auth::AUTH_COOKIE_NAME, user.id.to_string());
    cookie.set_path("/");
    Ok(cookies.add(cookie))
}

pub async fn me(
    user: auth::AuthenticatedUser,
    State(ref conn): State<DatabaseConnection>,
) -> Result<Json<user::Model>, ErrorMsg<()>> {
    match find_entity_by_id::<user::Entity>(conn, user.id).await? {
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
) -> Result<Json<user::Model>, ErrorMsg<()>> {
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
) -> Result<&'static str, ErrorMsg<()>> {
    match user_operations::activate_account(conn, token).await {
        Ok(_) => Ok("Account activated!"),
        Err(e) => Err(e.into()),
    }
}

use std::num::ParseIntError;

use crate::routes::{error::ErrorMsg, AppState};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use axum_extra::extract::cookie::{Key, PrivateCookieJar};
use tracing::debug;
use veryrezsi_core::Id;

/// Defines the name of the cookie used to authenticate users.
pub const AUTH_COOKIE_NAME: &str = "JSESSIONID";

/// Identifies a user. It is used to hold the decrypted content of the authentication cookie.
pub struct AuthenticatedUser {
    pub id: Id,
}

impl AuthenticatedUser {
    fn new(id: Id) -> Self {
        AuthenticatedUser { id }
    }
}

#[async_trait]
impl FromRequestParts<AppState> for AuthenticatedUser {
    type Rejection = ErrorMsg<()>;

    /// Extracts the authenticated user from the request.
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jar = PrivateCookieJar::<Key>::from_request_parts(parts, &state.secret_key)
            .await
            .expect("this should have been infalliable");
        return if let Some(cookie) = jar.get(AUTH_COOKIE_NAME) {
            let id = cookie.value().parse()?;
            Ok(AuthenticatedUser::new(id))
        } else {
            debug!("No authentication cookie found");
            Err(ErrorMsg::new(StatusCode::UNAUTHORIZED, "not logged in"))
        };
    }
}

impl From<ParseIntError> for ErrorMsg<()> {
    fn from(e: ParseIntError) -> Self {
        debug!("{e}");
        ErrorMsg::new(StatusCode::BAD_REQUEST, "malformed cookie, non-parsable")
    }
}

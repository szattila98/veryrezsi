use std::num::ParseIntError;

use crate::routes::error::ErrorMsg;
use axum::{
    async_trait,
    extract::{rejection::ExtensionRejection, FromRequest, RequestParts},
    http::StatusCode,
};
use axum_extra::extract::cookie::{Key, PrivateCookieJar};
use entity::Id;
use tracing::debug;

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
impl<B> FromRequest<B> for AuthenticatedUser
where
    B: Send, // required by `async_trait`
{
    type Rejection = ErrorMsg<()>;

    /// Extracts the authenticated user from the request.
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let jar = PrivateCookieJar::<Key>::from_request(req).await?;
        return if let Some(cookie) = jar.get(AUTH_COOKIE_NAME) {
            let id = cookie.value().parse()?;
            Ok(AuthenticatedUser::new(id))
        } else {
            debug!("No authentication cookie found");
            Err(ErrorMsg::new(StatusCode::UNAUTHORIZED, "not logged in"))
        };
    }
}

impl From<ExtensionRejection> for ErrorMsg<()> {
    fn from(e: ExtensionRejection) -> Self {
        debug!("{e}");
        ErrorMsg::new(
            StatusCode::BAD_REQUEST,
            "malformed request, cannot extract cookies",
        )
    }
}

impl From<ParseIntError> for ErrorMsg<()> {
    fn from(e: ParseIntError) -> Self {
        debug!("{e}");
        ErrorMsg::new(StatusCode::BAD_REQUEST, "malformed cookie, non-parsable")
    }
}

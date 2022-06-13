use crate::routes::error::ErrorMsg;
use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http::StatusCode,
};
use axum_extra::extract::cookie::{Key, PrivateCookieJar};
use tracing::debug;

pub const AUTH_COOKIE_NAME: &str = "JSESSIONID";

pub struct AuthenticatedUser {
    pub id: i32,
}

impl AuthenticatedUser {
    fn new(id: i32) -> Self {
        AuthenticatedUser { id }
    }
}

#[async_trait]
impl<B> FromRequest<B> for AuthenticatedUser
where
    B: Send, // required by `async_trait`
{
    type Rejection = ErrorMsg<()>;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        // TODO refactor to be more elegant
        if let Ok(jar) = PrivateCookieJar::<Key>::from_request(req).await {
            if let Some(cookie) = jar.get(AUTH_COOKIE_NAME) {
                if let Ok(id) = cookie.value().parse() {
                    return Ok(AuthenticatedUser::new(id));
                }
                debug!(
                    "Could not parse the value of the cookie, value was:\n{:?}",
                    cookie.value()
                );
            } else {
                debug!("Could not get {AUTH_COOKIE_NAME} cookie from the jar");
            }
        } else {
            debug!("Could not create PrivateCookieJar from request");
        }
        Err(ErrorMsg::new(StatusCode::UNAUTHORIZED, "Not logged in"))
    }
}

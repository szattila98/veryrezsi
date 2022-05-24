use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FromRequest, Request};
use thiserror::Error;

pub const AUTH_COOKIE_NAME: &str = "SESSIONID";

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("no user id was found in the cookie")]
    NoValidCookie,
}

pub struct AuthenticatedUser {
    pub id: i32,
}

impl AuthenticatedUser {
    fn new(id: i32) -> Self {
        AuthenticatedUser { id }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthenticatedUser {
    type Error = AuthError;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        req.cookies()
            .get_private(AUTH_COOKIE_NAME)
            .and_then(|c| c.value().parse().ok())
            .map(AuthenticatedUser::new)
            .into_outcome((Status::Unauthorized, AuthError::NoValidCookie))
    }
}

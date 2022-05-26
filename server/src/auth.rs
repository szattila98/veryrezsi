use axum::{
    async_trait,
    extract::{FromRequest, RequestParts},
    http,
};
use axum_extra::extract::cookie::{Key, PrivateCookieJar};

pub const AUTH_COOKIE_NAME: &str = "SESSION";

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
    type Rejection = http::StatusCode;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let result = PrivateCookieJar::<Key>::from_request(req).await;
        if let Ok(jar) = result {
            if let Some(cookie) = jar.get(AUTH_COOKIE_NAME) {
                if let Ok(id) = cookie.value().parse() {
                    return Ok(AuthenticatedUser::new(id));
                }
            }
        }
        Err(http::StatusCode::UNAUTHORIZED)
    }
}

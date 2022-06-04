use axum::{
    async_trait,
    body::HttpBody,
    extract::{rejection::JsonRejection, FromRequest, RequestParts},
    http::StatusCode,
    response::{IntoResponse, Response},
    BoxError, Json,
};
use serde::{de::DeserializeOwned, Serialize};
use validator::{Validate, ValidationErrors};

use crate::logic::error::UserError;

#[derive(Debug, Serialize)]
pub struct ErrorMsg {
    #[serde(skip_serializing)]
    status: StatusCode,
    reason: String,
}

impl ErrorMsg {
    pub fn new<S: AsRef<str>>(status: StatusCode, reason: S) -> Self {
        Self {
            status,
            reason: reason.as_ref().into(),
        }
    }
}

impl IntoResponse for ErrorMsg {
    fn into_response(self) -> Response {
        (self.status, Json(self)).into_response()
    }
}

impl From<JsonRejection> for ErrorMsg {
    fn from(e: JsonRejection) -> Self {
        Self::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    }
}

impl From<ValidationErrors> for ErrorMsg {
    fn from(e: ValidationErrors) -> Self {
        let message = format!("{}", e).replace('\n', ", ");
        Self::new(StatusCode::BAD_REQUEST, message)
    }
}

impl From<UserError> for ErrorMsg {
    fn from(e: UserError) -> Self {
        match e {
            UserError::UserNotFound(_) => Self::new(StatusCode::NOT_FOUND, e.to_string()),
            UserError::EmailAlreadyExists(_) => Self::new(StatusCode::BAD_REQUEST, e.to_string()),
            UserError::PasswordHashError(_) => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
            UserError::DatabaseError(_) => {
                Self::new(StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, B> FromRequest<B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    B: Send + HttpBody,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ErrorMsg;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

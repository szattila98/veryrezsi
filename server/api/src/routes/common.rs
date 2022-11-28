use super::error::ErrorMsg;
use axum::{async_trait, body::HttpBody, extract::FromRequest, http::Request, BoxError, Json};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};

/// A generic structure that represents a requests body, that is validated according to its defined validation rules.
/// Should be placed last in a function parameter signature because is consumes the request body.
#[derive(Debug, Clone, Copy, Default)]
pub struct ValidatedJson<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    B: Send + HttpBody + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = ErrorMsg<ValidationErrors>;

    /// Extracts and validates the request body and returns a `ValidatedJson` if both succeeds.
    /// It is bounded by traits so the generic type has to be something that can be deserialized into and validated.
    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

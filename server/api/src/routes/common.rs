use super::error::ErrorMsg;
use axum::{
    async_trait,
    body::HttpBody,
    extract::{FromRequest, RequestParts},
    BoxError, Json,
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};

/// A generic structure that represents a requests body, that is validated according to its defined validation rules.
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
    type Rejection = ErrorMsg<ValidationErrors>;

    /// Extracts and validates the request body and returns a `ValidatedJson` if both succeeds.
    /// It is bounded by traits so the generic type has to be something that can be deserialized into and validated.
    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req).await?;
        value.validate()?;
        Ok(ValidatedJson(value))
    }
}

#![allow(missing_docs)]

use fancy_regex::Regex;
use lazy_static::lazy_static;
use serde::Deserialize;
use validator::{Validate, ValidationError};

lazy_static! {
    /// Password validation regex.
    static ref PASSWORD_REGEX: Regex =
        Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,}$")
            .unwrap();
}

/// DTO for user login.
#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "username must be between 1 and 255 characters"
    ))]
    pub email: String,
    #[validate(length(
        min = 1,
        max = 255,
        message = "password must be between 1 and 255 characters"
    ))]
    pub password: String,
}

/// DTO for user registration.
#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NewUserRequest {
    #[validate(email(message = "email must be valid"))]
    #[validate(length(
        min = 1,
        max = 320,
        message = "email must be between 1 and 320 characters"
    ))]
    pub email: String,
    #[validate(length(
        min = 1,
        max = 255,
        message = "username must be between 1 and 255 characters"
    ))]
    pub username: String,
    #[validate(custom = "validate_password")]
    pub password: String,
    #[validate(must_match(
        other = "password",
        message = "password and password confirmation must match"
    ))]
    pub confirm_password: String,
}

/// Password validation function supplied to NewUserRequest.
fn validate_password(value: &str) -> Result<(), ValidationError> {
    if PASSWORD_REGEX.is_match(value).unwrap() {
        return Ok(());
    }
    Err(ValidationError::new("password must be at least 8 characters long and contain at least one uppercase letter, one lowercase letter, one number and one special character"))
}

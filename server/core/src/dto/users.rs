use entity::{user, Id};
use fancy_regex::Regex;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

lazy_static! {
    /// Password validation regex.
    static ref PASSWORD_REGEX: Regex =
        Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)(?=.*[@$!%*?&])[A-Za-z\d@$!%*?&]{8,120}$")
            .expect("incorrect password regex");
}

#[derive(Deserialize, Validate, Clone)]
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

#[derive(Deserialize, Validate, Clone)]
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

/// Password validation function supplied to `NewUserRequest`.
fn validate_password(value: &str) -> Result<(), ValidationError> {
    let Ok(result) = PASSWORD_REGEX.is_match(value) else {
        return Err(ValidationError::new(
            "password cannot be matched against regex",
        ));
    };
    if !result {
        return Err(ValidationError::new("password must be at least 8 characters long and contain at least one uppercase letter, one lowercase letter, one number and one special character"));
    }
    Ok(())
}

#[derive(Clone, Serialize, PartialEq)]
pub struct UserResponse {
    pub id: Id,
    pub email: String,
    pub username: String,
}

impl From<user::Model> for UserResponse {
    fn from(user: user::Model) -> Self {
        Self {
            id: user.id,
            email: user.email,
            username: user.username,
        }
    }
}

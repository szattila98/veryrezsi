use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "username must be between 1 and 255 characters"
    ))]
    pub username: String,
    #[validate(length(
        min = 1,
        max = 255,
        message = "password must be between 1 and 255 characters"
    ))]
    pub password: String,
}

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
    // TODO do some password validation, but not (or) with (complicated) regex, as look-around is not supported in the regex crate
    pub password: String,
    #[validate(must_match(
        other = "password",
        message = "password and password confirmation must match"
    ))]
    pub confirm_password: String,
}

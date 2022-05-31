use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
    pub confirm_password: String,
}

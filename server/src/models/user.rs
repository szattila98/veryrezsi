use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub username: String,
    pub pw_hash: String,
}

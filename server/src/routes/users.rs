use pwhash::bcrypt;
use rocket::http::{Cookie, CookieJar};
use rocket::response::status;
use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::auth;
use crate::database::Db;
use entity::user;
use sea_orm_rocket::Connection;

use crate::logic::user_operations;

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginData<'r> {
    pub username: &'r str,
    pub password: &'r str,
}

#[post("/auth", data = "<login_data>")]
pub async fn login(
    login_data: Json<LoginData<'_>>,
    conn: Connection<'_, Db>,
    cookies: &CookieJar<'_>,
) -> Result<(), status::Unauthorized<()>> {
    if let Ok(user) =
        user_operations::find_user_by_username(conn.into_inner(), login_data.username.to_string())
            .await
    {
        // isn't a secret key needed for bcrypt? it generates different hashes each time but can verify them so maybe not?
        if bcrypt::verify(login_data.password, &user.pw_hash) {
            cookies.add_private(Cookie::new(auth::AUTH_COOKIE_NAME, user.id.to_string()));
            return Ok(());
        }
    }
    Err(status::Unauthorized::<()>(None))
}

#[get("/me")]
pub async fn me(
    conn: Connection<'_, Db>,
    user: auth::AuthenticatedUser,
) -> Result<Json<user::Model>, status::Unauthorized<String>> {
    // TODO maybe query user from db in the guard and then there is even less repetition with always finding the user by id
    let result = user_operations::find_user_by_id(conn.into_inner(), user.id).await;
    return match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err(status::Unauthorized(Some(format!("{e}")))),
    };
}

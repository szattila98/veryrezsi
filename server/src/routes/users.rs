use rocket::http::{Cookie, CookieJar};
use rocket::response::status;
use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::database::Db;
use entity::user;
use sea_orm_rocket::Connection;

use crate::logic::user_operations;

const AUTH_COOKIE_NAME: &str = "JSESSIONID";

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest<'r> {
    pub username: &'r str,
    pub password: &'r str,
}

#[post("/auth", data = "<login_data>")]
pub async fn login(
    conn: Connection<'_, Db>,
    cookies: &CookieJar<'_>,
    login_data: Json<LoginRequest<'_>>,
) -> Result<(), status::Unauthorized<()>> {
    if let Ok(user) =
        user_operations::find_user_by_username(conn.into_inner(), login_data.username.to_string())
            .await
    {
        cookies.add_private(Cookie::new(AUTH_COOKIE_NAME, user.id.to_string()));
        return Ok(());
    }
    Err(status::Unauthorized::<()>(None))
}

#[get("/me")]
pub async fn me(
    conn: Connection<'_, Db>,
    cookies: &CookieJar<'_>,
) -> Result<Json<user::Model>, status::Unauthorized<String>> {
    if let Some(auth_cookie) = cookies.get_private(AUTH_COOKIE_NAME) {
        let id = auth_cookie.value().parse::<i32>().unwrap(); // what to do when cookie does not contain parsable integer?
        let result = user_operations::find_user_by_id(conn.into_inner(), id).await;
        return match result {
            Ok(user) => Ok(Json(user)),
            Err(e) => Err(status::Unauthorized(Some(format!("{e}")))), // what to do when there is no such user as the id in the cookie says?
        };
    }
    Err(status::Unauthorized(None))
}

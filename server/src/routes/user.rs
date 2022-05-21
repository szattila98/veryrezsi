use rocket::http::{Cookie, CookieJar};
use rocket::response::status;
use rocket::serde::{json::Json, Deserialize, Serialize};

pub use entity::user;
pub use entity::user::Entity as User;
// use sea_orm_rocket::{Connection, Database};

const AUTH_COOKIE_NAME: &str = "JSESSIONID";

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginRequest<'r> {
    pub username: &'r str,
    pub password: &'r str,
}

#[post("/auth", data = "<login_data>")]
pub async fn auth(
    cookies: &CookieJar<'_>,
    login_data: Json<LoginRequest<'_>>,
) -> Result<(), status::Unauthorized<()>> {
    if login_data.username == "admin" && login_data.password == "admin" {
        cookies.add_private(Cookie::new(AUTH_COOKIE_NAME, "1"));
        return Ok(());
    }
    Err(status::Unauthorized::<()>(None))
}

#[get("/me")]
pub async fn me(cookies: &CookieJar<'_>) -> Result<Json<user::Model>, status::Unauthorized<()>> {
    if let Some(auth_cookie) = cookies.get_private(AUTH_COOKIE_NAME) {
        println!("user_id: {}", auth_cookie.value());
        return Ok(Json(user::Model {
            id: 1,
            username: "admin".to_string(),
            email: "admin@mail.com".to_string(),
            pw_hash: "dont_send_this".to_string(),
        }));
    }
    Err(status::Unauthorized::<()>(None))
}

#[macro_use]
extern crate rocket;

use dotenv::dotenv;

mod config;
mod database;
mod errors;
mod models;
mod routes;

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    rocket::build().mount("/api/user", routes![routes::user::auth, routes::user::me])
}

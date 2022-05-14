#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_sync_db_pools;
#[macro_use]
extern crate diesel;

use dotenv::dotenv;
use rocket::serde::json::{json, Value};

mod config;
mod database;
mod errors;
mod models;
mod routes;
mod schema;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "404",
        "reason": "Resource was not found."
    })
}

#[launch]
pub fn rocket() -> _ {
    dotenv().ok();
    rocket::custom(config::from_env())
        .mount("/api/user", routes![routes::user::auth, routes::user::me])
        .register("/", catchers![not_found])
        .attach(database::Database::fairing())
}

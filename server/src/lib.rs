#[macro_use]
extern crate rocket;
use migration::MigratorTrait;
use rocket::fairing::{self, AdHoc};
use rocket::Build;
use rocket::{
    serde::json::{json, Value},
    Rocket,
};
use sea_orm_rocket::Database;

mod database;
use database::Db;

mod logic;
mod routes;

#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "404",
        "reason": "Resource was not found."
    })
}

async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    let conn = &Db::fetch(&rocket).unwrap().conn;
    let _ = migration::Migrator::up(conn, None).await;
    Ok(rocket)
}

#[launch]
pub fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("Migrations", run_migrations))
        .mount(
            "/api/user",
            routes![routes::users::login, routes::users::me],
        )
        .register("/", catchers![not_found])
}

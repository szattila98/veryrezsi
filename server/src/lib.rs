use axum::Router;
use axum_extra::extract::cookie::Key;
use config::Config;

mod database;

mod auth;
mod config;
mod logic;
mod routes;

pub async fn init() -> (String, String, Router) {
    let config = Config::init();
    tracing_subscriber::fmt::init();
    let conn = database::init(config.database_url).await;
    let cookie_key = Key::from(config.cookie_key.as_bytes());
    let router = routes::init(conn, cookie_key);
    (config.server_host, config.server_port, router)
}

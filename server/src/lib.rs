use std::{env, net::SocketAddr};

use axum::{
    routing::{get, post},
    Extension, Router,
};
use axum_extra::extract::cookie::Key;
use tower::ServiceBuilder;

mod database;

mod auth;
mod logic;
mod routes;

pub async fn init() -> (SocketAddr, Router) {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let cookie_key = env::var("COOKIE_KEY").expect("COOKIE_KEY is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let conn = database::init().await;

    let key = Key::from(cookie_key.as_bytes());

    let app = Router::new()
        .route("/api/user/auth", post(routes::users::login))
        .route("/api/user/me", get(routes::users::me))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(conn))
                .layer(Extension(key)),
        );

    let addr = server_url.parse().expect("Could not parse host and port");
    (addr, app)
}

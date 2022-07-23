use crate::{config::AppConfig, email::Mailer};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use axum_extra::extract::cookie::Key;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tower::ServiceBuilder;

pub mod common;
pub mod dto;
pub mod error;
pub mod users;

pub fn init(
    config: AppConfig,
    conn: DatabaseConnection,
    secret_key: Key,
    mailer: Mailer,
) -> Router {
    let user_api = Router::new()
        .route("/auth", post(users::login))
        .route("/me", get(users::me))
        .route("/logout", post(users::logout))
        .route("/register", post(users::register))
        .route("/activate/:token", get(users::activate_account));

    let api = Router::new().nest("/user", user_api);

    Router::new().nest("/api", api).layer(
        ServiceBuilder::new()
            .layer(Extension(config))
            .layer(Extension(conn))
            .layer(Extension(secret_key))
            .layer(Extension(Arc::new(mailer))),
    )
}

use crate::{config::AppConfig, email::Mailer};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use axum_extra::extract::cookie::Key;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tower::ServiceBuilder;

/// Common code for all routes.
pub mod common;
/// DTOs used in routes.
pub mod dto;
/// Error handling on the controller level.
pub mod error;
/// User route handlers.
pub mod users;

/// Initializes the router with the extension layers and the route handlers.
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

    let expense_api = Router::new()
        .route("/", get)

    let api = Router::new()
        .nest("/user", user_api)
        .nest("/expense", expense_api);

    Router::new().nest("/api", api).layer(
        ServiceBuilder::new()
            .layer(Extension(config))
            .layer(Extension(conn))
            .layer(Extension(secret_key))
            .layer(Extension(Arc::new(mailer))),
    )
}

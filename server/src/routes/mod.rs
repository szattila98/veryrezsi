#![allow(missing_docs)]

use crate::{config::AppConfig, email::Mailer};
use axum::{
    routing::{get, post, delete},
    Extension, Router,
};
use axum_extra::extract::cookie::Key;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tower::ServiceBuilder;

pub mod common;
pub mod currency_types;
pub mod dto;
pub mod error;
pub mod expenses;
pub mod transactions;
pub mod recurrence_types;
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
        .route("/:user_id", get(expenses::get_expenses))
        .route("/", post(expenses::create_expense))
        .route("/predefined", get(expenses::get_predefined_expenses))
        .route("/predefined", post(expenses::create_predefined_expense));

    let transaction_api = Router::new()
        .route("/", post(transactions::create_transaction))
        .route("/:transaction_id", delete(transactions::delete_transaction));

    let currency_api = Router::new().route("/", get(currency_types::get_currency_types));

    let recurrence_api = Router::new().route("/", get(recurrence_types::get_recurrence_types));

    let api = Router::new()
        .nest("/user", user_api)
        .nest("/expense", expense_api)
        .nest("/transaction/", transaction_api)
        .nest("/currency", currency_api)
        .nest("/recurrence", recurrence_api);

    Router::new().nest("/api", api).layer(
        ServiceBuilder::new()
            .layer(Extension(config))
            .layer(Extension(conn))
            .layer(Extension(secret_key))
            .layer(Extension(Arc::new(mailer))),
    )
}

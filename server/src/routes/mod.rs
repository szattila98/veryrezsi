use axum::{
    routing::{get, post},
    Extension, Router,
};
use axum_extra::extract::cookie::Key;
use sea_orm::DatabaseConnection;
use tower::ServiceBuilder;

pub mod users;

pub fn init(conn: DatabaseConnection, secret_key: Key) -> Router {
    let user_api = Router::new()
        .route("/auth", post(users::login))
        .route("/me", get(users::me))
        .route("/logout", post(users::logout));

    let api = Router::new().nest("/user", user_api);

    Router::new().nest("/api", api).layer(
        ServiceBuilder::new()
            .layer(Extension(conn))
            .layer(Extension(secret_key)),
    )
}

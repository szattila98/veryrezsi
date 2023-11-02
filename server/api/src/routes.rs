use axum::{
    extract::MatchedPath,
    http::Request,
    middleware::{self, Next},
    response::IntoResponse,
    routing::{delete, get, post},
    Router,
};
use axum_extra::extract::cookie::Key;
use axum_macros::FromRef;
use std::{sync::Arc, time::Instant};
use tower_http::trace::{self, TraceLayer};
use tracing::Level;
use veryrezsi_core::DatabaseConnection;
use veryrezsi_core::{config::AppConfig, email::MailTransport};

pub mod common;
pub mod currencies;
pub mod error;
pub mod expenses;
pub mod recurrences;
pub mod transactions;
pub mod users;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub config: AppConfig,
    pub conn: DatabaseConnection,
    pub secret_key: Key,
    pub mail_transport: Arc<MailTransport>,
}

/// Initializes the router with the extension layers and the route handlers.
pub fn init(
    config: AppConfig,
    conn: DatabaseConnection,
    secret_key: Key,
    mail_transport: MailTransport,
) -> Router {
    let user_api = Router::new()
        .route("/auth", post(users::login))
        .route("/me", get(users::me))
        .route("/logout", post(users::logout))
        .route("/register", post(users::register))
        .route("/activate/:token", post(users::activate_account));

    let expense_api = Router::new()
        .route("/:user_id", get(expenses::get_expenses))
        .route("/", post(expenses::create_expense))
        .route("/predefined", get(expenses::get_predefined_expenses))
        .route("/predefined", post(expenses::create_predefined_expense));

    let transaction_api = Router::new()
        .route("/", post(transactions::create_transaction))
        .route("/:transaction_id", delete(transactions::delete_transaction));

    let currency_api = Router::new().route("/", get(currencies::get_currencies));

    let recurrence_api = Router::new().route("/", get(recurrences::get_recurrences));

    let api = Router::new()
        .route("/", get(|| async {}))
        .nest("/user", user_api)
        .nest("/expense", expense_api)
        .nest("/transaction", transaction_api)
        .nest("/currency", currency_api)
        .nest("/recurrence", recurrence_api);

    let state = AppState {
        config,
        conn,
        secret_key,
        mail_transport: Arc::new(mail_transport),
    };

    Router::new()
        .nest("/api", api)
        .route_layer(middleware::from_fn(track_metrics))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state)
}

async fn track_metrics<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::increment_counter!("http_requests_total", &labels);
    metrics::histogram!("http_requests_duration_seconds", latency, &labels);

    response
}

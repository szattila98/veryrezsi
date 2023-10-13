use std::{future::ready, net::SocketAddr};

use axum::{routing::get, Router, Server};
use axum_extra::extract::cookie::Key;
use metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use tokio::signal;
use tracing::info;
use veryrezsi_core::config::AppConfig;

mod auth;
pub mod routes;

#[tokio::main]
pub async fn start() {
    let config = AppConfig::init();
    let (_main_server, _metrics_server) = tokio::join!(start_main_server(&config), start_metrics_server(&config));
}

pub async fn start_main_server(config: &AppConfig) {
    let (server_address, router) = init(config).await;
    info!("Server is listening on {}...", server_address);
    let _ = Server::bind(&server_address)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await;
    info!("Shutting down...");
}

pub async fn start_metrics_server(config: &AppConfig) {
    let (metrics_address, metrics_router) = metrics_init(config);
    info!("Metrics is listening on {}...", metrics_address);
    axum::Server::bind(&metrics_address)
        .serve(metrics_router.into_make_service())
        .await
        .unwrap()
}

/// Initializes every part of the application.
async fn init(config: &AppConfig) -> (SocketAddr, Router) {
    print_logo();

    info!("Initializing logging...");
    tracing_subscriber::fmt()
        .with_max_level(&config.log_level)
        .init();
    info!("Successfully initialized logging");

    info!("Establishing database connection...");
    let conn = veryrezsi_core::database::init(&config).await;
    info!("Successfully established database connection");

    info!(
        "Initializing mail transport with with relay: {}",
        &config.mail_config.smtp_address
    );
    let mail_transport = veryrezsi_core::email::get_mail_transport(&config.mail_config);
    info!("Successfully initialized mail transport");

    info!("Creating api routes and loading extensions...");
    let router = routes::init(
        config.clone(),
        conn,
        Key::from(config.cookie_key.as_bytes()),
        mail_transport,
    );
    info!("Successfully created api routes with extensions");

    (config.server_address, router)
}

fn metrics_init(config: &AppConfig) -> (SocketAddr, Router) {
    let recorder_handle = setup_metrics_recorder();
    let router = Router::new().route("/metrics", get(move || ready(recorder_handle.render())));

    (config.metrics_config.metrics_address, router)
}

fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}

fn print_logo() {
    println!(
        r#"
__     __                                _______                                 __ 
|  \   |  \                              |       \                               |  \
| $$   | $$  ______    ______   __    __ | $$$$$$$\  ______   ________   _______  \$$
| $$   | $$ /      \  /      \ |  \  |  \| $$__| $$ /      \ |        \ /       \|  \
 \$$\ /  $$|  $$$$$$\|  $$$$$$\| $$  | $$| $$    $$|  $$$$$$\ \$$$$$$$$|  $$$$$$$| $$
  \$$\  $$ | $$    $$| $$   \$$| $$  | $$| $$$$$$$\| $$    $$  /    $$  \$$    \ | $$
   \$$ $$  | $$$$$$$$| $$      | $$__/ $$| $$  | $$| $$$$$$$$ /  $$$$_  _\$$$$$$\| $$
    \$$$    \$$     \| $$       \$$    $$| $$  | $$ \$$     \|  $$    \|       $$| $$
     \$      \$$$$$$$ \$$       _\$$$$$$$ \$$   \$$  \$$$$$$$ \$$$$$$$$ \$$$$$$$  \$$
                               |  \__| $$                                            
                                \$$    $$                                            
                                 \$$$$$$                                             
====================================================================================="#
    );
}

/// Makes graceful shutdown possible.
async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

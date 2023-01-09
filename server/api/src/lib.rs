use std::net::SocketAddr;

use axum::{Router, Server};
use axum_extra::extract::cookie::Key;
use tokio::signal;
use tracing::info;
use veryrezsi_core::config::AppConfig;

mod auth;
mod dtos;
pub mod routes;

#[tokio::main]
pub async fn start() {
    let (server_address, router) = init().await;
    info!("Server is listening on {}...", server_address);
    let _ = Server::bind(&server_address)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await;
    info!("Shutting down...");
}

/// Initializes every part of the application.
async fn init() -> (SocketAddr, Router) {
    print_logo();
    let config = AppConfig::init();

    info!("Initializing logging...");
    tracing_subscriber::fmt()
        .with_max_level(&config.log_level)
        .init();
    info!("Successfully initialized logging");

    info!("Establishing database connection...");
    let conn = veryrezsi_core::database::init(&config).await;
    info!("Successfully established database connection");

    info!("Initializing mail transport...");
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

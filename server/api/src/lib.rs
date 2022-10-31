#![warn(missing_docs)]

//! Veryrezsi library, which makes serving the server-side logic possible.

use axum::Server;
use axum_extra::extract::cookie::Key;
use tokio::signal;
use tracing::info;
use veryrezsi_core::config::AppConfig;

mod auth;
mod routes;

/// Initializes every part of the application and starts the server.
#[tokio::main]
pub async fn start() {
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

    info!("Server is listening on {}...", config.server_address);
    let _ = Server::bind(&config.server_address)
        .serve(router.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await;
    info!("Shutting down...");
}

/// Prints the logo of the application.
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

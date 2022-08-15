#![warn(missing_docs)]

//! Veryrezsi library, which makes serving the server-side logic possible.

use axum::Router;
use axum_extra::extract::cookie::Key;
use config::AppConfig;
use std::net::SocketAddr;
use tracing::{debug, info};

mod auth;
mod config;
mod database;
mod email;
mod logic;
/// Exports the router for the binary.
pub mod routes;

/// Initializes every part of the application.
/// Returns the address of the server and the configured router.
pub async fn init() -> (SocketAddr, Router) {
    print_logo();
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Loading config from env...");
    let config = AppConfig::init();
    info!("Successfully loaded config");
    debug!("{config:#?}");

    info!("Establishing database connection...");
    let conn = database::init(&config).await;
    info!("Successfully established database connection");

    info!("Initializing mailer...");
    let mailer = email::Mailer::init(&config.mail_config);
    info!("Successfully initialized mailer");

    info!("Creating api routes and loading extensions...");
    let router = routes::init(
        config.clone(),
        conn,
        Key::from(config.cookie_key.as_bytes()),
        mailer,
    );
    info!("Successfully created api routes with extensions");

    info!("Server is listening on {}...", config.server_address);
    (config.server_address, router)
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

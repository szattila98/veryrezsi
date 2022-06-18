use axum::Router;
use axum_extra::extract::cookie::Key;
use config::Config;
use std::net::SocketAddr;
use tracing::{debug, info};

mod auth;
mod config;
mod database;
mod logic;
pub mod routes;

pub async fn init() -> (SocketAddr, Router) {
    print_logo();
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    info!("Loading config from env...");
    let config = Config::init();
    info!("Successfully loaded config");
    debug!("{config:#?}");

    info!("Establishing database connection...");
    let conn = database::init(&config).await;
    info!("Successfully established database connection");

    info!("Creating api routes and loading extensions...");
    let router = routes::init(conn, Key::from(config.cookie_key.as_bytes()));
    info!("Successfully created api routes with extensions");

    info!("Server is listening on {}...", config.server_address);
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
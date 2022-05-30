use std::{env, net::SocketAddr};

#[derive(Debug)]
pub struct Config {
    pub server_address: SocketAddr,
    pub database_url: String,
    pub cookie_key: String,
    pub log_level: String,
}

impl Config {
    pub fn init() -> Self {
        let server_host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let cookie_key = env::var("COOKIE_KEY").expect("COOKIE_KEY is not set in .env file");
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        let server_address: SocketAddr = format!("{}:{}", server_host, server_port)
            .parse()
            .expect("Could not parse valid address from server host and port");
        Config {
            server_address,
            database_url,
            cookie_key,
            log_level,
        }
    }
}

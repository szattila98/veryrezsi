use std::{env, net::SocketAddr};

/// Contains basic configuration entries.
#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server_address: SocketAddr,
    pub database_url: String,
    pub cookie_key: String,
    pub log_level: String,
    pub mail_config: MailConfig,
}

/// Contains configuration tied to email sending.
#[derive(Debug, Clone)]
pub struct MailConfig {
    pub smtp_address: String,
    pub smtp_port: u16,
    pub smtp_username: String,
    pub smtp_password: String,
}

impl AppConfig {
    /// Initializes the configuration from environment variables.
    pub fn init() -> Self {
        let server_host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let server_port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let cookie_key = env::var("COOKIE_KEY").expect("COOKIE_KEY is not set in .env file");
        let log_level = env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        let server_address = format!("{}:{}", server_host, server_port)
            .parse()
            .expect("Could not parse valid address from server host and port");
        let smtp_address = env::var("SMTP_ADDRESS").expect("SMTP_ADDRESS is not set in .env file");
        let smtp_port = env::var("SMTP_PORT")
            .expect("SMTP_ADDRESS is not set in .env file")
            .parse()
            .expect("Could not parse valid port from SMTP_PORT");
        let smtp_username =
            env::var("SMTP_USERNAME").expect("SMTP_USERNAME is not set in .env file");
        let smtp_password =
            env::var("SMTP_PASSWORD").expect("SMTP_PASSWORD is not set in .env file");
        AppConfig {
            server_address,
            database_url,
            cookie_key,
            log_level,
            mail_config: MailConfig {
                smtp_address,
                smtp_port,
                smtp_username,
                smtp_password,
            },
        }
    }
}

use confique::Config;
use serde::Deserialize;
use std::net::SocketAddr;
use tracing::Level;

/// Contains basic configuration entries.
#[derive(Debug, Clone, Config)]
pub struct AppConfig {
    #[config(env = "SERVER_ADDRESS")]
    pub server_address: SocketAddr,
    #[config(env = "DATABASE_URL")]
    pub database_url: String,
    #[config(env = "COOKIE_KEY")]
    pub cookie_key: String,
    #[config(env = "LOG_LEVEL")]
    pub log_level: LogLevel,
    #[config(nested)]
    pub mail_config: MailConfig,
}

/// Contains configuration tied to email sending.
#[derive(Debug, Clone, Config)]
pub struct MailConfig {
    #[config(env = "SMTP_ADDRESS")]
    pub smtp_address: String,
    #[config(env = "SMTP_PORT")]
    pub smtp_port: u16,
    #[config(env = "SMTP_USERNAME")]
    pub smtp_username: String,
    #[config(env = "SMTP_PASSWORD")]
    pub smtp_password: String,
}

impl AppConfig {
    /// Initializes the config, prioritizing reading from environment variables and then app-config.toml.
    pub fn init() -> Self {
        AppConfig::builder()
            .env()
            .file("resources/app-config.toml")
            .load()
            .expect("config loading failed")
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum LogLevel {
    #[serde(alias = "trace")]
    TRACE,
    #[serde(alias = "debug")]
    DEBUG,
    #[serde(alias = "info")]
    INFO,
    #[serde(alias = "warn")]
    WARN,
    #[serde(alias = "error")]
    ERROR,
}

impl From<LogLevel> for Level {
    fn from(l: LogLevel) -> Self {
        match l {
            LogLevel::TRACE => Level::TRACE,
            LogLevel::DEBUG => Level::DEBUG,
            LogLevel::INFO => Level::INFO,
            LogLevel::WARN => Level::WARN,
            LogLevel::ERROR => Level::ERROR,
        }
    }
}

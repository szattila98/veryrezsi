use confique::Config;
use serde::Deserialize;
use std::net::SocketAddr;
use tracing::metadata::LevelFilter;

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
    #[must_use]
    pub fn init() -> Self {
        AppConfig::builder()
            .env()
            .file("resources/app-config.toml")
            .load()
            .expect("config loading failed")
    }
}

/// Intermediate config enum for log levels as actual struct cannot be deserialized directly.
#[derive(Debug, Clone, Deserialize)]
pub enum LogLevel {
    #[serde(alias = "trace", alias = "TRACE")]
    Trace,
    #[serde(alias = "debug", alias = "DEBUG")]
    Debug,
    #[serde(alias = "info", alias = "INFO")]
    Info,
    #[serde(alias = "warn", alias = "WARN")]
    Warn,
    #[serde(alias = "error", alias = "ERROR")]
    Error,
}

impl From<&LogLevel> for LevelFilter {
    fn from(l: &LogLevel) -> Self {
        match l {
            LogLevel::Trace => LevelFilter::TRACE,
            LogLevel::Debug => LevelFilter::DEBUG,
            LogLevel::Info => LevelFilter::INFO,
            LogLevel::Warn => LevelFilter::WARN,
            LogLevel::Error => LevelFilter::ERROR,
        }
    }
}

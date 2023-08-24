use confique::Config;
use serde::Deserialize;
use std::net::SocketAddr;
use tracing::metadata::LevelFilter;

#[derive(Debug, Clone, Config)]
pub struct AppConfig {
    #[config(env = "SERVER_ADDRESS")]
    pub server_address: SocketAddr,
    #[config(env = "METRICS_ADDRESS")]
    pub metrics_address: SocketAddr,
    #[config(env = "DATABASE_URL")]
    pub database_url: String,
    #[config(env = "COOKIE_KEY")]
    pub cookie_key: String,
    #[config(env = "LOG_LEVEL")]
    pub log_level: LogLevel,
    #[config(nested)]
    pub mail_config: MailConfig,
}

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
    #[must_use]
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

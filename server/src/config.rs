use confique::Config;
use std::net::SocketAddr;

/// Contains basic configuration entries.
#[derive(Debug, Clone, Config)]
pub struct AppConfig {
    #[config(env = "SERVER_ADDRESS")]
    pub server_address: SocketAddr,
    #[config(env = "DATABASE_URL")]
    pub database_url: String,
    #[config(env = "COOKIE_KEY")]
    pub cookie_key: String,
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
    /// Initializes the configuration from environment variables.
    pub fn init() -> Self {
        AppConfig::builder()
            .env()
            .load()
            .expect("config loading failed")
    }
}

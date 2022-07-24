use crate::config::MailConfig;
use handlebars::Handlebars;
use lettre::{
    message::{header::ContentType, SinglePart},
    transport::smtp::{authentication::Credentials, PoolConfig},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use std::collections::HashMap;
use thiserror::Error;
use tracing::log::error;

/// Error types that can occur on email sending.
#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Could not build email, reason: '{0}'")]
    CannotBuild(#[from] lettre::error::Error),
    #[error("Could not send email, reason: '{0}'")]
    CannotSend(#[from] lettre::transport::smtp::Error),
}

/// The structure that handles email sending.
#[derive(Clone)]
pub struct Mailer {
    conn: AsyncSmtpTransport<Tokio1Executor>,
    from: String,
}

impl Mailer {
    /// Creates a new mailer.
    pub fn init(config: &MailConfig) -> Self {
        let credentials =
            Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());
        let conn: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&config.smtp_address)
                .port(config.smtp_port)
                .credentials(credentials)
                .pool_config(PoolConfig::default())
                .build();
        Mailer {
            conn,
            from: "Veryrezsi <noreply@veryrezsi.com>".to_string(),
        }
    }

    /// Sends an email.
    pub async fn send(&self, to: String, subject: &str, body: String) -> Result<(), EmailError> {
        let email = Message::builder()
            .to(to.parse().unwrap())
            .from(self.from.parse().unwrap())
            .subject(subject)
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(body),
            )?;
        self.conn.send(email).await?;
        Ok(())
    }
}

/// Renders a template using handlebars.
/// The `data` will be substituted into the `template` string.
pub fn render_template(template: &str, data: HashMap<&str, &String>) -> String {
    Handlebars::new()
        .render_template(template, &data)
        .expect("Failed to render template")
}

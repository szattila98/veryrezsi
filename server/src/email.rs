use std::collections::HashMap;

use crate::config::MailConfig;
use handlebars::Handlebars;
use lettre::{
    message::{header::ContentType, SinglePart},
    transport::smtp::{authentication::Credentials, PoolConfig},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use thiserror::Error;
use tracing::log::error;

#[derive(Debug, Error)]
pub enum EmailError {
    #[error("Could not build email, reason: '{0}'")]
    CannotBuild(#[from] lettre::error::Error),
    #[error("Could not send email, reason: '{0}'")]
    CannotSend(#[from] lettre::transport::smtp::Error),
}

#[derive(Clone)]
pub struct Mailer {
    conn: AsyncSmtpTransport<Tokio1Executor>,
    from: String,
}

impl Mailer {
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

pub fn render_template(template: &str, data: HashMap<&str, &String>) -> String {
    Handlebars::new()
        .render_template(template, &data)
        .expect("Failed to render template")
}

use crate::config::MailConfig;
use handlebars::Handlebars;
use lettre::{
    message::{header::ContentType, SinglePart},
    transport::smtp::{authentication::Credentials, PoolConfig},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde::Serialize;
use std::{collections::HashMap, hash::Hash};
use thiserror::Error;
use tracing::log::error;

pub const ACTIVATION_EMAIL_TEMPLATE: &str =
    include_str!("./../resources/email/activation_email.html");

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
/// Accepts data in the form of a hashmap of any type of string key/value pairs.
/// Rendering is strict, so it fails if the template is supplied with the wrong data. Additional data is ignored.
pub fn render_template<K, V>(template: &str, data: HashMap<K, V>) -> String
where
    K: AsRef<str> + Serialize + Hash + Eq,
    V: AsRef<str> + Serialize + Hash + Eq,
{
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars
        .render_template(template, &data)
        .expect("Failed to render template")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_template_substitutes_correctly() {
        let template = "{{ a }}-{{ b }}-{{ c }}";
        let data = "abc"
            .chars()
            .map(|c| (c.to_string(), c.to_string()))
            .collect::<HashMap<_, _>>();
        let rendered = render_template(template, data);
        assert_eq!(rendered, "a-b-c");
    }

    #[test]
    #[should_panic]
    fn render_template_panics_on_empty_data() {
        let template = "{{ a }}-{{ b }}-{{ c }}";
        let data: HashMap<String, String> = HashMap::new();
        render_template(template, data);
    }

    #[test]
    #[should_panic]
    fn render_template_panics_on_wrong_data() {
        let template = "{{ a }}-{{ b }}-{{ c }}";
        let data = "def"
            .chars()
            .map(|c| (c.to_string(), c.to_string()))
            .collect::<HashMap<_, _>>();
        render_template(template, data);
    }

    #[test]
    fn render_template_additional_data_is_ignored() {
        let template = "{{ a }}-{{ b }}-{{ c }}";
        let mut data = "abc"
            .chars()
            .map(|c| (c.to_string(), c.to_string()))
            .collect::<HashMap<_, _>>();
        data.insert("d".to_string(), "d".to_string());
        let rendered = render_template(template, data);
        assert_eq!(rendered, "a-b-c");
    }
}

use crate::config::MailConfig;
use handlebars::Handlebars;
use lettre::{
    message::{header::ContentType, SinglePart},
    transport::smtp::{authentication::Credentials, PoolConfig},
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
};
use serde::Serialize;
use std::sync::Arc;
use std::{collections::HashMap, hash::Hash};

// This is a workaround because cargo doc-test does not like our macro because the path cannot be relative to the current file, see our macro's doc for more info.
#[cfg(not(doctest))]
pub const ACTIVATION_EMAIL_TEMPLATE: &str =
    proc_macros::include_email_template!("./resources/email/activation_email.html");
#[cfg(doctest)]
pub const ACTIVATION_EMAIL_TEMPLATE: &str =
    include_str!("../../resources/email/activation_email.html");

pub type MailTransport = AsyncSmtpTransport<Tokio1Executor>;

/// Creates a mail transport object. Used when the server initializes.
#[must_use]
pub fn get_mail_transport(config: &MailConfig) -> MailTransport {
    let credentials = Credentials::new(config.smtp_username.clone(), config.smtp_password.clone());
    MailTransport::builder_dangerous(&config.smtp_address)
        .port(config.smtp_port)
        .credentials(credentials)
        .pool_config(PoolConfig::default())
        .build()
}

/// Renders a template using handlebars.
/// The `data` will be substituted into the `template` string.
/// Accepts data in the form of a hashmap of any type of string key/value pairs.
/// Rendering is strict, so it fails if the template is supplied with the wrong data. Additional data is ignored.
#[must_use]
pub fn render_template<K, V>(template: &str, data: &HashMap<K, V>) -> String
where
    K: AsRef<str> + Serialize + Hash + Eq,
    V: AsRef<str> + Serialize + Hash + Eq,
{
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars
        .render_template(template, data)
        .expect("Failed to render template")
}

/// Sends an email.
/// Panics if an error is encountered, but it should be running as a separate async task,
/// which if panics won't kill the function it is used in or the whole server.
pub async fn send_mail<T>(
    transport: Arc<T>,
    to: String,
    subject: &str,
    body: String,
) -> Result<(), String>
where
    T: AsyncTransport + Send + Sync,
    <T as AsyncTransport>::Error: std::fmt::Debug,
{
    let to = to.parse().expect("to should be a valid email address");
    let from = "Veryrezsi <noreply@veryrezsi.com>"
        .parse()
        .expect("from should be a valid email address");
    let email = Message::builder()
        .to(to)
        .from(from)
        .subject(subject)
        .singlepart(
            SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(body),
        )
        .expect("error while building a message");
    match transport.send(email).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("{e:?}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert2::check;
    use lettre::transport::stub::AsyncStubTransport;

    #[test]
    fn render_template_substitutes_correctly() {
        let template = "{{ a }}-{{ b }}-{{ c }}";
        let data = "abc"
            .chars()
            .map(|c| (c.to_string(), c.to_string()))
            .collect::<HashMap<_, _>>();
        let rendered = render_template(template, &data);
        check!(rendered == "a-b-c");
    }

    #[test]
    #[should_panic]
    fn render_template_panics_on_empty_data() {
        let template = "{{ a }}-{{ b }}-{{ c }}";
        let data: HashMap<String, String> = HashMap::new();
        let _ = render_template(template, &data);
    }

    #[test]
    #[should_panic]
    fn render_template_panics_on_wrong_data() {
        let template = "{{ a }}-{{ b }}-{{ c }}";
        let data = "def"
            .chars()
            .map(|c| (c.to_string(), c.to_string()))
            .collect::<HashMap<_, _>>();
        let _ = render_template(template, &data);
    }

    #[test]
    fn render_template_additional_data_is_ignored() {
        let template = "{{ a }}-{{ b }}-{{ c }}";
        let mut data = "abc"
            .chars()
            .map(|c| (c.to_string(), c.to_string()))
            .collect::<HashMap<_, _>>();
        data.insert("d".to_string(), "d".to_string());
        let rendered = render_template(template, &data);
        check!(rendered == "a-b-c");
    }

    #[tokio::test]
    async fn send_mail_correctly_sends_an_email() {
        let mail_transport = Arc::new(AsyncStubTransport::new_ok());
        let to = "aru@pizvo.jo".to_string();
        let subject = "UAmgResRKr";
        let body = "JghUysMeIASWJCCQMThIxqSanWRbIN".to_string();
        let result = send_mail(mail_transport.clone(), to, subject, body).await;
        check!(result == Ok(()));
        check!(mail_transport.messages().await.len() == 1);
    }

    #[tokio::test]
    #[should_panic]
    async fn send_mail_panics_on_empty_to() {
        let mail_transport = Arc::new(AsyncStubTransport::new_ok());
        let to = "".to_string();
        let subject = "BNqEeDcTSC";
        let body = "GTwZHvlJKsVrVmXzXHKOrePFxqZyHw".to_string();
        let _ = send_mail(mail_transport, to, subject, body).await;
    }

    #[tokio::test]
    async fn send_mail_correctly_sends_with_empty_subject_and_body() {
        let mail_transport = Arc::new(AsyncStubTransport::new_ok());
        let to = "colet@ku.aw".to_string();
        let subject = "";
        let body = "".to_string();
        let result = send_mail(mail_transport.clone(), to, subject, body).await;
        check!(result == Ok(()));
        check!(mail_transport.messages().await.len() == 1);
    }

    #[tokio::test]
    async fn send_mail_panics_when_transport_fails() {
        let mail_transport = Arc::new(AsyncStubTransport::new_error());
        let to = "bojwe@ahamef.bm".to_string();
        let subject = "ElcSmycmNq";
        let body = "CQkjGVIsVDuYwokwjOKRLdxlzCyXwM".to_string();
        let result = send_mail(mail_transport, to, subject, body).await;
        check!(result == Err("Error".to_string()));
    }
}

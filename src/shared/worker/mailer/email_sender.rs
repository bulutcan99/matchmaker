//! This module defines an [`EmailSender`] responsible for sending emails using
//! either the SMTP protocol. It includes an asynchronous method `mail` for
//! sending emails with options like sender, recipient, subject, and content.

use crate::shared::config::config::Config;
use crate::shared::error::Result;
use crate::shared::worker::mailer::service::{Email, DEFAULT_FROM_SENDER};
use anyhow::anyhow;
use lettre::message::MultiPart;
use lettre::{
    transport::smtp::authentication::Credentials, AsyncTransport, Message, Tokio1Executor,
    Transport,
};

/// An enumeration representing the possible transport methods for sending
/// emails.
#[derive(Clone)]
pub enum EmailTransport {
    /// SMTP (Simple Mail Transfer Protocol) transport.
    Smtp(lettre::AsyncSmtpTransport<lettre::Tokio1Executor>),
    /// Test/stub transport for testing purposes.
    Test(lettre::transport::stub::StubTransport),
}

/// A structure representing the email sender, encapsulating the chosen
/// transport method.
#[derive(Clone)]
pub struct EmailSender {
    pub transport: EmailTransport,
}

#[cfg(feature = "testing")]
#[derive(Default, Debug)]
pub struct Deliveries {
    pub count: usize,
    pub messages: Vec<String>,
}

impl EmailSender {
    pub fn smtp() -> Result<Self> {
        let config = Config::get();
        let mailer = config.clone().mailer.unwrap().smtp.unwrap();

        let mut email_builder =
            lettre::AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(&mailer.host)
                .port(mailer.port);

        if let Some(auth) = mailer.auth.as_ref() {
            email_builder = email_builder
                .credentials(Credentials::new(auth.user.clone(), auth.password.clone()));
        }

        Ok(Self {
            transport: EmailTransport::Smtp(email_builder.build()),
        })
    }

    #[cfg(feature = "testing")]
    #[must_use]
    pub fn deliveries(&self) -> Deliveries {
        if let EmailTransport::Test(stub) = &self.transport {
            return Deliveries {
                count: stub.messages().len(),
                messages: stub
                    .messages()
                    .iter()
                    .map(|(_, content)| content.to_string())
                    .collect(),
            };
        }

        Deliveries::default()
    }

    /// Sends an email using the configured transport method.
    ///
    /// # Errors
    ///
    /// When email did't send successfully or has an error to build the message
    pub async fn mail(&self, email: &Email) -> Result<()> {
        let content = MultiPart::alternative_plain_html(email.text.clone(), email.html.clone());
        let mut builder = Message::builder()
            .from(
                email
                    .from
                    .clone()
                    .unwrap_or_else(|| DEFAULT_FROM_SENDER.to_string())
                    .parse()?,
            )
            .to(email.to.parse()?);

        if let Some(bcc) = &email.bcc {
            builder = builder.bcc(bcc.parse()?);
        }

        if let Some(cc) = &email.cc {
            builder = builder.cc(cc.parse()?);
        }

        if let Some(reply_to) = &email.reply_to {
            builder = builder.reply_to(reply_to.parse()?);
        }

        let msg = builder
            .subject(email.subject.clone())
            .multipart(content)
            .map_err(|error| {
                tracing::error!(err.msg = %error, err.detail = ?error, "email_building_error");
                anyhow!("error building email message")
            })?;

        match &self.transport {
            EmailTransport::Smtp(xp) => {
                xp.send(msg).await?;
            }
            EmailTransport::Test(xp) => {
                xp.send(&msg)
                    .map_err(|_| anyhow!("sending email error".to_string().into()))?;
            }
        };
        Ok(())
    }
}

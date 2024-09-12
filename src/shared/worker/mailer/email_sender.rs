//! This module defines an [`EmailSender`] responsible for sending emails using
//! either the SMTP protocol. It includes an asynchronous method `mail` for
//! sending emails with options like sender, recipient, subject, and content.

use crate::shared::config::config::Config;
use crate::shared::error::Result;
use lettre::{transport::smtp::authentication::Credentials, Tokio1Executor};

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
}

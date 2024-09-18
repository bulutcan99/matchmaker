use lettre::error::Error as LettreError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MailerError {
    #[error("failed to render email template")]
    TemplateRenderError,

    #[error("failed to send email to {0}")]
    SendEmailError(String),

    #[error("mailer not configured in context")]
    NoMailerConfigured,

    #[error("unexpected error: {0}")]
    UnexpectedError(String),
}

impl From<MailerError> for sidekiq::Error {
    fn from(error: MailerError) -> Self {
        sidekiq::Error::Message(error.to_string())
    }
}

#[derive(Debug, Error)]
pub enum EmailSenderError {
    #[error("SMTP transport initialization failed: {0}")]
    SmtpTransportInitError(String),

    #[error("Failed to build email message: {0}")]
    EmailBuildError(String),

    #[error("Failed to send email via SMTP: {0}")]
    EmailSendError(String),

    #[error("Failed to send email via Test transport: {0}")]
    TestTransportSendError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Address parse error: {0}")]
    AddressParseError(#[from] lettre::address::AddressError),

    #[error("SMTP error: {0}")]
    LettreError(#[from] LettreError),
}

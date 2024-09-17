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

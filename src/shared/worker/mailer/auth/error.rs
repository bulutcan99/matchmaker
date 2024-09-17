#[derive(Debug, thiserror::Error)]
pub enum AuthMailerError {
    #[error("failed to send welcome email to {0}")]
    SendWelcomeError(String),

    #[error("failed to send forgot password email to {0}")]
    SendForgotPasswordError(String),

    #[error("template rendering failed")]
    TemplateRenderError,

    #[error("unexpected error occurred: {0}")]
    UnexpectedError(String),
}

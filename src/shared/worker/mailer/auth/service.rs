use crate::core::domain::entity::user::User;
use crate::shared::config::config::Config;
use crate::shared::worker::mailer::auth::error::AuthMailerError;
use crate::shared::worker::mailer::service::{Args, Mailer};
use crate::shared::worker::service::TaskContext;
use include_dir::{include_dir, Dir};
use serde_json::json;

static WELCOME: Dir<'_> = include_dir!("src/shared/worker/mailer/auth/welcome");
static FORGOT: Dir<'_> = include_dir!("src/shared/worker/mailer/auth/forgot");

#[allow(clippy::module_name_repetitions)]
pub struct AuthMailer {}
impl Mailer for AuthMailer {}
impl AuthMailer {
    /// Sending WELCOME email the the given user
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn send_welcome(ctx: &TaskContext, user: &User) -> Result<(), AuthMailerError> {
        let config = Config::get();
        let full_url = format!("{}:{}", config.server.host, config.server.port);
        Self::mail_template(
            ctx,
            &WELCOME,
            Args {
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "verifyToken": user.email_verification_token,
                  "domain": full_url,
                }),
                ..Default::default()
            },
        )
        .await
        .map_err(|_| AuthMailerError::SendWelcomeError(user.email.to_string()))?;
        Ok(())
    }

    /// Sending FORGOT password email
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn forgot_password(ctx: &TaskContext, user: &User) -> Result<(), AuthMailerError> {
        let config = Config::get();
        let full_url = format!("{}:{}", config.server.host, config.server.port);
        Self::mail_template(
            ctx,
            &FORGOT,
            Args {
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "resetToken": user.reset_token,
                  "domain": full_url,
                }),
                ..Default::default()
            },
        )
        .await
        .map_err(|_| AuthMailerError::SendForgotPasswordError(user.email.to_string()))?;

        Ok(())
    }
}

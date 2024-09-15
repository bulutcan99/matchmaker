use crate::core::domain::entity::user::User;
use crate::shared::error::Result;
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
    pub async fn send_welcome(ctx: &TaskContext, user: &User) -> Result<()> {
        Self::mail_template(
            ctx,
            &WELCOME,
            Args {
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "verifyToken": user.email_verification_token,
                  "domain": ctx.config.server.full_url()
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }

    /// Sending FORGOT password email
    ///
    /// # Errors
    ///
    /// When email sending is failed
    pub async fn forgot_password(ctx: &TaskContext, user: &User) -> Result<()> {
        Self::mail_template(
            ctx,
            &FORGOT,
            Args {
                to: user.email.to_string(),
                locals: json!({
                  "name": user.name,
                  "resetToken": user.reset_token,
                  "domain": ctx.config.server.full_url()
                }),
                ..Default::default()
            },
        )
        .await?;

        Ok(())
    }
}

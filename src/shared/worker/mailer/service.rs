use crate::shared::worker::mailer::error::MailerError;
use crate::shared::worker::mailer::template::Template;
use crate::shared::worker::service::{AppWorker, TaskContext};
use async_trait::async_trait;
use include_dir::Dir;
use serde::de::Error;
use serde_derive::{Deserialize, Serialize};
use sidekiq::Worker;

pub const DEFAULT_FROM_SENDER: &str = "System <system@example.com>";

/// The arguments struct for specifying email details such as sender, recipient,
/// reply-to, and locals.
#[derive(Debug, Clone, Default)]
pub struct Args {
    pub from: Option<String>,
    pub to: String,
    pub reply_to: Option<String>,
    pub locals: serde_json::Value,
    pub bcc: Option<String>,
    pub cc: Option<String>,
}

/// The structure representing an email details.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Email {
    /// Mailbox to `From` header
    pub from: Option<String>,
    /// Mailbox to `To` header
    pub to: String,
    /// Mailbox to `ReplyTo` header
    pub reply_to: Option<String>,
    /// Subject header to message
    pub subject: String,
    /// Plain text message
    pub text: String,
    /// HTML template
    pub html: String,
    /// BCC header to message
    pub bcc: Option<String>,
    /// CC header to message
    pub cc: Option<String>,
}

/// The options struct for configuring the email sender.
#[derive(Default)]
#[allow(clippy::module_name_repetitions)]
pub struct MailerOpts {
    pub from: String,
    pub reply_to: Option<String>,
}

#[async_trait]
pub trait Mailer {
    #[must_use]
    fn opts() -> MailerOpts {
        MailerOpts {
            from: DEFAULT_FROM_SENDER.to_string(),
            ..Default::default()
        }
    }
    /// Sends an email using the provided [`TaskContext`] and email details.
    async fn mail(ctx: &TaskContext, email: &Email) -> Result<(), MailerError> {
        let opts = Self::opts();
        let mut email = email.clone();

        email.from = Some(email.from.unwrap_or_else(|| opts.from.clone()));
        email.reply_to = email.reply_to.or_else(|| opts.reply_to.clone());

        MailerWorker::perform_later(ctx, email.clone())
            .await
            .map_err(|e| MailerError::SendEmailError(e.to_string()))?;

        Ok(())
    }

    /// Renders and sends an email using the provided [`TaskContext`], template
    /// directory, and arguments.
    async fn mail_template(
        ctx: &TaskContext,
        dir: &Dir<'_>,
        args: Args,
    ) -> Result<(), MailerError> {
        let content = Template::new(dir)
            .render(&args.locals)
            .map_err(|_| MailerError::TemplateRenderError)?;

        Self::mail(
            ctx,
            &Email {
                from: args.from.clone(),
                to: args.to.clone(),
                reply_to: args.reply_to.clone(),
                subject: content.subject,
                text: content.text,
                html: content.html,
                bcc: args.bcc.clone(),
                cc: args.cc.clone(),
            },
        )
        .await?;

        Ok(())
    }
}

#[allow(clippy::module_name_repetitions)]
pub struct MailerWorker {
    pub ctx: TaskContext,
}

impl AppWorker<Email> for MailerWorker {
    fn build(task_context: &TaskContext) -> Self {
        Self {
            ctx: task_context.clone(),
        }
    }
}

#[async_trait]
impl Worker<Email> for MailerWorker {
    async fn perform(&self, email: Email) -> Result<(), sidekiq::Error> {
        if let Some(mailer) = &self.ctx.mailer {
            mailer
                .mail(&email)
                .await
                .map_err(|e| MailerError::SendEmailError(e.to_string()))?;
            Ok(())
        } else {
            Err(MailerError::NoMailerConfigured.into())
        }
    }
}

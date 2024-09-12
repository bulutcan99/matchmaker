use crate::shared::error::Result;
use crate::shared::worker::mailer::template::Template;
use crate::shared::worker::service::{AppWorker, TaskContext};
use async_trait::async_trait;
use include_dir::Dir;
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

/// The `Mailer` trait defines methods for sending emails and processing email
/// templates.
#[async_trait]
pub trait Mailer {
    /// Returns default options for the mailer.
    #[must_use]
    fn opts() -> MailerOpts {
        MailerOpts {
            from: DEFAULT_FROM_SENDER.to_string(),
            ..Default::default()
        }
    }

    /// Sends an email using the provided [`TaskContext`] and email details.
    async fn mail(ctx: &TaskContext, email: &Email) -> Result<()> {
        let opts = Self::opts();
        let mut email = email.clone();

        email.from = Some(email.from.unwrap_or_else(|| opts.from.clone()));
        email.reply_to = email.reply_to.or_else(|| opts.reply_to.clone());

        MailerWorker::perform_later(ctx, email.clone())
            .await
            .map_err(Box::from)?;
        Ok(())
    }

    /// Renders and sends an email using the provided [`TaskContext`], template
    /// directory, and arguments.
    async fn mail_template(ctx: &TaskContext, dir: &Dir<'_>, args: Args) -> Result<()> {
        let content = Template::new(dir).render(&args.locals)?;
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
        .await
    }
}

/// The [`MailerWorker`] struct represents a worker responsible for asynchronous
/// email processing.
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

/// Implementation of the [`Worker`] trait for the [`MailerWorker`].
#[async_trait]
impl Worker<Email> for MailerWorker {
    /// Returns options for the mailer worker, specifying the queue to process.
    fn opts() -> sidekiq::WorkerOpts<Email, Self> {
        sidekiq::WorkerOpts::new().queue("mailer")
    }

    /// Performs the email sending operation using the provided [`TaskContext`]
    /// and email details.
    async fn perform(&self, email: Email) -> sidekiq::Result<()> {
        if let Some(mailer) = &self.ctx.mailer {
            Ok(mailer.mail(&email).await.map_err(Box::from)?)
        } else {
            Err(sidekiq::Error::Message(
                "attempting to send email but no email sender configured".to_string(),
            ))
        }
    }
}

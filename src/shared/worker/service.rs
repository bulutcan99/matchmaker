use crate::shared::config::config::{Config, WorkerMode};
use crate::shared::worker::mailer::email_sender::EmailSender;
use anyhow::anyhow;
use async_trait::async_trait;
pub use bb8::Pool;
use sidekiq::{Error, RedisConnectionManager, Worker};
use tracing::error;

#[derive(Clone)]
pub struct TaskContext {
    pub queue: Option<Pool<RedisConnectionManager>>,
    pub mailer: Option<EmailSender>,
}

impl TaskContext {
    pub fn new(queue: Option<Pool<RedisConnectionManager>>, mailer: Option<EmailSender>) -> Self {
        TaskContext { queue, mailer }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

#[async_trait]
#[allow(clippy::module_name_repetitions)]
pub trait AppWorker<T>: Worker<T>
where
    Self: Sized,
    T: Send + Sync + serde::Serialize + 'static,
{
    fn build(ctx: &TaskContext) -> Self;

    async fn perform_later(ctx: &TaskContext, args: T) -> Result<()> {
        let config = Config::get();

        match &config.workers.mode {
            WorkerMode::BackgroundQueue => {
                if let Some(queue) = &ctx.queue {
                    Self::perform_async(queue, args).await?;
                } else {
                    error!(
                        error.msg =
                            "Worker mode requested but no queue connection supplied, skipping job",
                    );
                }
            }
            WorkerMode::ForegroundBlocking => {
                Self::build(ctx).perform(args).await?;
            }
            WorkerMode::BackgroundAsync => {
                let ctx_clone = ctx.clone();
                tokio::spawn(async move {
                    if let Err(e) = Self::build(&ctx_clone).perform(args).await {
                        anyhow!("Error performing task due to: {:?}", e);
                    }
                });
            }
        }
        Ok(())
    }
}

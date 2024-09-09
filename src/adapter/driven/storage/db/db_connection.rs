use std::sync::Arc;

use anyhow::Error;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::shared::config::config::Config;

#[derive(Debug, Clone)]
pub struct DB {
    pub pool: Arc<Pool<Postgres>>,
}

impl DB {
    pub async fn new() -> Result<Self, Error> {
        let config = Config::get();
        let url = &config.database.uri;
        let max_conn = &config.database.max_connections;
        let pool = PgPoolOptions::new()
            .max_connections(*max_conn)
            .connect(url)
            .await?;

        Ok(DB {
            pool: Arc::new(pool),
        })
    }
}

use std::sync::Arc;

use anyhow::Error;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::config::Settings;

#[derive(Debug, Clone)]
pub struct DB {
	pub pool: Arc<Pool<Postgres>>,
	config: &'static Settings,
}

impl DB {
	pub async fn new(config: &Settings) -> Result<Self, Error> {
		let url = config.database.url.as_deref().unwrap_or("localhost:5432");
		let max_conn = config.database.max_conn.unwrap_or(10) as u32;
		let pool = PgPoolOptions::new()
			.max_connections(max_conn)
			.connect(url).await?;

		Ok(DB {
			pool: Arc::new(pool),
			config,
		})
	}
}
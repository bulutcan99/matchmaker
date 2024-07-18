use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::config::Settings;

pub struct DB {
	pub client: Arc<Surreal<Client>>,
}

#[async_trait]
impl DB {
	fn new() -> Self {
		DB {
			client: Arc::new((Surreal::new::<Ws>("127.0.0.1:8000").unwrap())),
		}
	}

	async fn connect(&self, config: &Settings) -> Result<(), Error> {
		let db = self.client.clone();
		let url = config.database.url.as_deref().unwrap_or("localhost:8000");
		let username = config.database.username.as_deref().unwrap_or("root");
		let password = config.database.password.as_deref().unwrap_or("root");
		let ns = config.database.db_name.as_deref().unwrap_or("todo");
		let db_name = config.database.db_name.as_deref().unwrap_or("todo");

		db.connect::<Ws>(url).await?;
		db.signin(Root {
			username,
			password,
		}).await?;

		db.use_ns(ns).use_db(db_name).await?;
		Ok(())
	}

	async fn disconnect(&self) -> Result<(), Error> {
		Ok(())
	}
}

use anyhow::Error;
use once_cell::sync::Lazy;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::config::Settings;

pub struct DB {
	pub client: Lazy<Surreal<Client>>,
}

impl DB {
	async fn new() -> Self {
		DB {
			client: Lazy::new(Surreal::init),
		}
	}

	async fn connect(&self, config: &Settings) -> Result<(), Error> {
		let url = config.database.url.as_deref().unwrap_or("localhost:8000");
		let username = config.database.username.as_deref().unwrap_or("root");
		let password = config.database.password.as_deref().unwrap_or("root");
		let ns = config.database.db_name.as_deref().unwrap_or("todo");
		let db_name = config.database.db_name.as_deref().unwrap_or("todo");

		self.client.connect::<Ws>(url).with_capacity(100_000).await?;
		self.client.signin(Root {
			username,
			password,
		}).await?;

		self.client.use_ns(ns).use_db(db_name).await?;
		Ok(())
	}

	async fn disconnect(&self) -> Result<(), Error> {
		Ok(())
	}
}

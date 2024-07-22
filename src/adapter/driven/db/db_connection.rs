use std::sync::Arc;

use anyhow::Error;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use crate::config::Settings;

pub struct DB {
	pub client: Arc<Surreal<Client>>,
}
impl DB {
	pub fn new() -> Self {
		DB {
			client: Arc::new(Surreal::init()),
		}
	}

	pub async fn connect(&self, config: &Settings) -> Result<(), Error> {
		let url = config.database.url.as_deref().unwrap_or("localhost:8000");
		let username = config.database.username.as_deref().unwrap_or("root");
		let password = config.database.password.as_deref().unwrap_or("root");
		let ns = config.database.db_name.as_deref().unwrap_or("matchmaker");
		let db_name = config.database.db_name.as_deref().unwrap_or("matchmaker");

		self.client.connect::<Ws>(url).with_capacity(10).await?;
		self.client.signin(Root {
			username,
			password,
		}).await?;

		self.client.use_ns(ns).use_db(db_name).await?;
		Ok(())
	}

	pub async fn disconnect(&self) -> Result<(), Error> {
		// implement disconnect logic if necessary
		Ok(())
	}
}
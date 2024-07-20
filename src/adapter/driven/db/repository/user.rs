use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Id;
use surrealdb::Surreal;

use crate::config::Settings;
use crate::core::domain::entity::user::User;
use crate::core::port::storage::Storage;

pub struct UserRepository {
	table: String,
	db: Arc<Surreal<Client>>,
}

impl UserRepository
{
	pub fn new(config: &Settings, db: Arc<Surreal<Client>>) -> Self {
		let table_name = config.database.user_table.as_deref().unwrap_or("user");
		UserRepository {
			table: table_name.to_string(),
			db,
		}
	}
}

#[async_trait]
impl Storage<User> for UserRepository {
	async fn find_by_id(&self, id: Id) -> Option<User> {
		match self.db.select((&self.table, id)).await {
			Ok(Some(record)) => Some(record),
			Ok(None) => None,
			Err(_) => None,
		}
	}

	async fn find_all(&self) -> Result<Vec<User>, Error> {
		match self.db.select(&self.table).await {
			Ok(records) => Ok(records),
			Err(e) => Err(e.into()),
		}
	}

	async fn save(&self, user: &User) -> Result<(), Error> {
		match self.db.insert(&self.table).content(user).await {
			Ok(_) => Ok(()),
			Err(e) => Err(e.into()),
		}
	}

	async fn update(&self, user: &User) -> Result<User, Error> {
		match self.db.update((&self.table, &user.id)).content(user).await {
			Ok(Some(updated_record)) => Ok(updated_record),
			Ok(None) => Err(Error::NotFound),
			Err(e) => Err(e.into()),
		}
	}

	async fn delete_by_id(&self, id: Id) -> Result<(), Error> {
		match self.db.delete((&self.table, id)).await {
			Ok(_) => Ok(()),
			Err(e) => Err(e.into()),
		}
	}
}
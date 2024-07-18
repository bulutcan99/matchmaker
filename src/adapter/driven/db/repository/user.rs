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
	pub fn new(config: &Settings, db: T) -> Self {
		let table_name = config.database.user_table.as_deref().unwrap_or("user");
		UserRepository {
			table: table_name.to_string(),
			db,
		}
	}
}

#[async_trait]
impl Storage<User> for UserRepository
{
	async fn find_by_id(&self, id: Id) -> Result<User, Error> {
		if let Some(record) = self.db.select((&self.table, id)).await? {
			return Ok(record);
		}

		Err(Error::msg(format!("User with id {} not found", id)))
	}

	async fn find_all(&self) -> Result<Vec<User>, Error> {
		let records = self.db.select(&self.table).await?;
		Ok(records)
	}

	async fn save(&self, user: &User) -> Result<(), Error> {
		self.db.insert(&self.table).content(user).await?;
		Ok(())
	}

	async fn update(&self, user: &User) -> Result<User, Error> {
		let updated_record = self.db.update((&self.table, &user.id)).content(user).await?;
		Ok(updated_record.unwrap())
	}

	async fn delete_by_id(&self, id: Id) -> Result<(), Error> {
		self.db.delete((&self.table, id)).await?;
		Ok(())
	}
}
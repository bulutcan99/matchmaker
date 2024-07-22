use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use config::ConfigError::NotFound;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::config::Settings;
use crate::core::domain::entity::user::User;
use crate::core::port::storage::Storage;

//sqlx and psg will integrate instead of surreal
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
	async fn find_by_id(&self, id: Thing) -> Result<User, Error> {
		match self.db.select((&self.table, id)).await? {
			Some(record) => Ok(record),
			None => Err(Error::from(NotFound("User".to_string()))),
		}
	}

	async fn find_all(&self) -> Result<Vec<User>, Error> {
		let record = self.db.select(&self.table).await?;
		Ok(record)
	}

	async fn save(&self, user: User) -> Result<User, Error> {
		let record: Option<User> = self.db.insert(&self.table, &user.name).content(user).await?;
		Ok(record)
	}

	async fn update(&self, id: Thing, user: User) -> Result<User, Error> {
		let record = self.db.update((&self.table, id)).content(user).await?.unwrap();
		Ok(record)
	}

	async fn delete_by_id(&self, id: Thing) -> Result<(), Error> {
		let result = self.db.delete((&self.table, id)).await?.unwrap();
		Ok(result)
	}
}
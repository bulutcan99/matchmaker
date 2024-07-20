use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use config::ConfigError::NotFound;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

use crate::config::Settings;
use crate::core::domain::entity::company::Company;
use crate::core::port::storage::Storage;

pub struct CompanyRepository {
	table: String,
	db: Arc<Surreal<Client>>,
}

impl CompanyRepository
{
	pub fn new(config: &Settings, db: Arc<Surreal<Client>>) -> Self {
		let table_name = config.database.company_table.as_deref().unwrap_or("company");
		CompanyRepository {
			table: table_name.to_string(),
			db,
		}
	}
}

#[async_trait]
impl Storage<Company> for CompanyRepository {
	async fn find_by_id(&self, id: Thing) -> Result<Company, Error> {
		match self.db.select((&self.table, id)).await? {
			Some(record) => Ok(record),
			None => Err(Error::from(NotFound("Company".to_string()))),
		}
	}

	async fn find_all(&self) -> Result<Vec<Company>, Error> {
		let record = self.db.select(&self.table).await?;
		Ok(record)
	}

	async fn save(&self, company: Company) -> Result<Vec<Company>, Error> {
		let record = self.db.insert(&self.table).content(company).await?;
		Ok(record)
	}

	async fn update(&self, id: Thing, company: Company) -> Result<Company, Error> {
		let record = self.db.update((&self.table, id)).content(company).await?.unwrap();
		Ok(record)
	}

	async fn delete_by_id(&self, id: Thing) -> Result<(), Error> {
		let result = self.db.delete((&self.table, id)).await?.unwrap();
		Ok(result)
	}
}
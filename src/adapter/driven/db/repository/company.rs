use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use surrealdb::engine::remote::ws::Client;
use surrealdb::sql::Id;
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
	pub fn new(config: &Settings, db: T) -> Self {
		let table_name = config.database.company_table.as_deref().unwrap_or("company");
		CompanyRepository {
			table: table_name.to_string(),
			db,
		}
	}
}

#[async_trait]
impl Storage<Company> for CompanyRepository
{
	async fn find_by_id(&self, id: Id) -> Result<Company, Error> {
		if let Some(record) = self.db.select((&self.table, id)).await? {
			return Ok(record);
		}

		Err(Error::msg(format!("Company with id {} not found", id)))
	}

	async fn find_all(&self) -> Result<Vec<Company>, Error> {
		let records = self.db.select(&self.table).await?;
		Ok(records)
	}

	async fn save(&self, company: &Company) -> Result<(), Error> {
		self.db.insert(&self.table).content(company).await?;
		Ok(())
	}

	async fn update(&self, company: &Company) -> Result<Company, Error> {
		let updated_record = self.db.update((&self.table, &company.id)).content(company).await?;
		Ok(updated_record.unwrap())
	}

	async fn delete_by_id(&self, id: Id) -> Result<(), Error> {
		self.db.delete((&self.table, id)).await?;
		Ok(())
	}
}
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
	async fn find_by_id(&self, id: Id) -> Option<Company> {
		match self.db.select((&self.table, id)).await {
			Ok(Some(record)) => Some(record),
			Ok(None) => None,
			Err(_) => None,
		}
	}

	async fn find_all(&self) -> Result<Vec<Company>, Error> {
		match self.db.select(&self.table).await {
			Ok(records) => Ok(records),
			Err(e) => Err(e.into()),
		}
	}

	async fn save(&self, company: &Company) -> Result<(), Error> {
		match self.db.insert(&self.table).content(company).await {
			Ok(_) => Ok(()),
			Err(e) => Err(e.into()),
		}
	}

	async fn update(&self, company: &Company) -> Result<Company, Error> {
		match self.db.update((&self.table, &company.id)).content(company).await {
			Ok(Some(updated_record)) => Ok(updated_record),
			Ok(None) => Err(Error::NotFound),
			Err(e) => Err(e.into()),
		}
	}

	async fn delete_by_id(&self, id: Id) -> Result<(), Error> {
		match self.db.delete((&self.table, id)).await {
			Ok(_) => Ok(()),
			Err(e) => Err(e.into())
		}
	}
}
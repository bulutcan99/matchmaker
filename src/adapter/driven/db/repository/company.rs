use std::sync::Arc;

use anyhow::Error;
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::core::domain::entity::company::Company;
use crate::core::domain::valueobject::date::DateService;
use crate::core::port::storage::Storage;

pub struct CompanyRepository {
	db: Arc<Pool<Postgres>>,
}

impl CompanyRepository {
	pub fn new(db: Arc<Pool<Postgres>>) -> Self {
		CompanyRepository { db }
	}
}
#[async_trait]
impl Storage<Company> for CompanyRepository {
	async fn find_by_id(&self, id: Uuid) -> Result<Company, Error> {
		let found_company = sqlx::query_as!(
            Company,
            r#"
                SELECT *
                FROM company WHERE id = $1
            "#,
            id
        )
			.fetch_one(&self.db)
			.await?;

		Ok(found_company)
	}

	async fn find_all(&self) -> Result<Vec<Company>, Error> {
		let companies = sqlx::query_as!(
            Company,
            r#"
                SELECT id, foundation_date, name, description, url, sector, created_at, updated_at
                FROM company
            "#
        )
			.fetch_all(&self.db)
			.await?;

		Ok(companies)
	}

	async fn save(&self, company: Company) -> Result<Company, Error> {
		let saved_company = sqlx::query_as!(
            Company,
            r#"
                INSERT INTO company (id, foundation_date, name, description, url, sector, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id, foundation_date, name, description, url, sector, created_at, updated_at
            "#,
            company.id,
            company.foundation_date as i16,
            company.name,
            company.description,
            company.url,
            company.sector.to_string(),
            DateService::convert_to_offset(company.created_at),
            DateService::convert_to_offset(company.updated_at)
        )
			.fetch_one(&self.db)
			.await?;

		Ok(saved_company)
	}

	async fn update(&self, id: Uuid, company: Company) -> Result<Company, Error> {
		let updated_company = sqlx::query_as!(
            Company,
            r#"
                UPDATE company SET foundation_date = $2, name = $3, description = $4, url = $5, sector = $6, updated_at = $7
                WHERE id = $1
                RETURNING id, foundation_date, name, description, url, sector, created_at, updated_at
            "#,
            id,
            company.foundation_date as i16,
            company.name,
            company.description,
            company.url,
            company.sector.to_string(),
            DateService::convert_to_offset(company.updated_at)
        )
			.fetch_one(&self.db)
			.await?;

		Ok(updated_company)
	}

	async fn delete_by_id(&self, id: Uuid) -> Result<(), Error> {
		sqlx::query!(
            r#"
                DELETE FROM company WHERE id = $1
            "#,
            id
        )
			.execute(&self.db)
			.await?;

		Ok(())
	}
}
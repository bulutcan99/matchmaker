use std::sync::Arc;

use anyhow::{anyhow, Context, Error};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::core::domain::entity::company::Company;
use crate::core::domain::valueobject::date::Timestamp;
use crate::core::domain::valueobject::sector::Sector;
use crate::core::port::company::CompanyRepo;

#[derive(Debug, Clone)]
pub struct CompanyRepository {
    db: Arc<Pool<Postgres>>,
}

impl CompanyRepository {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        CompanyRepository { db }
    }
}

#[async_trait]
impl CompanyRepo for CompanyRepository {
    async fn save(&self, company: &Company) -> Result<Uuid, Error> {
        let saved_company_id = sqlx::query_scalar!(
            r#"
            INSERT INTO company (id, foundation_date, name, description, url, sector, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id
            "#,
            company.id,
            company.foundation_date,
            company.name,
            company.description,
            company.url,
            company.sector.to_string(),
            Timestamp::now_utc().convert_to_offset(),
            Timestamp::now_utc().convert_to_offset(),
        )
        .fetch_one(&*self.db)
        .await
        .context("Error saving company to database")?;

        Ok(saved_company_id)
    }

    async fn update(&self, id_str: &str, company: &Company) -> Result<Company, Error> {
        let id = Uuid::parse_str(id_str).context("Invalid UUID format")?;
        let sector_str = company.sector.to_string();

        let row = sqlx::query!(
            r#"
            UPDATE company
            SET
                foundation_date = COALESCE($2, foundation_date),
                name = COALESCE($3, name),
                description = COALESCE($4, description),
                url = COALESCE($5, url),
                sector = COALESCE($6, sector),
                updated_at = COALESCE($7, updated_at)
            WHERE id = $1
            RETURNING id, foundation_date, name, description, url, sector, created_at, updated_at
            "#,
            id,
            company.foundation_date as i16,
            company.name,
            company.description,
            company.url,
            sector_str,
            Timestamp::now_utc().convert_to_offset(),
        )
        .fetch_one(&*self.db)
        .await
        .context("Error updating company in database")?;

        let updated_company = Company {
            id: Some(row.id),
            foundation_date: row.foundation_date,
            name: row.name,
            description: row.description,
            url: row.url.unwrap_or_default(),
            sector: Sector::from_string(&row.sector)
                .ok_or_else(|| anyhow!("Unknown sector value: {}", row.sector))?,
            created_at: Timestamp::from(row.created_at),
            updated_at: Timestamp::from(row.updated_at),
        };

        Ok(updated_company)
    }

    async fn delete(&self, id_str: &str) -> Result<(), Error> {
        let id = Uuid::parse_str(id_str).context("Invalid UUID format")?;
        sqlx::query!(
            r#"
            DELETE FROM company WHERE id = $1
            "#,
            id
        )
        .execute(&*self.db)
        .await
        .context("Error deleting company from database")?;

        Ok(())
    }

    async fn find_all(&self) -> Result<Vec<Company>, Error> {
        let rows = sqlx::query!(
            r#"
            SELECT id, foundation_date, name, description, url, sector, created_at, updated_at
            FROM company
            "#
        )
        .fetch_all(&*self.db)
        .await
        .context("Error fetching all companies from database")?;

        let companies = rows
            .into_iter()
            .map(|row| {
                let sector_enum = Sector::from_string(&row.sector)
                    .ok_or_else(|| anyhow!("Unknown sector value: {}", row.sector))?;

                Ok(Company {
                    id: Some(row.id),
                    foundation_date: row.foundation_date,
                    name: row.name,
                    description: row.description,
                    url: row.url.unwrap_or_default(),
                    sector: sector_enum,
                    created_at: Timestamp::from(row.created_at),
                    updated_at: Timestamp::from(row.updated_at),
                })
            })
            .collect::<Result<Vec<Company>, Error>>()
            .context("Error mapping rows to company entities")?;

        Ok(companies)
    }

    async fn find_by_id(&self, id_str: &str) -> Result<Option<Company>, Error> {
        let id = Uuid::parse_str(id_str).context("Invalid UUID format")?;

        let row = sqlx::query!(
            r#"
            SELECT id, foundation_date, name, description, url, sector, created_at, updated_at
            FROM company WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&*self.db)
        .await
        .context("Error querying company by id")?;

        match row {
            Some(row) => {
                let sector_enum = Sector::from_string(&row.sector)
                    .ok_or_else(|| anyhow!("Unknown sector value: {}", row.sector))?;

                let company = Company {
                    id: Some(row.id),
                    foundation_date: row.foundation_date,
                    name: row.name,
                    description: row.description,
                    url: row.url.unwrap_or_default(),
                    sector: sector_enum,
                    created_at: Timestamp::from(row.created_at),
                    updated_at: Timestamp::from(row.updated_at),
                };

                Ok(Some(company))
            }
            None => Ok(None),
        }
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Company>, Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, foundation_date, name, description, url, sector, created_at, updated_at
            FROM company WHERE name = $1
            "#,
            name
        )
        .fetch_optional(&*self.db)
        .await
        .context("Error querying company by name")?;

        if let Some(row) = row {
            let sector_enum = Sector::from_string(&row.sector)
                .ok_or_else(|| anyhow!("Unknown sector value: {}", row.sector))?;

            let company = Company {
                id: Some(row.id),
                foundation_date: row.foundation_date,
                name: row.name,
                description: row.description,
                url: row.url.unwrap_or_default(),
                sector: sector_enum,
                created_at: Timestamp::from(row.created_at),
                updated_at: Timestamp::from(row.updated_at),
            };

            Ok(Some(company))
        } else {
            Ok(None)
        }
    }
}

use std::sync::Arc;

use anyhow::{anyhow, Error};
use async_trait::async_trait;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::core::domain::entity::company::Company;
use crate::core::domain::valueobject::date::Timestamp;
use crate::core::domain::valueobject::sector::Sector;
use crate::core::port::company::CompanyRepo;
use crate::core::port::storage::Repo;

pub struct CompanyRepository {
    db: Arc<Pool<Postgres>>,
}

impl CompanyRepository {
    pub fn new(db: Arc<Pool<Postgres>>) -> Self {
        CompanyRepository { db }
    }
}

#[async_trait]
impl Repo<Company> for CompanyRepository {
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
    .await?;

        Ok(saved_company_id)
    }

    async fn update(&self, id_str: &str, company: &Company) -> Result<Company, Error> {
        let id = Uuid::parse_str(id_str)?;
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
            company.name.to_owned(),
            company.description.to_owned(),
            company.url.to_owned(),
            sector_str,
            Timestamp::now_utc().convert_to_offset(),
        )
        .fetch_one(&*self.db)
        .await?;

        let updated_company = Company {
            id: Some(row.id),
            foundation_date: row.foundation_date,
            name: row.name,
            description: row.description,
            url: row.url.unwrap_or_default(),
            sector: Sector::from_string(&row.sector)
                .ok_or_else(|| anyhow!("Unknown sector value"))?,
            created_at: Timestamp::from(row.created_at),
            updated_at: Timestamp::from(row.updated_at),
        };

        Ok(updated_company)
    }

    async fn delete(&self, id_str: &str) -> Result<(), Error> {
        let id = Uuid::parse_str(id_str)?;
        sqlx::query!(
            r#"
                DELETE FROM company WHERE id = $1
            "#,
            id
        )
        .execute(&*self.db)
        .await?;

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
        .await?;

        let companies = rows
            .into_iter()
            .map(|row| {
                let sector_enum = Sector::from_string(&row.sector)
                    .ok_or_else(|| anyhow!("Unknown sector value"))?;

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
            .collect::<Result<Vec<Company>, Error>>()?;

        Ok(companies)
    }

    async fn find_by_id(&self, id_str: &str) -> Result<Option<Company>, Error> {
        let id = Uuid::parse_str(id_str)?;

        let row = sqlx::query!(
            r#"
        SELECT id, foundation_date, name, description, url, sector, created_at, updated_at
        FROM "company" WHERE id = $1
    "#,
            id
        )
        .fetch_optional(&*self.db)
        .await
        .map_err(|err| anyhow!("Error from getting company by id"))?;

        if let Some(row) = row {
            let sector_enum =
                Sector::from_string(&row.sector).ok_or_else(|| anyhow!("Unknown sector value"))?;

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

    async fn find_by<F, Q>(&self, filter: &F) -> Result<Option<Company>, Error>
    where
        F: Fn(&Company) -> Q + Send + Sync,
        Q: PartialEq + Send,
    {
        let companies = self.find_all().await?;
        let filtered_company = companies
            .into_iter()
            .find(|company| filter(company) == filter(&companies[0]))
            .clone();

        Ok(filtered_company)
    }
}

#[async_trait]
impl CompanyRepo for CompanyRepository {
    async fn find_by_name(&self, name: &str) -> Result<Option<Company>, Error> {
        let row = sqlx::query!(
            r#"
        SELECT id, foundation_date, name, description, url, sector, created_at, updated_at
        FROM "company" WHERE name = $1
    "#,
            name
        )
        .fetch_optional(&*self.db)
        .await
        .map_err(|err| anyhow!("Error from getting company by name"))?;

        if let Some(row) = row {
            let sector_enum =
                Sector::from_string(&row.sector).ok_or_else(|| anyhow!("Unknown sector value"))?;

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

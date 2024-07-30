use std::sync::Arc;

use anyhow::Error;
use sqlx::{Pool, Postgres};

use crate::adapter::driven::db::db_connection::DB;
use crate::adapter::driven::db::repository::company::CompanyRepository;
use crate::adapter::driven::db::repository::user::UserRepository;
use crate::config::Settings;

pub struct Container {
    pub settings: Settings,
    pub db: Arc<Pool<Postgres>>,
    pub user_repository: UserRepository,
    pub company_repository: CompanyRepository,
}

impl Container {
    pub async fn new() -> Result<Self, Error> {
        let settings = Settings::init()?;

        // Initialize the DB
        let db = DB::new();
        db.connect(&settings).await?;

        // Initialize repositories
        let user_repository = UserRepository::new(Arc::clone(&db));
        let company_repository = CompanyRepository::new(Arc::clone(&db));

        Ok(Container {
            settings,
            db,
            user_repository,
            company_repository,
        })
    }
}

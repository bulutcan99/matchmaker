use std::sync::Arc;

use anyhow::Error;
use sqlx::{Pool, Postgres};

use crate::adapter::driven::db::db_connection::DB;
use crate::adapter::driven::db::repository::company::CompanyRepository;
use crate::adapter::driven::db::repository::user::UserRepository;
use crate::config::Settings;

pub struct Container {
    pub db: Arc<Pool<Postgres>>,
    pub user_repository: UserRepository,
    pub company_repository: CompanyRepository,
}

impl Container {
    pub async fn new() -> Result<Self, Error> {
        Settings::init()?;
        let db = DB::new().await?;
        let user_repository = UserRepository::new(Arc::clone(&db.pool));
        let company_repository = CompanyRepository::new(Arc::clone(&db.pool));

        Ok(Container {
            db: Arc::clone(&db.pool),
            user_repository,
            company_repository,
        })
    }
}

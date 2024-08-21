use std::sync::Arc;

use anyhow::Error;
use sqlx::{Pool, Postgres};

use crate::adapter::driven::auth::jwt::JwtTokenHandler;
use crate::adapter::driven::storage::db::db_connection::DB;
use crate::adapter::driven::storage::db::repository::company::CompanyRepository;
use crate::adapter::driven::storage::db::repository::user::UserRepository;
use crate::config::Settings;
use crate::core::application::usecase::user::service::UserService;

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
        let token_handler = JwtTokenHandler::new();
        let user_service = UserService::new(user_repository, token_handler);
    }
}

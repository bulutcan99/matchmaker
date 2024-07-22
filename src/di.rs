use std::sync::Arc;

use anyhow::Error;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use crate::adapter::driven::db::db_connection::DB;
use crate::adapter::driven::db::repository::company::CompanyRepository;
use crate::adapter::driven::db::repository::user::UserRepository;
use crate::config::Settings;

pub struct Container {
	pub settings: Settings,
	pub db: Arc<Surreal<Client>>,
	pub user_repository: UserRepository,
	pub company_repository: CompanyRepository,
}

impl Container {
	pub async fn new() -> Result<Self, Error> {
		let settings = Settings::new()?;

		// Initialize the DB
		let db = DB::new();
		db.connect(&settings).await?;

		// Initialize repositories
		let user_repository = UserRepository::new(&settings, Arc::clone(&db.client));
		let company_repository = CompanyRepository::new(&settings, Arc::clone(&db.client));

		Ok(Container {
			settings,
			db: db.client,
			user_repository,
			company_repository,
		})
	}
}
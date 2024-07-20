use anyhow::Error;
use surrealdb::sql::Thing;

use crate::core::domain::entity::company::Company;

pub trait CompanyUseCase {
	fn get_company(&self, id: Thing) -> Result<Option<Company>, Error>;
	fn create_company(&self, company: &Company) -> Result<(), Error>;
	fn modify_company(&self, company: &Company) -> Result<Company, Error>;
	fn remove_company(&self, id: Thing) -> Result<(), Error>;
}


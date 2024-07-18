use anyhow::Error;
use surrealdb::sql::Id;

use crate::core::domain::entity::user::User;

pub trait UserUseCase {
	fn get_user(&self, id: Id) -> Option<User>;
	fn create_user(&self, user: &User) -> Result<(), Error>;
	fn modify_user(&self, user: &User) -> Result<User, Error>;
	fn remove_user(&self, id: Id) -> Result<(), Error>;
}

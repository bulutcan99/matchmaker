use anyhow::Error;

use crate::core::application::usecase::user::dto::{AuthenticatedUserOutput, GetProfileInput, GetProfileOutput, UpdateUserPofileInput, UserLoginInput, UserRegisterInput};

pub trait UserManagement: Send + Sync {
	async fn register(&self, input: &UserRegisterInput) -> Result<AuthenticatedUserOutput, Error>;
	async fn login(&self, input: &UserLoginInput) -> Result<AuthenticatedUserOutput, Error>;
	async fn update_profile(&self, input: &UpdateUserPofileInput) -> Result<(), Error>;
	async fn get_profile(&self, input: &GetProfileInput) -> Result<GetProfileOutput, Error>;
}

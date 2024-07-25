use anyhow::Error;
use async_trait::async_trait;

use crate::core::application::usecase::user::dto::{
	AuthenticatedUserOutput, GetProfileInput, GetProfileOutput, UpdateUserPofileInput,
	UserLoginInput, UserRegisterInput,
};
use crate::core::domain::entity::user::User;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn find_by_email(&self, email: String) -> Result<Option<User>, Error>;
}

#[async_trait]
pub trait UserManagement: Send + Sync {
    async fn register(&self, input: &UserRegisterInput) -> Result<AuthenticatedUserOutput, Error>;
    async fn login(&self, input: &UserLoginInput) -> Result<AuthenticatedUserOutput, Error>;
    async fn update_profile(&self, input: &UpdateUserPofileInput) -> Result<(), Error>;
    async fn get_profile(&self, input: &GetProfileInput) -> Result<GetProfileOutput, Error>;
}

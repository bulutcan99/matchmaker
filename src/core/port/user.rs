use anyhow::Error;
use async_trait::async_trait;

use crate::adapter::driving::presentation::http::handler::auth::login::{
    UserLoginRequest, UserLoginResponse,
};
use crate::adapter::driving::presentation::http::handler::auth::register::{
    UserRegisterRequest, UserRegisterResponse,
};
use crate::core::domain::entity::user::User;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
}

#[async_trait]
pub trait UserManagement: Send + Sync {
    async fn register(&self, input: &UserRegisterRequest) -> Result<UserRegisterResponse, Error>;
    async fn login(&self, input: &UserLoginRequest) -> Result<UserLoginResponse, Error>;
    // async fn update_profile(&self, input: &UserRegisterRequest) -> Result<(), Error>;
    // async fn get_profile(&self, input: &UserRegisterRequest) -> Result<GetProfileOutput, Error>;
}

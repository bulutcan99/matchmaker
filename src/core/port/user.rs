use anyhow::Error;
use async_trait::async_trait;
use validator::ValidationErrors;

use crate::adapter::driving::presentation::http::handler::auth::login::UserLoginRequest;
use crate::adapter::driving::presentation::http::handler::auth::register::UserRegisterRequest;
use crate::core::application::usecase::auth::error::{LoginError, MeError, RegisterError};
use crate::core::domain::entity::user::User;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn save(&self, entity: &User) -> Result<User, Error>;
    async fn update(&self, id_str: &str, entity: &User) -> Result<User, Error>;
    async fn delete(&self, id_str: &str) -> Result<(), Error>;
    async fn find_all(&self) -> Result<Vec<User>, Error>;
    async fn find_by_id(&self, id_str: &str) -> Result<Option<User>, Error>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, Error>;
}

#[async_trait]
pub trait UserManagement: Send + Sync {
    async fn register(
        &self,
        input: &UserRegisterRequest,
    ) -> Result<User, RegisterError<ValidationErrors>>;
    async fn login(&self, input: &UserLoginRequest) -> Result<User, LoginError>;
    async fn me(&self, email: &str) -> Result<User, MeError>;
    // async fn update_profile(&self, input: &UserRegisterRequest) -> Result<(), Error>;
}

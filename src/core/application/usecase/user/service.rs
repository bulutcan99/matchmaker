use anyhow::Error;
use async_trait::async_trait;

use crate::core::application::usecase::user::dto::{
    AuthenticatedUserOutput, GetProfileInput, GetProfileOutput, UpdateUserPofileInput,
    UserLoginInput, UserRegisterInput,
};
use crate::core::domain::entity::user::User;
use crate::core::port::auth::TokenMaker;
use crate::core::port::storage::Repo;
use crate::core::port::user::UserManagement;

pub struct UserService<T, K>
where
    T: TokenMaker,
    K: Repo<User>,
{
    token_handler: T,
    user_repository: K,
}

impl<T, K> UserService<T, K>
where
    T: TokenMaker,
    K: Repo<User>,
{
    pub fn new(token_handler: T, user_repository: K) -> Self {
        Self {
            token_handler,
            user_repository,
        }
    }
}

#[async_trait]
impl<T, K> UserManagement for UserService<T, K>
where
    T: TokenMaker,
    K: Repo<User>,
{
    async fn register(&self, input: &UserRegisterInput) -> Result<AuthenticatedUserOutput, Error> {
        todo!()
    }

    async fn login(&self, input: &UserLoginInput) -> Result<AuthenticatedUserOutput, Error> {
        todo!()
    }

    async fn update_profile(&self, input: &UpdateUserPofileInput) -> Result<(), Error> {
        todo!()
    }

    async fn get_profile(&self, input: &GetProfileInput) -> Result<GetProfileOutput, Error> {
        todo!()
    }
}

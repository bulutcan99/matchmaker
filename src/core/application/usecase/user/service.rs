use anyhow::Error;
use async_trait::async_trait;

use crate::core::application::usecase::user::dto::{
    AuthenticatedUserOutput, GetProfileInput, GetProfileOutput, UpdateUserPofileInput,
    UserLoginInput, UserRegisterInput,
};
use crate::core::domain::entity::user;
use crate::core::domain::entity::user::User;
use crate::core::port::auth::TokenMaker;
use crate::core::port::storage::Repo;
use crate::core::port::user::{UserManagement, UserRepo};

pub struct UserService<T, K, U>
where
    T: TokenMaker,
    K: Repo<User>,
    U: UserRepo,
{
    token_handler: T,
    repo_user: K,
    user_repository: U,
}

impl<T, K, U> UserService<T, K, U>
where
    T: TokenMaker,
    K: Repo<User>,
    U: UserRepo,
{
    pub fn new(token_handler: T, repo_user: K, user_repository: U) -> Self {
        Self {
            token_handler,
            repo_user,
            user_repository,
        }
    }
}

#[async_trait]
impl<T, K, U> UserManagement for UserService<T, K, U>
where
    T: TokenMaker,
    K: Repo<User>,
    U: UserRepo,
{
    async fn register(&self, input: &UserRegisterInput) -> Result<AuthenticatedUserOutput, Error> {
        let found_user = self.user_repository.find_by_email(input.email.as_str());
        match found_user {
            Some(_) => Err(Error::from("This email already in use!")),
            None =>{
                let new_user = user::User::new(
                    input.first_name.clone(),
                    input.last_name.clone(),
                    input.email.clone(),
                    input.

                )
            }

        }
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

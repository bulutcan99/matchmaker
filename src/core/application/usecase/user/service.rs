use anyhow::{anyhow, Error};
use async_trait::async_trait;

use crate::adapter::driving::presentation::http::handler::auth::login::{
    UserLoginRequest, UserLoginResponse,
};
use crate::adapter::driving::presentation::http::handler::auth::register::{
    UserRegisterRequest, UserRegisterResponse,
};
use crate::core::domain::entity::user::User;
use crate::core::domain::valueobject::role;
use crate::core::port::auth::TokenMaker;
use crate::core::port::storage::Repo;
use crate::core::port::user::{UserManagement, UserRepo};

#[derive(Debug, Clone)]
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
    async fn register(&self, input: &UserRegisterRequest) -> Result<UserRegisterResponse, Error> {
        let found_user = self
            .user_repository
            .find_by_email(input.email.as_str())
            .await?;

        if found_user.is_some() {
            return Err(anyhow!("This email is already in use!"));
        }

        let new_user = User::new(
            input.first_name.clone(),
            input.last_name.clone(),
            input.email.clone(),
            input.password.clone(),
            role::Role::User,
        );

        let registered_id = self.repo_user.save(&new_user).await?;
        let register_response = UserRegisterResponse {
            uuid: registered_id.to_string(),
        };
        Ok(register_response)
    }

    async fn login(&self, input: &UserLoginRequest) -> Result<UserLoginResponse, Error> {
        let found_user = self
            .user_repository
            .find_by_email(input.email.as_str())
            .await?;

        let found_user = match found_user {
            Some(user) => user,
            None => return Err(anyhow!("There is no account with the given email!")),
        };

        let is_verified = match found_user.password_hash.verify_password(&input.password) {
            Ok(true) => true,
            Ok(false) => return Err(anyhow!("Password is incorrect!")),
            Err(e) => return Err(anyhow!("Failed to verify password: {}", e)),
        };

        let access_token = self.token_handler.generate_token(&found_user).await;
        let result = UserLoginResponse { access_token };
        Ok(result)
    }

    // async fn update_profile(&self, input: &UpdateUserPofileInput) -> Result<(), Error> {
    //     todo!()
    // }
    //
    // async fn get_profile(&self, input: &GetProfileInput) -> Result<GetProfileOutput, Error> {
    //     todo!()
    // }
}

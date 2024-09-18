use std::sync::Arc;

use async_trait::async_trait;
use validator::ValidationErrors;

use crate::adapter::driving::presentation::http::handler::auth::login::UserLoginRequest;
use crate::adapter::driving::presentation::http::handler::auth::register::UserRegisterRequest;
use crate::core::application::usecase::auth::error::{LoginError, MeError, RegisterError};
use crate::core::domain::entity::user::User;
use crate::core::domain::valueobject::role;
use crate::core::port::user::{UserManagement, UserRepo};

#[derive(Debug, Clone)]
pub struct UserService<K>
where
    K: UserRepo,
{
    user_repository: Arc<K>,
}

impl<K> UserService<K>
where
    K: UserRepo,
{
    pub fn new(user_repository: Arc<K>) -> Self {
        Self { user_repository }
    }
}

#[async_trait]
impl<K> UserManagement for UserService<K>
where
    K: UserRepo,
{
    async fn register(
        &self,
        input: &UserRegisterRequest,
    ) -> Result<User, RegisterError<ValidationErrors>> {
        let found_user = self
            .user_repository
            .find_by_email(input.email.as_str())
            .await
            .map_err(|_| RegisterError::DbInternalError)?;

        if found_user.is_some() {
            return Err(RegisterError::UserAlreadyRegistered);
        }

        let new_user = User::new(
            input.name.clone(),
            input.surname.clone(),
            input.email.clone(),
            input.password.clone(),
            role::Role::USER,
        );

        let registered_user = self
            .user_repository
            .save(&new_user)
            .await
            .map_err(|_| RegisterError::DbInternalError)?;

        Ok(registered_user)
    }

    async fn login(&self, input: &UserLoginRequest) -> Result<User, LoginError> {
        let found_user = self
            .user_repository
            .find_by_email(input.email.as_str())
            .await
            .map_err(|_| LoginError::DbInternalError)?;

        let found_user = match found_user {
            Some(user) => user,
            None => return Err(LoginError::UserNotFound),
        };

        match found_user.password_hash.verify_password(&input.password) {
            Ok(true) => Ok(found_user),
            Ok(false) => Err(LoginError::BadCredentials),
            Err(_) => Err(LoginError::JWTEncodingError),
        }
    }

    async fn me(&self, email: &str) -> Result<User, MeError> {
        let user = self
            .user_repository
            .find_by_email(email)
            .await
            .map_err(|_| MeError::DbInternalError)?
            .ok_or(MeError::UserNotFound)?;
        Ok(user)
    }

    // async fn update_profile(&self, input: &UpdateUserPofileInput) -> Result<(), Error> {
    //     todo!()
    // }
    //
}

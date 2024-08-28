use std::sync::Arc;

use async_trait::async_trait;
use validator::ValidationErrors;

use crate::adapter::driving::presentation::http::handler::auth::login::{
    UserLoginRequest, UserLoginResponse,
};
use crate::adapter::driving::presentation::http::handler::auth::me::UserMeResponse;
use crate::adapter::driving::presentation::http::handler::auth::register::{
    UserRegisterRequest, UserRegisterResponse,
};
use crate::core::application::usecase::auth::error::{LoginError, MeError, RegisterError};
use crate::core::domain::entity::user::User;
use crate::core::domain::valueobject::role;
use crate::core::port::auth::TokenMaker;
use crate::core::port::user::{UserManagement, UserRepo};

#[derive(Debug, Clone)]
pub struct UserService<T, K>
where
    T: TokenMaker,
    K: UserRepo,
{
    token_handler: Arc<T>,
    user_repository: Arc<K>,
}

impl<T, K> UserService<T, K>
where
    T: TokenMaker,
    K: UserRepo,
{
    pub fn new(token_handler: Arc<T>, user_repository: Arc<K>) -> Self {
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
    K: UserRepo,
{
    async fn register(
        &self,
        input: &UserRegisterRequest,
    ) -> Result<UserRegisterResponse, RegisterError<ValidationErrors>> {
        let found_user = self
            .user_repository
            .find_by_email(input.email.as_str())
            .await
            .map_err(|_| RegisterError::DbInternalError)?;

        if found_user.is_some() {
            return Err(RegisterError::UserAlreadyRegistered);
        }

        let new_user = User::new(
            input.first_name.clone(),
            input.last_name.clone(),
            input.email.clone(),
            input.password.clone(),
            role::Role::User,
        );

        let registered_id = self
            .user_repository
            .save(&new_user)
            .await
            .map_err(|_| RegisterError::DbInternalError)?;
        let register_response = UserRegisterResponse {
            uuid: registered_id.to_string(),
        };
        Ok(register_response)
    }
    async fn login(&self, input: &UserLoginRequest) -> Result<UserLoginResponse, LoginError> {
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
            Ok(true) => {
                let access_token = self.token_handler.generate_token(&found_user).await;
                let result = UserLoginResponse { access_token };
                Ok(result)
            }
            Ok(false) => Err(LoginError::BadCredentials),
            Err(_) => Err(LoginError::JWTEncodingError),
        }
    }

    async fn me(&self, token: &str) -> Result<UserMeResponse, MeError> {
        let decode_token = self
            .token_handler
            .decode_token(token)
            .map_err(|_| MeError::InvalidIdFormat)?;

        let user_id_str = decode_token.sub;
        let user = self
            .user_repository
            .find_by_id(user_id_str.as_str())
            .await
            .map_err(|_| MeError::DbInternalError)?
            .ok_or(MeError::UserNotFound)?;
        let user_me = UserMeResponse { user };
        Ok(user_me)
    }
    // async fn update_profile(&self, input: &UpdateUserPofileInput) -> Result<(), Error> {
    //     todo!()
    // }
    //
}

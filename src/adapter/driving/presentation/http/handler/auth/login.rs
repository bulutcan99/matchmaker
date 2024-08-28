use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use http::StatusCode;
use serde::Serialize;
use serde_derive::Deserialize;
use tower_cookies::Cookies;

use crate::adapter::driving::presentation::http::handler::auth::auth_handler::AuthHandler;
use crate::adapter::driving::presentation::http::response::field_error::ResponseError;
use crate::adapter::driving::presentation::http::response::response::{
    ApiResponse, ApiResponseData,
};
use crate::core::application::usecase::auth::error::LoginError;
use crate::core::port::user::UserManagement;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLoginResponse {
    pub access_token: String,
}
impl<E> From<LoginError> for ApiResponseData<E>
where
    E: Serialize + 'static,
{
    fn from(value: LoginError) -> Self {
        match value {
            LoginError::UserNotFound => {
                ApiResponseData::error(None, "user not found", StatusCode::NOT_ACCEPTABLE)
            }
            LoginError::BadCredentials => {
                ApiResponseData::error(None, "bad credentials", StatusCode::FORBIDDEN)
            }
            LoginError::UserProviderNotValid => {
                ApiResponseData::error(None, "bad provider", StatusCode::BAD_REQUEST)
            }
            LoginError::DbInternalError | LoginError::JWTEncodingError => {
                ApiResponseData::error(None, "internal error", StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

impl<S> AuthHandler<S>
where
    S: UserManagement,
{
    pub async fn login(
        login_user: Json<UserLoginRequest>,
        State(user_service): State<Arc<S>>,
        cookies: Cookies,
    ) -> ApiResponse<UserLoginResponse, ResponseError> {
        let result = user_service.login(&login_user).await;
        match result {
            Ok(response_data) => Ok(ApiResponseData::success_with_data(
                response_data,
                StatusCode::OK,
            )),
            Err(error) => Err(ApiResponseData::from(error)),
        }
    }
}

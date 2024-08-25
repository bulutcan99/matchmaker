use axum::Json;
use http::StatusCode;
use serde::Serialize;
use serde_derive::Deserialize;

use crate::adapter::driving::presentation::http::handler::auth::auth_handler::AuthHandler;
use crate::adapter::driving::presentation::http::response::field_error::ResponseError;
use crate::adapter::driving::presentation::http::response::response::{
    ApiResponse, ApiResponseData,
};
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

#[derive(Deserialize, Debug, Clone)]
pub enum ApiError {
    UserNotFound,
    BadCredentials,
    UserProviderNotValid,
    InternalError,
    JWTEncodingError,
}

impl<E> From<ApiError> for ApiResponseData<E>
where
    E: Serialize + 'static,
{
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::UserNotFound => {
                ApiResponseData::error(None, "user not found", StatusCode::NOT_ACCEPTABLE)
            }
            ApiError::BadCredentials => ApiResponseData::status_code(StatusCode::FORBIDDEN),
            ApiError::UserProviderNotValid => {
                ApiResponseData::error(None, "bad provider", StatusCode::BAD_REQUEST)
            }
            ApiError::InternalError | ApiError::JWTEncodingError => {
                ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

impl<S> AuthHandler<S>
where
    S: UserManagement,
{
    pub async fn login(
        &self,
        login_user: Json<UserLoginRequest>,
    ) -> ApiResponse<UserLoginResponse, ResponseError> {
        let result = self.user_service.login(&login_user).await;
        match result {
            Ok(response_data) => Ok(ApiResponseData::success_with_data(
                response_data,
                StatusCode::OK,
            )),
            Err(_) => Err(ApiResponseData::status_code(
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }
}

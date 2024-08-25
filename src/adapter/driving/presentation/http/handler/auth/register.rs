use axum::Json;
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::adapter::driving::presentation::http::handler::auth::auth_handler::AuthHandler;
use crate::adapter::driving::presentation::http::response::field_error::ResponseError;
use crate::adapter::driving::presentation::http::response::response::{
    ApiResponse, ApiResponseData,
};
use crate::core::port::user::UserManagement;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserRegisterRequest {
    #[serde(default)]
    #[validate(length(
        min = 3,
        max = 20,
        message = "First name is not valid. It should be between 3 and 20 characters."
    ))]
    pub first_name: String,

    #[serde(default)]
    #[validate(length(
        min = 3,
        max = 20,
        message = "Last name is not valid. It should be between 3 and 20 characters."
    ))]
    pub last_name: String,

    #[serde(default)]
    #[validate(email(message = "Email address is not valid."))]
    pub email: String,

    #[serde(default)]
    #[validate(length(
        min = 8,
        message = "Password is not valid. It should be at least 8 characters."
    ))]
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct UserRegisterResponse {
    pub uuid: String,
}

#[derive(Debug)]
pub enum ApiError {
    BadClientData(ValidationErrors),
    UserAlreadyRegistered,
    DbInternalError,
    HashingError,
}

impl From<ApiError> for ApiResponseData<ResponseError> {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::BadClientData(err) => ApiResponseData::error(
                Some(ResponseError::from(err)),
                "invalid data from client",
                StatusCode::BAD_REQUEST,
            ),
            ApiError::UserAlreadyRegistered => {
                ApiResponseData::error(None, "user already registered", StatusCode::FORBIDDEN)
            }
            ApiError::DbInternalError | ApiError::HashingError => {
                ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

impl<S> AuthHandler<S>
where
    S: UserManagement,
{
    pub async fn register(
        &self,
        register_user: Json<UserRegisterRequest>,
    ) -> ApiResponse<UserRegisterResponse, ResponseError> {
        register_user.validate().map_err(ApiError::BadClientData)?;
        let result = self.user_service.register(&register_user).await;
        match result {
            Ok(registered_user) => Ok(ApiResponseData::success_with_data(
                registered_user,
                StatusCode::OK,
            )),
            Err(_) => Err(ApiResponseData::status_code(
                StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    }
}

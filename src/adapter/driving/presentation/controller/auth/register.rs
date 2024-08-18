use axum::Json;
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::adapter::driving::presentation::controller::auth::auth_handler::AuthHandler;
use crate::adapter::driving::presentation::field_error::ResponseError;
use crate::adapter::driving::presentation::response::{ApiResponse, ApiResponseData};
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
pub struct RegisterResponseObject {
    pub uuid: String,
}

#[derive(Debug)]
pub enum ApiError {
    BadClientData(ValidationErrors),
    UserAlreadyRegistered,
    DbInternalError,
    HashingError,
    JWTEncodingError,
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
            ApiError::DbInternalError | ApiError::HashingError | ApiError::JWTEncodingError => {
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
        user_register: Json<UserRegisterRequest>,
    ) -> ApiResponse<RegisterResponseObject, ResponseError> {
    }
}

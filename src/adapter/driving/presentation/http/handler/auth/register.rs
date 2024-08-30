use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::adapter::driving::presentation::http::response::field_error::ResponseError;
use crate::adapter::driving::presentation::http::response::response::{
    ApiResponse, ApiResponseData,
};
use crate::core::application::usecase::auth::error::RegisterError;
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
    pub user_id: Uuid,
}

impl From<RegisterError<ValidationErrors>> for ApiResponseData<ResponseError> {
    fn from(value: RegisterError<ValidationErrors>) -> Self {
        match value {
            RegisterError::BadClientData(err) => ApiResponseData::error(
                Some(ResponseError::from(err)),
                "invalid data from client",
                StatusCode::BAD_REQUEST,
            ),
            RegisterError::UserAlreadyRegistered => {
                ApiResponseData::error(None, "user already registered", StatusCode::FORBIDDEN)
            }
            RegisterError::DbInternalError | RegisterError::HashingError => {
                ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

pub async fn register_handler<S>(
    State(user_service): State<Arc<S>>,
    register_user: Json<UserRegisterRequest>,
) -> ApiResponse<UserRegisterResponse, ResponseError>
where
    S: UserManagement,
{
    register_user
        .validate()
        .map_err(RegisterError::BadClientData)?;

    let result = user_service.register(&register_user).await;
    match result {
        Ok(registered_user) => {
            let res = UserRegisterResponse {
                user_id: registered_user,
            };
            Ok(ApiResponseData::success_with_data(res, StatusCode::OK))
        }
        Err(error) => Err(ApiResponseData::from(error)),
    }
}

use axum::Json;
use axum::response::IntoResponse;
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::adapter::driving::presentation::controller::auth::auth_handler::AuthHandler;
use crate::adapter::driving::presentation::field_error::ResponseError;
use crate::adapter::driving::presentation::response::ApiResponseData;
use crate::core::application::usecase::user::dto::UserRegisterInput;
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
    pub async fn register(&self, payload: Json<UserRegisterRequest>) -> impl IntoResponse {
        if let Err(e) = payload.validate() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error(Some(e.to_string()))),
            ));
        }

        let result = self
            .user_service
            .register(&UserRegisterInput {
                email: payload.email.clone(),
                first_name: payload.first_name.clone(),
                last_name: payload.last_name.clone(),
                password: payload.password.clone(),
            })
            .await;

        if let Some(user_id) = result {
            return Ok((
                StatusCode::OK,
                Json(ApiResponse::success(
                    Some("You have successfully registered.".to_string()),
                    Some(user_id),
                )),
            ));
        }

        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(Some("Registration failed".to_string()))),
        ))
    }
}

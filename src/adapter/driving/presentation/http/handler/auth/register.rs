use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use http::StatusCode;
use log::error;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;
use validator::{Validate, ValidationErrors};

use crate::adapter::driving::presentation::http::response::field_error::ResponseError;
use crate::adapter::driving::presentation::http::response::response::{
    ApiResponse, ApiResponseData,
};
use crate::adapter::driving::presentation::http::router::AppState;
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
    pub name: String,

    #[serde(default)]
    #[validate(length(
        min = 3,
        max = 20,
        message = "Last name is not valid. It should be between 3 and 20 characters."
    ))]
    pub surname: String,

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
    State(app): State<Arc<AppState<S>>>,
    register_user: Json<UserRegisterRequest>,
) -> ApiResponse<UserRegisterResponse, ResponseError>
where
    S: UserManagement,
{
    register_user
        .validate()
        .map_err(RegisterError::BadClientData)?;

    let result = app.user_service.register(&register_user).await;
    match result {
        Ok(registered_user) => {
            let email_token = VerificationToken::new();
            let email_token_string = email_token.token().to_string();

            let user_email = register_user.email.clone();

            tokio::spawn(async move {
                if let Err(e) = send_verification_email(&user_email, &email_token_string).await {
                    error!("Failed to send email: {:?}", e);
                }
            });

            let res = UserRegisterResponse {
                user_id: registered_user,
            };
            Ok(ApiResponseData::success_with_data(res, StatusCode::OK))
        }
        Err(error) => Err(ApiResponseData::from(error)),
    }
}

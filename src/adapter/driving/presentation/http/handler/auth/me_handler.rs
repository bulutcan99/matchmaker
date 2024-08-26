use erased_serde::Serialize;
use http::{HeaderMap, StatusCode};
use serde_derive::{Deserialize, Serialize};

use crate::adapter::driving::presentation::http::handler::auth::auth_handler::AuthHandler;
use crate::adapter::driving::presentation::http::response::field_error::ResponseError;
use crate::adapter::driving::presentation::http::response::response::{
    ApiResponse, ApiResponseData,
};
use crate::core::application::usecase::auth::error::MeError;
use crate::core::domain::entity::user::User;
use crate::core::port::user::UserManagement;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMeResponse {
    pub user: User,
}

impl<E> From<MeError> for ApiResponseData<E>
where
    E: Serialize + 'static,
{
    fn from(value: MeError) -> Self {
        match value {
            MeError::InvalidJwtToken | MeError::InvalidIdFormat => {
                ApiResponseData::status_code(StatusCode::NOT_ACCEPTABLE)
            }
            MeError::DbInternalError => {
                ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR)
            }
            MeError::UserNotFound => ApiResponseData::status_code(StatusCode::BAD_REQUEST),
        }
    }
}

impl<S> AuthHandler<S>
where
    S: UserManagement,
{
    pub async fn me(&self, headers: HeaderMap) -> ApiResponse<UserMeResponse, ResponseError> {
        let auth_header = match headers.get("Authorization") {
            Some(header_value) => header_value.to_str().ok(),
            None => None,
        };

        let token = match auth_header {
            Some(value) => {
                if value.starts_with("Bearer ") {
                    Some(value.trim_start_matches("Bearer ").to_string())
                } else {
                    None
                }
            }
            None => None,
        };

        let token = match token {
            Some(t) => t,
            None => {
                return ApiResponse::Err(ResponseError::Unauthorized(
                    "Invalid or missing token".into(),
                ))
            }
        };
    }
}

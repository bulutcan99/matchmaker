use http::{HeaderMap, StatusCode};
use serde_derive::{Deserialize, Serialize};

use crate::adapter::driving::presentation::http::handler::auth::auth_handler::AuthHandler;
use crate::adapter::driving::presentation::http::response::error::ApiResponseError;
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

impl From<MeError> for ApiResponseData<ResponseError> {
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
            Some(header_value) => match header_value.to_str() {
                Ok(value) => value,
                Err(_) => {
                    let error_body = ApiResponseError::simple_error("Invalid Authorization header");
                    return Err(ApiResponseData::Error {
                        error: error_body,
                        status: StatusCode::BAD_REQUEST,
                    });
                }
            },
            None => {
                let error_body = ApiResponseError::simple_error("Authorization header missing");
                return Err(ApiResponseData::Error {
                    error: error_body,
                    status: StatusCode::UNAUTHORIZED,
                });
            }
        };

        let result = self.user_service.me(auth_header).await;
        match result {
            Ok(user) => Ok(ApiResponseData::success_with_data(user, StatusCode::OK)),
            Err(error) => Err(ApiResponseData::from(error)),
        }
    }
}

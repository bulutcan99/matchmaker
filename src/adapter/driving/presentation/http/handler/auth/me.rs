use axum::Extension;
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};

use crate::adapter::driving::presentation::http::middleware::auth::ExtError;
use crate::adapter::driving::presentation::http::middleware::cookie::get_token_from_cookie;
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

impl From<User> for UserMeResponse {
    fn from(value: User) -> Self {
        UserMeResponse { user: value }
    }
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

impl From<ExtError> for ApiResponseData<ResponseError> {
    fn from(value: ExtError) -> Self {
        match value {
            ExtError::TokenNotInCookie
            | ExtError::TokenWrongFormat
            | ExtError::FailValidate
            | ExtError::CannotSetTokenCookie => {
                ApiResponseData::status_code(StatusCode::UNAUTHORIZED)
            }
            ExtError::UserNotFound => ApiResponseData::status_code(StatusCode::NOT_FOUND),
            ExtError::ModelAccessError(_) => {
                ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR)
            }
            ExtError::CtxNotInRequestExt | ExtError::CtxCreateFail(_) => {
                ApiResponseData::status_code(StatusCode::BAD_REQUEST)
            }
        }
    }
}

pub async fn me_handler<S>(
    Extension(user): Extension<User>,
) -> ApiResponse<UserMeResponse, ResponseError>
where
    S: UserManagement,
{
    Ok(ApiResponseData::success_with_data(
        user.into(),
        StatusCode::OK,
    ))
}

use std::sync::Arc;

use axum::Extension;
use axum::extract::State;
use http::StatusCode;
use serde_derive::{Deserialize, Serialize};

use crate::adapter::driving::presentation::http::middleware::auth::CtxExtError;
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

impl From<CtxExtError> for ApiResponseData<ResponseError> {
    fn from(value: CtxExtError) -> Self {
        match value {
            CtxExtError::TokenNotInCookie
            | CtxExtError::TokenWrongFormat
            | CtxExtError::FailValidate
            | CtxExtError::CannotSetTokenCookie => {
                ApiResponseData::status_code(StatusCode::UNAUTHORIZED)
            }
            CtxExtError::UserNotFound => ApiResponseData::status_code(StatusCode::NOT_FOUND),
            CtxExtError::ModelAccessError(_) => {
                ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR)
            }
            CtxExtError::CtxNotInRequestExt | CtxExtError::CtxCreateFail(_) => {
                ApiResponseData::status_code(StatusCode::BAD_REQUEST)
            }
        }
    }
}

pub async fn me_handler<S>(
    State(user_service): State<Arc<S>>,
    Extension(user): Extension<Result<User, CtxExtError>>,
) -> ApiResponse<UserMeResponse, ResponseError>
where
    S: UserManagement,
{
    match user {
        Ok(user) => Ok(ApiResponseData::success_with_data(
            user.into(),
            StatusCode::OK,
        )),
        Err(error) => Err(ApiResponseData::from(error)),
    }
}

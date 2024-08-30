use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use http::StatusCode;
use serde::Serialize;
use serde_derive::Deserialize;
use tower_cookies::Cookies;
use uuid::Uuid;

use crate::adapter::driving::presentation::http::middleware::cookie::set_token_cookie;
use crate::adapter::driving::presentation::http::response::field_error::ResponseError;
use crate::adapter::driving::presentation::http::response::response::{
	ApiResponse, ApiResponseData,
};
use crate::core::application::usecase::auth::error::{LoginError, TokenError};
use crate::core::port::user::UserManagement;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLoginResponse {
    pub user_id: Uuid,
}

impl<E> From<LoginError> for ApiResponseData<E>
where
    E: Serialize + 'static,
{
    fn from(value: LoginError) -> Self {
        match value {
            LoginError::UserNotFound => {
                ApiResponseData::error(None, "user not found", StatusCode::NOT_ACCEPTABLE)
            }
            LoginError::BadCredentials => {
                ApiResponseData::error(None, "bad credentials", StatusCode::FORBIDDEN)
            }
            LoginError::UserProviderNotValid => {
                ApiResponseData::error(None, "bad provider", StatusCode::BAD_REQUEST)
            }
            LoginError::DbInternalError | LoginError::JWTEncodingError => {
                ApiResponseData::error(None, "internal error", StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

impl From<TokenError> for ResponseError {
    fn from(value: TokenError) -> Self {
        match value {
            TokenError::HmacFailNewFromSlice => ResponseError::InternalError,
            TokenError::InvalidFormat => ResponseError::BadRequest,
            TokenError::CannotDecodeIdent => ResponseError::BadRequest,
            TokenError::CannotDecodeIat => ResponseError::BadRequest,
            TokenError::CannotDecodeExp => ResponseError::BadRequest,
            TokenError::SignatureNotMatching => ResponseError::Unauthorized,
            TokenError::ExpNotIso => ResponseError::BadRequest,
            TokenError::Expired => ResponseError::Unauthorized,
        }
    }
}

pub async fn login_handler<S>(
    State(user_service): State<Arc<S>>,
    cookies: Cookies,
    login_user: Json<UserLoginRequest>,
) -> ApiResponse<UserLoginResponse, ResponseError>
where
    S: UserManagement,
{
    let result = user_service.login(&login_user).await;
    match result {
        Ok(response_data) => {
            let user_id = response_data;
            match set_token_cookie(&cookies, &login_user.email, &user_id) {
                Ok(()) => {
                    let res = UserLoginResponse {
                        user_id: response_data,
                    };
                    Ok(ApiResponseData::success_with_data(res, StatusCode::OK))
                }
                Err(error) => {
                    let api_error = ResponseError::from(error);
                    Err(ApiResponseData::Error {
                        error: api_error,
                        status: StatusCode::NOT_FOUND,
                    })
                }
            }
        }
        Err(error) => {
            let api_error = ResponseError::from(error);
            Err(ApiResponseData::error_with_status(
                api_error,
                StatusCode::UNAUTHORIZED,
            ))
        }
    }
}

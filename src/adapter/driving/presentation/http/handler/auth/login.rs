use std::sync::Arc;

use axum::extract::State;
use axum::Json;
use http::StatusCode;
use serde::Serialize;
use serde_derive::Deserialize;
use tower_cookies::Cookies;

use crate::adapter::driving::presentation::http::middleware::cookie::set_token_cookie;
use crate::adapter::driving::presentation::http::response::field_error::ResponseError;
use crate::adapter::driving::presentation::http::response::response::{
    ApiResponse, ApiResponseData,
};
use crate::adapter::driving::presentation::http::router::AppState;
use crate::core::application::usecase::auth::error::{LoginError, TokenError};
use crate::core::port::user::UserManagement;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserLoginResponse {
    pub token: String,
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

impl<E> From<TokenError> for ApiResponseData<E>
where
    E: Serialize + 'static,
{
    fn from(value: TokenError) -> Self {
        match value {
            TokenError::HmacFailNewFromSlice => {
                ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR)
            }
            TokenError::InvalidFormat => ApiResponseData::status_code(StatusCode::BAD_REQUEST),
            TokenError::CannotDecodeIdent => ApiResponseData::status_code(StatusCode::BAD_REQUEST),
            TokenError::CannotDecodeIat => ApiResponseData::status_code(StatusCode::BAD_REQUEST),
            TokenError::CannotDecodeExp => ApiResponseData::status_code(StatusCode::BAD_REQUEST),
            TokenError::SignatureNotMatching => {
                ApiResponseData::status_code(StatusCode::UNAUTHORIZED)
            }
            TokenError::ExpNotIso => ApiResponseData::status_code(StatusCode::BAD_REQUEST),
            TokenError::Expired => ApiResponseData::status_code(StatusCode::UNAUTHORIZED),
        }
    }
}

impl<E> From<anyhow::Error> for ApiResponseData<E>
where
    E: Serialize + 'static,
{
    fn from(error: anyhow::Error) -> Self {
        tracing::error!("Unhandled error: {:?}", error);

        ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

pub async fn login_handler<S>(
    State(app): State<Arc<AppState<S>>>,
    cookies: Cookies,
    login_user: Json<UserLoginRequest>,
) -> ApiResponse<UserLoginResponse, ResponseError>
where
    S: UserManagement,
{
    let result = app.user_service.login(&login_user).await;

    match result {
        Ok(user) => match set_token_cookie(&cookies, &login_user.email, user.id.unwrap()) {
            Ok(()) => Ok(ApiResponseData::status_code(StatusCode::OK)),
            Err(error) => Err(ApiResponseData::from(error)),
        },
        Err(error) => Err(ApiResponseData::from(error)),
    }
}

use erased_serde::Serialize;
use http::StatusCode;
use axum::{
    extract::{Request, Json, Path, Extension, Query},
    routing::post,
    http::header::HeaderMap,
    body::{Bytes, Body},
    Router,
};

use serde_derive::{Deserialize, Serialize};
use crate::adapter::driving::presentation::http::handler::auth::auth_handler::AuthHandler;
use crate::adapter::driving::presentation::http::response::response::ApiResponseData;
use crate::core::application::usecase::auth::error::MeError;
use crate::core::domain::entity::user::User;
use crate::core::port::user::UserManagement;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeResponse {
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
    pub async fn me(
        &self,
            TypedHeader(token): TypedHeader<Authorization<Bearer>>,
    )
}
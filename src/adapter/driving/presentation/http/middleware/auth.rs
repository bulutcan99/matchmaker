use std::fmt::Display;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Request, State};
use axum::Json;
use axum::middleware::Next;
use axum::response::IntoResponse;
use http::{header, StatusCode};
use serde_derive::Serialize;
use tower_cookies::cookie::CookieJar;

use crate::adapter::driving::presentation::http::middleware::cookie::AUTH_TOKEN;
use crate::adapter::driving::presentation::http::router::AppState;
use crate::core::application::usecase::auth::token::Token;
use crate::core::port::auth::TokenMaker;
use crate::core::port::user::UserManagement;

#[derive(Clone, Serialize, Debug)]
pub enum ExtError {
    TokenNotInCookie,
    TokenWrongFormat,
    UserNotFound,
    ModelAccessError(String),
    FailValidate,
    CannotSetTokenCookie,
    CtxNotInRequestExt,
    CtxCreateFail(String),
}

pub async fn is_authenticated<S>(
    cookie_jar: CookieJar,
    State(app): State<Arc<AppState<S>>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, Json<ExtError>)>
where
    S: UserManagement + 'static,
{
    let token_str = cookie_jar
        .get(AUTH_TOKEN)
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .filter(|auth_value| auth_value.starts_with("Bearer "))
                .map(|auth_value| auth_value.trim_start_matches("Bearer ").to_owned())
        })
        .ok_or((StatusCode::UNAUTHORIZED, Json(ExtError::TokenNotInCookie)))?;

    let token: Token = token_str
        .parse()
        .map_err(|_| (StatusCode::BAD_REQUEST, Json(ExtError::TokenWrongFormat)))?;

    let user = app.user_service.me(&token.ident).await.map_err(|ex| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ExtError::ModelAccessError(ex.to_string())),
        )
    })?;

    Token::validate_token(
        &token,
        &user
            .id
            .ok_or((StatusCode::NOT_FOUND, Json(ExtError::UserNotFound)))?,
    )
    .map_err(|_| (StatusCode::UNAUTHORIZED, Json(ExtError::FailValidate)))?;

    req.extensions_mut().insert(user);
    let response = next.run(req).await;
    Ok(response)
}

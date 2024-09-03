use std::fmt::Display;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use http::{header, StatusCode};
use serde_derive::Serialize;
use tower_cookies::cookie::CookieJar;

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

pub async fn is_authenticated<S, B>(
    cookie_jar: CookieJar,
    State(app): State<Arc<AppState<S>>>,
    mut req: Request<B>,
    next: Next,
) -> Result<Response<Body>, (StatusCode, String)>
where
    S: UserManagement + Clone + 'static,
{
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .filter(|auth_value| auth_value.starts_with("Bearer "))
                .map(|auth_value| auth_value.trim_start_matches("Bearer ").to_owned())
        });

    if let Some(token_str) = token {
        let token: Token = token_str.parse().map_err(|_| ExtError::TokenWrongFormat)?;
        let user = app
            .user_service
            .me(&token.ident)
            .await
            .map_err(|ex| ExtError::ModelAccessError(ex.to_string()))?;

        Token::validate_token(&token, &user.id.unwrap()).map_err(|_| ExtError::FailValidate)?;

        req.extensions_mut().insert(user);
        Ok(next.run(req).await)
    }
}

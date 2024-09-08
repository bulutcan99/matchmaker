use std::convert::Infallible;
use std::fmt::Display;
use std::sync::Arc;

use axum::body::Body;
use axum::extract::{Request, State};
use axum::Json;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum_extra::extract::cookie::Cookie;
use http::{header, StatusCode};
use log::debug;
use serde_derive::Serialize;
use tower_cookies::cookie::CookieJar;
use tower_cookies::Cookies;

use crate::adapter::driving::presentation::http::middleware::cookie::{
    AUTH_TOKEN, set_token_cookie,
};
use crate::adapter::driving::presentation::http::router::AppState;
use crate::core::application::usecase::auth::token::Token;
use crate::core::port::auth::TokenMaker;
use crate::core::port::user::UserManagement;

#[derive(Clone, Serialize, Debug)]
pub enum ExtError {
    TokenNotInCookieOrHeader,
    TokenWrongFormat,
    UserNotFound,
    ModelAccessError(String),
    FailValidate,
    CannotSetTokenCookie,
    CtxNotInRequestExt,
    CtxCreateFail(String),
}

pub async fn is_authenticated<S>(
    State(app): State<Arc<AppState<S>>>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, Infallible>
where
    S: UserManagement + 'static,
{
    debug!("{:<12} - mw_ctx_resolve", "MIDDLEWARE");

    let ctx_ext_result = ctx_resolve(app, &cookies).await;

    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(ExtError::TokenNotInCookieOrHeader))
    {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    req.extensions_mut().insert(ctx_ext_result);

    Ok(next.run(req).await)
}

async fn ctx_resolve<S>(app_state: Arc<AppState<S>>, cookies: &Cookies) -> Result<(), ExtError>
where
    S: UserManagement + 'static,
{
    let token = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(ExtError::TokenNotInCookieOrHeader)?;
    let token: Token = token.parse().map_err(|_| ExtError::TokenWrongFormat)?;

    let user = app_state
        .user_service
        .me(&token.ident)
        .await
        .map_err(|_| ExtError::UserNotFound)?;
    Token::validate_token(&token, &user.id.unwrap()).map_err(|_| ExtError::FailValidate)?;
    set_token_cookie(cookies, &user.email, &user.id.unwrap())
        .map_err(|_| ExtError::CannotSetTokenCookie)?;

    Ok(())
}

use std::fmt::Display;

use axum::body::Body;
use axum::extract::{Request, State};
use axum::middleware::Next;
use axum::response::Response;
use http::StatusCode;
use serde_derive::Serialize;
use tower_cookies::{Cookie, Cookies};

use crate::adapter::driving::presentation::http::middleware::cookie::set_token_cookie;
use crate::core::application::usecase::auth::token::Token;
use crate::core::domain::entity::user::User;
use crate::core::port::auth::TokenMaker;
use crate::core::port::user::UserManagement;

pub const AUTH_TOKEN: &str = "auth-token";

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
    State(user_service): State<S>,
    cookies: Cookies,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, (StatusCode, String)>
where
    S: UserManagement + Clone + Send + Sync + 'static,
{
    let ctx_ext_result = resolver(State(user_service.clone()), cookies.clone()).await;

    if let Err(ref err) = ctx_ext_result {
        if !matches!(err, ExtError::TokenNotInCookie) {
            cookies.remove(Cookie::from(AUTH_TOKEN));
        }
    }

    req.extensions_mut().insert(ctx_ext_result);

    Ok(next.run(req).await)
}

pub async fn resolver<S>(State(user_service): State<S>, cookies: Cookies) -> Result<User, ExtError>
where
    S: UserManagement,
{
    let token_str = cookies
        .get(AUTH_TOKEN)
        .map(|c| c.value().to_string())
        .ok_or(ExtError::TokenNotInCookie)?;

    let token: Token = token_str.parse().map_err(|_| ExtError::TokenWrongFormat)?;

    let user = user_service
        .me(&token.ident)
        .await
        .map_err(|ex| ExtError::ModelAccessError(ex.to_string()))?;

    Token::validate_token(&token, &user.id.unwrap()).map_err(|_| ExtError::FailValidate)?;

    set_token_cookie(&cookies, &user.email, &user.id.unwrap())
        .map_err(|_| ExtError::CannotSetTokenCookie)?;

    Ok(user)
}

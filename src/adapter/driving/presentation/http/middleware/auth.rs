use std::convert::Infallible;
use std::fmt::Debug;
use std::sync::Arc;

use axum::body::Body;
use axum::http::Request;

use axum::extract::State;
use axum::middleware::Next;
use axum::response::Response;
use axum::Extension;
use axum_extra::extract::cookie::Cookie;
use serde_derive::Serialize;
use tower_cookies::Cookies;

use crate::adapter::driving::presentation::http::middleware::cookie::{
    set_token_cookie, AUTH_TOKEN,
};
use crate::adapter::driving::presentation::http::router::AppState;
use crate::core::application::usecase::auth::token::{validate_web_token, Token};
use crate::core::domain::entity::user::User;
use crate::core::port::user::UserManagement;

// pub async fn is_verified<S>(
//     State(app): State<Arc<AppState<S>>>,
//     mut req: Request<Body>,
//     next: Next,
// ) -> Result<Response, Infallible>
// where
//     S: UserManagement + 'static,
// {
//     let app_extension = Extension(app);
//     let user = app
//         .user_service
//         .me(req.body("email"))
//         .await
//         .map_err(|_| ExtError::UserNotFound)?;
//     if let Ok(user) = ctx_ext_result {
//         // Insert the authenticated user into request extensions
//         req.extensions_mut().insert(user);
//     } else {
//         req.extensions_mut().insert(ctx_ext_result);
//     }
//
//     Ok(next.run(req).await)
// }

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
    let app_extension = Extension(app);
    let ctx_ext_result = ctx_resolve(app_extension, &cookies).await;

    if ctx_ext_result.is_err() && !matches!(ctx_ext_result, Err(ExtError::TokenNotInCookieOrHeader))
    {
        cookies.remove(Cookie::from(AUTH_TOKEN));
    }

    if let Ok(user) = ctx_ext_result {
        // Insert the authenticated user into request extensions
        req.extensions_mut().insert(user);
    } else {
        req.extensions_mut().insert(ctx_ext_result);
    }

    Ok(next.run(req).await)
}

async fn ctx_resolve<S>(
    Extension(app_state): Extension<Arc<AppState<S>>>,
    cookies: &Cookies,
) -> Result<User, ExtError>
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

    validate_web_token(&token, user.id.unwrap()).map_err(|_| ExtError::FailValidate)?;

    set_token_cookie(&cookies, &user.email, user.id.unwrap())
        .map_err(|_| ExtError::CannotSetTokenCookie)?;

    Ok(user)
}

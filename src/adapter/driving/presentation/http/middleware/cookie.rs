use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::core::application::usecase::auth::error::TokenError;
use crate::core::application::usecase::auth::token::Token;
use crate::core::port::auth::TokenMaker;

pub const AUTH_TOKEN: &str = "auth-token";

pub fn set_token_cookie(cookies: &Cookies, user: &str, salt: &Uuid) -> Result<(), TokenError> {
    let token = Token::generate_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");

    cookies.add(cookie);

    Ok(())
}

pub fn get_token_from_cookie(cookies: &Cookies) -> Result<Token, TokenError> {
    let token_str = cookies
        .get(AUTH_TOKEN)
        .map(|cookie| cookie.value().to_string())
        .ok_or(TokenError::InvalidFormat)?;

    let token = token_str
        .parse::<Token>()
        .map_err(|_| TokenError::SignatureNotMatching)?;

    Ok(token)
}

pub fn remove_token_cookie(cookies: &Cookies) -> Result<(), TokenError> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}

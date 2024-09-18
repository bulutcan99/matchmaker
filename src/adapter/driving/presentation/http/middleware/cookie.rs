use crate::core::application::usecase::auth::error::TokenError;
use crate::core::application::usecase::auth::token::generate_web_token;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub const AUTH_TOKEN: &str = "token";

pub fn set_token_cookie(cookies: &Cookies, user: &str, salt: Uuid) -> Result<(), TokenError> {
    let token = generate_web_token(user, salt)?;

    let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
    cookie.set_http_only(true);
    cookie.set_path("/");
    cookie.set_max_age(time::Duration::seconds(604800));

    cookies.add(cookie);

    Ok(())
}

pub fn remove_token_cookie(cookies: &Cookies) -> Result<(), TokenError> {
    let mut cookie = Cookie::from(AUTH_TOKEN);
    cookie.set_path("/");

    cookies.remove(cookie);

    Ok(())
}

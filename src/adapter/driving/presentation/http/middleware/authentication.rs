use std::fmt::Display;

use axum::{http::StatusCode, middleware::Next, response::Response};
use axum::body::Body;
use axum::extract::State;
use tower_cookies::Cookies;

use crate::core::port::user::UserManagement;

pub async fn is_authorized<S>(
    State(user_service): State<S>,
    cookies: Cookies,
    mut req: http::Request<Body>,
    next: Next,
) -> Result<Response<Body>, (StatusCode, String)>
where
    S: UserManagement,
{
    let auth_header = match headers.get("Authorization") {
        Some(header_value) => match header_value.to_str() {
            Ok(value) => value,
            Err(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Invalid Authorization header".to_string(),
                ));
            }
        },
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Authorization header missing".to_string(),
            ));
        }
    };

    let token = auth_header.trim_start_matches("Bearer ");
    match user_service.me(token).await {
        Ok(user) => {
            println!("Adding user to request: {:?}", user);
            req.extensions_mut().insert(user);
            Ok(next.run(req).await)
        }
        Err(error) => Err((StatusCode::INTERNAL_SERVER_ERROR, error.to_string())),
    }
}

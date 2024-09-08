use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use axum::Router;
use axum::routing::{get, post};

use crate::adapter::driving::presentation::http::handler::_default::health_check_handler::health_checker_handler;
use crate::adapter::driving::presentation::http::handler::auth::login::login_handler;
use crate::adapter::driving::presentation::http::handler::auth::me::me_handler;
use crate::adapter::driving::presentation::http::handler::auth::register::register_handler;
use crate::adapter::driving::presentation::http::middleware::auth::is_authenticated;
use crate::core::port::user::UserManagement;

pub struct AppState<S>
where
    S: UserManagement + 'static,
{
    pub user_service: Arc<S>,
}

impl<S> AppState<S>
where
    S: UserManagement + 'static,
{
    pub fn new(user_service: Arc<S>) -> Self {
        Self { user_service }
    }
}

pub fn make_router<S>(app_state: Arc<AppState<S>>) -> Router
where
    S: UserManagement + 'static,
{
    let protected_routes = Router::new().route(
        "/api/users/me",
        get(me_handler).layer(from_fn_with_state(app_state.clone(), is_authenticated)),
    );

    // Public routes
    let public_routes = Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/login", post(login_handler));

    // Merge public and protected routes under a single router and apply the Extension layer at the top level
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .with_state(app_state)
}

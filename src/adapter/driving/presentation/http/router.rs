use std::sync::Arc;

use axum::middleware::from_fn_with_state;
use axum::routing::{get, post};
use axum::Router;
use tower_cookies::CookieManagerLayer;

use crate::adapter::driving::presentation::http::handler::_default::health_check_handler::health_checker_handler;
use crate::adapter::driving::presentation::http::handler::auth;
use crate::adapter::driving::presentation::http::handler::auth::login::login_handler;
use crate::adapter::driving::presentation::http::handler::auth::me::me_handler;
use crate::adapter::driving::presentation::http::handler::auth::register::register_handler;
use crate::adapter::driving::presentation::http::middleware::auth::is_authenticated;
use crate::core::port::user::UserManagement;
use crate::shared::worker::service::TaskContext;

pub struct AppState<S>
where
    S: UserManagement + 'static,
{
    pub user_service: Arc<S>,
    pub task_context: TaskContext,
}

impl<S> AppState<S>
where
    S: UserManagement + 'static,
{
    pub fn new(user_service: Arc<S>, task_context: TaskContext) -> Self {
        Self {
            user_service,
            task_context,
        }
    }
}

pub fn make_router<S>(app_state: Arc<AppState<S>>) -> Router
where
    S: UserManagement + 'static,
{
    let protected_routes = Router::new()
        .route(
            "/api/v1/users/me",
            get(me_handler).layer(from_fn_with_state(app_state.clone(), is_authenticated)),
        )
        .route(
            "/api/v1/companies/register",
            post(register_handler).layer(from_fn_with_state(app_state.clone(), is_authenticated)),
        );

    let public_routes = Router::new()
        .route("/api/v1/healthchecker", get(health_checker_handler))
        .route(
            "/api/v1/auth/register",
            post(auth::register::register_handler),
        )
        .route("/api/v1/auth/login", post(login_handler));

    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
        .layer(CookieManagerLayer::new())
        .with_state(app_state)
}

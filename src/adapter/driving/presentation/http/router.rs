use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};

use crate::adapter::driving::presentation::http::handler::auth::login::login_handler;
use crate::adapter::driving::presentation::http::handler::auth::me::me_handler;
use crate::adapter::driving::presentation::http::handler::auth::register::register_handler;
use crate::adapter::driving::presentation::http::middleware::auth;
use crate::core::port::user::UserManagement;

pub struct AppState<S>
where
    S: UserManagement + 'static,
{
    pub user_service: Arc<S>,
}

pub fn make_router<S>(user_service: Arc<S>) -> Router
where
    S: UserManagement + 'static,
{
    let state = Arc::new(AppState { user_service });

    Router::new()
        .merge(public_routes(state.clone()))
        .nest("/api", protected_routes(state))
}

pub fn public_routes<S>(state: Arc<AppState<S>>) -> Router
where
    S: UserManagement + 'static,
{
    Router::new()
        .route(
            "/register",
            post(register_handler).with_state(state.user_service.clone()),
        )
        .route(
            "/login",
            post(login_handler).with_state(state.user_service.clone()),
        )
}

pub fn protected_routes<S>(state: Arc<AppState<S>>) -> Router
where
    S: UserManagement + Clone + Send + Sync + 'static,
{
    Router::new()
        .route("/me", get(me_handler))
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            auth::is_authenticated,
        ))
        .with_state(state)
}

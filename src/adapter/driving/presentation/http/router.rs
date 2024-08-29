use std::sync::Arc;

use axum::Router;
use axum::routing::post;

use crate::adapter::driving::presentation::http::handler::auth::login::login;
use crate::adapter::driving::presentation::http::handler::auth::register::register;
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

    Router::new().merge(public_routes(state.clone()))
    // .nest("/api", protected_routes(state))
}

pub fn public_routes<S>(state: Arc<AppState<S>>) -> Router
where
    S: UserManagement + 'static,
{
    Router::new()
        .route(
            "/register",
            post(register).with_state(state.user_service.clone()),
        )
        .route("/login", post(login).with_state(state.user_service.clone()))
}
//
// pub fn protected_routes<S>(state: Arc<AppState<S>>) -> Router
// where
//     S: UserManagement + Clone + Send + Sync + 'static,
// {
//     Router::new()
//         .route("/me", get(me).with_state(state.clone()))
//         .layer(axum::middleware::from_fn_with_state(
//             state.clone(),
//             auth::middleware::auth_middleware,
//         ))
// }

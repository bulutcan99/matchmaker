use std::sync::Arc;

use axum::Router;
use axum::routing::{get, post};

use crate::adapter::driving::presentation::http::handler::_default::health_check_handler::health_checker_handler;
use crate::adapter::driving::presentation::http::handler::auth::login::login_handler;
use crate::adapter::driving::presentation::http::handler::auth::register::register_handler;
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
    S: UserManagement + Clone + 'static,
{
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/login", post(login_handler))
        // .route(
        //     "/api/users/me",
        //     get(me_handler).route_layer(middleware::from_fn_with_state(
        //         app_state.clone(),
        //         is_authenticated,
        //     )),
        // )
        .with_state(app_state)
}

//
// pub fn public_routes<S>(state: Arc<AppState<S>>) -> Router
// where
//     S: UserManagement + 'static,
// {
//     Router::new()
//         .route(
//             "/register",
//             post(register_handler).with_state(state.user_service.clone()),
//         )
//         .route(
//             "/login",
//             post(login_handler).with_state(state.user_service.clone()),
//         )
// }
//
// pub fn protected_routes<S>(state: Arc<AppState<S>>) -> Router
// where
//     S: UserManagement + Clone + Send + Sync + 'static,
// {
//     Router::new()
//         .route(
//             "/me",
//             get(me_handler).with_state(state.user_service.clone()),
//         )
//         .layer(middleware::from_fn_with_state(
//             state.clone(),
//             |state: State<S>, cookies: Cookies, req: Request<Body>, next: Next| async move {
//                 is_authenticated(state, cookies, req, next).await
//             },
//         ))
// }

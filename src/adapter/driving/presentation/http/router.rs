use std::sync::Arc;

use axum::{
    extract::Json,
    Router,
    routing::{get, post},
};

use crate::adapter::driving::presentation::http::handler::auth::auth_handler::AuthHandler;
use crate::adapter::driving::presentation::http::handler::auth::login::UserLoginRequest;
use crate::adapter::driving::presentation::http::handler::auth::register::UserRegisterRequest;
use crate::core::port::user::UserManagement;

pub struct Route<S>
where
    S: UserManagement + 'static,
{
    auth_handler: Arc<AuthHandler<S>>,
}

impl<S> Route<S>
where
    S: UserManagement + 'static,
{
    pub fn new(auth_handler: Arc<AuthHandler<S>>) -> Self {
        Self { auth_handler }
    }

    pub fn build(self) -> Router {
        let auth_handler = self.auth_handler.clone();

        Router::new().nest(
            "/auth",
            Router::new()
                .route(
                    "/register",
                    post({
                        let handler = auth_handler.clone();
                        move |register_user: Json<UserRegisterRequest>| {
                            let handler = handler.clone();
                            async move { handler.register(register_user).await }
                        }
                    }),
                )
                .route(
                    "/login",
                    post({
                        let handler = auth_handler.clone();
                        move |login_user: Json<UserLoginRequest>| {
                            let handler = handler.clone();
                            async move { handler.login(login_user).await }
                        }
                    }),
                )
                .route(
                    "/me",
                    get({
                        let handler = auth_handler.clone();
                        move |headers| {
                            let handler = handler.clone();
                            async move { handler.me(headers).await }
                        }
                    }),
                ),
        )
    }
}

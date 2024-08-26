use std::sync::Arc;

use axum::{Json, Router};
use axum::routing::post;

use crate::adapter::driving::presentation::http::handler::auth::auth_handler::AuthHandler;
use crate::adapter::driving::presentation::http::handler::auth::login::UserLoginRequest;
use crate::adapter::driving::presentation::http::handler::auth::register::UserRegisterRequest;
use crate::core::application::usecase::auth::service::UserService;
use crate::core::domain::entity::user::User;
use crate::core::port::auth::TokenMaker;
use crate::core::port::storage::Repo;
use crate::core::port::user::UserRepo;

pub struct Route<T, K, U>
where
    T: TokenMaker + 'static,
    K: Repo<User> + 'static,
    U: UserRepo + 'static,
{
    auth_handler: Arc<AuthHandler<UserService<T, K, U>>>,
}

impl<T, K, U> Route<T, K, U>
where
    T: TokenMaker + 'static,
    K: Repo<User> + 'static,
    U: UserRepo + 'static,
{
    pub fn new(auth_handler: Arc<AuthHandler<UserService<T, K, U>>>) -> Self {
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
                ),
        )
    }
}

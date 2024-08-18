use axum::Json;
use axum::response::IntoResponse;
use http::StatusCode;
use validator::Validate;

use crate::adapter::driving::presentation::controller::auth::dto::UserRegisterRequest;
use crate::core::application::usecase::user::dto::UserRegisterInput;
use crate::core::port::user::UserManagement;

pub struct AuthHandler<S>
where
    S: UserManagement,
{
    pub user_service: S,
}
impl<S> AuthHandler<S>
where
    S: UserManagement,
{
    pub fn new(user_management: S) -> Self {
        Self {
            user_service: user_management,
        }
    }

    pub async fn register(&self, payload: Json<UserRegisterRequest>) -> impl IntoResponse {
        if let Err(e) = payload.validate() {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error(Some(e.to_string()))),
            ));
        }

        let result = self
            .user_service
            .register(&UserRegisterInput {
                email: payload.email.clone(),
                first_name: payload.first_name.clone(),
                last_name: payload.last_name.clone(),
                password: payload.password.clone(),
            })
            .await;

        if let Some(user_id) = result {
            return Ok((
                StatusCode::OK,
                Json(ApiResponse::success(
                    Some("You have successfully registered.".to_string()),
                    Some(user_id),
                )),
            ));
        }

        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(Some("Registration failed".to_string()))),
        ))
    }
}

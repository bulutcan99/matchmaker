use http::StatusCode;
use serde_derive::{Deserialize, Serialize};

use crate::adapter::driving::presentation::http::response::field_error::ResponseError;
use crate::adapter::driving::presentation::http::response::response::ApiResponseData;
use crate::core::application::usecase::auth::error::MeError;
use crate::core::domain::entity::user::User;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserMeResponse {
    pub user: User,
}

impl From<MeError> for ApiResponseData<ResponseError> {
    fn from(value: MeError) -> Self {
        match value {
            MeError::InvalidJwtToken | MeError::InvalidIdFormat => {
                ApiResponseData::status_code(StatusCode::NOT_ACCEPTABLE)
            }
            MeError::DbInternalError => {
                ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR)
            }
            MeError::UserNotFound => ApiResponseData::status_code(StatusCode::BAD_REQUEST),
        }
    }
}

// pub async fn me_handle<S>(
//     State(user_service): State<Arc<S>>,
//     cookies: Cookies,
// ) -> ApiResponse<UserMeResponse, ResponseError>
// where
//     S: UserManagement,
// {
//     //get fron cookies
//
//     let result = user_service.me(auth_header).await;
//     match result {
//         Ok(user) => Ok(ApiResponseData::success_with_data(user, StatusCode::OK)),
//         Err(error) => Err(ApiResponseData::from(error)),
//     }
// }

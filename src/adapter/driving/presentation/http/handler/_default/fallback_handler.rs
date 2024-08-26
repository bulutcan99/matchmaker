use anyhow::anyhow;
use http::StatusCode;

use crate::adapter::driving::presentation::http::response::error::ApiResponseError;
use crate::adapter::driving::presentation::http::response::response::{
    ApiResponse, ApiResponseData,
};

pub async fn not_found_handler() -> ApiResponse<(), ApiResponseError> {
    let error_body = ApiResponseError::Complicated {
        error: Box::new(anyhow!("Not Found!")),
        message: "The requested resource could not be found.".to_string(),
    };

    Err(ApiResponseData::Error {
        error: error_body,
        status: StatusCode::NOT_FOUND,
    })
}

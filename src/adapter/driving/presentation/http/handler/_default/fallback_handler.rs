use http::StatusCode;

use crate::adapter::driving::presentation::http::response::error::ApiResponseError;
use crate::adapter::driving::presentation::http::response::field_error::ResponseError;
use crate::adapter::driving::presentation::http::response::response::{
    ApiResponse, ApiResponseData,
};

pub async fn not_found_handler() -> ApiResponse<(), ResponseError> {
    let error_body = ApiResponseError::simple_error("Not Found!");

    Err(ApiResponseData::Error {
        error: error_body,
        status: StatusCode::NOT_FOUND,
    })
}

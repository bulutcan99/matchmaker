use http::StatusCode;

use crate::adapter::driving::presentation::http::response::field_error::ResponseError;
use crate::adapter::driving::presentation::http::response::response::{
	ApiResponse, ApiResponseData,
};

pub async fn health_check_handler() -> ApiResponse<&'static str, ResponseError> {
    Ok(ApiResponseData::status_code(StatusCode::OK))
}

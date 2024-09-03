use http::StatusCode;

use crate::adapter::driving::presentation::http::response::field_error::ResponseError;
use crate::adapter::driving::presentation::http::response::response::{
    ApiResponse, ApiResponseData,
};

pub async fn health_checker_handler() -> ApiResponse<&'static str, ResponseError> {
    const MESSAGE: &str = "Health checked hahh!";
    Ok(ApiResponseData::success_with_data(MESSAGE, StatusCode::OK))
}

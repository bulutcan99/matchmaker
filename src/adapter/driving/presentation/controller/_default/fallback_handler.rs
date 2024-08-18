use axum::Json;
use axum::response::IntoResponse;
use http::StatusCode;
use serde_json::json;

pub async fn not_found_handler() -> impl IntoResponse {
    let body = Json(json!({
        "error": "Not Found",
        "message": "The requested resource could not be found."
    }));
    (StatusCode::NOT_FOUND, body)
}

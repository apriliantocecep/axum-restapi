use axum::{
    Json,
    extract::Request,
    response::IntoResponse,
    http::StatusCode,
};
use serde_json::json;

pub async fn error_404_handler(request: Request) -> impl IntoResponse {
    tracing::error!("route not found: {:?}", request);
    (StatusCode::NOT_FOUND, Json(json!({"message": "Resource not found"})))
}
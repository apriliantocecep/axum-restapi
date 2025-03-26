use axum::{response::{IntoResponse}, Json};
use serde_json::json;

pub async fn index() -> impl IntoResponse {
    Json(json!({"message": "The service is running perfectly!"}))
}
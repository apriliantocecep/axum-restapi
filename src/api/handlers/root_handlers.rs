use axum::{response::{IntoResponse}, Json};
use serde_json::json;
use crate::api::error::{ApiError};

pub async fn index() -> Result<impl IntoResponse, ApiError> {
    Ok(Json(json!({"message": "The service is running perfectly!"})))
}
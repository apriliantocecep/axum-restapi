use axum::{response::{IntoResponse}, Json};
use serde_json::json;
use crate::api::{ApiError, ApiVersion};

pub async fn index() -> Result<impl IntoResponse, ApiError> {
    Ok(Json(json!({"message": "The service is running perfectly!"})))
}

pub async fn health_handler(api_version: ApiVersion) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("api version: {} is healthy", api_version);
    Ok(Json(json!({"status": "healthy"})))
}
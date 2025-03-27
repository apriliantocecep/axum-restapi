use std::fmt::{Display, Formatter};
use axum::http::StatusCode;
use axum::{
    Json,
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiError {
    pub status: u16,
    pub errors: Vec<ApiErrorResponse>
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        tracing::error!("error response: {:?}", self);
        let status_code = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status_code, Json(self)).into_response()
    }
}

impl From<(StatusCode, ApiErrorResponse)> for ApiError {
    fn from(value: (StatusCode, ApiErrorResponse)) -> Self {
        let (status_code, error_response) = value;
        Self {
            status: status_code.as_u16(),
            errors: vec![error_response],
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
}

impl ApiErrorResponse {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            timestamp: Utc::now(),
            ..Default::default()
        }
    }

    pub fn code<S: ToString>(mut self, value: S) -> Self {
        self.code = Some(value.to_string());
        self
    }

    pub fn kind<S: ToString>(mut self, value: S) -> Self {
        self.kind = Some(value.to_string());
        self
    }

    pub fn description<S: ToString>(mut self, value: S) -> Self {
        self.description = Some(value.to_string());
        self
    }

    pub fn detail(mut self, value: serde_json::Value) -> Self {
        self.detail = Some(value);
        self
    }

    pub fn reason(mut self, value: &str) -> Self {
        self.reason = Some(value.to_owned());
        self
    }

    pub fn instance(mut self, value: &str) -> Self {
        self.instance = Some(value.to_owned());
        self
    }

    pub fn trace_id(mut self) -> Self {
        let mut trace_id = uuid::Uuid::new_v4().to_string();
        trace_id.retain(|c| c != '-');
        self.trace_id = Some(trace_id);
        self
    }

    pub fn help(mut self, value: &str) -> Self {
        self.help = Some(value.to_owned());
        self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorCode {
    ApiVersionError,
}

impl Display for ApiErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!(self).as_str().unwrap_or_default())
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ApiErrorKind {
    ValidationError,
}

impl Display for ApiErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::json!(self).as_str().unwrap_or_default())
    }
}
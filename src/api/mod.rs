mod error;
mod version;
mod routes;

pub mod server;
pub mod handlers;
pub mod middleware;
pub mod dto;

pub use version::ApiVersion;
pub use error::{ApiError, ApiErrorCode, ApiErrorKind, ApiErrorResponse};

mod error;
mod version;

pub mod server;
pub mod handlers;
pub mod middleware;

pub use version::ApiVersion;
pub use error::{ApiError, ApiErrorCode, ApiErrorKind, ApiErrorResponse};

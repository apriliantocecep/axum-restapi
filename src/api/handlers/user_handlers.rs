use axum::{
    Json,
    response::IntoResponse,
    extract::State,
    http::StatusCode,
};
use thiserror::Error;
use uuid::Uuid;
use crate::api::{ApiError, ApiErrorCode, ApiErrorKind, ApiErrorResponse};
use crate::application::{
    security::jwt::{AccessClaim, ClaimsMethods},
    state::SharedState,
};
use crate::application::repository::user_repository::UserRepositoryExt;

pub async fn me_handler(
    access_claim: AccessClaim,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, ApiError> {
    let user_id = access_claim.get_sub().parse().unwrap();
    let user = state.get_user_by_id(user_id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                let user_error = UserError::UserNotFound(user_id);
                (user_error.status_code(), ApiErrorResponse::from(user_error)).into()
            },
            _ => ApiError::from(e),
        })?;

    Ok(Json(user))
}

#[derive(Debug, Error)]
enum UserError {
    #[error("user not found: {0}")]
    UserNotFound(Uuid)
}

impl From<UserError> for ApiErrorResponse {
    fn from(user_error: UserError) -> Self {
        let message = user_error.to_string();
        match user_error {
            UserError::UserNotFound(user_id) => Self::new(&message)
                .code(ApiErrorCode::UserNotFound)
                .kind(ApiErrorKind::ResourceNotFound)
                .description(&format!("user with the ID '{}' does not exists", user_id))
                .detail(serde_json::json!({"user_id": user_id}))
                .reason("must be an existing user in the database")
                .instance(&format!("/api/v1/users/{}", user_id))
                .trace_id()
        }
    }
}

impl UserError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::UserNotFound(_) => StatusCode::NOT_FOUND,
        }
    }
}
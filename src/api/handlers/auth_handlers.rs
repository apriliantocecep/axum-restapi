use axum::{
    Json,
    extract::State,
    response::IntoResponse,
};
use serde_json::json;
use sqlx::{query_as};
use crate::api::{ApiError, ApiVersion};
use crate::application::state::SharedState;

#[tracing::instrument(level = tracing::Level::TRACE, name = "login", skip_all)]
pub async fn login_handler(
    api_version: ApiVersion,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("api version: {} login", api_version);
    let data = get_by_id(&state).await?;

    Ok(Json(json!({"data": data})))
}

async fn get_by_id(state: &SharedState) -> Result<i64, sqlx::Error> {
    let row: (i64,) = query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&*state.db_pool).await?;

    Ok(row.0)
}
use axum::{extract::{State}, response::IntoResponse, Json};
use serde_json::json;
use crate::api::{ApiError, ApiVersion, dto::auth_dto::LoginUserDto};
use crate::application::{
    state::SharedState,
    security::{
        validator::ValidatedJson,
        auth::{AuthError},
    },
    repository::{
        user_repository::UserRepositoryExt,
    },
};
use crate::application::security::{auth, password};

#[tracing::instrument(level = tracing::Level::TRACE, name = "login", skip_all, fields(identifier=body.identifier))]
pub async fn login_handler(
    api_version: ApiVersion,
    State(state): State<SharedState>,
    ValidatedJson(body): ValidatedJson<LoginUserDto>,
) -> Result<impl IntoResponse, ApiError> {
    tracing::trace!("api version: {} login", api_version);

    let data = state.get_user_by_identifier(&body.identifier).await?;

    if data.is_none() {
        return Err(AuthError::WrongCredentials.into())
    }

    let user = data.ok_or(AuthError::WrongCredentials)?;

    if !user.active {
        return Err(AuthError::WrongCredentials.into())
    }

    let password_matches = password::compare(&body.password, &user.password_hash)
        .map_err(|_| AuthError::WrongCredentials)?;

    if !password_matches {
        return Err(AuthError::WrongCredentials)?
    }

    let token = auth::create_token(user, &state.config);
    let access_token = token.access_token;

    Ok(Json(json!({"token": access_token})))
}
use std::sync::Arc;
use axum::{extract::{FromRef, FromRequestParts}, http::request::Parts, RequestPartsExt};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use crate::api::ApiError;
use crate::application::{
    state::{SharedState, AppState},
    security::{
        auth::AuthError,
        jwt::{AccessClaim, ClaimsMethods, decode_token},
    },
};

impl<S> FromRequestParts<S> for AccessClaim
where
    SharedState: FromRef<S>,
    S: Send + Sync
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        decode_token_from_request_part(parts, state).await
    }
}

async fn decode_token_from_request_part<S, T>(parts: &mut Parts, state: &S) -> Result<T, ApiError>
where
    SharedState: FromRef<S>,
    S: Send + Sync,
    T: for<'de> serde::Deserialize<'de> + std::fmt::Debug + ClaimsMethods + Sync + Send,
{
    // Extract the token from the authorization header.
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| {
            tracing::error!("invalid authorization header");
            AuthError::InvalidAuthorizationHeader
        })?;

    // Take the state from a reference.
    let state: Arc<AppState> = Arc::from_ref(state);

    // Decode the token.
    let claims = decode_token::<T>(bearer.token(), &state.config)?;

    // Check for revoked tokens if enabled by configuration.
    if state.config.jwt_enable_revoked_tokens {
        // TODO: validate revoked
    }
    Ok(claims)
}
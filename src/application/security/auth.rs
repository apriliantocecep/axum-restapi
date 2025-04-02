use axum::http::StatusCode;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;
use crate::api::{ApiError, ApiErrorCode, ApiErrorKind, ApiErrorResponse};
use crate::application::{
    config::Config,
    security::jwt::{AccessClaim, ClaimsMethods, JwtTokenType},
    service::token_service,
    state::SharedState,
};
use crate::domain::entities::user::User;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("invalid credential combination")]
    WrongCredentials,
    #[error("missing credentials")]
    MissingCredentials,
    #[error("token creation error")]
    TokenCreationError,
    #[error("invalid token")]
    InvalidToken,
    #[error("password cannot be empty")]
    EmptyPassword,
    #[error("invalid password hash format")]
    InvalidHashFormat,
    #[error("error while hashing password")]
    HashingError,
    #[error("invalid bearer token")]
    InvalidBearerToken,
    #[error("invalid authorization header")]
    InvalidAuthorizationHeader,
    #[error(transparent)]
    SQLxError(#[from] sqlx::Error),
    #[error(transparent)]
    RedisError(#[from] redis::RedisError),
}

impl From<AuthError> for ApiError {
    fn from(auth_error: AuthError) -> Self {
        let (status_code, code) = match auth_error {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, ApiErrorCode::AuthenticationWrongCredentials),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, ApiErrorCode::AuthenticationMissingCredentials),
            AuthError::TokenCreationError => (StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::AuthenticationTokenCreationError),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, ApiErrorCode::AuthenticationInvalidToken),
            AuthError::SQLxError(_) => (StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::DatabaseError),
            AuthError::EmptyPassword => (StatusCode::BAD_REQUEST, ApiErrorCode::AuthenticationMissingCredentials),
            AuthError::InvalidHashFormat => (StatusCode::BAD_REQUEST, ApiErrorCode::AuthenticationHashingPasswordError),
            AuthError::HashingError => (StatusCode::BAD_REQUEST, ApiErrorCode::AuthenticationHashingPasswordError),
            AuthError::InvalidBearerToken => (StatusCode::UNAUTHORIZED, ApiErrorCode::AuthenticationForbidden),
            AuthError::InvalidAuthorizationHeader => (StatusCode::BAD_REQUEST, ApiErrorCode::AuthenticationMissingCredentials),
            AuthError::RedisError(_) => (StatusCode::INTERNAL_SERVER_ERROR, ApiErrorCode::RedisError),
        };

        let error_response = ApiErrorResponse::new(&auth_error.to_string())
            .code(code)
            .kind(ApiErrorKind::AuthenticationError);

        Self {
            status: status_code.as_u16(),
            errors: vec![error_response],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtToken {
    pub access_token: String,
}

pub fn create_token(user: User, config: &Config) -> JwtToken {
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let sub = user.id.to_string();

    let access_token_id = Uuid::new_v4().to_string();
    let access_token_exp = (now + chrono::Duration::seconds(config.jwt_exp_access_token_second)).timestamp() as usize;

    let access_claim = AccessClaim {
        sub: sub.clone(),
        jti: access_token_id.clone(),
        iat,
        exp: access_token_exp,
        typ: JwtTokenType::AccessToken as u8,
        roles: user.roles.clone(),
    };

    let access_token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &access_claim,
        &jsonwebtoken::EncodingKey::from_secret(config.jwt_secret.as_ref())
    ).unwrap();

    JwtToken {
        access_token
    }
}

pub async fn validate_revoked<T: std::fmt::Debug + ClaimsMethods + Send + Sync>(
    claims: &T,
    state: &SharedState,
) -> Result<(), AuthError> {
    let revoked = token_service::is_revoked(claims, state).await?;
    if revoked {
        Err(AuthError::WrongCredentials)?
    }
    Ok(())
}
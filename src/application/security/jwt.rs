use std::fmt::Formatter;
use jsonwebtoken::{EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use crate::application::config::Config;
use crate::application::security::auth::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaim {
    /// Subject.
    pub sub: String,
    /// JWT ID.
    pub jti: String,
    /// Issued time.
    pub iat: usize,
    /// Expiration time.
    pub exp: usize,
    /// Token type.
    pub typ: u8,
    /// Roles.
    pub roles: String,
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum JwtTokenType {
    AccessToken,
    RefreshToken,
    UnknownToken,
}

impl From<u8> for JwtTokenType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::AccessToken,
            1 => Self::RefreshToken,
            _ => Self::UnknownToken,
        }
    }
}

#[derive(Clone)]
pub struct JwtKey {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl std::fmt::Debug for JwtKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JwtKeys").finish()
    }
}

impl JwtKey {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

pub trait ClaimsMethods {
    fn get_sub(&self) -> &str;
    fn get_exp(&self) -> usize;
    fn get_iat(&self) -> usize;
    fn get_jti(&self) -> &str;
}

impl ClaimsMethods for AccessClaim {
    fn get_sub(&self) -> &str {
        &self.sub
    }

    fn get_exp(&self) -> usize {
        self.exp
    }

    fn get_iat(&self) -> usize {
        self.iat
    }

    fn get_jti(&self) -> &str {
        &self.jti
    }
}

pub fn decode_token<T: for<'de> serde::Deserialize<'de>>(token: &str, config: &Config)  -> Result<T, AuthError> {
    let mut validation = jsonwebtoken::Validation::default();
    validation.leeway = config.jwt_validation_leeway_seconds as u64;
    let token_data = jsonwebtoken::decode::<T>(token, &config.jwt_key.decoding, &validation)
        .map_err(|_| {
            tracing::error!("invalid bearer token: {}", token);
            AuthError::InvalidBearerToken
        })?;

    Ok(token_data.claims)
}
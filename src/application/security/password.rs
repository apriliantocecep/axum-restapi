use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash,
        PasswordHasher,
        PasswordVerifier,
        SaltString
    },
    Argon2,
    Algorithm,
    Version,
    Params,
};
use crate::application::security::auth::AuthError;

pub fn compare(password: &str, hashed_password: &str) -> Result<bool, AuthError> {
    if password.is_empty() {
        return Err(AuthError::EmptyPassword)
    }

    let parsed_hash = PasswordHash::new(hashed_password)
        .map_err(|_| AuthError::InvalidHashFormat)?;

    let password_matched = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map_or(false, |_| true);

    Ok(password_matched)
}

pub fn hash(password: impl Into<String>) -> Result<String, AuthError> {
    let password = password.into();

    if password.is_empty() {
        return Err(AuthError::EmptyPassword)
    }

    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::new(
        Algorithm::Argon2id,
        Version::V0x13,
        Params::new(
            15 * 1024, // Memory cost (in KiB) - reduced from default (e.g., 32 * 1024)
            2,         // Time cost (iterations) - reduced from default (e.g., 3)
            1,         // Parallelism (threads) - reduced from default (e.g., 4)
            None,
        ).map_err(|_| AuthError::HashingError)?
    );

    let hashed_password = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| AuthError::HashingError)?
        .to_string();

    Ok(hashed_password)
}
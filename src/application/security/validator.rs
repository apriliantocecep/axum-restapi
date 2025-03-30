use axum::{
    extract::{FromRequest, Request, Json}
};
use regex::Regex;
use serde::{de::DeserializeOwned};
use validator::{Validate, ValidationError};
use crate::api::ApiError;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S>,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(data) = Json::<T>::from_request(req, state).await.map_err(|_| ApiError::invalid_json())?;
        data.validate().map_err(ApiError::validation_error)?;
        Ok(Self(data))
    }
}

pub fn validate_identifier(identifier: &str) -> Result<(), ValidationError> {
    if identifier.contains("@") {
        if !validate_email(identifier) {
            let mut err = ValidationError::new("invalid_email_format");
            err.message = Some("invalid email format".into());
            return Err(err);
        }
    } else if identifier.len() < 3 {
        let mut err = ValidationError::new("short_username");
        err.message = Some("username must be at least 3 characters long".into());
        return Err(err);
    }
    Ok(())
}

pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(
        r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$"
    ).unwrap();

    email_regex.is_match(email)
}
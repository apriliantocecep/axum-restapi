use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Validate, Debug, Serialize, Deserialize)]
pub struct LoginUserDto {
    #[validate(custom(function = "crate::application::security::validator::validate_identifier"))]
    pub identifier: String,
    #[validate(length(min = 3, max = 20, message = "password must be between 3 and 20 characters"))]
    pub password: String,
}
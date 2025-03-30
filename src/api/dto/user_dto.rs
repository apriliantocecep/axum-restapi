use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use crate::domain::entities::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct FilterUserDto {
    id: String,
    name: String,
    username: String,
    email: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl FilterUserDto {
    pub fn filter(user: &User) -> Self {
        Self {
            id: user.id.to_string(),
            name: user.name.to_owned(),
            username: user.username.to_owned(),
            email: user.email.to_owned(),
            created_at: user.created_at.unwrap(),
            updated_at: user.created_at.unwrap(),
        }
    }
}
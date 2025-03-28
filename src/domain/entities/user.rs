use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub password_salt: String,
    pub active: bool,
    pub roles: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
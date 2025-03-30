use async_trait::async_trait;
use sqlx::query_as;
use crate::application::{
    repository::RepositoryResult,
    state::SharedState,
};
use crate::application::state::AppState;
use crate::domain::entities::user::User;

#[async_trait]
pub trait UserRepositoryExt {
    async fn get_user_by_identifier(&self, identifier: &str) -> RepositoryResult<Option<User>>;
}

#[async_trait]
impl UserRepositoryExt for AppState {
    async fn get_user_by_identifier(&self, identifier: &str) -> RepositoryResult<Option<User>> {
        let query = r#"
            SELECT * FROM users WHERE username = $1 OR email = $1
        "#;

        let user = sqlx::query_as::<_, User>(query)
            .bind(identifier)
            .fetch_optional(&*self.db_pool)
            .await?;

        Ok(user)
    }
}

pub async fn list(state: &SharedState) -> RepositoryResult<Vec<User>> {
    let users = query_as::<_, User>("select * from users")
        .fetch_all(&*state.db_pool)
        .await?;

    Ok(users)
}
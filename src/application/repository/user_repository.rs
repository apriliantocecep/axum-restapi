use sqlx::query_as;
use crate::application::{
    repository::RepositoryResult,
    state::SharedState,
};
use crate::domain::entities::user::User;

pub async fn list(state: &SharedState) -> RepositoryResult<Vec<User>> {
    let users = query_as::<_, User>("select * from users")
        .fetch_all(&*state.db_pool)
        .await?;

    Ok(users)
}
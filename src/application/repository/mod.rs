pub mod user_repository;

pub type RepositoryResult<T> = Result<T, sqlx::Error>;
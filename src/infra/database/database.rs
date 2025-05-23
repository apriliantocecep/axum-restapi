use std::sync::Arc;
use sqlx::{PgPool};
use thiserror::Error;
use crate::application::config::Config;
use crate::infra::database::{
    postgres::{PostgresDatabase, PostgresOptions, DatabasePoolPostgres},
};

pub type DatabasePool = Arc<PgPool>;

struct Database;

impl Database {
    async fn connect(config: &Config) -> Result<DatabasePoolPostgres, DatabaseError> {
        let database_url = &config.database_url;

        let option = PostgresOptions {
            connection_url: database_url.to_owned(),
            max_connections: 10,
            min_connections: 5,
        };

        let db = PostgresDatabase::connect(Box::new(option)).await?;

        Ok(db.pool())
    }

    async fn migrate(pool: &DatabasePool) -> Result<(), DatabaseError> {
        sqlx::migrate!()
            .run(&**pool)
            .await?;
        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error(transparent)]
    SQLxError(#[from] sqlx::Error),
    #[error(transparent)]
    SQLxMigrateError(#[from] sqlx::migrate::MigrateError),
}

pub async fn load(config: &Config) -> DatabasePool {
    Database::connect(&config).await.unwrap_or_else(|e| {
        tracing::error!("{}", e);
        std::process::exit(1);
    })
}

pub async fn migrate(pool: &DatabasePool) {
    Database::migrate(pool).await.unwrap_or_else(|e| {
        tracing::error!("{}", e);
    })
}
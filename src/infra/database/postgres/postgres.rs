use std::sync::Arc;
use std::time::Duration;
use sqlx::{PgPool};
use sqlx::postgres::PgPoolOptions;
use crate::infra::database::{DatabaseError, option::DatabaseOption};

pub type DatabasePoolPostgres = Arc<PgPool>;

pub struct PostgresDatabase {
    pool: DatabasePoolPostgres,
}

impl PostgresDatabase {
    pub async fn connect(option: Box<dyn DatabaseOption>) -> Result<Self, DatabaseError> {
        let connection_url = option.connection_url();
        let max_connection = option.max_connections();
        let min_connection = option.min_connections();

        let pool = PgPoolOptions::new()
            .max_connections(max_connection)
            .min_connections(min_connection)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60))
            .connect(&connection_url)
            .await?;

        tracing::info!("connected to PostgresSQL database");

        Ok(Self {
            pool: Arc::new(pool)
        })
    }

    pub fn pool(&self) -> DatabasePoolPostgres {
        Arc::clone(&self.pool)
    }
}
use std::sync::Arc;
use crate::application::config::Config;
use crate::infra::database::DatabasePool;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub config: Config,
    pub db_pool: DatabasePool,
}
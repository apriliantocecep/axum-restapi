use std::sync::{Arc};
use tokio::sync::Mutex;
use crate::application::config::Config;
use crate::infra::database::DatabasePool;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub config: Config,
    pub db_pool: DatabasePool,
    pub cache: Mutex<redis::aio::MultiplexedConnection>,
}
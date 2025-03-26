use std::sync::Arc;
use crate::application::config::Config;

pub type SharedState = Arc<AppState>;

pub struct AppState {
    pub config: Config
}
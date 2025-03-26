use std::sync::Arc;
use crate::api::server;
use crate::application::{
    config,
    state::AppState,
};

pub async fn run() {
    let config = config::load();

    let shared_state = Arc::new(AppState {
        config,
    });

    server::start(shared_state).await
}
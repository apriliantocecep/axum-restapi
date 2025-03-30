use std::sync::Arc;
use crate::api::server;
use crate::application::{
    config,
    state::AppState,
};
use crate::infra::database;

pub async fn run() {
    let config = config::load();

    let db_pool = database::load(&config).await;

    // database::migrate(&db_pool).await;

    let shared_state = Arc::new(AppState {
        config,
        db_pool,
    });

    server::start(shared_state).await
}
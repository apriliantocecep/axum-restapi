use std::sync::{Arc};
use tokio::sync::Mutex;
use crate::api::server;
use crate::application::{
    config,
    state::AppState,
};
use crate::infra::{cache, database};

pub async fn run() {
    let config = config::load();

    let db_pool = database::load(&config).await;

    // database::migrate(&db_pool).await;

    let cache = cache::load(&config).await;

    let shared_state = Arc::new(AppState {
        config,
        db_pool,
        cache: Mutex::new(cache),
    });

    server::start(shared_state).await
}
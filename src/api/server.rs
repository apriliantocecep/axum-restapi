use std::sync::Arc;
use axum::{
    Router,
    routing::{get},
    middleware
};
use crate::api::{
    handlers::{
        root_handlers::{index, health_handler},
        error_handlers::error_404_handler,
    },
    middleware::{logging_middleware},
    routes::{auth_routes},
};
use tokio::{
    net::TcpListener,
    signal::{self, unix::{self, SignalKind}}
};
use tower_http::cors::{CorsLayer, Any};
use crate::application::{
    state::{SharedState},
};

pub async fn start(state: SharedState) {
    let cors_layer = CorsLayer::new()
        .allow_origin(Any);

    let router = Router::new()
        .route("/", get(index))
        .route("/{version}/health", get(health_handler))
        .nest("/{version}/auth", auth_routes::routes())
        .fallback(error_404_handler)
        .with_state(Arc::clone(&state))
        .layer(middleware::from_fn(logging_middleware))
        .layer(cors_layer);

    let addr = state.config.get_socket_addr();
    let listener = TcpListener::bind(&addr).await.unwrap();
    tracing::info!("listening on {}", addr);

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    tracing::info!("server shutdown successfully");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        unix::signal(SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("ctr+c signal awake")
        },
        _ = terminate => {
            tracing::info!("terminate signal awake")
        }
    }

    tracing::info!("received termination signal, shutting down...");
}


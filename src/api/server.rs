use std::net::SocketAddr;
use axum::{
    Router,
    routing::{get}
};
use crate::api::{
    handlers::{
        root_handlers::{index}
    }
};
use std::str::FromStr;
use tokio::{
    net::TcpListener,
    signal::{self, unix::{self, SignalKind}}
};

pub async fn start() {
    let router = Router::new()
        .route("/", get(index));

    let addr = SocketAddr::from_str(&format!("{}:{}", "127.0.0.1", "8000")).unwrap();
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
        _ = ctrl_c => {},
        _ = terminate => {}
    }
}


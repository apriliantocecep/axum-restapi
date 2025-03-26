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
use tokio::net::TcpListener;

pub async fn start() {
    let router = Router::new()
        .route("/", get(index));

    let addr = SocketAddr::from_str(&format!("{}:{}", "127.0.0.1", "8000")).unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, router)
        .await
        .unwrap()
}


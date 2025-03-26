use axum_restapi::application::app;

#[tokio::main]
async fn main() {
    app::run().await
}

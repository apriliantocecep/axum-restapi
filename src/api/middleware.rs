use axum::{
    extract::Request,
    response::Response,
    body::Body,
    middleware::Next
};

#[tracing::instrument(level = tracing::Level::TRACE, name = "axum", skip_all, fields(method=request.method().to_string(), uri=request.uri().to_string()))]
pub async fn logging_middleware(request: Request<Body>, next: Next) -> Response {
    tracing::trace!(
        "received a {} request to {}",
        request.method(),
        request.uri()
    );
    next.run(request).await
}
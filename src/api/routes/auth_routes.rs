use axum::{
    Router,
    routing::{post}
};
use crate::api::handlers::{
    auth_handlers::{login_handler}
};
use crate::application::state::SharedState;

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/login", post(login_handler))
}
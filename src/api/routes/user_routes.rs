use axum::{Router, routing::get};
use crate::application::state::SharedState;
use crate::api::handlers::user_handlers::{me_handler};

pub fn routes() -> Router<SharedState> {
    Router::new()
        .route("/me", get(me_handler))
}
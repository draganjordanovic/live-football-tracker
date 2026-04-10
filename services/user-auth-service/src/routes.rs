use std::sync::Arc;

use axum::{routing::get, Router};

use crate::app_state::AppState;

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health))
        .with_state(state)
}

async fn health() -> &'static str {
    "OK"
}
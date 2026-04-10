use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    app_state::AppState,
    handlers::auth::register,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/register", axum::routing::post(register))
        .with_state(state)
}
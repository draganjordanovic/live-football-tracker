use std::sync::Arc;

use axum::{routing::{get, post}, Router};

use crate::{
    app_state::AppState,
    handlers::auth::{register, login},
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/register", axum::routing::post(register))
        .route("/auth/login", post(login))
        .with_state(state)
}
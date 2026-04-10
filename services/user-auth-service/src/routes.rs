use std::sync::Arc;

use axum::{routing::{get, post}, Router};

use crate::{
    app_state::AppState,
    handlers::{
        auth::{login, register},
        users::get_me,
    },
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/users/me", get(get_me))
        .with_state(state)
}
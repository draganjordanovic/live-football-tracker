use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

use crate::{
    app_state::AppState,
    handlers::{
        teams::get_team_statistics,
    },
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/competitions/:code/teams/:teamId/statistics", get(get_team_statistics))
        .with_state(state)
        .layer(CorsLayer::permissive())
}
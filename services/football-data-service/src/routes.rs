use std::sync::Arc;

use axum::{
    routing::get,
    Router,
};
use tower_http::cors::CorsLayer;

use crate::{
    app_state::AppState,
    handlers::{
        competitions::{get_competition_standings, get_competitions},
        matches::{get_competition_matches, get_match_details},
    },
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/competitions", get(get_competitions))
        .route("/competitions/:code/standings", get(get_competition_standings))
        .route("/competitions/:code/matches", get(get_competition_matches))
        .route("/matches/:id", get(get_match_details))
        .with_state(state)
        .layer(CorsLayer::permissive())
}
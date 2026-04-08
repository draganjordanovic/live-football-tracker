use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

use crate::{
    app_state::AppState,
    handlers::reports::{
        download_competition_standings_pdf,
        download_match_pdf,
        download_team_statistics_pdf,
    },
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/reports/competitions/:code/standings/pdf",
            get(download_competition_standings_pdf),
        )
        .route(
            "/reports/matches/:id/pdf",
            get(download_match_pdf),
        )
        .route(
            "/reports/competitions/:code/teams/:teamId/statistics/pdf",
            get(download_team_statistics_pdf),
        )
        .with_state(state)
        .layer(CorsLayer::permissive())
}
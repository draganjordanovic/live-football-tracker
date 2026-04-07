use std::sync::Arc;

use axum::{routing::get, Router};
use tower_http::cors::CorsLayer;

use crate::{app_state::AppState, handlers::reports::download_competition_standings_pdf};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route(
            "/reports/competitions/:code/standings/pdf",
            get(download_competition_standings_pdf),
        )
        .with_state(state)
        .layer(CorsLayer::permissive())
}
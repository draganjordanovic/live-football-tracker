use std::sync::Arc;

use axum::{routing::{delete, get, post}, Router};

use crate::{
    app_state::AppState,
    handlers::{
        auth::{login, register},
        favorites::{add_favorite_club, get_my_favorite_clubs, remove_favorite_club},
        users::get_me,
    },
};

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/users/me", get(get_me))
        .route("/users/me/favorite-clubs", get(get_my_favorite_clubs))
        .route("/users/me/favorite-clubs", post(add_favorite_club))
        .route("/users/me/favorite-clubs/:clubId", delete(remove_favorite_club))
        .with_state(state)
}
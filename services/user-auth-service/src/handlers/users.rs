use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};

use crate::{
    app_state::AppState,
    extractors::AuthenticatedUser,
    models::user::MeResponse,
};

pub async fn get_me(
    State(state): State<Arc<AppState>>,
    auth_user: AuthenticatedUser,
) -> Result<Json<MeResponse>, (StatusCode, String)> {
    let user = sqlx::query!(
        r#"
        SELECT id, email, role, created_at
        FROM users
        WHERE id = $1
        "#,
        auth_user.user_id
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user = match user {
        Some(user) => user,
        None => return Err((StatusCode::NOT_FOUND, "User not found".into())),
    };

    let created_at = DateTime::<Utc>::from_naive_utc_and_offset(user.created_at, Utc);

    Ok(Json(MeResponse {
        id: user.id,
        email: user.email,
        role: user.role,
        created_at,
    }))
}
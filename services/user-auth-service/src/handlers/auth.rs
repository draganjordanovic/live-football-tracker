use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use bcrypt::{hash, DEFAULT_COST};
use chrono::{NaiveDateTime, Utc};
use uuid::Uuid;

use crate::{
    app_state::AppState,
    models::{auth::RegisterRequest, user::User},
};

pub async fn register(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<User>, (StatusCode, String)> {
    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let user_id = Uuid::new_v4();
    let now: NaiveDateTime = Utc::now().naive_utc();

    let result = sqlx::query!(
        r#"
        INSERT INTO users (id, email, password_hash, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        user_id,
        payload.email,
        password_hash,
        "USER",
        now,
        now
    )
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => Ok(Json(User {
            id: user_id,
            email: payload.email,
            role: "USER".to_string(),
            created_at: Utc::now(),
        })),
        Err(e) => {
            if e.to_string().contains("duplicate") {
                return Err((StatusCode::CONFLICT, "User already exists".into()));
            }

            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}
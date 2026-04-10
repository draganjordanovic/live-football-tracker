use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::{DateTime, Utc};

use crate::{
    app_state::AppState,
    extractors::AuthenticatedUser,
    models::favorite_club::{CreateFavoriteClubRequest, FavoriteClub},
};

pub async fn get_my_favorite_clubs(
    State(state): State<Arc<AppState>>,
    auth_user: AuthenticatedUser,
) -> Result<Json<Vec<FavoriteClub>>, (StatusCode, String)> {
    let rows = sqlx::query!(
        r#"
        SELECT id, user_id, club_external_id, competition_code, club_name, club_short_name, club_crest, created_at
        FROM favorite_clubs
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        auth_user.user_id
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let favorites = rows
        .into_iter()
        .map(|row| FavoriteClub {
            id: row.id,
            user_id: row.user_id,
            club_external_id: row.club_external_id,
            competition_code: row.competition_code,
            club_name: row.club_name,
            club_short_name: row.club_short_name,
            club_crest: row.club_crest,
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(row.created_at, Utc),
        })
        .collect();

    Ok(Json(favorites))
}

pub async fn add_favorite_club(
    State(state): State<Arc<AppState>>,
    auth_user: AuthenticatedUser,
    Json(payload): Json<CreateFavoriteClubRequest>,
) -> Result<Json<FavoriteClub>, (StatusCode, String)> {
    let now = Utc::now().naive_utc();

    let result = sqlx::query!(
        r#"
        INSERT INTO favorite_clubs (
            user_id,
            club_external_id,
            competition_code,
            club_name,
            club_short_name,
            club_crest,
            created_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, user_id, club_external_id, competition_code, club_name, club_short_name, club_crest, created_at
        "#,
        auth_user.user_id,
        payload.club_external_id,
        payload.competition_code,
        payload.club_name,
        payload.club_short_name,
        payload.club_crest,
        now
    )
    .fetch_one(&state.db)
    .await;

    match result {
        Ok(row) => Ok(Json(FavoriteClub {
            id: row.id,
            user_id: row.user_id,
            club_external_id: row.club_external_id,
            competition_code: row.competition_code,
            club_name: row.club_name,
            club_short_name: row.club_short_name,
            club_crest: row.club_crest,
            created_at: DateTime::<Utc>::from_naive_utc_and_offset(row.created_at, Utc),
        })),
        Err(e) => {
            if e.to_string().contains("duplicate") || e.to_string().contains("unique") {
                return Err((StatusCode::CONFLICT, "Club is already in favorites".into()));
            }

            Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
        }
    }
}

pub async fn remove_favorite_club(
    State(state): State<Arc<AppState>>,
    auth_user: AuthenticatedUser,
    Path(club_id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query!(
        r#"
        DELETE FROM favorite_clubs
        WHERE user_id = $1 AND club_external_id = $2
        "#,
        auth_user.user_id,
        club_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Favorite club not found".into()));
    }

    Ok(StatusCode::NO_CONTENT)
}
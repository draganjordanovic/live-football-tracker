use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use mongodb::bson::doc;

use crate::{
    app_state::AppState,
    errors::internal_error,
    models::{
        incoming_standings::{
            IncomingCompetitionStandingResponse,
            IncomingStandingRow,
        },
        team_statistics::TeamStatisticsDocument,
    },
};

pub async fn get_team_statistics(
    Path((code, team_id)): Path<(String, u32)>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<TeamStatisticsDocument>, (StatusCode, String)> {
    let url = format!(
        "{}/competitions/{}/standings",
        state.football_data_service_url, code
    );

    let response = state
        .http_client
        .get(&url)
        .send()
        .await
        .map_err(internal_error)?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error body".to_string());

        return Err((
            StatusCode::BAD_GATEWAY,
            format!("Football Data Service error: {} - {}", status, body),
        ));
    }

    let incoming: IncomingCompetitionStandingResponse = response
        .json()
        .await
        .map_err(internal_error)?;

    let mut found_row: Option<IncomingStandingRow> = None;
    let mut found_standing_type = String::from("TOTAL");

    for standing_group in &incoming.standings {
        if let Some(row) = standing_group.table.iter().find(|row| row.team_id == team_id) {
            found_row = Some(row.clone());
            found_standing_type = standing_group.standing_type.clone();
            break;
        }
    }

    let row = match found_row {
        Some(row) => row,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                format!("Team with id {} not found in competition {}", team_id, code),
            ))
        }
    };

    let played_games = row.played_games.max(1) as f64;

    let average_points_per_match = row.points as f64 / played_games;
    let average_goals_for_per_match = row.goals_for as f64 / played_games;
    let average_goals_against_per_match = row.goals_against as f64 / played_games;
    let average_goal_difference_per_match = row.goal_difference as f64 / played_games;

    let win_rate = row.won as f64 / played_games;
    let draw_rate = row.draw as f64 / played_games;
    let loss_rate = row.lost as f64 / played_games;

    let form_points = calculate_form_points(row.form.as_deref());

    let document = TeamStatisticsDocument {
        competition_code: incoming.competition.code.clone(),
        competition_name: incoming.competition.name.clone(),
        competition_id: incoming.competition.id,

        team_id: row.team_id,
        team_name: row.team_name.clone(),
        team_short_name: row.team_short_name.clone(),
        team_tla: row.team_tla.clone(),
        team_crest: row.team_crest.clone(),

        current_matchday: incoming.season.current_matchday,
        standing_type: found_standing_type,
        position: row.position,

        played_games: row.played_games,
        won: row.won,
        draw: row.draw,
        lost: row.lost,
        points: row.points,

        goals_for: row.goals_for,
        goals_against: row.goals_against,
        goal_difference: row.goal_difference,

        average_points_per_match,
        average_goals_for_per_match,
        average_goals_against_per_match,
        average_goal_difference_per_match,

        win_rate,
        draw_rate,
        loss_rate,

        form: row.form.clone(),
        form_points,

        last_synced_at: Utc::now(),
    };

    let collection = state
        .mongo_db
        .collection::<TeamStatisticsDocument>("team_statistics");

    collection
        .replace_one(
            doc! {
                "competition_code": &document.competition_code,
                "team_id": i64::from(document.team_id)
            },
            &document,
        )
        .upsert(true)
        .await
        .map_err(internal_error)?;

    Ok(Json(document))
}

fn calculate_form_points(form: Option<&str>) -> i32 {
    match form {
        Some(value) => value
            .split(',')
            .map(|token| match token.trim() {
                "W" => 3,
                "D" => 1,
                "L" => 0,
                _ => 0,
            })
            .sum(),
        None => 0,
    }
}
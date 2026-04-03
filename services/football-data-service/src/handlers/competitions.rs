use std::sync::Arc;

use axum::{extract::{Path, State}, http::StatusCode, Json};

use crate::{
    app_state::AppState,
    errors::internal_error,
    models::{
        common::CompetitionInfo,
        competitions::{Competition, FootballDataCompetitionsResponse},
        standings::{
            CompetitionStandingResponse,
            FootballDataStandingsResponse,
            SeasonInfo,
            StandingGroup,
            TableRow,
        },
    },
};

/// GET /competitions
pub async fn get_competitions(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<Competition>>, (StatusCode, String)> {
    let url = "https://api.football-data.org/v4/competitions";

    let response = state
        .client
        .get(url)
        .header("X-Auth-Token", &state.api_key)
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
            format!("Football Data API error: {} - {}", status, body),
        ));
    }

    let api_response: FootballDataCompetitionsResponse =
        response.json().await.map_err(internal_error)?;

    let allowed_codes = [
        "PL",   // Premier League
        "PD",   // La Liga
        "SA",   // Serie A
        "BL1",  // Bundesliga
        "FL1",  // Ligue 1
        "DED",  // Eredivisie
        "PPL",  // Primeira Liga
        "ELC",  // Championship
        "CL",   // UEFA Champions League
        "BSA",  // Campeonato Brasileiro Série A
        "WC",   // FIFA World Cup
        "EC",   // European Championship
    ];

    let competitions = api_response
        .competitions
        .into_iter()
        .filter(|competition| allowed_codes.contains(&competition.code.as_str()))
        .map(|competition| Competition {
            id: competition.id,
            name: competition.name,
            code: competition.code,
            image_url: competition.emblem.unwrap_or_default(),
        })
        .collect();

    Ok(Json(competitions))
}

/// GET /competitions/:code/standings
pub async fn get_competition_standings(
    Path(code): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<CompetitionStandingResponse>, (StatusCode, String)> {
    let url = format!("https://api.football-data.org/v4/competitions/{}/standings", code);

    let response = state
        .client
        .get(&url)
        .header("X-Auth-Token", &state.api_key)
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
            format!("Football Data API error: {} - {}", status, body),
        ));
    }

    let api_response: FootballDataStandingsResponse =
        response.json().await.map_err(internal_error)?;

    let standings = api_response
        .standings
        .into_iter()
        .map(|standing| StandingGroup {
            standing_type: standing.standing_type,
            table: standing
                .table
                .into_iter()
                .map(|row| TableRow {
                    position: row.position,
                    team_id: row.team.id,
                    team_name: row.team.name,
                    team_short_name: row.team.short_name,
                    team_tla: row.team.tla,
                    team_crest: row.team.crest,
                    played_games: row.played_games,
                    form: row.form,
                    won: row.won,
                    draw: row.draw,
                    lost: row.lost,
                    points: row.points,
                    goals_for: row.goals_for,
                    goals_against: row.goals_against,
                    goal_difference: row.goal_difference,
                })
                .collect(),
        })
        .collect();

    let result = CompetitionStandingResponse {
        competition: CompetitionInfo {
            id: api_response.competition.id,
            name: api_response.competition.name,
            code: api_response.competition.code,
            image_url: api_response.competition.emblem.unwrap_or_default(),
        },
        season: SeasonInfo {
            current_matchday: api_response.season.current_matchday,
        },
        standings,
    };

    Ok(Json(result))
}
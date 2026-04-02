use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Json,
    Router,
};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    client: Client,
    api_key: String,
}

#[derive(Serialize)]
struct Competition {
    id: u32,
    name: String,
    code: String,
    image_url: String,
}

#[derive(Deserialize)]
struct FootballDataCompetitionsResponse {
    competitions: Vec<FootballDataCompetition>,
}

#[derive(Deserialize)]
struct FootballDataCompetition {
    id: u32,
    name: String,
    code: String,
    emblem: Option<String>,
}

#[derive(Serialize)]
struct CompetitionStandingResponse {
    competition: CompetitionInfo,
    season: SeasonInfo,
    standings: Vec<StandingGroup>,
}

#[derive(Serialize)]
struct CompetitionInfo {
    id: u32,
    name: String,
    code: String,
    image_url: String,
}

#[derive(Serialize)]
struct SeasonInfo {
    current_matchday: Option<u32>,
}

#[derive(Serialize)]
struct StandingGroup {
    standing_type: String,
    table: Vec<TableRow>,
}

#[derive(Serialize)]
struct TableRow {
    position: u32,
    team_id: u32,
    team_name: String,
    team_short_name: String,
    team_tla: Option<String>,
    team_crest: String,
    played_games: u32,
    form: Option<String>,
    won: u32,
    draw: u32,
    lost: u32,
    points: u32,
    goals_for: i32,
    goals_against: i32,
    goal_difference: i32,
}

#[derive(Deserialize)]
struct FootballDataStandingsResponse {
    competition: FootballDataStandingCompetition,
    season: FootballDataStandingSeason,
    standings: Vec<FootballDataStanding>,
}

#[derive(Deserialize)]
struct FootballDataStandingCompetition {
    id: u32,
    name: String,
    code: String,
    emblem: Option<String>,
}

#[derive(Deserialize)]
struct FootballDataStandingSeason {
    #[serde(rename = "currentMatchday")]
    current_matchday: Option<u32>,
}

#[derive(Deserialize)]
struct FootballDataStanding {
    #[serde(rename = "type")]
    standing_type: String,
    table: Vec<FootballDataTableRow>,
}

#[derive(Deserialize)]
struct FootballDataTableRow {
    position: u32,
    team: FootballDataTableTeam,
    #[serde(rename = "playedGames")]
    played_games: u32,
    form: Option<String>,
    won: u32,
    draw: u32,
    lost: u32,
    points: u32,
    #[serde(rename = "goalsFor")]
    goals_for: i32,
    #[serde(rename = "goalsAgainst")]
    goals_against: i32,
    #[serde(rename = "goalDifference")]
    goal_difference: i32,
}

#[derive(Deserialize)]
struct FootballDataTableTeam {
    id: u32,
    name: String,
    #[serde(rename = "shortName")]
    short_name: String,
    tla: Option<String>,
    crest: String,
}

async fn get_competition_standings(
    axum::extract::Path(code): axum::extract::Path<String>,
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

    let api_response: FootballDataStandingsResponse = response
        .json()
        .await
        .map_err(internal_error)?;

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

async fn get_competitions(
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

    let api_response: FootballDataCompetitionsResponse = response
        .json()
        .await
        .map_err(internal_error)?;

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

fn internal_error<E: std::fmt::Display>(error: E) -> (StatusCode, String) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("Internal server error: {}", error),
    )
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let api_key = env::var("FOOTBALL_DATA_API_KEY")
        .expect("FOOTBALL_DATA_API_KEY must be set in .env file");

    let state = Arc::new(AppState {
        client: Client::new(),
        api_key,
    });

    let app = Router::new()
        .route("/competitions", get(get_competitions))
        .route("/competitions/:code/standings", get(get_competition_standings))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
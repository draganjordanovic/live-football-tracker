use axum::{
    extract::{Query, State},
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

#[derive(Serialize)]
struct CompetitionMatchesResponse {
    competition: CompetitionInfo,
    matches: Vec<MatchItem>,
}

#[derive(Serialize)]
struct MatchItem {
    id: u32,
    utc_date: String,
    status: String,
    matchday: Option<u32>,
    stage: Option<String>,
    home_team: MatchTeam,
    away_team: MatchTeam,
    score: MatchScore,
}

#[derive(Serialize)]
struct MatchTeam {
    id: u32,
    name: String,
    short_name: String,
    tla: Option<String>,
    crest: String,
}

#[derive(Serialize)]
struct MatchScore {
    home: Option<i32>,
    away: Option<i32>,
}

#[derive(Deserialize)]
struct FootballDataMatchesResponse {
    competition: FootballDataStandingCompetition,
    matches: Vec<FootballDataMatch>,
}

#[derive(Deserialize)]
struct FootballDataMatch {
    id: u32,
    #[serde(rename = "utcDate")]
    utc_date: String,
    status: String,
    matchday: Option<u32>,
    stage: Option<String>,
    #[serde(rename = "homeTeam")]
    home_team: FootballDataMatchTeam,
    #[serde(rename = "awayTeam")]
    away_team: FootballDataMatchTeam,
    score: FootballDataMatchScore,
}

#[derive(Deserialize)]
struct FootballDataMatchTeam {
    id: u32,
    name: String,
    #[serde(rename = "shortName")]
    short_name: String,
    tla: Option<String>,
    crest: String,
}

#[derive(Deserialize)]
struct FootballDataMatchScore {
    #[serde(rename = "fullTime")]
    full_time: FootballDataFullTimeScore,
}

#[derive(Deserialize)]
struct FootballDataFullTimeScore {
    home: Option<i32>,
    away: Option<i32>,
}

#[derive(Deserialize)]
struct CompetitionMatchesQuery {
    matchday: u32,
}

#[derive(Serialize)]
struct MatchDetailsResponse {
    id: u32,
    utc_date: String,
    status: String,
    venue: Option<String>,
    matchday: Option<u32>,
    stage: Option<String>,
    home_team: MatchTeam,
    away_team: MatchTeam,
    competition: MatchCompetitionInfo,
    score: DetailedMatchScore,
    referees: Vec<RefereeItem>,
    goals: Vec<GoalItem>,
    bookings: Vec<BookingItem>,
}

#[derive(Serialize)]
struct MatchCompetitionInfo {
    id: u32,
    name: String,
    code: String,
    emblem: String,
}

#[derive(Serialize)]
struct DetailedMatchScore {
    winner: Option<String>,
    full_time: ScorePair,
    half_time: ScorePair,
}

#[derive(Serialize)]
struct ScorePair {
    home: Option<i32>,
    away: Option<i32>,
}

#[derive(Serialize)]
struct RefereeItem {
    id: u32,
    name: String,
    r#type: Option<String>,
    nationality: Option<String>,
}

#[derive(Serialize)]
struct GoalItem {
    minute: Option<u32>,
    injury_time: Option<u32>,
    team_id: Option<u32>,
    team_name: Option<String>,
    scorer: Option<String>,
    assist: Option<String>,
    score_home: Option<i32>,
    score_away: Option<i32>,
}

#[derive(Serialize)]
struct BookingItem {
    minute: Option<u32>,
    team_id: Option<u32>,
    team_name: Option<String>,
    player: Option<String>,
    card: Option<String>,
}

#[derive(Deserialize)]
struct FootballDataMatchDetailsResponse {
    id: u32,
    #[serde(rename = "utcDate")]
    utc_date: String,
    status: String,
    venue: Option<String>,
    matchday: Option<u32>,
    stage: Option<String>,
    #[serde(rename = "homeTeam")]
    home_team: FootballDataMatchTeam,
    #[serde(rename = "awayTeam")]
    away_team: FootballDataMatchTeam,
    competition: FootballDataStandingCompetition,
    score: FootballDataDetailedScore,
    referees: Vec<FootballDataReferee>,
    goals: Option<Vec<FootballDataGoal>>,
    bookings: Option<Vec<FootballDataBooking>>,
}

#[derive(Deserialize)]
struct FootballDataDetailedScore {
    winner: Option<String>,
    #[serde(rename = "fullTime")]
    full_time: FootballDataFullTimeScore,
    #[serde(rename = "halfTime")]
    half_time: FootballDataFullTimeScore,
}

#[derive(Deserialize)]
struct FootballDataReferee {
    id: u32,
    name: String,
    #[serde(rename = "type")]
    r#type: Option<String>,
    nationality: Option<String>,
}

#[derive(Deserialize)]
struct FootballDataGoal {
    minute: Option<u32>,
    #[serde(rename = "injuryTime")]
    injury_time: Option<u32>,
    team: Option<FootballDataEventTeam>,
    scorer: Option<FootballDataPerson>,
    assist: Option<FootballDataPerson>,
    score: Option<FootballDataFullTimeScore>,
}

#[derive(Deserialize)]
struct FootballDataBooking {
    minute: Option<u32>,
    team: Option<FootballDataEventTeam>,
    player: Option<FootballDataPerson>,
    card: Option<String>,
}

#[derive(Deserialize)]
struct FootballDataEventTeam {
    id: u32,
    name: String,
}

#[derive(Deserialize)]
struct FootballDataPerson {
    name: String,
}

async fn get_match_details(
    axum::extract::Path(id): axum::extract::Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<MatchDetailsResponse>, (StatusCode, String)> {
    let url = format!("https://api.football-data.org/v4/matches/{}", id);

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

    let api_response: FootballDataMatchDetailsResponse = response
        .json()
        .await
        .map_err(internal_error)?;

    let result = MatchDetailsResponse {
        id: api_response.id,
        utc_date: api_response.utc_date,
        status: api_response.status,
        venue: api_response.venue,
        matchday: api_response.matchday,
        stage: api_response.stage,
        home_team: MatchTeam {
            id: api_response.home_team.id,
            name: api_response.home_team.name,
            short_name: api_response.home_team.short_name,
            tla: api_response.home_team.tla,
            crest: api_response.home_team.crest,
        },
        away_team: MatchTeam {
            id: api_response.away_team.id,
            name: api_response.away_team.name,
            short_name: api_response.away_team.short_name,
            tla: api_response.away_team.tla,
            crest: api_response.away_team.crest,
        },
        competition: MatchCompetitionInfo {
            id: api_response.competition.id,
            name: api_response.competition.name,
            code: api_response.competition.code,
            emblem: api_response.competition.emblem.unwrap_or_default(),
        },
        score: DetailedMatchScore {
            winner: api_response.score.winner,
            full_time: ScorePair {
                home: api_response.score.full_time.home,
                away: api_response.score.full_time.away,
            },
            half_time: ScorePair {
                home: api_response.score.half_time.home,
                away: api_response.score.half_time.away,
            },
        },
        referees: api_response
            .referees
            .into_iter()
            .map(|r| RefereeItem {
                id: r.id,
                name: r.name,
                r#type: r.r#type,
                nationality: r.nationality,
            })
            .collect(),
        goals: api_response
            .goals
            .unwrap_or_default()
            .into_iter()
            .map(|g| GoalItem {
                minute: g.minute,
                injury_time: g.injury_time,
                team_id: g.team.as_ref().map(|t| t.id),
                team_name: g.team.as_ref().map(|t| t.name.clone()),
                scorer: g.scorer.map(|p| p.name),
                assist: g.assist.map(|p| p.name),
                score_home: g.score.as_ref().and_then(|s| s.home),
                score_away: g.score.as_ref().and_then(|s| s.away),
            })
            .collect(),
        bookings: api_response
            .bookings
            .unwrap_or_default()
            .into_iter()
            .map(|b| BookingItem {
                minute: b.minute,
                team_id: b.team.as_ref().map(|t| t.id),
                team_name: b.team.as_ref().map(|t| t.name.clone()),
                player: b.player.map(|p| p.name),
                card: b.card,
            })
            .collect(),
    };

    Ok(Json(result))
}

async fn get_competition_matches(
    axum::extract::Path(code): axum::extract::Path<String>,
    Query(query): Query<CompetitionMatchesQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<CompetitionMatchesResponse>, (StatusCode, String)> {
    let url = format!(
        "https://api.football-data.org/v4/competitions/{}/matches?matchday={}",
        code, query.matchday
    );

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

    let api_response: FootballDataMatchesResponse = response
        .json()
        .await
        .map_err(internal_error)?;

    let matches = api_response
        .matches
        .into_iter()
        .map(|m| MatchItem {
            id: m.id,
            utc_date: m.utc_date,
            status: m.status,
            matchday: m.matchday,
            stage: m.stage,
            home_team: MatchTeam {
                id: m.home_team.id,
                name: m.home_team.name,
                short_name: m.home_team.short_name,
                tla: m.home_team.tla,
                crest: m.home_team.crest,
            },
            away_team: MatchTeam {
                id: m.away_team.id,
                name: m.away_team.name,
                short_name: m.away_team.short_name,
                tla: m.away_team.tla,
                crest: m.away_team.crest,
            },
            score: MatchScore {
                home: m.score.full_time.home,
                away: m.score.full_time.away,
            },
        })
        .collect();

    let result = CompetitionMatchesResponse {
        competition: CompetitionInfo {
            id: api_response.competition.id,
            name: api_response.competition.name,
            code: api_response.competition.code,
            image_url: api_response.competition.emblem.unwrap_or_default(),
        },
        matches,
    };

    Ok(Json(result))
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
        .route("/competitions/:code/matches", get(get_competition_matches))
        .route("/matches/:id", get(get_match_details))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
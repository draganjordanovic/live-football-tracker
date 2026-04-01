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
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
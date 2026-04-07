mod app_state;
mod errors;
mod handlers;
mod models;
mod pdf;
mod routes;

use std::{env, sync::Arc};

use app_state::AppState;
use reqwest::Client;
use routes::create_router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let football_data_service_url = env::var("FOOTBALL_DATA_SERVICE_URL")
        .expect("FOOTBALL_DATA_SERVICE_URL must be set in .env file");

    let match_details_service_url = env::var("MATCH_DETAILS_SERVICE_URL")
        .expect("MATCH_DETAILS_SERVICE_URL must be set in .env file");

    let port = env::var("PORT").unwrap_or_else(|_| "3002".to_string());

    let state = Arc::new(AppState {
        http_client: Client::new(),
        football_data_service_url,
        match_details_service_url,
    });

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    println!(
        "Analytics & Reporting Service running on http://127.0.0.1:{}",
        port
    );

    axum::serve(listener, app).await.unwrap();
}
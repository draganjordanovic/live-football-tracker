mod app_state;
mod errors;
mod handlers;
mod models;
mod routes;

use app_state::AppState;
use reqwest::Client;
use std::{env, sync::Arc};
use routes::create_router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let api_key = env::var("FOOTBALL_DATA_API_KEY")
        .expect("FOOTBALL_DATA_API_KEY must be set in .env file");

    let state = Arc::new(AppState {
        client: Client::new(),
        api_key,
    });

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
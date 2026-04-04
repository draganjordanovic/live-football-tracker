mod app_state;
mod errors;
mod handlers;
mod models;
mod routes;
mod cache;
mod cache_keys;

use app_state::AppState;
use reqwest::Client;
use redis::Client as RedisClient;
use std::{env, sync::Arc};
use routes::create_router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let api_key = env::var("FOOTBALL_DATA_API_KEY")
        .expect("FOOTBALL_DATA_API_KEY must be set in .env file");

    let redis_url =
        env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379/".to_string());

    let redis_client =
        RedisClient::open(redis_url).expect("Failed to create Redis client");

    let state = Arc::new(AppState {
        client: Client::new(),
        api_key,
        redis_client,
    });

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
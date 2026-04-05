mod app_state;
mod cache;
mod cache_keys;
mod errors;
mod handlers;
mod models;
mod routes;
mod db;

use app_state::AppState;
use redis::Client as RedisClient;
use reqwest::Client;
use routes::create_router;
use sqlx::postgres::PgPoolOptions;
use std::{env, sync::Arc};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let api_key = env::var("FOOTBALL_DATA_API_KEY")
        .expect("FOOTBALL_DATA_API_KEY must be set in .env file");

    let redis_url =
        env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379/".to_string());

    let redis_client =
        RedisClient::open(redis_url).expect("Failed to create Redis client");

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to PostgreSQL");

    let state = Arc::new(AppState {
        client: Client::new(),
        api_key,
        redis_client,
        db,
    });

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
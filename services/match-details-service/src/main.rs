mod app_state;
mod errors;
mod handlers;
mod models;
mod routes;

use std::{env, sync::Arc};

use app_state::AppState;
use mongodb::Client as MongoClient;
use reqwest::Client;
use routes::create_router;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let mongo_url =
        env::var("MONGO_URL").expect("MONGO_URL must be set in .env file");

    let mongo_db_name =
        env::var("MONGO_DB_NAME").expect("MONGO_DB_NAME must be set in .env file");

    let football_data_service_url = env::var("FOOTBALL_DATA_SERVICE_URL")
        .expect("FOOTBALL_DATA_SERVICE_URL must be set in .env file");

    let port = env::var("PORT").unwrap_or_else(|_| "3001".to_string());

    let mongo_client = MongoClient::with_uri_str(&mongo_url)
        .await
        .expect("Failed to connect to MongoDB");

    let mongo_db = mongo_client.database(&mongo_db_name);

    let state = Arc::new(AppState {
        http_client: Client::new(),
        mongo_db,
        football_data_service_url,
    });

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    println!("Match Details Service running on http://127.0.0.1:{}", port);

    axum::serve(listener, app).await.unwrap();
}
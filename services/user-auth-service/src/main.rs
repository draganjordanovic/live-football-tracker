mod app_state;
mod routes;
mod handlers;
mod models;

use std::{env, sync::Arc};

use axum::Router;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::app_state::AppState;
use crate::routes::create_router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let jwt_secret =
        env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let port = env::var("PORT").unwrap_or_else(|_| "3003".to_string());

    // DB connection
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let state = Arc::new(AppState {
        db,
        jwt_secret,
    });

    let app: Router = create_router(state).layer(CorsLayer::permissive());

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
        .await
        .unwrap();

    println!("User/Auth Service running on http://127.0.0.1:{}", port);

    axum::serve(listener, app).await.unwrap();
}
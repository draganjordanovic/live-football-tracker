use axum::{
    routing::get,
    Json,
    Router,
};
use serde::Serialize;
use tower_http::cors::CorsLayer;

#[derive(Serialize)]
struct Competition {
    id: u32,
    name: String,
    code: String,
}

async fn get_competitions() -> Json<Vec<Competition>> {
    let competitions = vec![
        Competition {
            id: 1,
            name: "Premier League".to_string(),
            code: "PL".to_string(),
        },
        Competition {
            id: 2,
            name: "La Liga".to_string(),
            code: "PD".to_string(),
        },
        Competition {
            id: 3,
            name: "Serie A".to_string(),
            code: "SA".to_string(),
        },
    ];

    Json(competitions)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/competitions", get(get_competitions))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
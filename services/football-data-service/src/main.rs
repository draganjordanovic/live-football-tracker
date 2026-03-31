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
    image_url: String,
}

async fn get_competitions() -> Json<Vec<Competition>> {
    let competitions = vec![
        Competition {
            id: 1,
            name: "Premier League".to_string(),
            code: "PL".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/en/f/f2/Premier_League_Logo.svg".to_string(),
        },
        Competition {
            id: 2,
            name: "La Liga".to_string(),
            code: "PD".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/commons/1/13/LaLiga.svg".to_string(),
        },
        Competition {
            id: 3,
            name: "Serie A".to_string(),
            code: "SA".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/en/e/e1/Serie_A_logo_%282019%29.svg".to_string(),
        },
        Competition {
            id: 4,
            name: "Bundesliga".to_string(),
            code: "BL1".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/en/d/df/Bundesliga_logo_%282017%29.svg".to_string(),
        },
        Competition {
            id: 1,
            name: "Premier League".to_string(),
            code: "PL".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/en/f/f2/Premier_League_Logo.svg".to_string(),
        },
        Competition {
            id: 2,
            name: "La Liga".to_string(),
            code: "PD".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/commons/1/13/LaLiga.svg".to_string(),
        },
        Competition {
            id: 3,
            name: "Serie A".to_string(),
            code: "SA".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/en/e/e1/Serie_A_logo_%282019%29.svg".to_string(),
        },
        Competition {
            id: 4,
            name: "Bundesliga".to_string(),
            code: "BL1".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/en/d/df/Bundesliga_logo_%282017%29.svg".to_string(),
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
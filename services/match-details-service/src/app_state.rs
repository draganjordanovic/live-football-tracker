use mongodb::Database;
use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
    pub mongo_db: Database,
    pub football_data_service_url: String,
}
use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub http_client: Client,
    pub football_data_service_url: String,
    pub match_details_service_url: String,
}
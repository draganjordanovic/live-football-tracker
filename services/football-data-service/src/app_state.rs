use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub api_key: String,
}
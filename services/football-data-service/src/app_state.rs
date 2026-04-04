use redis::Client as RedisClient;
use reqwest::Client;

#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub api_key: String,
    pub redis_client: RedisClient,
}
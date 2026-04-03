use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Competition {
    pub id: u32,
    pub name: String,
    pub code: String,
    pub image_url: String,
}

#[derive(Deserialize)]
pub struct FootballDataCompetitionsResponse {
    pub competitions: Vec<FootballDataCompetition>,
}

#[derive(Deserialize)]
pub struct FootballDataCompetition {
    pub id: u32,
    pub name: String,
    pub code: String,
    pub emblem: Option<String>,
}
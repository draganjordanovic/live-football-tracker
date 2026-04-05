use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CompetitionInfo {
    pub id: u32,
    pub name: String,
    pub code: String,
    pub image_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct MatchTeam {
    pub id: u32,
    pub name: String,
    pub short_name: String,
    pub tla: Option<String>,
    pub crest: String,
}

#[derive(Deserialize)]
pub struct FootballDataStandingCompetition {
    pub id: u32,
    pub name: String,
    pub code: String,
    pub emblem: Option<String>,
}

#[derive(Deserialize)]
pub struct FootballDataMatchTeam {
    pub id: u32,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub tla: Option<String>,
    pub crest: String,
}

#[derive(Deserialize)]
pub struct FootballDataFullTimeScore {
    pub home: Option<i32>,
    pub away: Option<i32>,
}
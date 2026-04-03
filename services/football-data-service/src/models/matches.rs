use serde::{Deserialize, Serialize};

use super::common::{CompetitionInfo, FootballDataFullTimeScore, FootballDataMatchTeam, FootballDataStandingCompetition, MatchTeam};

#[derive(Serialize)]
pub struct CompetitionMatchesResponse {
    pub competition: CompetitionInfo,
    pub matches: Vec<MatchItem>,
}

#[derive(Serialize)]
pub struct MatchItem {
    pub id: u32,
    pub utc_date: String,
    pub status: String,
    pub matchday: Option<u32>,
    pub stage: Option<String>,
    pub home_team: MatchTeam,
    pub away_team: MatchTeam,
    pub score: MatchScore,
}

#[derive(Serialize)]
pub struct MatchScore {
    pub home: Option<i32>,
    pub away: Option<i32>,
}

#[derive(Deserialize)]
pub struct FootballDataMatchesResponse {
    pub competition: FootballDataStandingCompetition,
    pub matches: Vec<FootballDataMatch>,
}

#[derive(Deserialize)]
pub struct FootballDataMatch {
    pub id: u32,
    #[serde(rename = "utcDate")]
    pub utc_date: String,
    pub status: String,
    pub matchday: Option<u32>,
    pub stage: Option<String>,
    #[serde(rename = "homeTeam")]
    pub home_team: FootballDataMatchTeam,
    #[serde(rename = "awayTeam")]
    pub away_team: FootballDataMatchTeam,
    pub score: FootballDataMatchScore,
}

#[derive(Deserialize)]
pub struct FootballDataMatchScore {
    #[serde(rename = "fullTime")]
    pub full_time: FootballDataFullTimeScore,
}

#[derive(Deserialize)]
pub struct CompetitionMatchesQuery {
    pub matchday: u32,
}
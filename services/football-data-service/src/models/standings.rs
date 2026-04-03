use serde::{Deserialize, Serialize};

use super::common::FootballDataStandingCompetition;

#[derive(Serialize)]
pub struct CompetitionStandingResponse {
    pub competition: super::common::CompetitionInfo,
    pub season: SeasonInfo,
    pub standings: Vec<StandingGroup>,
}

#[derive(Serialize)]
pub struct SeasonInfo {
    pub current_matchday: Option<u32>,
}

#[derive(Serialize)]
pub struct StandingGroup {
    pub standing_type: String,
    pub table: Vec<TableRow>,
}

#[derive(Serialize)]
pub struct TableRow {
    pub position: u32,
    pub team_id: u32,
    pub team_name: String,
    pub team_short_name: String,
    pub team_tla: Option<String>,
    pub team_crest: String,
    pub played_games: u32,
    pub form: Option<String>,
    pub won: u32,
    pub draw: u32,
    pub lost: u32,
    pub points: u32,
    pub goals_for: i32,
    pub goals_against: i32,
    pub goal_difference: i32,
}

#[derive(Deserialize)]
pub struct FootballDataStandingsResponse {
    pub competition: FootballDataStandingCompetition,
    pub season: FootballDataStandingSeason,
    pub standings: Vec<FootballDataStanding>,
}

#[derive(Deserialize)]
pub struct FootballDataStandingSeason {
    #[serde(rename = "currentMatchday")]
    pub current_matchday: Option<u32>,
}

#[derive(Deserialize)]
pub struct FootballDataStanding {
    #[serde(rename = "type")]
    pub standing_type: String,
    pub table: Vec<FootballDataTableRow>,
}

#[derive(Deserialize)]
pub struct FootballDataTableRow {
    pub position: u32,
    pub team: FootballDataTableTeam,
    #[serde(rename = "playedGames")]
    pub played_games: u32,
    pub form: Option<String>,
    pub won: u32,
    pub draw: u32,
    pub lost: u32,
    pub points: u32,
    #[serde(rename = "goalsFor")]
    pub goals_for: i32,
    #[serde(rename = "goalsAgainst")]
    pub goals_against: i32,
    #[serde(rename = "goalDifference")]
    pub goal_difference: i32,
}

#[derive(Deserialize)]
pub struct FootballDataTableTeam {
    pub id: u32,
    pub name: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    pub tla: Option<String>,
    pub crest: String,
}
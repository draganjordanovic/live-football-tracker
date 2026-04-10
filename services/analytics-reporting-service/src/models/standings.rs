use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompetitionStandingResponse {
    pub competition: CompetitionInfo,
    pub season: SeasonInfo,
    pub standings: Vec<StandingGroup>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompetitionInfo {
    pub id: u32,
    pub name: String,
    pub code: String,
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SeasonInfo {
    pub current_matchday: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StandingGroup {
    pub standing_type: String,
    pub table: Vec<TableRow>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
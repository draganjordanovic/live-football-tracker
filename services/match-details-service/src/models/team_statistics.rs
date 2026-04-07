use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamStatisticsDocument {
    pub competition_code: String,
    pub competition_name: String,
    pub competition_id: u32,

    pub team_id: u32,
    pub team_name: String,
    pub team_short_name: String,
    pub team_tla: Option<String>,
    pub team_crest: String,

    pub current_matchday: Option<u32>,
    pub standing_type: String,
    pub position: u32,

    pub played_games: u32,
    pub won: u32,
    pub draw: u32,
    pub lost: u32,
    pub points: u32,

    pub goals_for: i32,
    pub goals_against: i32,
    pub goal_difference: i32,

    pub average_points_per_match: f64,
    pub average_goals_for_per_match: f64,
    pub average_goals_against_per_match: f64,
    pub average_goal_difference_per_match: f64,

    pub win_rate: f64,
    pub draw_rate: f64,
    pub loss_rate: f64,

    pub form: Option<String>,
    pub form_points: i32,

    pub last_synced_at: DateTime<Utc>,
}
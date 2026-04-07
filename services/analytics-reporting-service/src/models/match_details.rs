use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchDetailsResponse {
    pub id: u32,
    pub utc_date: String,
    pub status: String,
    pub venue: Option<String>,
    pub matchday: Option<u32>,
    pub stage: Option<String>,
    pub home_team: MatchTeam,
    pub away_team: MatchTeam,
    pub competition: MatchCompetitionInfo,
    pub score: DetailedMatchScore,
    pub referees: Vec<RefereeItem>,
    pub goals: Vec<GoalItem>,
    pub bookings: Vec<BookingItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchCompetitionInfo {
    pub id: u32,
    pub name: String,
    pub code: String,
    pub emblem: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetailedMatchScore {
    pub winner: Option<String>,
    pub full_time: ScorePair,
    pub half_time: ScorePair,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ScorePair {
    pub home: Option<i32>,
    pub away: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MatchTeam {
    pub id: u32,
    pub name: String,
    pub short_name: String,
    pub tla: Option<String>,
    pub crest: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RefereeItem {
    pub id: u32,
    pub name: String,
    pub r#type: Option<String>,
    pub nationality: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoalItem {
    pub minute: Option<u32>,
    pub injury_time: Option<u32>,
    pub team_id: Option<u32>,
    pub team_name: Option<String>,
    pub scorer: Option<String>,
    pub assist: Option<String>,
    pub score_home: Option<i32>,
    pub score_away: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BookingItem {
    pub minute: Option<u32>,
    pub team_id: Option<u32>,
    pub team_name: Option<String>,
    pub player: Option<String>,
    pub card: Option<String>,
}
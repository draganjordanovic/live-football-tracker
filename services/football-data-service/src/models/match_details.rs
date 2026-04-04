use serde::{Deserialize, Serialize};

use super::common::{FootballDataFullTimeScore, FootballDataMatchTeam, FootballDataStandingCompetition, MatchTeam};

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct MatchCompetitionInfo {
    pub id: u32,
    pub name: String,
    pub code: String,
    pub emblem: String,
}

#[derive(Serialize, Deserialize)]
pub struct DetailedMatchScore {
    pub winner: Option<String>,
    pub full_time: ScorePair,
    pub half_time: ScorePair,
}

#[derive(Serialize, Deserialize)]
pub struct ScorePair {
    pub home: Option<i32>,
    pub away: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct RefereeItem {
    pub id: u32,
    pub name: String,
    pub r#type: Option<String>,
    pub nationality: Option<String>,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
pub struct BookingItem {
    pub minute: Option<u32>,
    pub team_id: Option<u32>,
    pub team_name: Option<String>,
    pub player: Option<String>,
    pub card: Option<String>,
}

#[derive(Deserialize)]
pub struct FootballDataMatchDetailsResponse {
    pub id: u32,
    #[serde(rename = "utcDate")]
    pub utc_date: String,
    pub status: String,
    pub venue: Option<String>,
    pub matchday: Option<u32>,
    pub stage: Option<String>,
    #[serde(rename = "homeTeam")]
    pub home_team: FootballDataMatchTeam,
    #[serde(rename = "awayTeam")]
    pub away_team: FootballDataMatchTeam,
    pub competition: FootballDataStandingCompetition,
    pub score: FootballDataDetailedScore,
    pub referees: Vec<FootballDataReferee>,
    pub goals: Option<Vec<FootballDataGoal>>,
    pub bookings: Option<Vec<FootballDataBooking>>,
}

#[derive(Deserialize)]
pub struct FootballDataDetailedScore {
    pub winner: Option<String>,
    #[serde(rename = "fullTime")]
    pub full_time: FootballDataFullTimeScore,
    #[serde(rename = "halfTime")]
    pub half_time: FootballDataFullTimeScore,
}

#[derive(Deserialize)]
pub struct FootballDataReferee {
    pub id: u32,
    pub name: String,
    #[serde(rename = "type")]
    pub r#type: Option<String>,
    pub nationality: Option<String>,
}

#[derive(Deserialize)]
pub struct FootballDataGoal {
    pub minute: Option<u32>,
    #[serde(rename = "injuryTime")]
    pub injury_time: Option<u32>,
    pub team: Option<FootballDataEventTeam>,
    pub scorer: Option<FootballDataPerson>,
    pub assist: Option<FootballDataPerson>,
    pub score: Option<FootballDataFullTimeScore>,
}

#[derive(Deserialize)]
pub struct FootballDataBooking {
    pub minute: Option<u32>,
    pub team: Option<FootballDataEventTeam>,
    pub player: Option<FootballDataPerson>,
    pub card: Option<String>,
}

#[derive(Deserialize)]
pub struct FootballDataEventTeam {
    pub id: u32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct FootballDataPerson {
    pub name: String,
}
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct FavoriteClub {
    pub id: i32,
    pub user_id: uuid::Uuid,
    pub club_external_id: i32,
    pub competition_code: String,
    pub club_name: String,
    pub club_short_name: Option<String>,
    pub club_crest: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateFavoriteClubRequest {
    pub club_external_id: i32,
    pub competition_code: String,
    pub club_name: String,
    pub club_short_name: Option<String>,
    pub club_crest: Option<String>,
}
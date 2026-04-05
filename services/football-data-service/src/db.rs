use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};

use crate::models::{
    common::MatchTeam,
    competitions::Competition,
    match_details::MatchDetailsResponse,
};

pub async fn upsert_competition(
    db: &PgPool,
    external_id: u32,
    name: &str,
    code: &str,
    emblem_url: &str,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO competitions (external_id, name, code, emblem_url, last_synced_at)
        VALUES ($1, $2, $3, $4, NOW())
        ON CONFLICT (external_id)
        DO UPDATE SET
            name = EXCLUDED.name,
            code = EXCLUDED.code,
            emblem_url = EXCLUDED.emblem_url,
            last_synced_at = NOW()
        RETURNING id
        "#,
    )
    .bind(i32::try_from(external_id).unwrap())
    .bind(name)
    .bind(code)
    .bind(emblem_url)
    .fetch_one(db)
    .await?;

    Ok(row.get::<i64, _>("id"))
}

pub async fn upsert_team(
    db: &PgPool,
    team: &MatchTeam,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO teams (external_id, name, short_name, tla, crest_url, last_synced_at)
        VALUES ($1, $2, $3, $4, $5, NOW())
        ON CONFLICT (external_id)
        DO UPDATE SET
            name = EXCLUDED.name,
            short_name = EXCLUDED.short_name,
            tla = EXCLUDED.tla,
            crest_url = EXCLUDED.crest_url,
            last_synced_at = NOW()
        RETURNING id
        "#,
    )
    .bind(i32::try_from(team.id).unwrap())
    .bind(&team.name)
    .bind(&team.short_name)
    .bind(&team.tla)
    .bind(&team.crest)
    .fetch_one(db)
    .await?;

    Ok(row.get::<i64, _>("id"))
}

pub async fn upsert_match(
    db: &PgPool,
    match_details: &MatchDetailsResponse,
    competition_id: i64,
    home_team_id: i64,
    away_team_id: i64,
) -> Result<i64, sqlx::Error> {
    let utc_date: DateTime<Utc> = match_details.utc_date.parse().map_err(|e| {
        sqlx::Error::Protocol(format!("Failed to parse utc_date '{}': {}", match_details.utc_date, e))
    })?;

    let row = sqlx::query(
        r#"
        INSERT INTO matches (
            external_id,
            competition_id,
            home_team_id,
            away_team_id,
            utc_date,
            status,
            venue,
            matchday,
            stage,
            full_time_home,
            full_time_away,
            half_time_home,
            half_time_away,
            winner,
            last_synced_at
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, NOW()
        )
        ON CONFLICT (external_id)
        DO UPDATE SET
            competition_id = EXCLUDED.competition_id,
            home_team_id = EXCLUDED.home_team_id,
            away_team_id = EXCLUDED.away_team_id,
            utc_date = EXCLUDED.utc_date,
            status = EXCLUDED.status,
            venue = EXCLUDED.venue,
            matchday = EXCLUDED.matchday,
            stage = EXCLUDED.stage,
            full_time_home = EXCLUDED.full_time_home,
            full_time_away = EXCLUDED.full_time_away,
            half_time_home = EXCLUDED.half_time_home,
            half_time_away = EXCLUDED.half_time_away,
            winner = EXCLUDED.winner,
            last_synced_at = NOW()
        RETURNING id
        "#,
    )
    .bind(i32::try_from(match_details.id).unwrap())
    .bind(competition_id)
    .bind(home_team_id)
    .bind(away_team_id)
    .bind(utc_date)
    .bind(&match_details.status)
    .bind(&match_details.venue)
    .bind(match_details.matchday.map(|v| i32::try_from(v).unwrap()))
    .bind(&match_details.stage)
    .bind(match_details.score.full_time.home)
    .bind(match_details.score.full_time.away)
    .bind(match_details.score.half_time.home)
    .bind(match_details.score.half_time.away)
    .bind(&match_details.score.winner)
    .fetch_one(db)
    .await?;

    Ok(row.get::<i64, _>("id"))
}

pub async fn replace_match_events(
    db: &PgPool,
    match_db_id: i64,
    home_team_db_id: i64,
    away_team_db_id: i64,
    match_details: &MatchDetailsResponse,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        DELETE FROM match_events
        WHERE match_id = $1
        "#,
    )
    .bind(match_db_id)
    .execute(db)
    .await?;

    for goal in &match_details.goals {
        let team_db_id = map_team_external_to_db_id(
            goal.team_id,
            match_details.home_team.id,
            home_team_db_id,
            match_details.away_team.id,
            away_team_db_id,
        );

        sqlx::query(
            r#"
            INSERT INTO match_events (
                match_id,
                event_type,
                minute,
                injury_time,
                team_id,
                player_name,
                assist_name,
                card_type,
                score_home,
                score_away
            )
            VALUES ($1, 'GOAL', $2, $3, $4, $5, $6, NULL, $7, $8)
            "#,
        )
        .bind(match_db_id)
        .bind(goal.minute.map(|v| i32::try_from(v).unwrap()))
        .bind(goal.injury_time.map(|v| i32::try_from(v).unwrap()))
        .bind(team_db_id)
        .bind(&goal.scorer)
        .bind(&goal.assist)
        .bind(goal.score_home)
        .bind(goal.score_away)
        .execute(db)
        .await?;
    }

    for booking in &match_details.bookings {
        let team_db_id = map_team_external_to_db_id(
            booking.team_id,
            match_details.home_team.id,
            home_team_db_id,
            match_details.away_team.id,
            away_team_db_id,
        );

        sqlx::query(
            r#"
            INSERT INTO match_events (
                match_id,
                event_type,
                minute,
                injury_time,
                team_id,
                player_name,
                assist_name,
                card_type,
                score_home,
                score_away
            )
            VALUES ($1, 'BOOKING', $2, NULL, $3, $4, NULL, $5, NULL, NULL)
            "#,
        )
        .bind(match_db_id)
        .bind(booking.minute.map(|v| i32::try_from(v).unwrap()))
        .bind(team_db_id)
        .bind(&booking.player)
        .bind(&booking.card)
        .execute(db)
        .await?;
    }

    Ok(())
}

fn map_team_external_to_db_id(
    event_team_external_id: Option<u32>,
    home_external_id: u32,
    home_db_id: i64,
    away_external_id: u32,
    away_db_id: i64,
) -> Option<i64> {
    match event_team_external_id {
        Some(id) if id == home_external_id => Some(home_db_id),
        Some(id) if id == away_external_id => Some(away_db_id),
        _ => None,
    }
}

pub async fn upsert_competition_summary(
    db: &PgPool,
    competition: &Competition,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        r#"
        INSERT INTO competitions (external_id, name, code, emblem_url, last_synced_at)
        VALUES ($1, $2, $3, $4, NOW())
        ON CONFLICT (external_id)
        DO UPDATE SET
            name = EXCLUDED.name,
            code = EXCLUDED.code,
            emblem_url = EXCLUDED.emblem_url,
            last_synced_at = NOW()
        RETURNING id
        "#,
    )
    .bind(i32::try_from(competition.id).unwrap())
    .bind(&competition.name)
    .bind(&competition.code)
    .bind(&competition.image_url)
    .fetch_one(db)
    .await?;

    Ok(row.get::<i64, _>("id"))
}

pub async fn upsert_match_summary(
    db: &PgPool,
    external_id: u32,
    competition_id: i64,
    home_team_id: i64,
    away_team_id: i64,
    utc_date_raw: &str,
    status: &str,
    matchday: Option<u32>,
    stage: &Option<String>,
    full_time_home: Option<i32>,
    full_time_away: Option<i32>,
) -> Result<i64, sqlx::Error> {
    let utc_date = chrono::DateTime::parse_from_rfc3339(utc_date_raw)
        .map_err(|e| sqlx::Error::Protocol(format!("Failed to parse utc_date '{}': {}", utc_date_raw, e)))?
        .with_timezone(&Utc);

    let row = sqlx::query(
        r#"
        INSERT INTO matches (
            external_id,
            competition_id,
            home_team_id,
            away_team_id,
            utc_date,
            status,
            venue,
            matchday,
            stage,
            full_time_home,
            full_time_away,
            half_time_home,
            half_time_away,
            winner,
            last_synced_at
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, NULL, $7, $8, $9, $10, NULL, NULL, NULL, NOW()
        )
        ON CONFLICT (external_id)
        DO UPDATE SET
            competition_id = EXCLUDED.competition_id,
            home_team_id = EXCLUDED.home_team_id,
            away_team_id = EXCLUDED.away_team_id,
            utc_date = EXCLUDED.utc_date,
            status = EXCLUDED.status,
            matchday = EXCLUDED.matchday,
            stage = EXCLUDED.stage,
            full_time_home = EXCLUDED.full_time_home,
            full_time_away = EXCLUDED.full_time_away,
            last_synced_at = NOW()
        RETURNING id
        "#,
    )
    .bind(i32::try_from(external_id).unwrap())
    .bind(competition_id)
    .bind(home_team_id)
    .bind(away_team_id)
    .bind(utc_date)
    .bind(status)
    .bind(matchday.map(|v| i32::try_from(v).unwrap()))
    .bind(stage)
    .bind(full_time_home)
    .bind(full_time_away)
    .fetch_one(db)
    .await?;

    Ok(row.get::<i64, _>("id"))
}
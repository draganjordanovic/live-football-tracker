use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::{
    app_state::AppState,
    cache,
    cache_keys::{
    MATCH_DETAILS_FINISHED_TTL,
    MATCH_DETAILS_LIVE_TTL,
    MATCH_DETAILS_SCHEDULED_TTL,
    MATCHES_TTL,
},
    errors::internal_error,
    models::{
        common::{CompetitionInfo, MatchTeam},
        matches::{
            CompetitionMatchesQuery,
            CompetitionMatchesResponse,
            FootballDataMatchesResponse,
            MatchItem,
            MatchScore,
        },
        match_details::{
            BookingItem,
            DetailedMatchScore,
            FootballDataMatchDetailsResponse,
            GoalItem,
            MatchCompetitionInfo,
            MatchDetailsResponse,
            RefereeItem,
            ScorePair,
        },
    },
};


/// GET /competitions/:code/matches?matchday=...
pub async fn get_competition_matches(
    Path(code): Path<String>,
    Query(query): Query<CompetitionMatchesQuery>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<CompetitionMatchesResponse>, (StatusCode, String)> {
    let cache_key = format!("competition:{}:matches:{}", code, query.matchday);

    if let Ok(Some(cached)) =
        cache::get_json::<CompetitionMatchesResponse>(&state.redis_client, &cache_key).await
    {
        println!("Cache HIT: {}", cache_key);
        return Ok(Json(cached));
    }

    println!("Cache MISS: {}", cache_key);

    let url = format!(
        "https://api.football-data.org/v4/competitions/{}/matches?matchday={}",
        code, query.matchday
    );

    let response = state
        .client
        .get(&url)
        .header("X-Auth-Token", &state.api_key)
        .send()
        .await
        .map_err(internal_error)?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error body".to_string());

        return Err((
            StatusCode::BAD_GATEWAY,
            format!("Football Data API error: {} - {}", status, body),
        ));
    }

    let api_response: FootballDataMatchesResponse =
        response.json().await.map_err(internal_error)?;

    let matches = api_response
        .matches
        .into_iter()
        .map(|m| MatchItem {
            id: m.id,
            utc_date: m.utc_date,
            status: m.status,
            matchday: m.matchday,
            stage: m.stage,
            home_team: MatchTeam {
                id: m.home_team.id,
                name: m.home_team.name,
                short_name: m.home_team.short_name,
                tla: m.home_team.tla,
                crest: m.home_team.crest,
            },
            away_team: MatchTeam {
                id: m.away_team.id,
                name: m.away_team.name,
                short_name: m.away_team.short_name,
                tla: m.away_team.tla,
                crest: m.away_team.crest,
            },
            score: MatchScore {
                home: m.score.full_time.home,
                away: m.score.full_time.away,
            },
        })
        .collect();

    let result = CompetitionMatchesResponse {
        competition: CompetitionInfo {
            id: api_response.competition.id,
            name: api_response.competition.name,
            code: api_response.competition.code,
            image_url: api_response.competition.emblem.unwrap_or_default(),
        },
        matches,
    };

    let _ = cache::set_json(&state.redis_client, &cache_key, MATCHES_TTL, &result).await;

    Ok(Json(result))
}

/// GET /matches/:id
pub async fn get_match_details(
    Path(id): Path<u32>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<MatchDetailsResponse>, (StatusCode, String)> {
    let cache_key = format!("match:{}:details", id);

    if let Ok(Some(cached)) =
        cache::get_json::<MatchDetailsResponse>(&state.redis_client, &cache_key).await
    {
        println!("Cache HIT: {}", cache_key);
        return Ok(Json(cached));
    }

    println!("Cache MISS: {}", cache_key);

    let url = format!("https://api.football-data.org/v4/matches/{}", id);

    let response = state
        .client
        .get(&url)
        .header("X-Auth-Token", &state.api_key)
        .send()
        .await
        .map_err(internal_error)?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response
            .text()
            .await
            .unwrap_or_else(|_| "Failed to read error body".to_string());

        return Err((
            StatusCode::BAD_GATEWAY,
            format!("Football Data API error: {} - {}", status, body),
        ));
    }

    let api_response: FootballDataMatchDetailsResponse =
        response.json().await.map_err(internal_error)?;

    let result = MatchDetailsResponse {
        id: api_response.id,
        utc_date: api_response.utc_date,
        status: api_response.status.clone(),
        venue: api_response.venue,
        matchday: api_response.matchday,
        stage: api_response.stage,

        home_team: MatchTeam {
            id: api_response.home_team.id,
            name: api_response.home_team.name,
            short_name: api_response.home_team.short_name,
            tla: api_response.home_team.tla,
            crest: api_response.home_team.crest,
        },

        away_team: MatchTeam {
            id: api_response.away_team.id,
            name: api_response.away_team.name,
            short_name: api_response.away_team.short_name,
            tla: api_response.away_team.tla,
            crest: api_response.away_team.crest,
        },

        competition: MatchCompetitionInfo {
            id: api_response.competition.id,
            name: api_response.competition.name,
            code: api_response.competition.code,
            emblem: api_response.competition.emblem.unwrap_or_default(),
        },

        score: DetailedMatchScore {
            winner: api_response.score.winner,
            full_time: ScorePair {
                home: api_response.score.full_time.home,
                away: api_response.score.full_time.away,
            },
            half_time: ScorePair {
                home: api_response.score.half_time.home,
                away: api_response.score.half_time.away,
            },
        },

        referees: api_response
            .referees
            .into_iter()
            .map(|r| RefereeItem {
                id: r.id,
                name: r.name,
                r#type: r.r#type,
                nationality: r.nationality,
            })
            .collect(),

        goals: api_response
            .goals
            .unwrap_or_default()
            .into_iter()
            .map(|g| GoalItem {
                minute: g.minute,
                injury_time: g.injury_time,
                team_id: g.team.as_ref().map(|t| t.id),
                team_name: g.team.as_ref().map(|t| t.name.clone()),
                scorer: g.scorer.map(|p| p.name),
                assist: g.assist.map(|p| p.name),
                score_home: g.score.as_ref().and_then(|s| s.home),
                score_away: g.score.as_ref().and_then(|s| s.away),
            })
            .collect(),

        bookings: api_response
            .bookings
            .unwrap_or_default()
            .into_iter()
            .map(|b| BookingItem {
                minute: b.minute,
                team_id: b.team.as_ref().map(|t| t.id),
                team_name: b.team.as_ref().map(|t| t.name.clone()),
                player: b.player.map(|p| p.name),
                card: b.card,
            })
            .collect(),
    };

    let ttl = match result.status.as_str() {
        "IN_PLAY" | "PAUSED" => MATCH_DETAILS_LIVE_TTL,
        "FINISHED" => MATCH_DETAILS_FINISHED_TTL,
        _ => MATCH_DETAILS_SCHEDULED_TTL,
    };

    let _ = cache::set_json(&state.redis_client, &cache_key, ttl, &result).await;

    Ok(Json(result))
}
CREATE TABLE IF NOT EXISTS competitions (
    id BIGSERIAL PRIMARY KEY,
    external_id INTEGER NOT NULL UNIQUE,
    name TEXT NOT NULL,
    code TEXT NOT NULL UNIQUE,
    emblem_url TEXT,
    last_synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS teams (
    id BIGSERIAL PRIMARY KEY,
    external_id INTEGER NOT NULL UNIQUE,
    name TEXT NOT NULL,
    short_name TEXT NOT NULL,
    tla TEXT,
    crest_url TEXT,
    last_synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS matches (
    id BIGSERIAL PRIMARY KEY,
    external_id INTEGER NOT NULL UNIQUE,
    competition_id BIGINT NOT NULL REFERENCES competitions(id),
    home_team_id BIGINT NOT NULL REFERENCES teams(id),
    away_team_id BIGINT NOT NULL REFERENCES teams(id),
    utc_date TIMESTAMPTZ NOT NULL,
    status TEXT NOT NULL,
    venue TEXT,
    matchday INTEGER,
    stage TEXT,
    full_time_home INTEGER,
    full_time_away INTEGER,
    half_time_home INTEGER,
    half_time_away INTEGER,
    winner TEXT,
    last_synced_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS match_events (
    id BIGSERIAL PRIMARY KEY,
    match_id BIGINT NOT NULL REFERENCES matches(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    minute INTEGER,
    injury_time INTEGER,
    team_id BIGINT REFERENCES teams(id),
    player_name TEXT,
    assist_name TEXT,
    card_type TEXT,
    score_home INTEGER,
    score_away INTEGER
);

CREATE TABLE IF NOT EXISTS competition_standings_snapshots (
    id BIGSERIAL PRIMARY KEY,
    competition_id BIGINT NOT NULL REFERENCES competitions(id) ON DELETE CASCADE,
    current_matchday INTEGER,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS competition_standing_rows (
    id BIGSERIAL PRIMARY KEY,
    snapshot_id BIGINT NOT NULL REFERENCES competition_standings_snapshots(id) ON DELETE CASCADE,
    team_id BIGINT NOT NULL REFERENCES teams(id),
    standing_type TEXT NOT NULL,
    position INTEGER NOT NULL,
    played_games INTEGER NOT NULL,
    form TEXT,
    won INTEGER NOT NULL,
    draw INTEGER NOT NULL,
    lost INTEGER NOT NULL,
    points INTEGER NOT NULL,
    goals_for INTEGER NOT NULL,
    goals_against INTEGER NOT NULL,
    goal_difference INTEGER NOT NULL
);
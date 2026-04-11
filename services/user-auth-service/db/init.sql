CREATE DATABASE user_auth_service;

-- USERS
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL,
    updated_at TIMESTAMP NOT NULL
);

-- FAVORITE CLUBS
CREATE TABLE IF NOT EXISTS favorite_clubs (
    id SERIAL PRIMARY KEY,
    user_id UUID NOT NULL,
    club_external_id INT NOT NULL,
    competition_code TEXT NOT NULL,
    club_name TEXT NOT NULL,
    club_short_name TEXT,
    club_crest TEXT,
    created_at TIMESTAMP NOT NULL,

    UNIQUE(user_id, club_external_id)
);
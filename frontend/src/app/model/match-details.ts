export interface MatchDetailsResponse {
  id: number;
  utc_date: string;
  status: string;
  venue: string | null;
  matchday: number | null;
  stage: string | null;
  home_team: MatchTeam;
  away_team: MatchTeam;
  competition: MatchCompetitionInfo;
  score: DetailedMatchScore;
  referees: RefereeItem[];
  goals: GoalItem[];
  bookings: BookingItem[];
}

export interface MatchTeam {
  id: number;
  name: string;
  short_name: string;
  tla: string | null;
  crest: string;
}

export interface MatchCompetitionInfo {
  id: number;
  name: string;
  code: string;
  emblem: string;
}

export interface DetailedMatchScore {
  winner: string | null;
  full_time: ScorePair;
  half_time: ScorePair;
}

export interface ScorePair {
  home: number | null;
  away: number | null;
}

export interface RefereeItem {
  id: number;
  name: string;
  type: string | null;
  nationality: string | null;
}

export interface GoalItem {
  minute: number | null;
  injury_time: number | null;
  team_id: number | null;
  team_name: string | null;
  scorer: string | null;
  assist: string | null;
  score_home: number | null;
  score_away: number | null;
}

export interface BookingItem {
  minute: number | null;
  team_id: number | null;
  team_name: string | null;
  player: string | null;
  card: string | null;
}
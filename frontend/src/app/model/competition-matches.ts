export interface CompetitionMatchesResponse {
  competition: MatchCompetitionInfo;
  matches: MatchItem[];
}

export interface MatchCompetitionInfo {
  id: number;
  name: string;
  code: string;
  image_url: string;
}

export interface MatchItem {
  id: number;
  utc_date: string;
  status: string;
  matchday: number | null;
  stage: string | null;
  home_team: MatchTeam;
  away_team: MatchTeam;
  score: MatchScore;
}

export interface MatchTeam {
  id: number;
  name: string;
  short_name: string;
  tla: string | null;
  crest: string;
}

export interface MatchScore {
  home: number | null;
  away: number | null;
}
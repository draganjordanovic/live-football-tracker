export interface CompetitionStandingsResponse {
  competition: CompetitionInfo;
  season: SeasonInfo;
  standings: StandingGroup[];
}

export interface CompetitionInfo {
  id: number;
  name: string;
  code: string;
  image_url: string;
}

export interface SeasonInfo {
  current_matchday: number | null;
}

export interface StandingGroup {
  standing_type: string;
  table: TableRow[];
}

export interface TableRow {
  position: number;
  team_id: number;
  team_name: string;
  team_short_name: string;
  team_tla: string | null;
  team_crest: string;
  played_games: number;
  form: string | null;
  won: number;
  draw: number;
  lost: number;
  points: number;
  goals_for: number;
  goals_against: number;
  goal_difference: number;
}
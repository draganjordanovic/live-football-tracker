export interface TeamStatisticsResponse {
  competition_code: string;
  competition_name: string;
  competition_id: number;

  team_id: number;
  team_name: string;
  team_short_name: string;
  team_tla: string | null;
  team_crest: string;

  current_matchday: number | null;
  standing_type: string;
  position: number;

  played_games: number;
  won: number;
  draw: number;
  lost: number;
  points: number;

  goals_for: number;
  goals_against: number;
  goal_difference: number;

  average_points_per_match: number;
  average_goals_for_per_match: number;
  average_goals_against_per_match: number;
  average_goal_difference_per_match: number;

  win_rate: number;
  draw_rate: number;
  loss_rate: number;

  form: string | null;
  form_points: number;

  last_synced_at: string;
}
export interface Player {
  id: number;
  nickname: string;
  community_id: number;
  created_at: string;
  updated_at: string;
}

export interface Team {
  id: number;
  name: string;
  community_id: number;
  players: Player[];
  created_at: string;
  updated_at: string;
}

export interface Party {
  id: number;
  community_id: number;
  game_name: string;
  team_winner_id?: number;
  finished_at?: string;
  created_at: string;
  updated_at: string;
  teams: Team[];
}

export interface Community {
  id: number;
  name: string;
  created_at: string;
  updated_at: string;
  players: Player[];
  teams: Team[];
}

export interface CommunitiesResponse {
  data: Community[];
  timestamp: string;
}

export interface CreateCommunityRequest {
  name: string;
}

export interface ErrorResponse {
  message: string;
  timestamp: string;
} 
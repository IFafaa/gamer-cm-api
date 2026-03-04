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

export interface PaginationMeta {
  total: number;
  page: number;
  limit: number;
  total_pages: number;
  has_next_page: boolean;
  has_previous_page: boolean;
}

export interface PaginationParams {
  page?: number;
  limit?: number;
}

export interface PaginatedResponse<T> {
  data: T;
  meta?: PaginationMeta;
  timestamp: string;
}

export type CommunitiesResponse = PaginatedResponse<Community[]>;

export interface CreateCommunityRequest {
  name: string;
}

export interface ErrorResponse {
  message: string;
  timestamp: string;
}

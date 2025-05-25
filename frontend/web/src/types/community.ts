export interface Community {
  id: number;
  name: string;
  created_at: string;
  updated_at: string;
  players: unknown[];
  teams: unknown[];
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
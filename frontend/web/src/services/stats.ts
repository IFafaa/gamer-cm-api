import axios from 'axios';
import api from '@/lib/api';

export interface TeamRanking {
  team_id: number;
  team_name: string;
  wins: number;
  total_parties: number;
  win_rate: number;
}

export interface PlayerRanking {
  player_id: number;
  player_nickname: string;
  wins: number;
  total_parties: number;
  win_rate: number;
}

export interface GameStats {
  game_name: string;
  times_played: number;
}

export interface CommunityStats {
  community_id: number;
  community_name: string;
  total_players: number;
  total_teams: number;
  total_parties: number;
  active_parties: number;
  finished_parties: number;
  team_rankings: TeamRanking[];
  player_rankings: PlayerRanking[];
  most_played_games: GameStats[];
}

interface ApiResponse<T> {
  data: T;
  timestamp: string;
}

export async function getCommunityStats(communityId: number): Promise<ApiResponse<CommunityStats>> {
  try {
    const { data } = await api.get(`/stats/communities/${communityId}`);
    return data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to fetch stats');
    }
    throw error;
  }
}

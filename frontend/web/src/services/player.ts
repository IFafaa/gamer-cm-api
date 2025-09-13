import axios from 'axios';
import api from '@/lib/api';

interface CreatePlayerParams {
  nickname: string;
  community_id: number;
}

interface Player {
  id: number;
  nickname: string;
  created_at: string;
  updated_at: string;
}

interface ApiResponse<T> {
  data: T;
  timestamp: string;
}

export async function createPlayer({ nickname, community_id }: CreatePlayerParams): Promise<ApiResponse<Player>> {
  try {
    const { data } = await api.post('/players', {
      nickname,
      community_id,
    });
    return data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to create player');
    }
    throw error;
  }
}

export async function deletePlayer(playerId: number): Promise<void> {
  try {
    await api.delete(`/players/${playerId}`);
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to delete player');
    }
    throw error;
  }
} 
import axios from 'axios';

interface CreateTeamParams {
  name: string;
  community_id: number;
}

interface Team {
  id: number;
  name: string;
  created_at: string;
  updated_at: string;
  players: unknown[];
}

interface ApiResponse<T> {
  data: T;
  timestamp: string;
}

const api = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API_URL,
});

export async function createTeam({ name, community_id }: CreateTeamParams): Promise<ApiResponse<Team>> {
  try {
    const { data } = await api.post('/teams', {
      name,
      community_id,
    });
    return data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to create team');
    }
    throw error;
  }
} 
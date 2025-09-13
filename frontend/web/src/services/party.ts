import axios from 'axios';

interface CreatePartyParams {
  game_name: string;
  teams_ids: number[];
  community_id: number;
}

interface EndPartyParams {
  party_id: number;
  team_winner_id?: number;
}

interface Party {
  id: number;
  community_id: number;
  game_name: string;
  team_winner_id?: number;
  finished_at?: string;
  created_at: string;
  updated_at: string;
  teams: Team[];
}

interface Team {
  id: number;
  name: string;
  community_id: number;
  players: Player[];
  created_at: string;
  updated_at: string;
}

interface Player {
  id: number;
  nickname: string;
  community_id: number;
  created_at: string;
  updated_at: string;
}

interface ApiResponse<T> {
  data: T;
  timestamp: string;
}

const api = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API_URL,
});

export async function createParty({ game_name, teams_ids, community_id }: CreatePartyParams): Promise<ApiResponse<Party>> {
  try {
    const { data } = await api.post('/parties', {
      game_name,
      teams_ids,
      community_id,
    });
    return data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to create party');
    }
    throw error;
  }
}

export async function endParty({ party_id, team_winner_id }: EndPartyParams): Promise<void> {
  try {
    await api.patch('/parties/end', {
      party_id,
      team_winner_id,
    });
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to end party');
    }
    throw error;
  }
}

export async function getParties(): Promise<ApiResponse<Party[]>> {
  try {
    const { data } = await api.get('/parties');
    
    // Se o backend retorna uma mensagem de "no parties found", 
    // tratamos como uma lista vazia (não é um erro)
    if (data.message && data.message.includes("No parties found")) {
      return {
        data: [],
        timestamp: data.timestamp
      };
    }
    
    return data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to fetch parties');
    }
    throw error;
  }
}

export async function getPartyById(partyId: number): Promise<ApiResponse<Party>> {
  try {
    const { data } = await api.get(`/parties/${partyId}`);
    return data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to fetch party');
    }
    throw error;
  }
}

export async function deleteParty(partyId: number): Promise<void> {
  try {
    await api.delete(`/parties/${partyId}`);
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to delete party');
    }
    throw error;
  }
}

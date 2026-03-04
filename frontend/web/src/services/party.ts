import axios from 'axios';
import api from '@/lib/api';
import { Party, PaginatedResponse, PaginationParams } from '@/types/community';

interface CreatePartyParams {
  game_name: string;
  teams_ids: number[];
  community_id: number;
}

interface EndPartyParams {
  party_id: number;
  team_winner_id?: number;
}

type ApiResponse<T> = PaginatedResponse<T>;

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

export async function getParties(
  communityId?: number,
  params?: PaginationParams
): Promise<ApiResponse<Party[]>> {
  try {
    const query = {
      ...(params ?? {}),
      community_id: communityId,
    };
    const { data } = await api.get('/parties', { params: query });
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

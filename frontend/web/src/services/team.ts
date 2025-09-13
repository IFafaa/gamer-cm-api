import axios from 'axios';
import api from '@/lib/api';

interface CreateTeamParams {
  name: string;
  community_id: number;
}

interface AddPlayersToTeamParams {
  team_id: number;
  players_ids: number[];
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

export async function addPlayersToTeam({ team_id, players_ids }: AddPlayersToTeamParams): Promise<void> {
  try {
    await api.post('/teams/add-players', {
      team_id,
      players_ids,
    });
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to add players to team');
    }
    throw error;
  }
}

export async function deletePlayersOfTeam({ team_id, name, players_ids }: { team_id: number; name?: string; players_ids: number[] }): Promise<void> {
  try {
    await api.patch('/teams/delete-players', {
      team_id,
      name,
      player_ids: players_ids,
    });
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to update team players');
    }
    throw error;
  }
}

export async function updateTeam(teamId: number, name: string): Promise<void> {
  try {
    await api.patch(`/teams/${teamId}`, {
      name,
    });
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to update team');
    }
    throw error;
  }
}

export async function deleteTeam(teamId: number): Promise<void> {
  try {
    await api.delete(`/teams/${teamId}`);
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to delete team');
    }
    throw error;
  }
}
import {
  createTeam,
  updateTeam,
  addPlayersToTeam,
  deletePlayersOfTeam,
} from '@/services/team';

jest.mock('@/lib/api', () => ({
  __esModule: true,
  default: {
    get: jest.fn(),
    post: jest.fn(),
    put: jest.fn(),
    patch: jest.fn(),
    delete: jest.fn(),
  },
}));

import api from '@/lib/api';
const mockedApi = api as jest.Mocked<typeof api>;

const teamFixture = {
  id: 1,
  name: 'Alpha Squad',
  created_at: '2024-01-01T00:00:00Z',
  updated_at: '2024-01-01T00:00:00Z',
  players: [],
};

afterEach(() => {
  jest.clearAllMocks();
});

describe('createTeam', () => {
  it('returns created team on success', async () => {
    (mockedApi.post as jest.Mock).mockResolvedValueOnce({
      data: { data: teamFixture, timestamp: '2024-01-01T00:00:00Z' },
    });
    const result = await createTeam({ name: 'Alpha Squad', community_id: 1 });
    expect(result.data.name).toBe('Alpha Squad');
    expect(mockedApi.post).toHaveBeenCalledWith('/teams', {
      name: 'Alpha Squad',
      community_id: 1,
    });
  });

  it('throws with server message on conflict', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '409', undefined, undefined, {
      status: 409,
      data: { message: 'Team already exists' },
    } as any);
    (mockedApi.post as jest.Mock).mockRejectedValueOnce(error);
    await expect(createTeam({ name: 'Alpha Squad', community_id: 1 })).rejects.toThrow(
      'Team already exists'
    );
  });

  it('throws generic message when no server message', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '500', undefined, undefined, {
      status: 500,
      data: {},
    } as any);
    (mockedApi.post as jest.Mock).mockRejectedValueOnce(error);
    await expect(createTeam({ name: 'X', community_id: 1 })).rejects.toThrow(
      'Failed to create team'
    );
  });
});

describe('updateTeam', () => {
  it('resolves on success', async () => {
    (mockedApi.put as jest.Mock).mockResolvedValueOnce({ data: {} });
    await expect(updateTeam(1, 'New Name')).resolves.not.toThrow();
    expect(mockedApi.put).toHaveBeenCalledWith('/teams/1', { name: 'New Name' });
  });

  it('throws with server message on not found', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '404', undefined, undefined, {
      status: 404,
      data: { message: 'Team not found' },
    } as any);
    (mockedApi.put as jest.Mock).mockRejectedValueOnce(error);
    await expect(updateTeam(99, 'X')).rejects.toThrow('Team not found');
  });
});

describe('addPlayersToTeam', () => {
  it('resolves on success', async () => {
    (mockedApi.post as jest.Mock).mockResolvedValueOnce({ data: {} });
    await expect(
      addPlayersToTeam({ team_id: 1, players_ids: [1, 2, 3] })
    ).resolves.not.toThrow();
    expect(mockedApi.post).toHaveBeenCalledWith('/teams/add-players', {
      team_id: 1,
      players_ids: [1, 2, 3],
    });
  });

  it('throws with server message on failure', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '400', undefined, undefined, {
      status: 400,
      data: { message: 'Some players not found' },
    } as any);
    (mockedApi.post as jest.Mock).mockRejectedValueOnce(error);
    await expect(
      addPlayersToTeam({ team_id: 1, players_ids: [99] })
    ).rejects.toThrow('Some players not found');
  });
});

describe('deletePlayersOfTeam', () => {
  it('resolves on success', async () => {
    (mockedApi.patch as jest.Mock).mockResolvedValueOnce({ data: {} });
    await expect(
      deletePlayersOfTeam({ team_id: 1, players_ids: [2] })
    ).resolves.not.toThrow();
    expect(mockedApi.patch).toHaveBeenCalledWith('/teams/delete-players', {
      team_id: 1,
      name: undefined,
      player_ids: [2],
    });
  });

  it('throws generic message on failure', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '500', undefined, undefined, {
      status: 500,
      data: {},
    } as any);
    (mockedApi.patch as jest.Mock).mockRejectedValueOnce(error);
    await expect(
      deletePlayersOfTeam({ team_id: 1, players_ids: [1] })
    ).rejects.toThrow('Failed to update team players');
  });
});

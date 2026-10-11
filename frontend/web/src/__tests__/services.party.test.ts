import {
  createParty,
  endParty,
  getParties,
  getPartyById,
  deleteParty,
} from '@/services/party';

jest.mock('@/lib/api', () => ({
  __esModule: true,
  default: {
    get: jest.fn(),
    post: jest.fn(),
    patch: jest.fn(),
    delete: jest.fn(),
  },
}));

import api from '@/lib/api';
const mockedApi = api as jest.Mocked<typeof api>;

const partyFixture = {
  id: 1,
  community_id: 10,
  game_name: 'CS:GO',
  team_winner_id: undefined,
  finished_at: undefined,
  created_at: '2024-01-01T00:00:00Z',
  updated_at: '2024-01-01T00:00:00Z',
  teams: [],
};

afterEach(() => {
  jest.clearAllMocks();
});

describe('createParty', () => {
  const params = { game_name: 'CS:GO', teams_ids: [1, 2], community_id: 10 };

  it('returns api response on success', async () => {
    (mockedApi.post as jest.Mock).mockResolvedValueOnce({
      data: { data: partyFixture, timestamp: '2024-01-01T00:00:00Z' },
    });
    const result = await createParty(params);
    expect(result.data.game_name).toBe('CS:GO');
    expect(mockedApi.post).toHaveBeenCalledWith('/parties', params);
  });

  it('throws with server message on failure', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '400', undefined, undefined, {
      status: 400,
      data: { message: 'There must be at least two teams' },
    } as any);
    (mockedApi.post as jest.Mock).mockRejectedValueOnce(error);
    await expect(createParty(params)).rejects.toThrow('There must be at least two teams');
  });

  it('throws generic message when no server message', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '500', undefined, undefined, {
      status: 500,
      data: {},
    } as any);
    (mockedApi.post as jest.Mock).mockRejectedValueOnce(error);
    await expect(createParty(params)).rejects.toThrow('Failed to create party');
  });
});

describe('endParty', () => {
  it('resolves on success without winner', async () => {
    (mockedApi.patch as jest.Mock).mockResolvedValueOnce({ data: {} });
    await expect(endParty({ party_id: 1 })).resolves.not.toThrow();
    expect(mockedApi.patch).toHaveBeenCalledWith('/parties/end', {
      party_id: 1,
      team_winner_id: undefined,
    });
  });

  it('resolves on success with winner', async () => {
    (mockedApi.patch as jest.Mock).mockResolvedValueOnce({ data: {} });
    await expect(endParty({ party_id: 1, team_winner_id: 2 })).resolves.not.toThrow();
    expect(mockedApi.patch).toHaveBeenCalledWith('/parties/end', {
      party_id: 1,
      team_winner_id: 2,
    });
  });

  it('throws with server message on failure', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '400', undefined, undefined, {
      status: 400,
      data: { message: 'Party is already finished' },
    } as any);
    (mockedApi.patch as jest.Mock).mockRejectedValueOnce(error);
    await expect(endParty({ party_id: 1 })).rejects.toThrow('Party is already finished');
  });
});

describe('getParties', () => {
  it('fetches all parties without community filter', async () => {
    (mockedApi.get as jest.Mock).mockResolvedValueOnce({
      data: { data: [partyFixture], timestamp: '2024-01-01T00:00:00Z' },
    });
    const result = await getParties();
    expect(result.data).toHaveLength(1);
    expect(mockedApi.get).toHaveBeenCalledWith('/parties', { params: {} });
  });

  it('fetches parties filtered by community_id', async () => {
    (mockedApi.get as jest.Mock).mockResolvedValueOnce({
      data: { data: [partyFixture], timestamp: '2024-01-01T00:00:00Z' },
    });
    await getParties(10);
    expect(mockedApi.get).toHaveBeenCalledWith('/parties', {
      params: { community_id: 10 },
    });
  });

  it('passes pagination params alongside community_id', async () => {
    const paginatedPayload = {
      data: [partyFixture],
      meta: { total: 20, page: 2, limit: 5, total_pages: 4, has_next_page: true, has_previous_page: true },
      timestamp: '2024-01-01T00:00:00Z',
    };
    (mockedApi.get as jest.Mock).mockResolvedValueOnce({ data: paginatedPayload });
    const result = await getParties(10, { page: 2, limit: 5 });
    expect(mockedApi.get).toHaveBeenCalledWith('/parties', {
      params: { community_id: 10, page: 2, limit: 5 },
    });
    expect(result.meta?.page).toBe(2);
    expect(result.meta?.has_next_page).toBe(true);
  });

  it('passes pagination params without community filter', async () => {
    (mockedApi.get as jest.Mock).mockResolvedValueOnce({
      data: { data: [], timestamp: '2024-01-01T00:00:00Z' },
    });
    await getParties(undefined, { page: 1, limit: 10 });
    expect(mockedApi.get).toHaveBeenCalledWith('/parties', {
      params: { page: 1, limit: 10 },
    });
  });

  it('throws generic message on failure', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '500', undefined, undefined, {
      status: 500,
      data: {},
    } as any);
    (mockedApi.get as jest.Mock).mockRejectedValueOnce(error);
    await expect(getParties()).rejects.toThrow('Failed to fetch parties');
  });
});

describe('getPartyById', () => {
  it('returns single party on success', async () => {
    (mockedApi.get as jest.Mock).mockResolvedValueOnce({
      data: { data: partyFixture, timestamp: '2024-01-01T00:00:00Z' },
    });
    const result = await getPartyById(1);
    expect(result.data.id).toBe(1);
    expect(mockedApi.get).toHaveBeenCalledWith('/parties/1');
  });

  it('throws with server message when party not found', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '404', undefined, undefined, {
      status: 404,
      data: { message: 'Party not found' },
    } as any);
    (mockedApi.get as jest.Mock).mockRejectedValueOnce(error);
    await expect(getPartyById(99)).rejects.toThrow('Party not found');
  });
});

describe('deleteParty', () => {
  it('resolves on success', async () => {
    (mockedApi.delete as jest.Mock).mockResolvedValueOnce({ data: {} });
    await expect(deleteParty(1)).resolves.not.toThrow();
    expect(mockedApi.delete).toHaveBeenCalledWith('/parties/1');
  });

  it('throws generic message on failure', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '500', undefined, undefined, {
      status: 500,
      data: {},
    } as any);
    (mockedApi.delete as jest.Mock).mockRejectedValueOnce(error);
    await expect(deleteParty(1)).rejects.toThrow('Failed to delete party');
  });
});

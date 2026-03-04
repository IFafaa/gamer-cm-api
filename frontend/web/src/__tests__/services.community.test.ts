import {
  getCommunities,
  createCommunity,
  updateCommunity,
  deleteCommunity,
} from '@/services/community';

// Mock the shared api instance
jest.mock('@/lib/api', () => ({
  __esModule: true,
  default: {
    get: jest.fn(),
    post: jest.fn(),
    put: jest.fn(),
    delete: jest.fn(),
  },
}));

import api from '@/lib/api';
const mockedApi = api as jest.Mocked<typeof api>;

const communitiesPayload = {
  data: [
    {
      id: 1,
      name: 'Pro Gamers',
      players: [],
      teams: [],
      created_at: '2024-01-01T00:00:00Z',
      updated_at: '2024-01-01T00:00:00Z',
    },
  ],
  timestamp: '2024-01-01T00:00:00Z',
};

afterEach(() => {
  jest.resetAllMocks();
});

describe('getCommunities', () => {
  it('returns communities data on success', async () => {
    (mockedApi.get as jest.Mock).mockResolvedValueOnce({ data: communitiesPayload });
    const result = await getCommunities();
    expect(result.data).toHaveLength(1);
    expect(result.data[0].name).toBe('Pro Gamers');
    expect(mockedApi.get).toHaveBeenCalledWith('/communities', undefined);
  });

  it('passes pagination params to the API', async () => {
    const paginatedPayload = {
      ...communitiesPayload,
      meta: { total: 10, page: 2, limit: 3, total_pages: 4, has_next_page: true, has_previous_page: true },
    };
    (mockedApi.get as jest.Mock).mockResolvedValueOnce({ data: paginatedPayload });
    const result = await getCommunities({ page: 2, limit: 3 });
    expect(mockedApi.get).toHaveBeenCalledWith('/communities', { params: { page: 2, limit: 3 } });
    expect(result.meta?.page).toBe(2);
    expect(result.meta?.has_next_page).toBe(true);
  });

  it('returns response without meta when API omits it', async () => {
    (mockedApi.get as jest.Mock).mockResolvedValueOnce({ data: communitiesPayload });
    const result = await getCommunities({ page: 1, limit: 10 });
    expect(result.meta).toBeUndefined();
  });

  it('throws with server error message on failure', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('Unauthorized', '401', undefined, undefined, {
      status: 401,
      data: { message: 'Unauthorized' },
    } as any);
    (mockedApi.get as jest.Mock).mockRejectedValueOnce(error);
    await expect(getCommunities()).rejects.toThrow('Unauthorized');
  });

  it('throws generic message when server has no message', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '500', undefined, undefined, {
      status: 500,
      data: {},
    } as any);
    (mockedApi.get as jest.Mock).mockRejectedValueOnce(error);
    await expect(getCommunities()).rejects.toThrow('Failed to fetch communities');
  });
});

describe('createCommunity', () => {
  it('resolves without error on success', async () => {
    (mockedApi.post as jest.Mock).mockResolvedValueOnce({ data: {} });
    await expect(createCommunity({ name: 'New Community' })).resolves.not.toThrow();
    expect(mockedApi.post).toHaveBeenCalledWith('/communities', { name: 'New Community' });
  });

  it('throws with server error message on failure', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '409', undefined, undefined, {
      status: 409,
      data: { message: 'Community already exists' },
    } as any);
    (mockedApi.post as jest.Mock).mockRejectedValueOnce(error);
    await expect(createCommunity({ name: 'Dup' })).rejects.toThrow(
      'Community already exists'
    );
  });
});

describe('updateCommunity', () => {
  it('resolves without error on success', async () => {
    (mockedApi.put as jest.Mock).mockResolvedValueOnce({ data: {} });
    await expect(updateCommunity(1, 'Renamed')).resolves.not.toThrow();
    expect(mockedApi.put).toHaveBeenCalledWith('/communities/1', { name: 'Renamed' });
  });

  it('throws with server error message on failure', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '404', undefined, undefined, {
      status: 404,
      data: { message: 'Community not found' },
    } as any);
    (mockedApi.put as jest.Mock).mockRejectedValueOnce(error);
    await expect(updateCommunity(99, 'X')).rejects.toThrow('Community not found');
  });
});

describe('deleteCommunity', () => {
  it('resolves without error on success', async () => {
    (mockedApi.delete as jest.Mock).mockResolvedValueOnce({ data: {} });
    await expect(deleteCommunity(1)).resolves.not.toThrow();
    expect(mockedApi.delete).toHaveBeenCalledWith('/communities/1');
  });

  it('throws generic message when no server message', async () => {
    const { default: axios } = await import('axios');
    const error = new axios.AxiosError('error', '500', undefined, undefined, {
      status: 500,
      data: {},
    } as any);
    (mockedApi.delete as jest.Mock).mockRejectedValueOnce(error);
    await expect(deleteCommunity(1)).rejects.toThrow('Failed to delete community');
  });
});

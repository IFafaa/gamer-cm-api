// auth service creates its own axios instance via axios.create()
// so we mock axios.create to return a controlled mock
const mockPost = jest.fn();
const mockGet = jest.fn();

jest.mock('axios', () => {
  const actual = jest.requireActual('axios');
  return {
    ...actual,
    default: {
      ...actual.default,
      create: jest.fn(() => ({ post: mockPost, get: mockGet })),
      isAxiosError: actual.isAxiosError,
    },
    create: jest.fn(() => ({ post: mockPost, get: mockGet })),
    isAxiosError: actual.isAxiosError,
    AxiosError: actual.AxiosError,
  };
});

import { authService } from '@/services/auth';
import axios from 'axios';

afterEach(() => {
  mockPost.mockReset();
  mockGet.mockReset();
});

function makeAxiosError(status: number, message?: string) {
  const err = new (axios as any).AxiosError('error', String(status));
  err.response = { status, data: message ? { message } : {} };
  return err;
}

describe('authService.login', () => {
  const credentials = { username: 'alice', password: 'pass123' };
  const responseData = {
    token: 'jwt_token',
    user: { id: 1, username: 'alice', email: 'alice@example.com' },
  };

  it('returns auth data on success', async () => {
    mockPost.mockResolvedValueOnce({ data: { data: responseData } });
    const result = await authService.login(credentials);
    expect(result.token).toBe('jwt_token');
    expect(result.user.username).toBe('alice');
  });

  it('throws with server message on 401', async () => {
    mockPost.mockRejectedValueOnce(makeAxiosError(401, 'Invalid credentials'));
    await expect(authService.login(credentials)).rejects.toThrow(
      'Invalid credentials'
    );
  });

  it('throws generic message when server has no message', async () => {
    mockPost.mockRejectedValueOnce(makeAxiosError(500));
    await expect(authService.login(credentials)).rejects.toThrow('Login failed');
  });

  it('throws on non-axios error', async () => {
    mockPost.mockRejectedValueOnce(new Error('Network Error'));
    await expect(authService.login(credentials)).rejects.toThrow('Network Error');
  });
});

describe('authService.register', () => {
  const userData = {
    username: 'bob',
    email: 'bob@example.com',
    password: 'password123',
  };
  const responseData = {
    token: 'jwt_bob',
    user: { id: 2, username: 'bob', email: 'bob@example.com' },
  };

  it('returns auth data on success', async () => {
    mockPost.mockResolvedValueOnce({ data: { data: responseData } });
    const result = await authService.register(userData);
    expect(result.token).toBe('jwt_bob');
    expect(result.user.id).toBe(2);
  });

  it('throws with server message on 409 conflict', async () => {
    mockPost.mockRejectedValueOnce(makeAxiosError(409, 'Username already exists'));
    await expect(authService.register(userData)).rejects.toThrow(
      'Username already exists'
    );
  });

  it('throws generic message when server has no message', async () => {
    mockPost.mockRejectedValueOnce(makeAxiosError(500));
    await expect(authService.register(userData)).rejects.toThrow(
      'Registration failed'
    );
  });
});

import axios from 'axios';
import { CommunitiesResponse, CreateCommunityRequest, ErrorResponse } from "@/types/community";

const api = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API_URL,
});

export async function getCommunities(): Promise<CommunitiesResponse> {
  try {
    const { data } = await api.get('/communities');
    return data;
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to fetch communities');
    }
    throw error;
  }
}

export async function createCommunity(data: CreateCommunityRequest): Promise<void> {
  try {
    await api.post('/communities', data);
  } catch (error) {
    if (axios.isAxiosError(error)) {
      throw new Error(error.response?.data?.message || 'Failed to create community');
    }
    throw error;
  }
} 
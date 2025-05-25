import { CommunitiesResponse, CreateCommunityRequest, ErrorResponse } from "@/types/community";

export async function getCommunities(): Promise<CommunitiesResponse> {
  const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/communities`);
  
  if (!response.ok) {
    throw new Error('Failed to fetch communities');
  }

  return response.json();
}

export async function createCommunity(data: CreateCommunityRequest): Promise<void> {
  const response = await fetch(`${process.env.NEXT_PUBLIC_API_URL}/communities`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(data),
  });

  if (!response.ok) {
    const error: ErrorResponse = await response.json();
    throw new Error(error.message);
  }
} 
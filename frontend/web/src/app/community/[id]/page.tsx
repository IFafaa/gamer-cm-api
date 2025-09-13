import { notFound } from 'next/navigation';
import { CommunityContent } from '@/components/CommunityContent';
import axios from 'axios';

interface Player {
  id: number;
  nickname: string;
  created_at: string;
  updated_at: string;
}

interface Team {
  id: number;
  name: string;
  players: Player[];
  created_at: string;
  updated_at: string;
}

interface Community {
  id: number;
  name: string;
  created_at: string;
  updated_at: string;
  players: Player[];
  teams: Team[];
}

interface CommunityResponse {
  data: Community;
  timestamp: string;
}

const api = axios.create({
  baseURL: process.env.NEXT_PUBLIC_API_URL,
});

async function getCommunity(id: string): Promise<CommunityResponse> {
  const res = await api.get(`/communities/${id}`);

  if (res.status !== 200) {
    throw new Error('Failed to fetch community');
  }

  return res.data;
}

export default async function CommunityPage({
  params,
}: {
  params: Promise<{ id: string }>;
}) {
  let community: Community;
  
  try {
    const { id } = await params;
    const response = await getCommunity(id);
    community = response.data;
  } catch {
    notFound();
  }

  return <CommunityContent community={community} />;
} 
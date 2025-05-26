import { notFound } from 'next/navigation';
import { CommunityContent } from '@/components/CommunityContent';

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

async function getCommunity(id: string): Promise<CommunityResponse> {
  const res = await fetch(`http://localhost:3333/communities/${id}`, {
    cache: 'no-store'
  });

  if (!res.ok) {
    throw new Error('Failed to fetch community');
  }

  return res.json();
}

export default async function CommunityPage({
  params,
}: {
  params: { id: string };
}) {
  let community: Community;
  
  try {
    const response = await getCommunity(params.id);
    community = response.data;
  } catch {
    notFound();
  }

  return <CommunityContent community={community} />;
} 
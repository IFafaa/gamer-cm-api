"use client";

import { useEffect, useState, useCallback } from 'react';
import { useParams } from 'next/navigation';
import { CommunityContent } from '@/components/CommunityContent';
import { Community } from "@/types/community";
import api from '@/lib/api';
import { useAuth } from '@/contexts/AuthContext';

interface CommunityResponse {
  data: Community;
  timestamp: string;
}

export default function CommunityPage() {
  const params = useParams();
  const { isAuthenticated, isLoading: authLoading } = useAuth();
  const [community, setCommunity] = useState<Community | null>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  const fetchCommunity = useCallback(async () => {
    try {
      const id = params.id as string;
      const response = await api.get<CommunityResponse>(`/communities/${id}`);
      setCommunity(response.data.data);
      setError(null);
    } catch (err) {
      console.error('Error fetching community:', err);
      setError('Failed to fetch community');
    }
  }, [params.id]);

  useEffect(() => {
    const loadCommunity = async () => {
      if (!isAuthenticated || authLoading) return;
      
      await fetchCommunity();
      setIsLoading(false);
    };

    loadCommunity();
  }, [isAuthenticated, authLoading, fetchCommunity]);

  if (authLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-lg">Authenticating...</div>
      </div>
    );
  }

  if (isLoading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-lg">Loading community...</div>
      </div>
    );
  }

  if (error || !community) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="text-lg text-red-500">
          {error || 'Community not found'}
        </div>
      </div>
    );
  }

  return <CommunityContent community={community} onCommunityUpdate={fetchCommunity} />;
} 
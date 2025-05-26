"use client";

import { useState } from 'react';
import { Plus } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { CreatePlayerDialog } from '@/components/CreatePlayerDialog';
import { useRouter } from 'next/navigation';

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

interface CommunityContentProps {
  community: Community;
}

export function CommunityContent({ community }: CommunityContentProps) {
  const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false);
  const router = useRouter();

  const refreshCommunity = () => {
    router.refresh();
  };

  return (
    <div className="p-8">
      <div className="mx-auto max-w-4xl space-y-8">
        <div className="rounded-lg border bg-card p-8 shadow-sm">
          <h1 className="mb-4 text-4xl font-bold text-foreground">
            {community.name}
          </h1>
          <p className="text-lg text-muted-foreground">
            Community Details
          </p>
        </div>

        <div className="grid gap-4 md:grid-cols-2">
          <div className="rounded-lg border bg-card p-6 shadow-sm">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-2xl font-semibold text-foreground">
                Players ({community.players.length})
              </h2>
              <Button
                onClick={() => setIsCreateDialogOpen(true)}
                size="sm"
                className="flex items-center gap-2 cursor-pointer"
              >
                <Plus className="w-4 h-4" />
                Add Player
              </Button>
            </div>
            <div className="space-y-2">
              {community.players.map((player) => (
                <div
                  key={player.id}
                  className="rounded-md border p-3 text-muted-foreground"
                >
                  {player.nickname}
                </div>
              ))}
            </div>
          </div>

          <div className="rounded-lg border bg-card p-6 shadow-sm">
            <h2 className="mb-4 text-2xl font-semibold text-foreground">
              Teams ({community.teams.length})
            </h2>
            <div className="space-y-4">
              {community.teams.map((team) => (
                <div
                  key={team.id}
                  className="rounded-md border p-4"
                >
                  <h3 className="mb-2 text-lg font-medium text-foreground">
                    {team.name}
                  </h3>
                  <div className="space-y-1">
                    {team.players.length > 0 ? (
                      team.players.map((player) => (
                        <div
                          key={player.id}
                          className="text-sm text-muted-foreground"
                        >
                          • {player.nickname}
                        </div>
                      ))
                    ) : (
                      <p className="text-sm text-muted-foreground">
                        No players in this team
                      </p>
                    )}
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </div>

      <CreatePlayerDialog
        isOpen={isCreateDialogOpen}
        onClose={() => setIsCreateDialogOpen(false)}
        onSuccess={refreshCommunity}
        communityId={community.id}
      />
    </div>
  );
} 
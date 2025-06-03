"use client";

import { useState } from "react";
import { Plus, X, Trash2 } from "lucide-react";
import { Button } from "@/components/ui/button";
import { CreatePlayerDialog } from "@/components/CreatePlayerDialog";
import { CreateTeamDialog } from "@/components/CreateTeamDialog";
import { useRouter } from "next/navigation";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { toast } from "sonner";
import { deletePlayer } from "@/services/player";
import { deleteCommunity } from "@/services/community";

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
  const [isCreateTeamDialogOpen, setIsCreateTeamDialogOpen] = useState(false);
  const [playerToDelete, setPlayerToDelete] = useState<Player | null>(null);
  const [isDeleteCommunityDialogOpen, setIsDeleteCommunityDialogOpen] =
    useState(false);
  const router = useRouter();

  const refreshCommunity = () => {
    router.refresh();
  };

  const handleDeletePlayer = async () => {
    if (!playerToDelete) return;

    try {
      await deletePlayer(playerToDelete.id);
      toast.success("Player deleted successfully");
      refreshCommunity();
    } catch (error) {
      toast.error(
        error instanceof Error ? error.message : "Failed to delete player"
      );
    } finally {
      setPlayerToDelete(null);
    }
  };

  const handleDeleteCommunity = async () => {
    try {
      await deleteCommunity(community.id);
      toast.success("Community deleted successfully");
      router.push("/home");
      router.refresh();
    } catch (error) {
      toast.error(
        error instanceof Error ? error.message : "Failed to delete community"
      );
    } finally {
      setIsDeleteCommunityDialogOpen(false);
    }
  };

  return (
    <div className="p-8">
      <div className="mx-auto max-w-4xl space-y-8">
        <div className="rounded-lg border bg-card p-8 shadow-sm">
          <div className="flex items-center justify-between">
            <div>
              <h1 className="mb-4 text-4xl font-bold text-foreground">
                {community.name}
              </h1>
              <p className="text-lg text-muted-foreground">Community Details</p>
            </div>
            <Button
              variant="destructive"
              size="sm"
              className="flex items-center gap-2"
              onClick={() => setIsDeleteCommunityDialogOpen(true)}
            >
              <Trash2 className="w-4 h-4" />
              Delete Community
            </Button>
          </div>
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
                  className="rounded-md border p-3 text-muted-foreground flex items-center justify-between"
                >
                  <div className="flex flex-col">
                    <span>{player.nickname}</span>
                    <span className="text-xs text-muted-foreground/70">
                      Joined {new Date(player.created_at).toLocaleDateString()}
                    </span>
                  </div>
                  <Button
                    variant="ghost"
                    size="icon"
                    className="h-8 w-8 text-muted-foreground hover:text-destructive cursor-pointer"
                    onClick={() => setPlayerToDelete(player)}
                  >
                    <X className="h-4 w-4" />
                  </Button>
                </div>
              ))}
            </div>
          </div>

          <div className="rounded-lg border bg-card p-6 shadow-sm">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-2xl font-semibold text-foreground">
                Teams ({community.teams.length})
              </h2>
              <Button
                onClick={() => setIsCreateTeamDialogOpen(true)}
                size="sm"
                className="flex items-center gap-2 cursor-pointer"
              >
                <Plus className="w-4 h-4" />
                Create Team
              </Button>
            </div>
            <div className="space-y-4">
              {community.teams.map((team) => (
                <div key={team.id} className="rounded-md border p-4">
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

      <CreateTeamDialog
        isOpen={isCreateTeamDialogOpen}
        onClose={() => setIsCreateTeamDialogOpen(false)}
        onSuccess={refreshCommunity}
        communityId={community.id}
      />

      <AlertDialog
        open={!!playerToDelete}
        onOpenChange={() => setPlayerToDelete(null)}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Are you sure?</AlertDialogTitle>
            <AlertDialogDescription>
              This action cannot be undone. This will permanently delete the
              player
              {playerToDelete && ` "${playerToDelete.nickname}"`}.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={handleDeletePlayer}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

      <AlertDialog
        open={isDeleteCommunityDialogOpen}
        onOpenChange={setIsDeleteCommunityDialogOpen}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Community</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete this community? This action cannot
              be undone. All teams and players associated with this community
              will be affected.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction
              onClick={handleDeleteCommunity}
              className="bg-destructive text-destructive-foreground hover:bg-destructive/90"
            >
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>
    </div>
  );
}

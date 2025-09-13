"use client";

import { useState, useEffect, useCallback } from "react";
import { Plus, X, Trash2, Users, Gamepad2, Crown } from "lucide-react";
import { Button } from "@/components/ui/button";
import { CreatePlayerDialog } from "@/components/CreatePlayerDialog";
import { CreateTeamDialog } from "@/components/CreateTeamDialog";
import { CreatePartyDialog } from "@/components/CreatePartyDialog";
import { AddPlayersToTeamDialog } from "@/components/AddPlayersToTeamDialog";
import { RemovePlayersFromTeamDialog } from "@/components/RemovePlayersFromTeamDialog";
import { EndPartyDialog } from "@/components/EndPartyDialog";
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
import { getParties, deleteParty } from "@/services/party";
import { Party } from "@/types/community";

import { Player, Team, Community } from "@/types/community";

interface CommunityContentProps {
  community: Community;
}

export function CommunityContent({ community }: CommunityContentProps) {
  const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false);
  const [isCreateTeamDialogOpen, setIsCreateTeamDialogOpen] = useState(false);
  const [isCreatePartyDialogOpen, setIsCreatePartyDialogOpen] = useState(false);
  const [playerToDelete, setPlayerToDelete] = useState<Player | null>(null);
  const [isDeleteCommunityDialogOpen, setIsDeleteCommunityDialogOpen] = useState(false);
  const [selectedTeamForPlayers, setSelectedTeamForPlayers] = useState<Team | null>(null);
  const [isAddPlayersDialogOpen, setIsAddPlayersDialogOpen] = useState(false);
  const [isRemovePlayersDialogOpen, setIsRemovePlayersDialogOpen] = useState(false);
  const [partyToEnd, setPartyToEnd] = useState<Party | null>(null);
  const [isEndPartyDialogOpen, setIsEndPartyDialogOpen] = useState(false);
  const [partyToDelete, setPartyToDelete] = useState<Party | null>(null);
  const [isDeletePartyDialogOpen, setIsDeletePartyDialogOpen] = useState(false);
  const [parties, setParties] = useState<Party[]>([]);
  const [partiesLoading, setPartiesLoading] = useState(false);
  const router = useRouter();

  const refreshCommunity = () => {
    router.refresh();
    loadParties();
  };

  const loadParties = useCallback(async () => {
    setPartiesLoading(true);
    try {
      const response = await getParties();
      // Filtrar partidas apenas desta comunidade
      const communityParties = response.data.filter(party => party.community_id === community.id);
      setParties(communityParties);
    } catch (error) {
      console.error('Failed to load parties:', error);
    } finally {
      setPartiesLoading(false);
    }
  }, [community.id]);

  useEffect(() => {
    loadParties();
  }, [community.id, loadParties]);

  const handleAddPlayersToTeam = (team: Team) => {
    setSelectedTeamForPlayers(team);
    setIsAddPlayersDialogOpen(true);
  };

  const handleRemovePlayersFromTeam = (team: Team) => {
    setSelectedTeamForPlayers(team);
    setIsRemovePlayersDialogOpen(true);
  };

  const handleEndParty = (party: Party) => {
    setPartyToEnd(party);
    setIsEndPartyDialogOpen(true);
  };

  const handleDeleteParty = (party: Party) => {
    setPartyToDelete(party);
    setIsDeletePartyDialogOpen(true);
  };

  const handleDeletePartyConfirm = async () => {
    if (!partyToDelete) return;

    try {
      await deleteParty(partyToDelete.id);
      toast.success("Party deleted successfully");
      refreshCommunity();
      setPartyToDelete(null);
      setIsDeletePartyDialogOpen(false);
    } catch (error) {
      toast.error(
        error instanceof Error ? error.message : "Failed to delete party"
      );
    }
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
                className="flex items-center gap-2"
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
                    className="h-8 w-8 text-muted-foreground hover:text-destructive"
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
                className="flex items-center gap-2"
              >
                <Plus className="w-4 h-4" />
                Create Team
              </Button>
            </div>
            <div className="space-y-4">
              {community.teams.map((team) => (
                <div key={team.id} className="rounded-md border p-4">
                  <div className="flex items-center justify-between mb-2">
                    <h3 className="text-lg font-medium text-foreground">
                      {team.name}
                    </h3>
                    <div className="flex items-center gap-2">
                      <Button
                        variant="outline"
                        size="sm"
                        onClick={() => handleAddPlayersToTeam(team)}
                        className="flex items-center gap-1"
                      >
                        <Users className="w-3 h-3" />
                        Add
                      </Button>
                      {team.players.length > 0 && (
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleRemovePlayersFromTeam(team)}
                          className="flex items-center gap-1"
                        >
                          <X className="w-3 h-3" />
                          Remove
                        </Button>
                      )}
                    </div>
                  </div>
                  <div className="space-y-1">
                    {team.players.length > 0 ? (
                      team.players.map((player) => (
                        <div
                          key={player.id}
                          className="text-sm text-muted-foreground flex items-center gap-2"
                        >
                          <Users className="w-3 h-3" />
                          {player.nickname}
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

        {/* Parties Section */}
        <div className="rounded-lg border bg-card p-6 shadow-sm">
          <div className="flex items-center justify-between mb-4">
            <h2 className="text-2xl font-semibold text-foreground">
              Parties ({parties.length})
            </h2>
            <Button
              onClick={() => setIsCreatePartyDialogOpen(true)}
              size="sm"
              className="flex items-center gap-2 cursor-pointer"
              disabled={community.teams.length < 2}
            >
              <Gamepad2 className="w-4 h-4" />
              Create Party
            </Button>
          </div>
          
          {community.teams.length < 2 && (
            <p className="text-sm text-muted-foreground mb-4">
              You need at least 2 teams to create a party.
            </p>
          )}

          <div className="space-y-4">
            {partiesLoading ? (
              <div className="animate-pulse space-y-2">
                <div className="h-16 bg-muted rounded"></div>
                <div className="h-16 bg-muted rounded"></div>
              </div>
            ) : parties.length === 0 ? (
              <div className="text-center py-8">
                <Gamepad2 className="w-12 h-12 mx-auto text-muted-foreground mb-4" />
                <p className="text-muted-foreground">No parties yet</p>
                <p className="text-sm text-muted-foreground">
                  Create your first gaming party to start organizing matches.
                </p>
              </div>
            ) : (
              parties.map((party) => (
                <div key={party.id} className="rounded-md border p-4">
                  <div className="flex items-center justify-between mb-3">
                    <div className="flex items-center gap-2">
                      <h3 className="text-lg font-medium text-foreground">
                        {party.game_name}
                      </h3>
                      {party.finished_at ? (
                        <span className="px-2 py-1 text-xs bg-green-100 text-green-800 rounded-full">
                          Finished
                        </span>
                      ) : (
                        <span className="px-2 py-1 text-xs bg-blue-100 text-blue-800 rounded-full">
                          Active
                        </span>
                    )}
                    </div>
                    <div className="flex items-center gap-2">
                      {!party.finished_at && (
                        <Button
                          variant="outline"
                          size="sm"
                          onClick={() => handleEndParty(party)}
                          className="flex items-center gap-1"
                        >
                          <Crown className="w-3 h-3" />
                          End Party
                        </Button>
                      )}
                      <Button
                        variant="destructive"
                        size="sm"
                        onClick={() => handleDeleteParty(party)}
                        className="flex items-center gap-1"
                      >
                        <Trash2 className="w-3 h-3" />
                        Delete
                      </Button>
                    </div>
                  </div>
                  
                  <div className="grid gap-2">
                    {party.teams.map((team) => (
                      <div
                        key={team.id}
                        className={`flex items-center justify-between p-2 rounded ${
                          party.team_winner_id === team.id 
                            ? 'bg-green-100 border-2 border-green-300 shadow-sm' 
                            : 'bg-muted/50'
                        }`}
                      >
                        <div className="flex items-center gap-2">
                          {party.team_winner_id === team.id && (
                            <Crown className="w-4 h-4 text-yellow-500" />
                          )}
                          <span className={`font-medium ${
                            party.team_winner_id === team.id 
                              ? 'text-green-800' 
                              : 'text-foreground'
                          }`}>
                            {team.name}
                          </span>
                          <span className="text-sm text-muted-foreground">
                            ({team.players.length} players)
                          </span>
                        </div>
                        {party.team_winner_id === team.id && (
                          <span className="text-sm font-bold text-green-800 bg-green-200 px-2 py-1 rounded-full">
                            Winner!
                          </span>
                        )}
                      </div>
                    ))}
                  </div>
                  
                  <div className="mt-3 text-xs text-muted-foreground">
                    Created: {new Date(party.created_at).toLocaleDateString()}
                    {party.finished_at && (
                      <span className="ml-4">
                        Finished: {new Date(party.finished_at).toLocaleDateString()}
                      </span>
                    )}
                  </div>
                </div>
              ))
            )}
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

      <CreatePartyDialog
        isOpen={isCreatePartyDialogOpen}
        onClose={() => setIsCreatePartyDialogOpen(false)}
        onSuccess={refreshCommunity}
        communityId={community.id}
        availableTeams={community.teams}
      />

      {selectedTeamForPlayers && (
        <>
          <AddPlayersToTeamDialog
            isOpen={isAddPlayersDialogOpen}
            onClose={() => {
              setIsAddPlayersDialogOpen(false);
              setSelectedTeamForPlayers(null);
            }}
            onSuccess={refreshCommunity}
            team={selectedTeamForPlayers}
            availablePlayers={community.players}
          />

          <RemovePlayersFromTeamDialog
            isOpen={isRemovePlayersDialogOpen}
            onClose={() => {
              setIsRemovePlayersDialogOpen(false);
              setSelectedTeamForPlayers(null);
            }}
            onSuccess={refreshCommunity}
            team={selectedTeamForPlayers}
          />
        </>
      )}

      {partyToEnd && (
        <EndPartyDialog
          isOpen={isEndPartyDialogOpen}
          onClose={() => {
            setIsEndPartyDialogOpen(false);
            setPartyToEnd(null);
          }}
          onSuccess={refreshCommunity}
          party={partyToEnd}
        />
      )}

      <AlertDialog
        open={isDeletePartyDialogOpen}
        onOpenChange={() => {
          setIsDeletePartyDialogOpen(false);
          setPartyToDelete(null);
        }}
      >
        <AlertDialogContent>
          <AlertDialogHeader>
            <AlertDialogTitle>Delete Party</AlertDialogTitle>
            <AlertDialogDescription>
              Are you sure you want to delete the party &quot;{partyToDelete?.game_name}&quot;? 
              This action cannot be undone.
            </AlertDialogDescription>
          </AlertDialogHeader>
          <AlertDialogFooter>
            <AlertDialogCancel>Cancel</AlertDialogCancel>
            <AlertDialogAction onClick={handleDeletePartyConfirm} className="bg-destructive text-destructive-foreground hover:bg-destructive/90">
              Delete
            </AlertDialogAction>
          </AlertDialogFooter>
        </AlertDialogContent>
      </AlertDialog>

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

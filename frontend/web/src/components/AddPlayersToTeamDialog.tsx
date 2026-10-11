"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Checkbox } from "@/components/ui/checkbox";
import { Player, Team } from "@/types/community";
import { addPlayersToTeam } from "@/services/team";
import { toast } from "sonner";

interface AddPlayersToTeamDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
  team: Team;
  availablePlayers: Player[];
}

export function AddPlayersToTeamDialog({
  isOpen,
  onClose,
  onSuccess,
  team,
  availablePlayers,
}: AddPlayersToTeamDialogProps) {
  const [selectedPlayers, setSelectedPlayers] = useState<number[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  const handlePlayerToggle = (playerId: number) => {
    setSelectedPlayers(prev =>
      prev.includes(playerId)
        ? prev.filter(id => id !== playerId)
        : [...prev, playerId]
    );
  };

  const handleSubmit = async () => {
    if (selectedPlayers.length === 0) {
      toast.error("Please select at least one player");
      return;
    }

    setIsLoading(true);
    try {
      await addPlayersToTeam({
        team_id: team.id,
        players_ids: selectedPlayers,
      });
      toast.success("Players added to team successfully");
      onSuccess();
      onClose();
      setSelectedPlayers([]);
    } catch (error) {
      toast.error(
        error instanceof Error ? error.message : "Failed to add players to team"
      );
    } finally {
      setIsLoading(false);
    }
  };

  const handleClose = () => {
    setSelectedPlayers([]);
    onClose();
  };

  // Filtrar players que já estão no time
  const playersNotInTeam = availablePlayers.filter(
    player => !team.players.some(teamPlayer => teamPlayer.id === player.id)
  );

  return (
    <Dialog open={isOpen} onOpenChange={handleClose}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Add Players to {team.name}</DialogTitle>
          <DialogDescription>
            Select players to add to this team. Players already in the team are not shown.
          </DialogDescription>
        </DialogHeader>

        <div className="grid gap-4 py-4">
          <div className="space-y-4">
            {playersNotInTeam.length === 0 ? (
              <p className="text-sm text-muted-foreground text-center py-4">
                All available players are already in this team.
              </p>
            ) : (
              playersNotInTeam.map((player) => (
                <div key={player.id} className="flex items-center space-x-2">
                  <Checkbox
                    id={`player-${player.id}`}
                    checked={selectedPlayers.includes(player.id)}
                    onCheckedChange={() => handlePlayerToggle(player.id)}
                  />
                  <Label
                    htmlFor={`player-${player.id}`}
                    className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                  >
                    {player.nickname}
                  </Label>
                </div>
              ))
            )}
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={handleClose} disabled={isLoading}>
            Cancel
          </Button>
          <Button onClick={handleSubmit} disabled={isLoading || selectedPlayers.length === 0}>
            {isLoading ? "Adding..." : `Add ${selectedPlayers.length} Player${selectedPlayers.length !== 1 ? 's' : ''}`}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

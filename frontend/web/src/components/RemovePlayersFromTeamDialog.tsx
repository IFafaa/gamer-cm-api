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
import { Team } from "@/types/community";
import { deletePlayersOfTeam } from "@/services/team";
import { toast } from "sonner";

interface RemovePlayersFromTeamDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
  team: Team;
}

export function RemovePlayersFromTeamDialog({
  isOpen,
  onClose,
  onSuccess,
  team,
}: RemovePlayersFromTeamDialogProps) {
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
      const playersToKeep = team.players
        .filter(player => !selectedPlayers.includes(player.id))
        .map(player => player.id);

      await deletePlayersOfTeam({
        team_id: team.id,
        name: team.name,
        players_ids: playersToKeep,
      });
      toast.success("Team updated successfully");
      onSuccess();
      onClose();
      setSelectedPlayers([]);
    } catch (error) {
      toast.error(
        error instanceof Error ? error.message : "Failed to update team"
      );
    } finally {
      setIsLoading(false);
    }
  };

  const handleClose = () => {
    setSelectedPlayers([]);
    onClose();
  };

  return (
    <Dialog open={isOpen} onOpenChange={handleClose}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Remove Players from {team.name}</DialogTitle>
          <DialogDescription>
            Select players to remove from this team.
          </DialogDescription>
        </DialogHeader>

        <div className="grid gap-4 py-4">
          <div className="space-y-4">
            {team.players.length === 0 ? (
              <p className="text-sm text-muted-foreground text-center py-4">
                No players in this team.
              </p>
            ) : (
              team.players.map((player) => (
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
          <Button 
            variant="destructive"
            onClick={handleSubmit} 
            disabled={isLoading || selectedPlayers.length === 0}
          >
            {isLoading ? "Removing..." : `Remove ${selectedPlayers.length} Player${selectedPlayers.length !== 1 ? 's' : ''}`}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

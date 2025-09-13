"use client";

import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
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
import { createParty } from "@/services/party";
import { toast } from "sonner";

interface CreatePartyDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
  communityId: number;
  availableTeams: Team[];
}

export function CreatePartyDialog({
  isOpen,
  onClose,
  onSuccess,
  communityId,
  availableTeams,
}: CreatePartyDialogProps) {
  const [gameName, setGameName] = useState("");
  const [selectedTeams, setSelectedTeams] = useState<number[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  const handleTeamToggle = (teamId: number) => {
    setSelectedTeams(prev =>
      prev.includes(teamId)
        ? prev.filter(id => id !== teamId)
        : [...prev, teamId]
    );
  };

  const handleSubmit = async () => {
    if (!gameName.trim()) {
      toast.error("Game name is required");
      return;
    }

    if (selectedTeams.length < 2) {
      toast.error("At least 2 teams are required for a party");
      return;
    }

    setIsLoading(true);
    try {
      await createParty({
        game_name: gameName.trim(),
        teams_ids: selectedTeams,
        community_id: communityId,
      });
      toast.success("Party created successfully");
      onSuccess();
      onClose();
      setGameName("");
      setSelectedTeams([]);
    } catch (error) {
      toast.error(
        error instanceof Error ? error.message : "Failed to create party"
      );
    } finally {
      setIsLoading(false);
    }
  };

  const handleClose = () => {
    setGameName("");
    setSelectedTeams([]);
    onClose();
  };

  return (
    <Dialog open={isOpen} onOpenChange={handleClose}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>Create New Party</DialogTitle>
          <DialogDescription>
            Create a new gaming party with multiple teams.
          </DialogDescription>
        </DialogHeader>

        <div className="grid gap-4 py-4">
          <div className="grid gap-2">
            <Label htmlFor="game-name">Game Name</Label>
            <Input
              id="game-name"
              value={gameName}
              onChange={(e) => setGameName(e.target.value)}
              placeholder="Enter game name (e.g., Counter-Strike, Valorant)"
              disabled={isLoading}
            />
          </div>

          <div className="space-y-3">
            <Label>Select Teams ({selectedTeams.length} selected)</Label>
            <div className="space-y-2 max-h-40 overflow-y-auto">
              {availableTeams.length === 0 ? (
                <p className="text-sm text-muted-foreground text-center py-4">
                  No teams available. Create teams first.
                </p>
              ) : (
                availableTeams.map((team) => (
                  <div key={team.id} className="flex items-center space-x-2">
                    <Checkbox
                      id={`team-${team.id}`}
                      checked={selectedTeams.includes(team.id)}
                      onCheckedChange={() => handleTeamToggle(team.id)}
                      disabled={isLoading}
                    />
                    <Label
                      htmlFor={`team-${team.id}`}
                      className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
                    >
                      {team.name} ({team.players.length} players)
                    </Label>
                  </div>
                ))
              )}
            </div>
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={handleClose} disabled={isLoading}>
            Cancel
          </Button>
          <Button 
            onClick={handleSubmit} 
            disabled={isLoading || !gameName.trim() || selectedTeams.length < 2}
          >
            {isLoading ? "Creating..." : "Create Party"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

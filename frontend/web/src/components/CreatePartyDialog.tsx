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
      <DialogContent className="max-w-md">
        <DialogHeader>
          <DialogTitle>Create New Party</DialogTitle>
          <DialogDescription>
            Create a new party with at least 2 teams
          </DialogDescription>
        </DialogHeader>

        <div className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="gameName">Game Name</Label>
            <Input
              id="gameName"
              value={gameName}
              onChange={(e) => setGameName(e.target.value)}
              placeholder="Enter game name"
              required
            />
          </div>

          <div className="space-y-2">
            <Label>Select Teams ({selectedTeams.length} selected)</Label>
            <div className="max-h-40 overflow-y-auto space-y-2">
              {availableTeams.length === 0 ? (
                <p className="text-sm text-muted-foreground">
                  No teams available. Create teams first.
                </p>
              ) : (
                availableTeams.map((team) => (
                  <div key={team.id} className="flex items-center space-x-2">
                    <Checkbox
                      id={`team-${team.id}`}
                      checked={selectedTeams.includes(team.id)}
                      onCheckedChange={() => handleTeamToggle(team.id)}
                    />
                    <Label
                      htmlFor={`team-${team.id}`}
                      className="text-sm font-normal cursor-pointer"
                    >
                      {team.name}
                    </Label>
                  </div>
                ))
              )}
            </div>
          </div>
        </div>

        <DialogFooter>
          <Button
            type="button"
            variant="outline"
            onClick={handleClose}
            disabled={isLoading}
          >
            Cancel
          </Button>
          <Button
            type="button"
            onClick={handleSubmit}
            disabled={isLoading || selectedTeams.length < 2}
          >
            {isLoading ? "Creating..." : "Create Party"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
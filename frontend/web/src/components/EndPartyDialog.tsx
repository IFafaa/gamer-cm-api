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
import { RadioGroup, RadioGroupItem } from "@/components/ui/radio-group";
import { Party } from "@/types/community";
import { endParty } from "@/services/party";
import { toast } from "sonner";

interface EndPartyDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
  party: Party;
}

export function EndPartyDialog({
  isOpen,
  onClose,
  onSuccess,
  party,
}: EndPartyDialogProps) {
  const [winnerTeamId, setWinnerTeamId] = useState<string>("");
  const [isLoading, setIsLoading] = useState(false);

  const handleSubmit = async () => {
    setIsLoading(true);
    try {
      await endParty({
        party_id: party.id,
        team_winner_id: winnerTeamId ? parseInt(winnerTeamId) : undefined,
      });
      toast.success("Party ended successfully");
      onSuccess();
      onClose();
      setWinnerTeamId("");
    } catch (error) {
      toast.error(
        error instanceof Error ? error.message : "Failed to end party"
      );
    } finally {
      setIsLoading(false);
    }
  };

  const handleClose = () => {
    setWinnerTeamId("");
    onClose();
  };

  return (
    <Dialog open={isOpen} onOpenChange={handleClose}>
      <DialogContent className="sm:max-w-[425px]">
        <DialogHeader>
          <DialogTitle>End Party</DialogTitle>
          <DialogDescription>
            End the party &quot;{party.game_name}&quot;. Select the winning team or leave blank for no winner.
          </DialogDescription>
        </DialogHeader>

        <div className="grid gap-4 py-4">
          <div className="space-y-3">
            <Label>Winner Team (Optional)</Label>
            <RadioGroup value={winnerTeamId} onValueChange={setWinnerTeamId}>
              <div className="flex items-center space-x-2">
                <RadioGroupItem value="" id="no-winner" />
                <Label htmlFor="no-winner" className="text-sm">
                  No winner / Draw
                </Label>
              </div>
              {party.teams.map((team) => (
                <div key={team.id} className="flex items-center space-x-2">
                  <RadioGroupItem value={team.id.toString()} id={`team-${team.id}`} />
                  <Label htmlFor={`team-${team.id}`} className="text-sm">
                    {team.name}
                  </Label>
                </div>
              ))}
            </RadioGroup>
          </div>
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={handleClose} disabled={isLoading}>
            Cancel
          </Button>
          <Button 
            variant="destructive"
            onClick={handleSubmit} 
            disabled={isLoading}
          >
            {isLoading ? "Ending..." : "End Party"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

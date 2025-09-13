"use client";

import { useState } from "react";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { createTeam } from "@/services/team";
import { toast } from "sonner";

interface CreateTeamDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
  communityId: number;
}

export function CreateTeamDialog({
  isOpen,
  onClose,
  onSuccess,
  communityId,
}: CreateTeamDialogProps) {
  const [name, setName] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);

    try {
      await createTeam({
        name,
        community_id: communityId,
      });
      toast.success("Team created successfully");
      onSuccess();
      onClose();
      setName("");
    } catch (error) {
      toast.error(
        error instanceof Error ? error.message : "Failed to create team"
      );
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Create Team</DialogTitle>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="name">Team Name</Label>
            <Input
              id="name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="Enter team name"
              required
            />
          </div>
          <div className="flex justify-end gap-2">
            <Button
              type="button"
              variant="outline"
              onClick={onClose}
              className=""
            >
              Cancel
            </Button>
            <Button
              type="submit"
              disabled={isLoading}
              className=""
            >
              {isLoading ? "Creating..." : "Create Team"}
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}

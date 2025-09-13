"use client";

import { useState } from "react";
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "./ui/dialog";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { Label } from "./ui/label";
import { toast } from "sonner";
import { createPlayer } from "@/services/player";

interface CreatePlayerDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
  communityId: number;
}

export function CreatePlayerDialog({
  isOpen,
  onClose,
  onSuccess,
  communityId,
}: CreatePlayerDialogProps) {
  const [nickname, setNickname] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setIsLoading(true);

    try {
      await createPlayer({
        nickname,
        community_id: communityId,
      });

      toast.success("Player added successfully!");
      onSuccess();
      onClose();
      setNickname("");
    } catch (error) {
      if (error)
        toast.error(
          error instanceof Error ? error.message : "Failed to add player"
        );
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={onClose}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add New Player</DialogTitle>
        </DialogHeader>
        <form onSubmit={handleSubmit} className="space-y-4">
          <div className="space-y-2">
            <Label htmlFor="nickname">Nickname</Label>
            <Input
              id="nickname"
              value={nickname}
              onChange={(e) => setNickname(e.target.value)}
              placeholder="Enter player nickname"
              required
            />
          </div>
          <div className="flex justify-end space-x-2">
            <Button
              className=""
              type="button"
              variant="outline"
              onClick={onClose}
              disabled={isLoading}
            >
              Cancel
            </Button>
            <Button
              className=""
              type="submit"
              disabled={isLoading}
            >
              {isLoading ? "Adding..." : "Add Player"}
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}

"use client";

import { useState } from "react";
import { createCommunity } from "@/services/community";
import { Input } from "@/components/ui/input";
import { X } from "lucide-react";

interface CreateCommunityDialogProps {
  isOpen: boolean;
  onClose: () => void;
  onSuccess: () => void;
}

export function CreateCommunityDialog({ isOpen, onClose, onSuccess }: CreateCommunityDialogProps) {
  const [name, setName] = useState("");
  const [error, setError] = useState<string | null>(null);
  const [isLoading, setIsLoading] = useState(false);

  if (!isOpen) return null;

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);
    setIsLoading(true);

    try {
      await createCommunity({ name });
      setName("");
      onSuccess();
      onClose();
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to create community");
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
      <div className="bg-card rounded-lg p-6 w-full max-w-md relative">
        <button
          onClick={onClose}
          className="absolute right-4 top-4 text-muted-foreground hover:text-foreground
                   transition-colors duration-200 cursor-pointer"
          disabled={isLoading}
        >
          <X className="w-5 h-5" />
        </button>

        <h2 className="text-2xl font-semibold text-foreground mb-4 pr-8">
          Create New Community
        </h2>

        <form onSubmit={handleSubmit} className="space-y-4">
          <div>
            <label htmlFor="name" className="block text-sm font-medium text-foreground mb-1">
              Name
            </label>
            <Input
              id="name"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="Enter community name"
              required
            />
          </div>

          {error && (
            <p className="text-destructive text-sm">{error}</p>
          )}

          <div className="flex justify-end">
            <button
              type="submit"
              className="px-4 py-2 text-sm font-medium text-primary-foreground bg-primary
                       hover:bg-primary/90 rounded-md transition-colors duration-200
                       disabled:opacity-50 disabled:cursor-not-allowed cursor-pointer"
              disabled={isLoading}
            >
              {isLoading ? "Creating..." : "Create"}
            </button>
          </div>
        </form>
      </div>
    </div>
  );
} 
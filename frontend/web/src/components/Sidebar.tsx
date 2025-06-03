"use client";

import { useEffect, useState } from "react";
import { Community } from "@/types/community";
import { getCommunities } from "@/services/community";
import { CreateCommunityDialog } from "./CreateCommunityDialog";
import { Plus } from "lucide-react";
import { useRouter, usePathname } from "next/navigation";

export function Sidebar() {
  const router = useRouter();
  const pathname = usePathname();
  const [communities, setCommunities] = useState<Community[]>([]);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false);

  const loadCommunities = async () => {
    try {
      const response = await getCommunities();
      setCommunities(response.data);
      setError(null);
    } catch (err) {
      setError('Failed to load communities');
      console.error(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadCommunities();
  }, [pathname]);

  if (loading) {
    return (
      <div className="w-20 h-screen bg-sidebar flex flex-col items-center py-4">
        <div className="animate-pulse space-y-4">
          {[1, 2, 3].map((i) => (
            <div key={i} className="w-12 h-12 rounded-full bg-sidebar-accent" />
          ))}
        </div>
      </div>
    );
  }

  if (error) {
    return (
      <div className="w-20 h-screen bg-sidebar flex items-center justify-center">
        <p className="text-sidebar-foreground text-sm text-center px-2">{error}</p>
      </div>
    );
  }

  return (
    <>
      <div className="w-20 h-screen bg-sidebar flex flex-col items-center py-4">
        <div className="flex-1 space-y-4">
          {communities.map((community) => (
            <button
              key={community.id}
              onClick={() => router.push(`/community/${community.id}`)}
              className="w-12 h-12 rounded-full bg-sidebar-accent hover:bg-sidebar-accent/80 
                       text-sidebar-accent-foreground font-medium flex items-center justify-center
                       transition-colors duration-200 cursor-pointer"
              title={community.name}
            >
              {community.name.slice(0, 2).toUpperCase()}
            </button>
          ))}
        </div>

        <button
          onClick={() => setIsCreateDialogOpen(true)}
          className="w-12 h-12 rounded-full bg-sidebar-accent hover:bg-sidebar-accent/80 
                   text-sidebar-accent-foreground flex items-center justify-center
                   transition-colors duration-200 cursor-pointer"
          title="Create Community"
        >
          <Plus className="w-6 h-6" />
        </button>
      </div>

      <CreateCommunityDialog
        isOpen={isCreateDialogOpen}
        onClose={() => setIsCreateDialogOpen(false)}
        onSuccess={loadCommunities}
      />
    </>
  );
} 
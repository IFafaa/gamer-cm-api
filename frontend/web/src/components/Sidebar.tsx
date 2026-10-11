"use client";

import { useCallback, useEffect, useState } from "react";
import { Community, PaginationMeta } from "@/types/community";
import { getCommunities } from "@/services/community";
import { CreateCommunityDialog } from "./CreateCommunityDialog";
import { Plus, ChevronLeft, ChevronRight } from "lucide-react";
import { useRouter, usePathname } from "next/navigation";

const COMMUNITIES_PER_PAGE = 6;

export function Sidebar() {
  const router = useRouter();
  const pathname = usePathname();
  const [communities, setCommunities] = useState<Community[]>([]);
  const [meta, setMeta] = useState<PaginationMeta | null>(null);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [isCreateDialogOpen, setIsCreateDialogOpen] = useState(false);
  const [currentPage, setCurrentPage] = useState(1);

  const loadCommunities = useCallback(async (page: number) => {
    setLoading(true);
    try {
      const response = await getCommunities({ page, limit: COMMUNITIES_PER_PAGE });
      const pagination = response.meta ?? null;
      const totalPages = pagination?.total_pages ?? 0;

      if (pagination && totalPages > 0 && page > totalPages) {
        setCurrentPage(totalPages);
        return;
      }

      if (pagination && pagination.total === 0 && page !== 1) {
        setCurrentPage(1);
        return;
      }

      setCommunities(response.data);
      setMeta(pagination);
      setError(null);
    } catch (err) {
      setError("Failed to load communities");
      console.error(err);
    } finally {
      setLoading(false);
    }
  }, []);

  useEffect(() => {
    loadCommunities(currentPage);
  }, [currentPage, pathname, loadCommunities]);

  const handlePrevPage = () => {
    if (currentPage > 1) {
      setCurrentPage((prev) => Math.max(prev - 1, 1));
    }
  };

  const handleNextPage = () => {
    if (meta?.has_next_page) {
      setCurrentPage((prev) => prev + 1);
    }
  };

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
          {communities.length > 0 ? (
            communities.map((community) => (
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
            ))
          ) : (
            <div className="flex flex-col items-center space-y-3 px-2">
              <div className="w-12 h-12 rounded-full bg-sidebar-accent/50 flex items-center justify-center">
                <span className="text-sidebar-accent-foreground/70 text-xs text-center leading-tight">
                  No communities yet
                </span>
              </div>
            </div>
          )}
        </div>

        {meta && (
          <div className="flex flex-col items-center gap-2 py-3">
            <span className="text-xs text-sidebar-foreground/80">
              Page {meta.total_pages === 0 ? 0 : meta.page} / {Math.max(meta.total_pages, 1)}
            </span>
            <div className="flex gap-2">
              <button
                onClick={handlePrevPage}
                disabled={currentPage === 1}
                className="w-8 h-8 rounded-full bg-sidebar-accent/60 text-sidebar-accent-foreground disabled:opacity-40 disabled:cursor-not-allowed flex items-center justify-center"
              >
                <ChevronLeft className="w-4 h-4" />
              </button>
              <button
                onClick={handleNextPage}
                disabled={!meta.has_next_page}
                className="w-8 h-8 rounded-full bg-sidebar-accent/60 text-sidebar-accent-foreground disabled:opacity-40 disabled:cursor-not-allowed flex items-center justify-center"
              >
                <ChevronRight className="w-4 h-4" />
              </button>
            </div>
          </div>
        )}

        <button
          onClick={() => setIsCreateDialogOpen(true)}
          className={`w-12 h-12 rounded-full flex items-center justify-center
                     transition-all duration-200 cursor-pointer ${
            communities.length === 0
              ? "bg-primary hover:bg-primary/90 text-primary-foreground shadow-lg animate-pulse"
              : "bg-sidebar-accent hover:bg-sidebar-accent/80 text-sidebar-accent-foreground"
          }`}
          title={communities.length === 0 ? "Create your first community!" : "Create Community"}
        >
          <Plus className="w-6 h-6" />
        </button>
      </div>

      <CreateCommunityDialog
        isOpen={isCreateDialogOpen}
        onClose={() => setIsCreateDialogOpen(false)}
        onSuccess={() => loadCommunities(currentPage)}
      />
    </>
  );
}

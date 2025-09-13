"use client";

import { useEffect, useState } from "react";
import { getCommunities } from "@/services/community";
import { Community } from "@/types/community";
import { Plus, Users, Trophy, Gamepad2, Zap } from "lucide-react";

export default function HomePage() {
  const [communities, setCommunities] = useState<Community[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const loadCommunities = async () => {
      try {
        const response = await getCommunities();
        setCommunities(response.data);
      } catch (err) {
        console.error('Failed to load communities:', err);
      } finally {
        setLoading(false);
      }
    };

    loadCommunities();
  }, []);

  if (loading) {
    return (
      <div className="p-8">
        <div className="mx-auto max-w-4xl space-y-8">
          <div className="animate-pulse">
            <div className="h-8 bg-muted rounded w-1/3 mb-4"></div>
            <div className="h-4 bg-muted rounded w-1/2"></div>
          </div>
        </div>
      </div>
    );
  }

  const hasCommunities = communities.length > 0;

  return (
    <div className="p-8">
      <div className="mx-auto max-w-4xl space-y-8">
        <div className="rounded-lg border bg-card p-8 shadow-sm">
          <h1 className="mb-4 text-4xl font-bold text-foreground">
            Welcome to Gamer CM
          </h1>
          <p className="text-lg text-muted-foreground">
            {hasCommunities 
              ? "Manage your gaming communities with ease."
              : "Get started by creating your first gaming community!"
            }
          </p>
        </div>

        {!hasCommunities ? (
          <div className="rounded-lg border border-dashed border-muted-foreground/25 bg-muted/10 p-12 text-center">
            <div className="mx-auto max-w-md space-y-6">
              <div className="w-16 h-16 mx-auto rounded-full bg-primary/10 flex items-center justify-center">
                <Users className="w-8 h-8 text-primary" />
              </div>
              
              <div className="space-y-2">
                <h2 className="text-2xl font-semibold text-foreground">
                  No communities yet
                </h2>
                <p className="text-muted-foreground">
                  Create your first gaming community to start organizing players and teams.
                </p>
              </div>

              <div className="flex items-center justify-center space-x-2 text-sm text-muted-foreground">
                <Plus className="w-4 h-4 animate-pulse text-primary" />
                <span>Click the + button in the sidebar to get started</span>
              </div>
            </div>
          </div>
        ) : (
          <div className="grid gap-4 md:grid-cols-2">
            <div className="rounded-lg border bg-card p-6 shadow-sm">
              <h2 className="mb-2 text-2xl font-semibold text-foreground">
                Features
              </h2>
              <ul className="space-y-2 text-muted-foreground">
                <li className="flex items-center gap-2">
                  <Users className="w-4 h-4" />
                  Community Management
                </li>
                <li className="flex items-center gap-2">
                  <Trophy className="w-4 h-4" />
                  Team Organization
                </li>
                <li className="flex items-center gap-2">
                  <Gamepad2 className="w-4 h-4" />
                  Player Tracking
                </li>
                <li className="flex items-center gap-2">
                  <Zap className="w-4 h-4" />
                  Real-time Updates
                </li>
              </ul>
            </div>

            <div className="rounded-lg border bg-card p-6 shadow-sm">
              <h2 className="mb-2 text-2xl font-semibold text-foreground">
                Quick Start
              </h2>
              <p className="text-muted-foreground">
                Select a community from the sidebar to get started.
              </p>
              <div className="mt-4 p-3 bg-muted rounded-md">
                <p className="text-sm text-muted-foreground">
                  <strong>{communities.length}</strong> communit{communities.length === 1 ? 'y' : 'ies'} available
                </p>
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
} 
"use client";

import React, { useEffect } from "react";
import { useRouter } from "next/navigation";
import { apiFetch, SetupStatusDto, ProjectSummaryDto } from "@/lib/api";

export default function HomePage() {
  const router = useRouter();

  useEffect(() => {
    let mounted = true;
    async function routeInitial() {
      try {
        const status = await apiFetch<SetupStatusDto>("/api/setup/status");
        if (!mounted) return;
        if (!status.setup_completed) {
          router.replace("/setup");
          return;
        }
        router.replace("/projects");
      } catch {
        if (mounted) router.replace("/setup");
      }
    }
    routeInitial();
    return () => {
      mounted = false;
    };
  }, [router]);

  return (
    <div className="min-h-screen flex items-center justify-center bg-stone-canvas">
      <div className="flex items-center gap-3 text-sm text-stone-muted">
        <img
          src="/dm-logo.png"
          alt="Design Master DM Logo"
          className="h-8 w-8 rounded-lg border border-stone-border object-cover shadow-sm"
        />
        <span>Loading Design Master workbench...</span>
      </div>
    </div>
  );
}

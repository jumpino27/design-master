"use client";

import React, { useState } from "react";
import {
  CheckCircle2,
  GitBranch,
  Layers,
  Play,
  Trash2,
  Sparkles,
} from "lucide-react";
import {
  apiFetch,
  formatMicrosUsd,
  PlanItem,
  PlannerPlan,
  RunDto,
} from "@/lib/api";

interface FullPagePlanReviewProps {
  run: RunDto;
  onApproved: (updatedRun: RunDto) => void;
}

export function FullPagePlanReview({
  run,
  onApproved,
}: FullPagePlanReviewProps) {
  const [plan, setPlan] = useState<PlannerPlan | null>(run.plan);
  const [approving, setApproving] = useState(false);
  const [error, setError] = useState<string | null>(null);

  if (!plan) return null;

  function handleUpdateItem(idx: number, patch: Partial<PlanItem>) {
    setPlan((prev) => {
      if (!prev) return prev;
      const items = [...prev.items];
      items[idx] = { ...items[idx], ...patch };
      return { ...prev, items };
    });
  }

  function handleRemoveItem(id: string) {
    setPlan((prev) => {
      if (!prev || prev.items.length <= 1) return prev;
      const items = prev.items
        .filter((i) => i.id !== id)
        .map((i) => ({
          ...i,
          depends_on: i.depends_on.filter((dep) => dep !== id),
        }));
      return { ...prev, items };
    });
  }

  async function handleGenerateSet() {
    if (!plan) return;
    setApproving(true);
    setError(null);
    try {
      const res = await apiFetch<{ run: RunDto }>(
        `/api/runs/${run.id}/approve-plan`,
        {
          method: "POST",
          body: JSON.stringify({ plan }),
        }
      );
      onApproved(res.run);
    } catch (err: unknown) {
      setError(
        err instanceof Error ? err.message : "Failed to approve and start set"
      );
    } finally {
      setApproving(false);
    }
  }

  return (
    <div className="rounded-xl border-2 border-burnt bg-stone-surface p-6 shadow-md space-y-5">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-stone-border pb-4">
        <div>
          <div className="inline-flex items-center gap-1.5 rounded-full bg-burnt/20 border border-burnt/40 px-2.5 py-0.5 text-xs font-semibold text-purple-200 mb-1.5">
            <Sparkles className="h-3.5 w-3.5 text-burnt" />
            <span>Full-Page Coordinated Set Plan Ready for Review</span>
          </div>
          <h2 className="text-lg font-bold text-stone-ink">
            One Coordinated Design Direction · {plan.items.length} Planned
            Output Images
          </h2>
          <p className="text-xs text-stone-muted mt-0.5">
            {plan.request_summary}
          </p>
        </div>

        <div className="flex flex-col sm:items-end gap-1.5 shrink-0">
          <div className="text-xs text-stone-muted">
            Est. cost for {plan.items.length} outputs:{" "}
            <strong className="text-stone-ink font-mono">
              {run.is_test_adapter
                ? "$0.00 (Test Fixture)"
                : formatMicrosUsd(run.estimated_cost_micros)}
            </strong>
          </div>
          <button
            type="button"
            disabled={approving}
            onClick={handleGenerateSet}
            className="inline-flex items-center gap-2 rounded-lg bg-burnt px-5 py-2.5 text-xs font-semibold text-white shadow-sm hover:bg-burnt-hover disabled:opacity-50"
          >
            <Play className="h-3.5 w-3.5 fill-current" />
            <span>
              {approving
                ? "Scheduling dependency graph..."
                : `Approve & Generate Set (${plan.items.length} images)`}
            </span>
          </button>
        </div>
      </div>

      {error && (
        <div className="rounded-lg border border-red-200 bg-red-50 px-3.5 py-2 text-xs text-red-900">
          {error}
        </div>
      )}

      {/* Page Composition & Shared Component Inventory */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
        {plan.page_composition.length > 0 && (
          <div className="rounded-lg border border-stone-border bg-white p-4 space-y-2">
            <div className="text-xs font-bold text-stone-ink flex items-center gap-1.5">
              <Layers className="h-3.5 w-3.5 text-burnt" />
              <span>Pages, Routes & Section Order</span>
            </div>
            <div className="space-y-2">
              {plan.page_composition.map((pc, idx) => (
                <div
                  key={idx}
                  className="text-xs border-b border-stone-border last:border-0 pb-2 last:pb-0"
                >
                  <div className="font-semibold text-stone-ink">
                    {pc.page_name}{" "}
                    <code className="rounded bg-stone-canvas px-1.5 py-0.5 text-[11px] font-mono">
                      {pc.route}
                    </code>{" "}
                    <span className="text-[11px] text-stone-muted">
                      ({pc.viewport_targets.join(", ")})
                    </span>
                  </div>
                  <div className="text-[11px] text-stone-muted mt-0.5">
                    Sections: {pc.ordered_sections.join(" → ")}
                  </div>
                  {pc.required_ctas.length > 0 && (
                    <div className="text-[11px] text-stone-muted">
                      CTAs: {pc.required_ctas.join(", ")}
                    </div>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}

        {plan.shared_components.length > 0 && (
          <div className="rounded-lg border border-stone-border bg-white p-4 space-y-2">
            <div className="text-xs font-bold text-stone-ink flex items-center gap-1.5">
              <GitBranch className="h-3.5 w-3.5 text-burnt" />
              <span>Shared Component Inventory & Reuse</span>
            </div>
            <div className="space-y-2">
              {plan.shared_components.map((sc) => (
                <div key={sc.id} className="text-xs">
                  <div className="font-semibold text-stone-ink">
                    {sc.name}{" "}
                    <span className="text-[11px] font-mono text-stone-muted">
                      (Canonical: {sc.canonical_item_id})
                    </span>
                  </div>
                  <div className="text-[11px] text-stone-muted">
                    Used on routes: {sc.used_on_routes.join(", ")} —{" "}
                    {sc.description}
                  </div>
                </div>
              ))}
            </div>
          </div>
        )}
      </div>

      {/* Editable Planned Outputs List */}
      <div className="space-y-3">
        <div className="text-xs font-bold text-stone-ink">
          Planned Outputs ({plan.items.length} separate images — edit titles/prompts or remove unneeded items before generating):
        </div>
        <div className="space-y-2.5">
          {plan.items.map((item, idx) => (
            <div
              key={item.id}
              className="rounded-lg border border-stone-border bg-white p-3.5 space-y-2"
            >
              <div className="flex flex-wrap items-center justify-between gap-2">
                <div className="flex items-center gap-2">
                  <span className="text-xs font-mono text-stone-muted">
                    {String(idx + 1).padStart(2, "0")}
                  </span>
                  <span className="rounded bg-charcoal px-2 py-0.5 text-[10px] font-mono uppercase text-white">
                    {item.kind}
                  </span>
                  {item.is_visual_anchor && (
                    <span className="inline-flex items-center gap-1 rounded bg-burnt-light border border-burnt/30 px-2 py-0.5 text-[10px] font-semibold text-burnt">
                      <CheckCircle2 className="h-3 w-3" />
                      Visual Anchor
                    </span>
                  )}
                  {item.page_route && (
                    <code className="text-[11px] font-mono text-stone-muted">
                      route: {item.page_route}
                    </code>
                  )}
                  <span className="text-[11px] text-stone-muted">
                    · {item.viewport.intent} ({item.aspect_ratio})
                  </span>
                  {item.depends_on.length > 0 && (
                    <span className="text-[11px] font-mono text-stone-muted">
                      · depends on: [{item.depends_on.join(", ")}]
                    </span>
                  )}
                </div>

                {plan.items.length > 1 && (
                  <button
                    type="button"
                    onClick={() => handleRemoveItem(item.id)}
                    title="Remove item from planned set"
                    className="inline-flex items-center gap-1 rounded px-2 py-1 text-[11px] text-red-700 hover:bg-red-50"
                  >
                    <Trash2 className="h-3.5 w-3.5" />
                    <span>Remove</span>
                  </button>
                )}
              </div>

              <div className="grid grid-cols-1 sm:grid-cols-3 gap-2">
                <div>
                  <label className="block text-[10px] text-stone-muted mb-0.5">
                    Item Title
                  </label>
                  <input
                    type="text"
                    value={item.name}
                    onChange={(e) =>
                      handleUpdateItem(idx, { name: e.target.value })
                    }
                    className="w-full rounded border border-stone-border px-2.5 py-1 text-xs font-medium"
                  />
                </div>
                <div className="sm:col-span-2">
                  <label className="block text-[10px] text-stone-muted mb-0.5">
                    Art Direction & Visual Prompt
                  </label>
                  <input
                    type="text"
                    value={item.image_prompt}
                    onChange={(e) =>
                      handleUpdateItem(idx, { image_prompt: e.target.value })
                    }
                    className="w-full rounded border border-stone-border px-2.5 py-1 text-xs"
                  />
                </div>
              </div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}

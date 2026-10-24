"use client";

import React from "react";
import {
  CheckCircle2,
  Code2,
  Cpu,
  FileSpreadsheet,
  Layers,
  ShieldCheck,
  Sparkles,
  AlertTriangle,
} from "lucide-react";
import { AssetDto, formatMicrosUsd, RunDto } from "@/lib/api";
import { LightDismissDialog } from "@/components/LightDismissDialog";

interface DetailsDrawerProps {
  open: boolean;
  onClose: () => void;
  run: RunDto | null;
  projectAssets: AssetDto[];
  onCheckConsistency?: (runId: string) => void;
  checkingConsistency?: boolean;
  onAdoptTokens?: (runId: string) => void;
  adoptingTokens?: boolean;
}

export function DetailsDrawer({
  open,
  onClose,
  run,
  projectAssets,
  onCheckConsistency,
  checkingConsistency = false,
  onAdoptTokens,
  adoptingTokens = false,
}: DetailsDrawerProps) {
  if (!run) return null;

  const assetMap = new Map(projectAssets.map((a) => [a.id, a]));
  const hasCompletedOutputs = run.outputs.some((o) => o.status === "completed");

  return (
    <LightDismissDialog
      open={open}
      onClose={onClose}
      title="Run Diagnostics & Technical Details"
      subtitle={`Run ID: ${run.id} · Mode: ${run.mode} · Brand Snapshot v${run.brand_version}`}
      maxWidthClass="max-w-4xl"
    >
      <div className="space-y-6 text-xs">
        {/* 1. Models, Context & Cost Summary */}
        <section className="rounded-lg border border-stone-border bg-white p-4 space-y-2">
          <div className="flex items-center gap-2 font-bold text-stone-ink">
            <Cpu className="h-4 w-4 text-burnt" />
            <span>Models, Capabilities & Cost Accounting</span>
          </div>
          <div className="grid grid-cols-1 sm:grid-cols-3 gap-3 pt-1">
            <div>
              <span className="text-stone-muted block">Planner Model:</span>
              <code className="font-mono font-semibold">
                {run.model_settings.planner_model}
              </code>
            </div>
            <div>
              <span className="text-stone-muted block">Image Model:</span>
              <code className="font-mono font-semibold">
                {run.model_settings.image_model}
              </code>
            </div>
            <div>
              <span className="text-stone-muted block">
                Quality / Aspect / Size:
              </span>
              <code className="font-mono font-semibold">
                quality={run.model_settings.quality} ·{" "}
                {run.model_settings.aspect_ratio} ·{" "}
                {run.model_settings.output_size}
              </code>
            </div>
          </div>

          <div className="grid grid-cols-1 sm:grid-cols-3 gap-3 pt-2 border-t border-stone-border">
            <div>
              <span className="text-stone-muted block">Context Isolation:</span>
              <span className="font-medium text-stone-ink">
                {run.context_summary}
              </span>
            </div>
            <div>
              <span className="text-stone-muted block">Estimated Cost:</span>
              <span className="font-mono">
                {run.is_test_adapter
                  ? "$0.00 (Test Fixture)"
                  : formatMicrosUsd(run.estimated_cost_micros)}
              </span>
            </div>
            <div>
              <span className="text-stone-muted block">
                Reported Actual Cost:
              </span>
              <span className="font-mono">
                {run.is_test_adapter
                  ? "$0.00 (Test Fixture)"
                  : run.cost_is_unknown
                  ? "Unknown (not reported by provider)"
                  : formatMicrosUsd(run.actual_cost_micros)}
              </span>
            </div>
          </div>
        </section>

        {/* 2. Selected Reference Roles & Input Resizing Diagnostics */}
        <section className="rounded-lg border border-stone-border bg-white p-4 space-y-2">
          <div className="flex items-center gap-2 font-bold text-stone-ink">
            <Layers className="h-4 w-4 text-burnt" />
            <span>
              Selected Reference Roles & Input Derivatives (
              {run.selected_references.length})
            </span>
          </div>
          {run.selected_references.length === 0 ? (
            <p className="text-stone-muted">
              No explicit reference images attached to this run.
            </p>
          ) : (
            <div className="space-y-2">
              {run.selected_references.map((sr) => {
                const fullAsset = assetMap.get(sr.asset_id);
                return (
                  <div
                    key={sr.asset_id}
                    className="rounded border border-stone-border bg-stone-surface p-2.5 flex items-center justify-between gap-3"
                  >
                    <div>
                      <div className="font-semibold text-stone-ink">
                        {sr.label}{" "}
                        <span className="rounded bg-charcoal px-1.5 py-0.5 text-[10px] font-mono text-white">
                          {sr.role}
                        </span>
                      </div>
                      <div className="text-[11px] text-stone-muted mt-0.5">
                        Asset ID: <code>{sr.asset_id}</code> · Preserve-as-is:{" "}
                        {sr.preserve_as_is ? "Yes" : "No"}
                        {sr.placement_notes
                          ? ` · Notes: ${sr.placement_notes}`
                          : ""}
                      </div>
                    </div>
                    {fullAsset?.metadata && (
                      <div className="text-right text-[11px] font-mono text-stone-muted">
                        {fullAsset.metadata.resized_for_model ? (
                          <span className="text-amber-800 font-semibold">
                            Resized for model:{" "}
                            {fullAsset.metadata.original_dimensions?.width}×
                            {fullAsset.metadata.original_dimensions?.height} →{" "}
                            {fullAsset.metadata.model_input_dimensions?.width}×
                            {fullAsset.metadata.model_input_dimensions?.height}
                          </span>
                        ) : fullAsset.metadata.svg_sanitized ? (
                          <span className="text-emerald-800">
                            Sanitized SVG rasterized to PNG for model input
                          </span>
                        ) : (
                          <span>Original dimensions sent unmodified</span>
                        )}
                      </div>
                    )}
                  </div>
                );
              })}
            </div>
          )}
        </section>

        {/* 3. Compiled Image Prompts & Output Diagnostics */}
        <section className="rounded-lg border border-stone-border bg-white p-4 space-y-3">
          <div className="flex items-center gap-2 font-bold text-stone-ink">
            <Code2 className="h-4 w-4 text-burnt" />
            <span>Compiled Image Prompts & Output Metadata</span>
          </div>
          <div className="space-y-3">
            {run.outputs.map((o) => (
              <details
                key={o.id}
                className="rounded border border-stone-border bg-stone-surface p-3"
              >
                <summary className="cursor-pointer font-semibold text-stone-ink flex items-center justify-between">
                  <span>
                    #{o.variant_index} {o.title} ({o.kind} · {o.status})
                  </span>
                  <span className="text-[11px] font-mono text-stone-muted">
                    Req ID: {o.provider_request_id || "n/a"} ·{" "}
                    {o.usage?.duration_ms ? `${o.usage.duration_ms}ms` : ""}
                  </span>
                </summary>
                <div className="mt-2 space-y-2">
                  {o.absolute_path && (
                    <div className="text-[11px] font-mono text-stone-muted">
                      Original path: <code>{o.absolute_path}</code> ({o.width}×
                      {o.height}px, alpha={o.has_alpha ? "true" : "false"})
                    </div>
                  )}
                  <pre className="whitespace-pre-wrap rounded bg-white border border-stone-border p-2.5 text-[11px] font-mono text-stone-ink max-h-48 overflow-y-auto">
                    {o.compiled_prompt}
                  </pre>
                </div>
              </details>
            ))}
          </div>
        </section>

        {/* 4. Visual Consistency Audit & Brand Token Actions */}
        {(hasCompletedOutputs || run.plan) && (
          <section className="rounded-lg border border-stone-border bg-white p-4 space-y-3">
            <div className="flex flex-wrap items-center justify-between gap-2">
              <div className="flex items-center gap-2 font-bold text-stone-ink">
                <ShieldCheck className="h-4 w-4 text-burnt" />
                <span>Visual Consistency Audit & Brand Token Actions</span>
              </div>
              <div className="flex items-center gap-2">
                {hasCompletedOutputs && onCheckConsistency && (
                  <button
                    type="button"
                    disabled={checkingConsistency}
                    onClick={() => onCheckConsistency(run.id)}
                    className="inline-flex items-center gap-1.5 rounded-lg border border-stone-border bg-stone-surface px-3 py-1.5 text-[11px] font-semibold text-stone-ink hover:bg-stone-canvas disabled:opacity-50"
                    title="Run vision consistency audit across all completed outputs in this run"
                  >
                    <ShieldCheck className="h-3.5 w-3.5 text-stone-muted" />
                    <span>
                      {checkingConsistency
                        ? "Checking Consistency..."
                        : run.consistency_report
                        ? "Re-run Consistency Check"
                        : "Check Consistency"}
                    </span>
                  </button>
                )}
                {run.plan && onAdoptTokens && (
                  <button
                    type="button"
                    disabled={adoptingTokens}
                    onClick={() => onAdoptTokens(run.id)}
                    className="inline-flex items-center gap-1.5 rounded-lg border border-stone-border bg-stone-surface px-3 py-1.5 text-[11px] font-semibold text-stone-ink hover:bg-stone-canvas disabled:opacity-50"
                    title="Save the AI planner's colors & typography from this run into a new Brand Kit version"
                  >
                    <Sparkles className="h-3.5 w-3.5 text-burnt" />
                    <span>
                      {adoptingTokens ? "Adopting..." : "Adopt Run Tokens"}
                    </span>
                  </button>
                )}
              </div>
            </div>

            {run.consistency_report ? (
              <div className="rounded-lg border border-stone-border bg-stone-surface p-3 space-y-2">
                <div className="flex items-center justify-between">
                  <div className="flex items-center gap-1.5 font-semibold text-stone-ink">
                    {run.consistency_report.overall_consistent ? (
                      <CheckCircle2 className="h-4 w-4 text-emerald-600" />
                    ) : (
                      <AlertTriangle className="h-4 w-4 text-amber-600" />
                    )}
                    <span>
                      {run.consistency_report.overall_consistent
                        ? "Outputs are Visually Consistent"
                        : "Visual Inconsistencies Detected"}
                    </span>
                  </div>
                  <span className="text-[10px] font-mono text-stone-muted">
                    Checked {run.consistency_report.checked_at}
                  </span>
                </div>
                <p className="text-stone-ink leading-relaxed">
                  {run.consistency_report.summary}
                </p>
                {run.consistency_report.findings.length > 0 && (
                  <div>
                    <div className="font-semibold text-stone-ink mb-1">
                      Findings:
                    </div>
                    <ul className="list-disc pl-4 space-y-0.5 text-stone-muted">
                      {run.consistency_report.findings.map((f, i) => (
                        <li key={i}>{f}</li>
                      ))}
                    </ul>
                  </div>
                )}
                {run.consistency_report.recommendations.length > 0 && (
                  <div>
                    <div className="font-semibold text-stone-ink mb-1">
                      Recommendations:
                    </div>
                    <ul className="list-disc pl-4 space-y-0.5 text-stone-muted">
                      {run.consistency_report.recommendations.map((r, i) => (
                        <li key={i}>{r}</li>
                      ))}
                    </ul>
                  </div>
                )}
              </div>
            ) : (
              <p className="text-stone-muted">
                Run a visual consistency check to audit color palette,
                typography, and layout alignment across this run&apos;s outputs,
                or adopt this run&apos;s generated design tokens into your Brand
                Kit.
              </p>
            )}
          </section>
        )}

        {/* 5. Structured Planner Output & Immutable Brand Snapshot */}
        <section className="rounded-lg border border-stone-border bg-white p-4 space-y-3">
          <div className="flex items-center gap-2 font-bold text-stone-ink">
            <FileSpreadsheet className="h-4 w-4 text-burnt" />
            <span>Structured Planner Schema & Immutable Brand Snapshot</span>
          </div>
          <details className="rounded border border-stone-border bg-stone-surface p-3">
            <summary className="cursor-pointer font-semibold text-stone-ink">
              Planner JSON Output (schema_version=1)
            </summary>
            <pre className="mt-2 whitespace-pre-wrap rounded bg-white border border-stone-border p-2.5 text-[11px] font-mono text-stone-ink max-h-60 overflow-y-auto">
              {JSON.stringify(run.plan, null, 2)}
            </pre>
          </details>
          <details className="rounded border border-stone-border bg-stone-surface p-3">
            <summary className="cursor-pointer font-semibold text-stone-ink">
              Frozen Brand Kit Snapshot (v{run.brand_version})
            </summary>
            <pre className="mt-2 whitespace-pre-wrap rounded bg-white border border-stone-border p-2.5 text-[11px] font-mono text-stone-ink max-h-60 overflow-y-auto">
              {JSON.stringify(run.brand_snapshot, null, 2)}
            </pre>
          </details>
        </section>
      </div>
    </LightDismissDialog>
  );
}

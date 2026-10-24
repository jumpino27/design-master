"use client";

import React, { useRef, useState } from "react";
import {
  Archive,
  Check,
  Copy,
  Download,
  FileText,
  AlertTriangle,
} from "lucide-react";
import { apiFetch, HandoffResponse } from "@/lib/api";
import { LightDismissDialog } from "@/components/LightDismissDialog";

interface HandoffDialogProps {
  open: boolean;
  onClose: () => void;
  handoff: HandoffResponse | null;
  runId: string;
  outputId?: string | null;
  outputIds?: string[] | null;
  tabLabel?: string | null;
  onHandoffUpdated: (h: HandoffResponse) => void;
}

export function HandoffDialog({
  open,
  onClose,
  handoff,
  runId,
  outputId,
  outputIds,
  tabLabel,
  onHandoffUpdated,
}: HandoffDialogProps) {
  const textareaRef = useRef<HTMLTextAreaElement | null>(null);
  const [copied, setCopied] = useState(false);
  const [copyFailed, setCopyFailed] = useState(false);
  const [exportingBundle, setExportingBundle] = useState(false);
  const [togglingExtract, setTogglingExtract] = useState(false);

  if (!handoff) return null;

  async function handleCopyClick() {
    if (!handoff) return;
    setCopyFailed(false);
    try {
      await navigator.clipboard.writeText(handoff.clipboard_text);
      setCopied(true);
      setTimeout(() => setCopied(false), 2500);
    } catch {
      setCopyFailed(true);
      textareaRef.current?.focus();
      textareaRef.current?.select();
    }
  }

  async function handleToggleExtraction(checked: boolean) {
    setTogglingExtract(true);
    try {
      const endpoint = outputId
        ? `/api/outputs/${outputId}/handoff`
        : `/api/runs/${runId}/handoff`;
      const res = await apiFetch<{ handoff: HandoffResponse }>(endpoint, {
        method: "POST",
        body: JSON.stringify({
          include_extraction_instructions: checked,
          output_ids: outputId ? undefined : outputIds || undefined,
        }),
      });
      onHandoffUpdated(res.handoff);
    } finally {
      setTogglingExtract(false);
    }
  }

  async function handleExportPortableBundle() {
    setExportingBundle(true);
    try {
      const res = await apiFetch<{
        handoff: HandoffResponse;
        download_url: string;
      }>(`/api/runs/${runId}/export-bundle`, {
        method: "POST",
        body: JSON.stringify({
          include_extraction_instructions:
            handoff?.include_extraction_instructions ?? true,
          output_ids: outputId ? [outputId] : outputIds || undefined,
        }),
      });
      onHandoffUpdated(res.handoff);
      window.location.href = res.download_url;
    } finally {
      setExportingBundle(false);
    }
  }

  return (
    <LightDismissDialog
      open={open}
      onClose={onClose}
      title={
        outputId
          ? "Copy Prompt for Coding Agent (Single Image)"
          : tabLabel
          ? `Copy Prompt for Whole "${tabLabel}" Tab`
          : handoff.is_complete_set
          ? "Copy Entire Set for Coding Agent"
          : "Copy Available Set Items (Partial / Incomplete Set)"
      }
      subtitle="Deterministic implementation handoff containing local absolute paths, brand tokens, original assets, and extraction instructions."
      maxWidthClass="max-w-4xl"
    >
      <div className="space-y-4">
        {!handoff.is_complete_set && handoff.missing_items.length > 0 && (
          <div className="rounded-lg border border-amber-300 bg-amber-50 p-3 text-xs text-amber-900 flex items-start gap-2">
            <AlertTriangle className="h-4 w-4 shrink-0 mt-0.5 text-amber-700" />
            <div>
              <div className="font-semibold">
                Incomplete Set Notice ({handoff.missing_items.length} missing):
              </div>
              <ul className="list-disc pl-4 mt-1 space-y-0.5">
                {handoff.missing_items.map((m, i) => (
                  <li key={i}>{m}</li>
                ))}
              </ul>
            </div>
          </div>
        )}

        <div className="flex flex-wrap items-center justify-between gap-3 bg-white border border-stone-border rounded-lg px-4 py-2.5">
          <label className="inline-flex items-center gap-2 text-xs font-medium text-stone-ink cursor-pointer">
            <input
              type="checkbox"
              disabled={togglingExtract}
              checked={handoff.include_extraction_instructions}
              onChange={(e) => handleToggleExtraction(e.target.checked)}
              className="rounded border-stone-border text-burnt focus:ring-burnt"
            />
            <span>
              Include embedded asset extraction & crop instructions (Default ON)
            </span>
          </label>

          <div className="flex flex-wrap items-center gap-2">
            <a
              href={`/api/exports/${handoff.export_id}/download`}
              download
              className="inline-flex items-center gap-1.5 rounded-lg border border-stone-border bg-stone-surface px-3 py-1.5 text-xs font-medium text-stone-ink hover:bg-stone-canvas"
            >
              <Download className="h-3.5 w-3.5" />
              <span>Download implementation.md</span>
            </a>

            <button
              type="button"
              disabled={exportingBundle}
              onClick={handleExportPortableBundle}
              className="inline-flex items-center gap-1.5 rounded-lg border border-stone-border bg-stone-surface px-3 py-1.5 text-xs font-medium text-stone-ink hover:bg-stone-canvas disabled:opacity-50"
            >
              <Archive className="h-3.5 w-3.5 text-burnt" />
              <span>
                {exportingBundle
                  ? "Packaging ZIP..."
                  : "Export portable bundle (.zip)"}
              </span>
            </button>

            <button
              type="button"
              onClick={handleCopyClick}
              className="inline-flex items-center gap-1.5 rounded-lg bg-burnt px-4 py-1.5 text-xs font-semibold text-white hover:bg-burnt-hover"
            >
              {copied ? (
                <>
                  <Check className="h-3.5 w-3.5" />
                  <span>Copied</span>
                </>
              ) : (
                <>
                  <Copy className="h-3.5 w-3.5" />
                  <span>Copy handoff prompt</span>
                </>
              )}
            </button>
          </div>
        </div>

        {copyFailed && (
          <div className="rounded-lg border border-amber-300 bg-amber-50 px-3 py-2 text-xs text-amber-900">
            Browser clipboard access was blocked. The handoff text below is
            selected—press <kbd className="font-mono font-bold">Cmd+C</kbd> or{" "}
            <kbd className="font-mono font-bold">Ctrl+C</kbd> to copy.
          </div>
        )}

        <div>
          <div className="flex items-center justify-between text-[11px] text-stone-muted mb-1">
            <span className="inline-flex items-center gap-1">
              <FileText className="h-3 w-3" />
              Saved at:{" "}
              <code className="font-mono">{handoff.implementation_path}</code>
            </span>
            <button
              type="button"
              onClick={() => {
                textareaRef.current?.focus();
                textareaRef.current?.select();
              }}
              className="text-burnt hover:underline font-medium"
            >
              Select all text
            </button>
          </div>
          <textarea
            ref={textareaRef}
            readOnly
            rows={18}
            value={handoff.clipboard_text}
            aria-label="Handoff implementation prompt text"
            className="w-full rounded-lg border border-stone-border bg-white p-3.5 text-xs font-mono text-stone-ink leading-relaxed focus:outline-burnt"
          />
        </div>
      </div>
    </LightDismissDialog>
  );
}

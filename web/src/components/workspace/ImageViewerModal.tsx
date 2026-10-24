"use client";

import React, { useEffect, useState } from "react";
import {
  Check,
  ChevronLeft,
  ChevronRight,
  Columns,
  Copy,
  Crosshair,
  Download,
  Grid,
  Maximize,
  RotateCcw,
  Sparkles,
  X,
  ZoomIn,
  ZoomOut,
} from "lucide-react";
import { OutputDto } from "@/lib/api";

interface ImageViewerModalProps {
  open: boolean;
  onClose: () => void;
  activeOutput: OutputDto | null;
  allOutputs: OutputDto[];
  onSelectOutput: (o: OutputDto) => void;
  onCopyPrompt?: (o: OutputDto) => void;
  onStartHighlightEdit?: (o: OutputDto) => void;
  copiedId?: string | null;
}

export function ImageViewerModal({
  open,
  onClose,
  activeOutput,
  allOutputs,
  onSelectOutput,
  onCopyPrompt,
  onStartHighlightEdit,
  copiedId,
}: ImageViewerModalProps) {
  const [zoomMode, setZoomMode] = useState<"fit" | number>("fit");
  const [showCheckerboard, setShowCheckerboard] = useState(true);
  const [compareMode, setCompareMode] = useState(false);
  const [compareOutputId, setCompareOutputId] = useState<string>("");

  const completedOutputs = allOutputs.filter(
    (o) => o.status === "completed" && o.original_url
  );

  const currentIndex = activeOutput
    ? completedOutputs.findIndex((o) => o.id === activeOutput.id)
    : -1;

  useEffect(() => {
    if (!open) return;
    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === "Escape") {
        onClose();
      } else if (
        e.key === "ArrowLeft" &&
        !compareMode &&
        completedOutputs.length > 1 &&
        currentIndex > 0
      ) {
        onSelectOutput(completedOutputs[currentIndex - 1]);
      } else if (
        e.key === "ArrowRight" &&
        !compareMode &&
        completedOutputs.length > 1 &&
        currentIndex >= 0 &&
        currentIndex < completedOutputs.length - 1
      ) {
        onSelectOutput(completedOutputs[currentIndex + 1]);
      }
    };
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [open, onClose, compareMode, completedOutputs, currentIndex, onSelectOutput]);

  if (!open || !activeOutput || !activeOutput.original_url) return null;

  const compareTarget =
    completedOutputs.find((o) => o.id === compareOutputId) ||
    completedOutputs.find((o) => o.id !== activeOutput.id) ||
    activeOutput;

  const numericZoom = typeof zoomMode === "number" ? zoomMode : 1;

  return (
    <div
      role="dialog"
      aria-modal="true"
      aria-label={`Fullscreen viewer: ${activeOutput.title}`}
      className="fixed inset-0 z-50 flex flex-col bg-[#121110]/95 backdrop-blur-md text-white select-none"
      onClick={onClose}
    >
      {/* ================================================================= */}
      {/* TOP STUDIO TOOLBAR                                                */}
      {/* ================================================================= */}
      <div
        onClick={(e) => e.stopPropagation()}
        className="flex flex-wrap items-center justify-between gap-3 border-b border-white/10 bg-[#1B1A18] px-5 py-3 shrink-0"
      >
        {/* Left: Image Title & Technical Metadata */}
        <div className="flex items-center gap-3 min-w-0">
          <div className="min-w-0">
            <div className="flex items-center gap-2">
              <span className="truncate text-sm font-bold text-white">
                {activeOutput.title}
              </span>
              {activeOutput.tab_label && (
                <span className="rounded-md bg-white/10 px-2 py-0.5 text-[10px] font-semibold text-white/80">
                  {activeOutput.tab_label}
                </span>
              )}
              {activeOutput.has_alpha && (
                <span className="inline-flex items-center gap-1 rounded-md bg-emerald-500/20 border border-emerald-400/40 px-2 py-0.5 text-[10px] font-semibold text-emerald-300">
                  <Sparkles className="h-2.5 w-2.5" />
                  Transparent PNG
                </span>
              )}
            </div>
            <div className="flex items-center gap-2 text-[11px] text-white/55 font-mono mt-0.5">
              <span>
                {activeOutput.width || 1024}×{activeOutput.height || 1024}px
              </span>
              <span>·</span>
              <span>{activeOutput.aspect_ratio || "16:9"}</span>
              {activeOutput.extracted_from_output_id && (
                <>
                  <span>·</span>
                  <span className="text-emerald-300 font-sans">
                    Extracted Foreground Asset
                  </span>
                </>
              )}
            </div>
          </div>
        </div>

        {/* Center: Zoom & Viewport Controls */}
        <div className="flex items-center gap-1.5 rounded-xl border border-white/15 bg-black/40 p-1">
          <button
            type="button"
            onClick={() =>
              setZoomMode((prev) => {
                const cur = prev === "fit" ? 1 : prev;
                return Math.max(0.5, Math.round((cur - 0.25) * 100) / 100);
              })
            }
            className="rounded-lg p-1.5 text-white/80 hover:bg-white/10 hover:text-white transition-colors"
            title="Zoom Out"
          >
            <ZoomOut className="h-3.5 w-3.5" />
          </button>

          <button
            type="button"
            onClick={() => setZoomMode("fit")}
            className={`rounded-lg px-2.5 py-1 text-[11px] font-semibold transition-colors ${
              zoomMode === "fit"
                ? "bg-burnt text-white"
                : "text-white/75 hover:bg-white/10 hover:text-white"
            }`}
            title="Fit entire image inside screen"
          >
            Fit
          </button>

          <button
            type="button"
            onClick={() => setZoomMode(1)}
            className={`rounded-lg px-2.5 py-1 font-mono text-[11px] font-semibold transition-colors ${
              zoomMode !== "fit"
                ? "bg-burnt text-white"
                : "text-white/75 hover:bg-white/10 hover:text-white"
            }`}
            title="View at 100% pixel scale (scrollable for tall pages)"
          >
            {zoomMode === "fit" ? "100%" : `${Math.round(numericZoom * 100)}%`}
          </button>

          <button
            type="button"
            onClick={() =>
              setZoomMode((prev) => {
                const cur = prev === "fit" ? 1 : prev;
                return Math.min(2.5, Math.round((cur + 0.25) * 100) / 100);
              })
            }
            className="rounded-lg p-1.5 text-white/80 hover:bg-white/10 hover:text-white transition-colors"
            title="Zoom In"
          >
            <ZoomIn className="h-3.5 w-3.5" />
          </button>

          <div className="h-4 w-px bg-white/15 mx-0.5" />

          <button
            type="button"
            onClick={() => setShowCheckerboard((v) => !v)}
            className={`inline-flex items-center gap-1 rounded-lg px-2.5 py-1 text-[11px] font-medium transition-colors ${
              showCheckerboard
                ? "bg-white/15 text-white"
                : "text-white/60 hover:bg-white/10 hover:text-white"
            }`}
            title="Toggle alpha checkerboard background"
          >
            <Grid className="h-3.5 w-3.5" />
            <span className="hidden sm:inline">Grid BG</span>
          </button>

          {completedOutputs.length > 1 && (
            <button
              type="button"
              onClick={() => setCompareMode((c) => !c)}
              className={`inline-flex items-center gap-1 rounded-lg px-2.5 py-1 text-[11px] font-medium transition-colors ${
                compareMode
                  ? "bg-burnt text-white"
                  : "text-white/70 hover:bg-white/10 hover:text-white"
              }`}
              title="Compare two images side-by-side"
            >
              <Columns className="h-3.5 w-3.5" />
              <span className="hidden sm:inline">Compare</span>
            </button>
          )}
        </div>

        {/* Right: Well-Organized Action Buttons */}
        <div className="flex items-center gap-2">
          {onCopyPrompt && (
            <button
              type="button"
              onClick={() => onCopyPrompt(activeOutput)}
              className="inline-flex items-center gap-1.5 rounded-lg border border-white/15 bg-white/10 px-3 py-1.5 text-xs font-semibold text-white hover:bg-white/20 transition-colors"
              title="Copy Coding-Agent Handoff Prompt for this image"
            >
              {copiedId === activeOutput.id ? (
                <>
                  <Check className="h-3.5 w-3.5 text-emerald-400" />
                  <span>Copied Prompt ✓</span>
                </>
              ) : (
                <>
                  <Copy className="h-3.5 w-3.5 text-burnt" />
                  <span>Copy Prompt</span>
                </>
              )}
            </button>
          )}

          {onStartHighlightEdit && !activeOutput.extracted_from_output_id && (
            <button
              type="button"
              onClick={() => {
                onClose();
                onStartHighlightEdit(activeOutput);
              }}
              className="inline-flex items-center gap-1.5 rounded-lg border border-burnt/50 bg-burnt/20 px-3 py-1.5 text-xs font-semibold text-amber-200 hover:bg-burnt hover:text-white transition-colors"
              title="Highlight a region on this image to edit with Sunburst 2.5 Medium"
            >
              <Crosshair className="h-3.5 w-3.5" />
              <span className="hidden md:inline">Highlight & Edit</span>
            </button>
          )}

          <a
            href={activeOutput.original_url}
            download={`${activeOutput.title
              .toLowerCase()
              .replace(/[^a-z0-9]+/g, "-")}.png`}
            className="inline-flex items-center gap-1.5 rounded-lg bg-burnt px-3.5 py-1.5 text-xs font-bold text-white hover:bg-burnt-hover transition-colors shadow-sm"
          >
            <Download className="h-3.5 w-3.5" />
            <span className="hidden sm:inline">Download PNG</span>
          </a>

          <button
            type="button"
            onClick={onClose}
            aria-label="Close fullscreen viewer"
            className="ml-1 flex h-8 w-8 items-center justify-center rounded-lg border border-white/15 bg-white/5 text-white/80 hover:bg-white/15 hover:text-white transition-colors"
            title="Close (Esc)"
          >
            <X className="h-4 w-4" />
          </button>
        </div>
      </div>

      {/* ================================================================= */}
      {/* CENTER VIEWPORT (SMOOTH SCROLL & ZOOM, NEVER CLIPS TALL PAGES)    */}
      {/* ================================================================= */}
      <div
        onClick={(e) => e.stopPropagation()}
        className="relative flex-1 min-h-0 overflow-auto flex items-center justify-center p-6"
      >
        {/* Left/Right Navigation Arrows when not comparing */}
        {!compareMode && completedOutputs.length > 1 && currentIndex > 0 && (
          <button
            type="button"
            onClick={() => onSelectOutput(completedOutputs[currentIndex - 1])}
            className="fixed left-5 top-1/2 -translate-y-1/2 z-30 flex h-11 w-11 items-center justify-center rounded-full border border-white/20 bg-black/65 text-white shadow-xl hover:bg-burnt transition-colors"
            title="Previous image (←)"
          >
            <ChevronLeft className="h-5 w-5" />
          </button>
        )}

        {!compareMode &&
          completedOutputs.length > 1 &&
          currentIndex >= 0 &&
          currentIndex < completedOutputs.length - 1 && (
            <button
              type="button"
              onClick={() => onSelectOutput(completedOutputs[currentIndex + 1])}
              className="fixed right-5 top-1/2 -translate-y-1/2 z-30 flex h-11 w-11 items-center justify-center rounded-full border border-white/20 bg-black/65 text-white shadow-xl hover:bg-burnt transition-colors"
              title="Next image (→)"
            >
              <ChevronRight className="h-5 w-5" />
            </button>
          )}

        {compareMode ? (
          <div className="grid grid-cols-1 lg:grid-cols-2 gap-6 w-full max-w-7xl h-full">
            <div className="flex flex-col rounded-2xl border border-white/15 bg-[#1B1A18] p-4 min-h-0">
              <div className="text-xs font-bold text-white mb-2 flex items-center justify-between">
                <span>Left: {activeOutput.title}</span>
                <span className="font-mono text-[11px] text-white/50">
                  {activeOutput.width}×{activeOutput.height}
                </span>
              </div>
              <div
                className={`flex-1 overflow-auto rounded-xl border border-white/10 flex items-center justify-center p-3 ${
                  showCheckerboard ? "checkerboard-bg" : "bg-[#22211F]"
                }`}
              >
                <img
                  src={activeOutput.original_url}
                  alt={activeOutput.title}
                  className="max-h-full max-w-full object-contain rounded shadow-lg"
                />
              </div>
            </div>

            <div className="flex flex-col rounded-2xl border border-white/15 bg-[#1B1A18] p-4 min-h-0">
              <div className="flex items-center justify-between mb-2 gap-2">
                <span className="text-xs font-bold text-white">
                  Compare With:
                </span>
                <select
                  value={compareTarget.id}
                  onChange={(e) => setCompareOutputId(e.target.value)}
                  className="rounded-lg border border-white/20 bg-black/50 px-2.5 py-1 text-xs text-white focus:border-burnt focus:outline-none"
                >
                  {completedOutputs.map((o, idx) => (
                    <option key={o.id} value={o.id}>
                      #{idx + 1} · {o.title}
                    </option>
                  ))}
                </select>
              </div>
              <div
                className={`flex-1 overflow-auto rounded-xl border border-white/10 flex items-center justify-center p-3 ${
                  showCheckerboard ? "checkerboard-bg" : "bg-[#22211F]"
                }`}
              >
                {compareTarget.original_url && (
                  <img
                    src={compareTarget.original_url}
                    alt={compareTarget.title}
                    className="max-h-full max-w-full object-contain rounded shadow-lg"
                  />
                )}
              </div>
            </div>
          </div>
        ) : (
          <div
            className={`rounded-2xl border border-white/15 shadow-2xl overflow-auto ${
              showCheckerboard ? "checkerboard-bg" : "bg-[#1E1D1B]"
            } ${
              zoomMode === "fit"
                ? "max-h-full max-w-full flex items-center justify-center p-3"
                : "p-4"
            }`}
          >
            <img
              src={activeOutput.original_url}
              alt={activeOutput.title}
              style={
                zoomMode === "fit"
                  ? { maxHeight: "calc(100vh - 210px)", maxWidth: "88vw" }
                  : {
                      width: `${Math.round(
                        (activeOutput.width || 1024) * numericZoom
                      )}px`,
                      maxWidth: "none",
                    }
              }
              className="block object-contain rounded-lg select-none"
            />
          </div>
        )}
      </div>

      {/* ================================================================= */}
      {/* BOTTOM THUMBNAIL STRIP                                            */}
      {/* ================================================================= */}
      {completedOutputs.length > 1 && (
        <div
          onClick={(e) => e.stopPropagation()}
          className="flex items-center gap-2.5 border-t border-white/10 bg-[#1B1A18] px-5 py-2.5 overflow-x-auto shrink-0"
        >
          {completedOutputs.map((o, idx) => {
            const isSelected = o.id === activeOutput.id;
            return (
              <button
                key={o.id}
                type="button"
                onClick={() => onSelectOutput(o)}
                className={`flex items-center gap-2.5 rounded-xl border p-1.5 pr-3 text-left transition-all shrink-0 ${
                  isSelected
                    ? "border-burnt bg-burnt/20 text-white shadow-sm ring-1 ring-burnt"
                    : "border-white/10 bg-white/5 text-white/70 hover:bg-white/10 hover:text-white"
                }`}
              >
                <div className="h-10 w-12 rounded-lg overflow-hidden border border-white/15 checkerboard-bg shrink-0 flex items-center justify-center">
                  <img
                    src={o.thumbnail_url || o.original_url || ""}
                    alt={o.title}
                    className="h-full w-full object-contain"
                  />
                </div>
                <div className="min-w-0">
                  <div className="truncate max-w-[140px] text-xs font-bold">
                    {idx + 1}. {o.title}
                  </div>
                  <div className="text-[10px] font-mono text-white/50">
                    {o.width || 1024}×{o.height || 1024}
                  </div>
                </div>
              </button>
            );
          })}
        </div>
      )}
    </div>
  );
}

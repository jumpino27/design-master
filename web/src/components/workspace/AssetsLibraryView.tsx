"use client";

import React, { useState } from "react";
import {
  AlertCircle,
  Check,
  CheckCircle2,
  Eye,
  FileImage,
  FolderOpen,
  Layers,
  MessageSquarePlus,
  Palette,
  Save,
  ShieldCheck,
  Sparkles,
  Trash2,
  Type,
  Upload,
  X,
} from "lucide-react";
import { apiFetch, AssetDto, AttachmentRole } from "@/lib/api";

interface AssetsLibraryViewProps {
  projectId: string;
  assets: AssetDto[];
  onRefresh: () => void;
  onAttachToComposer: (asset: AssetDto) => void;
}

interface RoleConfig {
  role: AttachmentRole;
  shortLabel: string;
  title: string;
  description: string;
  badgeColor: string;
}

const ROLE_CONFIGS: RoleConfig[] = [
  {
    role: "include_asset",
    shortLabel: "Include in Design",
    title: "Include Asset in Design",
    description:
      "Product photo, 3D illustration, icon, or graphic that the AI must incorporate into the generated visual.",
    badgeColor: "bg-crimson-600/15 text-crimson-300 border-crimson-600/30",
  },
  {
    role: "brand_logo",
    shortLabel: "Brand Logo",
    title: "Official Brand Logo",
    description:
      "Primary logo or mark. Used by AI for brand palette/style harmony and preserved verbatim in handoff.",
    badgeColor: "bg-amber-500/15 text-amber-800 border-amber-500/30",
  },
  {
    role: "current_screenshot",
    shortLabel: "Current UI Screenshot",
    title: "Current UI Screenshot (Polish & Extend)",
    description:
      "Screenshot of an existing navbar, hero, or page so the AI knows what buttons/elements exist and can polish or add features.",
    badgeColor: "bg-crimson-500/15 text-crimson-300 border-crimson-500/30",
  },
  {
    role: "style_reference",
    shortLabel: "Style Inspiration",
    title: "Style & Aesthetic Reference",
    description:
      "Visual inspiration for lighting, textures, surface craft, or color mood without copying exact content.",
    badgeColor: "bg-crimson-500/15 text-crimson-300 border-crimson-500/30",
  },
  {
    role: "content_reference",
    shortLabel: "Content Reference",
    title: "Content / Layout Structure Reference",
    description:
      "Reference for copywriting hierarchy, data tables, or wireframe structure.",
    badgeColor: "bg-emerald-500/15 text-emerald-800 border-emerald-500/30",
  },
  {
    role: "font_file",
    shortLabel: "Font (.woff2)",
    title: "Custom Web Font (.woff / .woff2)",
    description:
      "Bundled into the coding-agent handoff package for exact typography implementation.",
    badgeColor: "bg-stone-200 text-stone-700 border-stone-300",
  },
];

function getRoleConfig(role: AttachmentRole): RoleConfig {
  return (
    ROLE_CONFIGS.find((r) => r.role === role) || {
      role,
      shortLabel: role.replace(/_/g, " "),
      title: role.replace(/_/g, " "),
      description: "Project reference asset",
      badgeColor: "bg-stone-200 text-stone-700 border-stone-300",
    }
  );
}

export function AssetsLibraryView({
  projectId,
  assets,
  onRefresh,
  onAttachToComposer,
}: AssetsLibraryViewProps) {
  const [uploadRole, setUploadRole] = useState<AttachmentRole>("include_asset");
  const [uploadLabel, setUploadLabel] = useState("");
  const [uploadNotes, setUploadNotes] = useState("");
  const [preserveAsIs, setPreserveAsIs] = useState(true);
  const [uploading, setUploading] = useState(false);
  const [isDraggingOver, setIsDraggingOver] = useState(false);
  const [roleFilter, setRoleFilter] = useState<string>("all");
  const [previewAsset, setPreviewAsset] = useState<AssetDto | null>(null);

  // Per-asset inline comment editing state
  const [draftComments, setDraftComments] = useState<Record<string, string>>(
    {}
  );
  const [draftLabels, setDraftLabels] = useState<Record<string, string>>({});
  const [savingAssetId, setSavingAssetId] = useState<string | null>(null);
  const [savedAssetId, setSavedAssetId] = useState<string | null>(null);

  const [msg, setMsg] = useState<{
    type: "success" | "error";
    text: string;
  } | null>(null);

  const activeRoleCfg = getRoleConfig(uploadRole);

  async function uploadFilesList(files: FileList | File[]) {
    const arr = Array.from(files);
    if (arr.length === 0) return;
    setUploading(true);
    setMsg(null);
    try {
      for (let i = 0; i < arr.length; i++) {
        const file = arr[i];
        const fd = new FormData();
        fd.append("role", uploadRole);
        fd.append(
          "label",
          arr.length === 1 && uploadLabel.trim()
            ? uploadLabel.trim()
            : file.name
        );
        fd.append("placement_notes", uploadNotes.trim());
        fd.append("preserve_as_is", preserveAsIs ? "true" : "false");
        fd.append("file", file);
        await apiFetch(`/api/projects/${projectId}/assets`, {
          method: "POST",
          body: fd,
        });
      }
      setUploadLabel("");
      setUploadNotes("");
      onRefresh();
      setMsg({
        type: "success",
        text: `Uploaded ${arr.length} asset${
          arr.length > 1 ? "s" : ""
        } with your AI usage instructions!`,
      });
    } catch (err: unknown) {
      setMsg({
        type: "error",
        text: err instanceof Error ? err.message : "Asset upload failed",
      });
    } finally {
      setUploading(false);
    }
  }

  async function handleFileSelect(e: React.ChangeEvent<HTMLInputElement>) {
    if (e.target.files && e.target.files.length > 0) {
      await uploadFilesList(e.target.files);
      e.target.value = "";
    }
  }

  async function handleSaveAssetDetails(
    asset: AssetDto,
    overrides?: {
      role?: AttachmentRole;
      preserve_as_is?: boolean;
      placement_notes?: string;
      label?: string;
    }
  ) {
    setSavingAssetId(asset.id);
    try {
      const nextNotes =
        overrides?.placement_notes !== undefined
          ? overrides.placement_notes
          : draftComments[asset.id] !== undefined
          ? draftComments[asset.id]
          : asset.placement_notes;
      const nextLabel =
        overrides?.label !== undefined
          ? overrides.label
          : draftLabels[asset.id] !== undefined
          ? draftLabels[asset.id]
          : asset.label;

      await apiFetch(`/api/assets/${asset.id}`, {
        method: "PUT",
        body: JSON.stringify({
          role: overrides?.role ?? asset.role,
          preserve_as_is: overrides?.preserve_as_is ?? asset.preserve_as_is,
          placement_notes: nextNotes,
          label: nextLabel,
        }),
      });
      onRefresh();
      setSavedAssetId(asset.id);
      setTimeout(() => {
        setSavedAssetId((prev) => (prev === asset.id ? null : prev));
      }, 2000);
    } catch (err: unknown) {
      setMsg({
        type: "error",
        text:
          err instanceof Error ? err.message : "Failed to update asset details",
      });
    } finally {
      setSavingAssetId(null);
    }
  }

  async function handleDelete(assetId: string) {
    await apiFetch(`/api/assets/${assetId}`, { method: "DELETE" });
    onRefresh();
  }

  const filteredAssets = assets.filter((a) =>
    roleFilter === "all" ? true : a.role === roleFilter
  );

  return (
    <div className="max-w-6xl mx-auto space-y-6 pb-14">
      {/* Header */}
      <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-stone-border pb-4">
        <div>
          <div className="flex items-center gap-2.5">
            <h1 className="text-2xl font-bold text-stone-ink">
              Project Assets & Reference Studio
            </h1>
            <span className="rounded-full bg-crimson-600/15 border border-crimson-600/30 px-2.5 py-0.5 text-xs font-bold text-crimson-300">
              {assets.length} {assets.length === 1 ? "Asset" : "Assets"}
            </span>
          </div>
          <p className="text-xs text-stone-muted mt-1">
            Upload brand logos, product mockups, UI screenshots to polish, or
            style inspirations. Add custom comments/instructions to any asset so
            the AI knows exactly how to use or place it.
          </p>
        </div>
      </div>

      {/* ================================================================= */}
      {/* UPLOAD STUDIO CARD WITH ROLE PILLS + AI COMMENT BOX               */}
      {/* ================================================================= */}
      <div className="rounded-2xl border border-stone-border bg-stone-surface p-5 shadow-card space-y-5">
        <div className="flex items-center justify-between gap-2">
          <div className="flex items-center gap-2">
            <span className="flex h-7 w-7 items-center justify-center rounded-lg bg-crimson-600/15 text-crimson-300">
              <Upload className="h-4 w-4" />
            </span>
            <div>
              <h2 className="text-sm font-bold text-stone-ink">
                Upload New Asset with AI Instructions
              </h2>
              <p className="text-[11px] text-stone-muted">
                Choose how the AI should treat this file, leave an optional
                comment or placement instruction, and drop your image(s).
              </p>
            </div>
          </div>
        </div>

        {/* Step 1: Custom Role Pills (No ugly native OS dropdown!) */}
        <div className="space-y-2">
          <label className="block text-xs font-bold text-stone-ink">
            1. Select Asset Role
          </label>
          <div className="flex flex-wrap gap-2">
            {ROLE_CONFIGS.map((rc) => {
              const isSelected = uploadRole === rc.role;
              return (
                <button
                  key={rc.role}
                  type="button"
                  onClick={() => {
                    setUploadRole(rc.role);
                    if (
                      rc.role === "brand_logo" ||
                      rc.role === "include_asset"
                    ) {
                      setPreserveAsIs(true);
                    }
                  }}
                  className={`rounded-xl border px-3.5 py-2 text-xs font-semibold transition-all text-left ${
                    isSelected
                      ? "border-crimson-600 bg-charcoal text-white shadow-sm"
                      : "border-stone-border bg-white text-stone-ink hover:border-crimson-600/60"
                  }`}
                >
                  {rc.shortLabel}
                </button>
              );
            })}
          </div>
          <p className="text-[11px] text-stone-muted bg-stone-canvas/70 rounded-lg px-3 py-1.5 border border-stone-border/60">
            <strong className="text-stone-ink">{activeRoleCfg.title}:</strong>{" "}
            {activeRoleCfg.description}
          </p>
        </div>

        {/* Step 2: Asset Comment / AI Instructions + Optional Custom Name */}
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
          <div className="md:col-span-2">
            <label className="block text-xs font-bold text-stone-ink mb-1">
              2. Comment / Instructions for AI (Optional but Recommended)
            </label>
            <textarea
              rows={2}
              data-testid="asset-upload-comment-input"
              placeholder="e.g., 'Place this glossy 3D phone mockup on the right side of the Hero section', or 'Use this current Navbar screenshot and add a glassy Sign Up button on the right'..."
              value={uploadNotes}
              onChange={(e) => setUploadNotes(e.target.value)}
              className="w-full rounded-xl border border-stone-border bg-white p-2.5 text-xs text-stone-ink placeholder:text-stone-muted focus:border-crimson-600 focus:outline-none"
            />
          </div>

          <div className="flex flex-col justify-between space-y-2">
            <div>
              <label className="block text-xs font-bold text-stone-ink mb-1">
                Asset Title (Optional)
              </label>
              <input
                type="text"
                placeholder="Defaults to filename..."
                value={uploadLabel}
                onChange={(e) => setUploadLabel(e.target.value)}
                className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs text-stone-ink placeholder:text-stone-muted focus:border-crimson-600 focus:outline-none"
              />
            </div>

            <label className="inline-flex items-center gap-2 text-xs font-medium text-stone-ink cursor-pointer pt-1">
              <input
                type="checkbox"
                checked={preserveAsIs}
                onChange={(e) => setPreserveAsIs(e.target.checked)}
                className="h-3.5 w-3.5 rounded border-stone-border text-crimson-300 focus:ring-crimson-600"
              />
              <span>Include original file in Coding-Agent Handoff</span>
            </label>
          </div>
        </div>

        {/* Step 3: Drag & Drop Upload Box */}
        <div
          onDragOver={(e) => {
            e.preventDefault();
            setIsDraggingOver(true);
          }}
          onDragLeave={() => setIsDraggingOver(false)}
          onDrop={async (e) => {
            e.preventDefault();
            setIsDraggingOver(false);
            if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
              await uploadFilesList(e.dataTransfer.files);
            }
          }}
          className={`flex flex-col sm:flex-row items-center justify-between gap-4 rounded-xl border-2 border-dashed p-4 transition-colors ${
            isDraggingOver
              ? "border-crimson-600 bg-crimson-600/10"
              : "border-stone-border bg-white hover:border-crimson-600/50"
          }`}
        >
          <div className="flex items-center gap-3 text-xs">
            <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-stone-canvas text-stone-ink border border-stone-border">
              <FileImage className="h-5 w-5 text-crimson-300" />
            </div>
            <div>
              <div className="font-bold text-stone-ink">
                Drag & drop images, SVGs, or fonts here
              </div>
              <div className="text-[11px] text-stone-muted">
                Supports PNG, JPEG, WebP, sanitized SVG vector logos, and
                WOFF/WOFF2 fonts
              </div>
            </div>
          </div>

          <label className="inline-flex shrink-0 items-center justify-center gap-2 rounded-xl bg-crimson-600 px-5 py-2.5 text-xs font-bold text-white shadow-sm hover:bg-crimson-700 cursor-pointer transition-colors">
            <Upload className="h-4 w-4" />
            <span>
              {uploading ? "Uploading & Validating..." : "Choose File(s)"}
            </span>
            <input
              type="file"
              multiple
              accept=".png,.jpg,.jpeg,.webp,.svg,.woff,.woff2"
              onChange={handleFileSelect}
              className="hidden"
            />
          </label>
        </div>

        {msg && (
          <div
            role="status"
            className={`rounded-xl border px-3.5 py-2.5 text-xs font-medium flex items-center justify-between gap-2 ${
              msg.type === "success"
                ? "border-emerald-200 bg-emerald-50 text-emerald-900"
                : "border-red-200 bg-red-50 text-red-900"
            }`}
          >
            <div className="flex items-center gap-2">
              {msg.type === "success" ? (
                <CheckCircle2 className="h-4 w-4 text-emerald-600 shrink-0" />
              ) : (
                <AlertCircle className="h-4 w-4 text-red-600 shrink-0" />
              )}
              <span>{msg.text}</span>
            </div>
            <button
              type="button"
              onClick={() => setMsg(null)}
              className="opacity-60 hover:opacity-100"
            >
              <X className="h-3.5 w-3.5" />
            </button>
          </div>
        )}
      </div>

      {/* ================================================================= */}
      {/* FILTER BAR                                                        */}
      {/* ================================================================= */}
      {assets.length > 0 && (
        <div className="flex flex-wrap items-center justify-between gap-2">
          <div className="flex flex-wrap items-center gap-1.5">
            <button
              type="button"
              onClick={() => setRoleFilter("all")}
              className={`rounded-lg px-3 py-1.5 text-xs font-semibold transition-colors ${
                roleFilter === "all"
                  ? "bg-charcoal text-white"
                  : "bg-stone-surface text-stone-muted border border-stone-border hover:text-stone-ink"
              }`}
            >
              All ({assets.length})
            </button>
            {ROLE_CONFIGS.map((rc) => {
              const count = assets.filter((a) => a.role === rc.role).length;
              if (count === 0) return null;
              return (
                <button
                  key={rc.role}
                  type="button"
                  onClick={() => setRoleFilter(rc.role)}
                  className={`rounded-lg px-3 py-1.5 text-xs font-semibold transition-colors ${
                    roleFilter === rc.role
                      ? "bg-charcoal text-white"
                      : "bg-stone-surface text-stone-muted border border-stone-border hover:text-stone-ink"
                  }`}
                >
                  {rc.shortLabel} ({count})
                </button>
              );
            })}
          </div>
        </div>
      )}

      {/* ================================================================= */}
      {/* ASSETS GRID WITH INLINE COMMENT / INSTRUCTION EDITOR              */}
      {/* ================================================================= */}
      {filteredAssets.length === 0 ? (
        <div className="rounded-2xl border border-dashed border-stone-border bg-stone-surface p-12 text-center text-xs text-stone-muted">
          <FolderOpen className="h-8 w-8 mx-auto mb-2.5 text-stone-400" />
          <div className="text-sm font-bold text-stone-ink mb-1">
            No project assets in this view yet
          </div>
          <p>
            Upload logos, product screenshots, or design references above, or
            paste images directly into the chat composer with{" "}
            <kbd className="rounded bg-stone-canvas px-1.5 py-0.5 font-mono border border-stone-border">
              ⌘V
            </kbd>
            .
          </p>
        </div>
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
          {filteredAssets.map((asset) => {
            const roleCfg = getRoleConfig(asset.role);
            const commentValue =
              draftComments[asset.id] !== undefined
                ? draftComments[asset.id]
                : asset.placement_notes || "";
            const labelValue =
              draftLabels[asset.id] !== undefined
                ? draftLabels[asset.id]
                : asset.label;
            const hasUnsavedChanges =
              (draftComments[asset.id] !== undefined &&
                draftComments[asset.id] !== (asset.placement_notes || "")) ||
              (draftLabels[asset.id] !== undefined &&
                draftLabels[asset.id] !== asset.label);

            return (
              <div
                key={asset.id}
                data-testid={`asset-card-${asset.id}`}
                className="rounded-2xl border border-stone-border bg-stone-surface overflow-hidden flex flex-col justify-between shadow-card hover:border-stone-400 transition-all"
              >
                <div>
                  {/* Image Preview Header */}
                  <div
                    onClick={() => setPreviewAsset(asset)}
                    className="group relative h-44 bg-[#0F0F12] border-b border-stone-border flex items-center justify-center p-3 checkerboard-bg cursor-pointer overflow-hidden"
                  >
                    <img
                      src={asset.thumbnail_url}
                      alt={asset.label}
                      className="max-h-full max-w-full object-contain rounded transition-transform duration-200 group-hover:scale-[1.03]"
                    />

                    {/* Role Badge Top-Left */}
                    <span
                      className={`absolute top-2.5 left-2.5 rounded-md border px-2 py-0.5 text-[10px] font-bold shadow-sm backdrop-blur-sm bg-white/90 ${roleCfg.badgeColor}`}
                    >
                      {roleCfg.shortLabel}
                    </span>

                    {/* Dimensions Top-Right */}
                    <span className="absolute top-2.5 right-2.5 rounded-md bg-charcoal/80 px-2 py-0.5 font-mono text-[10px] font-medium text-white backdrop-blur-sm">
                      {asset.width > 0 ? `${asset.width}×${asset.height} · ` : ""}
                      {Math.max(1, Math.round(asset.byte_size / 1024))} KB
                    </span>

                    {/* Hover Fullscreen Inspect Pill */}
                    <div className="opacity-0 group-hover:opacity-100 transition-opacity absolute inset-0 bg-charcoal/25 flex items-center justify-center">
                      <span className="inline-flex items-center gap-1.5 rounded-lg bg-charcoal/90 px-3 py-1.5 text-xs font-semibold text-white shadow">
                        <Eye className="h-3.5 w-3.5 text-crimson-300" />
                        Inspect Asset
                      </span>
                    </div>
                  </div>

                  {/* Card Body: Editable Label, Role Pill Switcher, and Comment Box */}
                  <div className="p-4 space-y-3">
                    {/* Asset Title Input */}
                    <div>
                      <input
                        type="text"
                        value={labelValue}
                        onChange={(e) =>
                          setDraftLabels((prev) => ({
                            ...prev,
                            [asset.id]: e.target.value,
                          }))
                        }
                        onBlur={() => {
                          if (
                            draftLabels[asset.id] !== undefined &&
                            draftLabels[asset.id] !== asset.label
                          ) {
                            handleSaveAssetDetails(asset, {
                              label: draftLabels[asset.id],
                            });
                          }
                        }}
                        className="w-full rounded-lg border border-transparent hover:border-stone-border focus:border-crimson-600 bg-transparent focus:bg-[#151518] px-1.5 py-0.5 text-xs font-bold text-white focus:outline-none transition-colors"
                        title="Click to rename asset"
                      />
                    </div>

                    {/* Custom Role Pills on Card */}
                    <div>
                      <div className="text-[10px] font-bold uppercase tracking-wider text-stone-muted mb-1">
                        Role in AI Pipeline
                      </div>
                      <div className="flex flex-wrap gap-1">
                        {ROLE_CONFIGS.slice(0, 5).map((rc) => {
                          const isCurrent = asset.role === rc.role;
                          return (
                            <button
                              key={rc.role}
                              type="button"
                              onClick={() =>
                                handleSaveAssetDetails(asset, { role: rc.role })
                              }
                              className={`rounded-md px-2 py-1 text-[10px] font-semibold transition-colors ${
                                isCurrent
                                  ? "bg-charcoal text-white shadow-sm"
                                  : "bg-white border border-stone-border text-stone-muted hover:text-stone-ink"
                              }`}
                            >
                              {rc.shortLabel}
                            </button>
                          );
                        })}
                      </div>
                    </div>

                    {/* Inline Editable Comment / AI Usage Instructions */}
                    <div>
                      <div className="flex items-center justify-between mb-1">
                        <label className="text-[10px] font-bold uppercase tracking-wider text-stone-muted">
                          Comment / AI Usage Instructions
                        </label>
                        {savedAssetId === asset.id && (
                          <span className="inline-flex items-center gap-1 text-[10px] font-semibold text-emerald-700">
                            <Check className="h-3 w-3" />
                            Saved
                          </span>
                        )}
                      </div>
                      <textarea
                        rows={2}
                        data-testid={`asset-comment-input-${asset.id}`}
                        value={commentValue}
                        onChange={(e) =>
                          setDraftComments((prev) => ({
                            ...prev,
                            [asset.id]: e.target.value,
                          }))
                        }
                        placeholder="Add a comment so the AI knows what this asset is or where to place it..."
                        className="w-full rounded-xl border border-stone-border bg-white p-2 text-xs text-stone-ink placeholder:text-stone-muted focus:border-crimson-600 focus:outline-none"
                      />
                      {hasUnsavedChanges && (
                        <div className="mt-1.5 flex justify-end">
                          <button
                            type="button"
                            disabled={savingAssetId === asset.id}
                            onClick={() => handleSaveAssetDetails(asset)}
                            className="inline-flex items-center gap-1 rounded-lg bg-crimson-600 px-2.5 py-1 text-[11px] font-bold text-white shadow-sm hover:bg-crimson-700 disabled:opacity-50"
                          >
                            <Save className="h-3 w-3" />
                            <span>
                              {savingAssetId === asset.id
                                ? "Saving..."
                                : "Save Comment"}
                            </span>
                          </button>
                        </div>
                      )}
                    </div>

                    {/* Preserve As-Is Checkbox */}
                    <label className="inline-flex items-center gap-1.5 text-[11px] text-stone-muted cursor-pointer">
                      <input
                        type="checkbox"
                        checked={asset.preserve_as_is}
                        onChange={(e) =>
                          handleSaveAssetDetails(asset, {
                            preserve_as_is: e.target.checked,
                          })
                        }
                        className="rounded border-stone-border text-crimson-300 focus:ring-crimson-600"
                      />
                      <span>Preserve exact file in coding-agent handoff</span>
                    </label>
                  </div>
                </div>

                {/* Card Footer */}
                <div className="px-4 py-2.5 bg-white border-t border-stone-border flex items-center justify-between gap-2">
                  <button
                    type="button"
                    onClick={() => handleDelete(asset.id)}
                    aria-label={`Delete asset ${asset.label}`}
                    className="rounded-lg p-1.5 text-stone-muted hover:bg-red-50 hover:text-red-700 transition-colors"
                    title="Delete asset"
                  >
                    <Trash2 className="h-4 w-4" />
                  </button>

                  <button
                    type="button"
                    onClick={() => onAttachToComposer(asset)}
                    className="inline-flex items-center gap-1.5 rounded-xl bg-charcoal px-3.5 py-1.5 text-xs font-bold text-white hover:bg-charcoal-hover transition-colors shadow-sm"
                  >
                    <MessageSquarePlus className="h-3.5 w-3.5 text-crimson-300" />
                    <span>Attach to Chat</span>
                  </button>
                </div>
              </div>
            );
          })}
        </div>
      )}

      {/* ================================================================= */}
      {/* FULLSCREEN ASSET PREVIEW MODAL                                    */}
      {/* ================================================================= */}
      {previewAsset && (
        <div
          onClick={() => setPreviewAsset(null)}
          className="fixed inset-0 z-50 flex items-center justify-center bg-black/85 backdrop-blur-sm p-6"
        >
          <div
            onClick={(e) => e.stopPropagation()}
            className="max-w-4xl w-full rounded-2xl border border-white/15 bg-[#1B1A18] p-5 text-white shadow-2xl space-y-4"
          >
            <div className="flex items-center justify-between">
              <div>
                <h3 className="text-sm font-bold">{previewAsset.label}</h3>
                <p className="text-xs text-stone-muted">
                  {getRoleConfig(previewAsset.role).title} ·{" "}
                  {previewAsset.width}×{previewAsset.height}px
                </p>
              </div>
              <button
                type="button"
                onClick={() => setPreviewAsset(null)}
                className="rounded-lg bg-white/10 p-1.5 text-white/80 hover:bg-white/20"
              >
                <X className="h-4 w-4" />
              </button>
            </div>
            <div className="max-h-[70vh] overflow-auto rounded-xl border border-white/10 checkerboard-bg flex items-center justify-center p-4">
              <img
                src={previewAsset.original_url || previewAsset.thumbnail_url}
                alt={previewAsset.label}
                className="max-h-[65vh] max-w-full object-contain rounded"
              />
            </div>
            {previewAsset.placement_notes && (
              <div className="rounded-xl bg-white/5 border border-white/10 p-3 text-xs text-white/85">
                <strong className="text-crimson-300">AI Comment / Notes:</strong>{" "}
                {previewAsset.placement_notes}
              </div>
            )}
          </div>
        </div>
      )}
    </div>
  );
}

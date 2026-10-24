"use client";

import React, { useState } from "react";
import {
  AlertCircle,
  Check,
  CheckCircle2,
  History,
  Loader2,
  Moon,
  Palette,
  Save,
  Sparkles,
  Sun,
  Type,
  Upload,
  Wand2,
  X,
} from "lucide-react";
import {
  AiBrandProposalDto,
  apiFetch,
  AssetDto,
  BrandSpec,
} from "@/lib/api";
import { DesignStylePickerModal } from "@/components/DesignStylePickerModal";
import {
  DESIGN_STYLE_MODIFIERS,
  DESIGN_STYLES,
  DesignStylePreset,
  findDesignStyleById,
} from "@/lib/designStyles";

interface BrandKitViewProps {
  projectId: string;
  projectMeta: {
    name: string;
    description: string;
    purpose: string;
    industry: string;
    target_audience: string;
    audience_needs: string;
    desired_user_action: string;
  };
  brandVersion: number;
  initialBrand: BrandSpec;
  brandHistory: Array<{
    version: number;
    brand: BrandSpec;
    created_at: string;
  }>;
  assets: AssetDto[];
  onSaved: () => void;
}

export function BrandKitView({
  projectId,
  projectMeta,
  brandVersion,
  initialBrand,
  brandHistory,
  assets,
  onSaved,
}: BrandKitViewProps) {
  const [meta, setMeta] = useState(projectMeta);
  const [brand, setBrand] = useState<BrandSpec>(initialBrand);
  const [saving, setSaving] = useState(false);
  const [suggesting, setSuggesting] = useState(false);
  const [aiCustomPrompt, setAiCustomPrompt] = useState("");
  const [aiLogoDataUrl, setAiLogoDataUrl] = useState<string | null>(null);
  const [aiLogoName, setAiLogoName] = useState<string | null>(null);
  const [proposal, setProposal] = useState<AiBrandProposalDto | null>(null);

  const [statusMsg, setStatusMsg] = useState<{
    type: "success" | "error";
    text: string;
  } | null>(null);

  const [rulesText, setRulesText] = useState(
    initialBrand.brand_rules.join("\n")
  );
  const [mandatoryText, setMandatoryText] = useState(
    initialBrand.mandatory_inclusions.join("\n")
  );
  const [forbiddenText, setForbiddenText] = useState(
    initialBrand.forbidden_treatments.join("\n")
  );
  const [styleModalOpen, setStyleModalOpen] = useState(false);

  const activeStylePreset = findDesignStyleById(brand.design_style?.style_id);
  const activeModifierIds = DESIGN_STYLE_MODIFIERS.filter((m) =>
    brand.design_style?.modifiers?.includes(m.term)
  ).map((m) => m.id);

  const logoAssets = assets.filter((a) => a.role === "brand_logo");
  const activeLogoAsset =
    logoAssets.find((a) => a.id === brand.logo.primary_asset_id) ||
    logoAssets[0] ||
    null;

  function handleSelectStyleInBrandKit(
    style: DesignStylePreset,
    modifierIds: string[],
    combinedBrief: string
  ) {
    const modTerms = DESIGN_STYLE_MODIFIERS.filter((m) =>
      modifierIds.includes(m.id)
    ).map((m) => m.term);
    const nextRules = [
      `Strictly follow Web Design Style #${style.number} (${style.name}): ${style.definition}`,
      style.visualRules,
      ...(modTerms.length > 0
        ? [`Combine with layout/system modifiers: ${modTerms.join(" + ")}`]
        : []),
    ];
    setRulesText(nextRules.join("\n"));
    setBrand((prev) => ({
      ...prev,
      brand_personality: style.name,
      design_direction: `${style.definition} ${style.visualRules}`,
      colors: {
        ...prev.colors,
        light: {
          ...prev.colors.light,
          primary: style.recommendedBrand.accentColor,
          accent: style.recommendedBrand.primaryColor,
          background: style.recommendedBrand.secondaryColor,
        },
      },
      typography: {
        ...prev.typography,
        heading_family: style.recommendedBrand.fontHeadline,
        body_family: style.recommendedBrand.fontBody,
      },
      brand_rules: nextRules,
      design_style: {
        style_id: style.id,
        style_number: style.number,
        style_name: style.name,
        definition: style.definition,
        category: style.category,
        visual_rules: style.visualRules,
        keywords: style.keywords,
        modifiers: modTerms,
        combined_brief: combinedBrief,
      },
    }));
    setStatusMsg({
      type: "success",
      text: `Selected Style #${style.number} (${style.name}). Click 'Save Brand v${
        brandVersion + 1
      }' to persist.`,
    });
  }

  async function handleUploadLogoOrFont(
    e: React.ChangeEvent<HTMLInputElement>,
    role: "brand_logo" | "font_file",
    slot?: "primary" | "mark_only",
    alsoSetAiPreview?: boolean
  ) {
    const file = e.target.files?.[0];
    if (!file) return;
    setStatusMsg(null);

    if (alsoSetAiPreview && file.type.startsWith("image/")) {
      const reader = new FileReader();
      reader.onload = () => {
        if (typeof reader.result === "string") {
          setAiLogoDataUrl(reader.result);
          setAiLogoName(file.name);
        }
      };
      reader.readAsDataURL(file);
    }

    try {
      const fd = new FormData();
      fd.append("role", role);
      fd.append("label", file.name);
      fd.append("preserve_as_is", "true");
      fd.append("file", file);

      const res = await apiFetch<{ asset: AssetDto }>(
        `/api/projects/${projectId}/assets`,
        {
          method: "POST",
          body: fd,
        }
      );

      if (role === "brand_logo") {
        setBrand((prev) => ({
          ...prev,
          logo: {
            ...prev.logo,
            mode: "uploaded",
            primary_asset_id:
              slot === "mark_only"
                ? prev.logo.primary_asset_id
                : res.asset.id,
            mark_only_asset_id:
              slot === "mark_only"
                ? res.asset.id
                : prev.logo.mark_only_asset_id,
          },
        }));
      } else {
        setBrand((prev) => ({
          ...prev,
          typography: {
            ...prev.typography,
            font_source: "uploaded_woff2",
            uploaded_font_asset_ids: [
              ...prev.typography.uploaded_font_asset_ids,
              res.asset.id,
            ],
          },
        }));
      }
      onSaved();
      setStatusMsg({
        type: "success",
        text: `Uploaded '${res.asset.label}'. Click 'Save Brand v${
          brandVersion + 1
        }' to commit this logo to the brand snapshot.`,
      });
    } catch (err: unknown) {
      setStatusMsg({
        type: "error",
        text: err instanceof Error ? err.message : "Upload failed",
      });
    } finally {
      e.target.value = "";
    }
  }

  async function handleSuggestBrandWithAi() {
    setSuggesting(true);
    setStatusMsg(null);
    try {
      const res = await apiFetch<{
        proposal: AiBrandProposalDto;
      }>(`/api/projects/${projectId}/suggest-brand`, {
        method: "POST",
        body: JSON.stringify({
          description: meta.description,
          logo_data_url: aiLogoDataUrl || undefined,
          custom_instructions: aiCustomPrompt.trim() || undefined,
        }),
      });
      setProposal(res.proposal);
    } catch (err: unknown) {
      setStatusMsg({
        type: "error",
        text:
          err instanceof Error
            ? err.message
            : "Failed to generate AI brand design",
      });
    } finally {
      setSuggesting(false);
    }
  }

  function buildUpdatedStateFromProposal(p: AiBrandProposalDto): {
    nextMeta: typeof meta;
    nextBrand: BrandSpec;
    nextRulesText: string;
    nextForbiddenText: string;
  } {
    const nextMeta = {
      ...meta,
      purpose: p.purpose || meta.purpose,
      industry: p.industry || meta.industry,
      target_audience: p.target_audience || meta.target_audience,
      audience_needs: p.audience_needs || meta.audience_needs,
      desired_user_action: p.desired_user_action || meta.desired_user_action,
    };

    const matchedStyle =
      DESIGN_STYLES.find((s) => s.number === p.recommended_style_number) ||
      activeStylePreset ||
      DESIGN_STYLES[0];

    const modTerms =
      p.recommended_modifiers && p.recommended_modifiers.length > 0
        ? p.recommended_modifiers
        : brand.design_style?.modifiers || [];

    const nextRules =
      p.brand_rules && p.brand_rules.length > 0
        ? p.brand_rules
        : brand.brand_rules;
    const nextForbidden =
      p.forbidden_treatments && p.forbidden_treatments.length > 0
        ? p.forbidden_treatments
        : brand.forbidden_treatments;

    const nextBrand: BrandSpec = {
      ...brand,
      brand_personality: p.brand_personality || brand.brand_personality,
      design_direction: p.design_direction || brand.design_direction,
      tone_of_voice: p.tone_of_voice || brand.tone_of_voice,
      ui_density: p.ui_density || brand.ui_density,
      colors: {
        light: {
          ...brand.colors.light,
          primary: p.colors_light?.primary || brand.colors.light.primary,
          accent: p.colors_light?.accent || brand.colors.light.accent,
          background:
            p.colors_light?.background || brand.colors.light.background,
          surface: p.colors_light?.surface || brand.colors.light.surface,
          text: p.colors_light?.text || brand.colors.light.text,
          muted_text:
            p.colors_light?.muted_text || brand.colors.light.muted_text,
          border: p.colors_light?.border || brand.colors.light.border,
          success: p.colors_light?.success || brand.colors.light.success,
          warning: p.colors_light?.warning || brand.colors.light.warning,
          danger: p.colors_light?.danger || brand.colors.light.danger,
        },
        dark: {
          ...brand.colors.dark,
          primary: p.colors_dark?.primary || brand.colors.dark.primary,
          accent: p.colors_dark?.accent || brand.colors.dark.accent,
          background: p.colors_dark?.background || brand.colors.dark.background,
          surface: p.colors_dark?.surface || brand.colors.dark.surface,
          text: p.colors_dark?.text || brand.colors.dark.text,
          muted_text: p.colors_dark?.muted_text || brand.colors.dark.muted_text,
          border: p.colors_dark?.border || brand.colors.dark.border,
          success: p.colors_dark?.success || brand.colors.dark.success,
          warning: p.colors_dark?.warning || brand.colors.dark.warning,
          danger: p.colors_dark?.danger || brand.colors.dark.danger,
        },
      },
      typography: {
        ...brand.typography,
        heading_family:
          p.typography_heading || brand.typography.heading_family,
        body_family: p.typography_body || brand.typography.body_family,
        mono_family: p.typography_mono || brand.typography.mono_family,
      },
      layout_tokens: {
        ...brand.layout_tokens,
        spacing_scale:
          p.spacing_scale || brand.layout_tokens.spacing_scale,
        radii: p.radii || brand.layout_tokens.radii,
        button_treatment:
          p.button_treatment || brand.layout_tokens.button_treatment,
      },
      brand_rules: nextRules,
      forbidden_treatments: nextForbidden,
      design_style: matchedStyle
        ? {
            style_id: matchedStyle.id,
            style_number: matchedStyle.number,
            style_name: matchedStyle.name,
            definition: matchedStyle.definition,
            category: matchedStyle.category,
            visual_rules: matchedStyle.visualRules,
            keywords: matchedStyle.keywords,
            modifiers: modTerms,
            combined_brief: `${matchedStyle.name} (#${matchedStyle.number}): ${matchedStyle.definition} ${matchedStyle.visualRules}`,
          }
        : brand.design_style,
    };

    return {
      nextMeta,
      nextBrand,
      nextRulesText: nextRules.join("\n"),
      nextForbiddenText: nextForbidden.join("\n"),
    };
  }

  function applyProposalToEditor() {
    if (!proposal) return;
    const { nextMeta, nextBrand, nextRulesText, nextForbiddenText } =
      buildUpdatedStateFromProposal(proposal);
    setMeta(nextMeta);
    setBrand(nextBrand);
    setRulesText(nextRulesText);
    setForbiddenText(nextForbiddenText);
    setProposal(null);
    setStatusMsg({
      type: "success",
      text: "Applied AI Light & Dark Mode palettes, typography, and style to the editor! Click 'Save Brand Version' when ready.",
    });
  }

  async function applyAndSaveProposalImmediately() {
    if (!proposal) return;
    const { nextMeta, nextBrand, nextRulesText, nextForbiddenText } =
      buildUpdatedStateFromProposal(proposal);
    setMeta(nextMeta);
    setBrand(nextBrand);
    setRulesText(nextRulesText);
    setForbiddenText(nextForbiddenText);
    setProposal(null);

    setSaving(true);
    setStatusMsg(null);
    try {
      await apiFetch(`/api/projects/${projectId}`, {
        method: "PUT",
        body: JSON.stringify(nextMeta),
      });
      const res = await apiFetch<{ brand_version: number }>(
        `/api/projects/${projectId}/brand`,
        {
          method: "PUT",
          body: JSON.stringify(nextBrand),
        }
      );
      onSaved();
      setStatusMsg({
        type: "success",
        text: `Applied & saved AI Brand Kit v${res.brand_version} (Light & Dark palettes + Style #${nextBrand.design_style?.style_number})!`,
      });
    } catch (err: unknown) {
      setStatusMsg({
        type: "error",
        text: err instanceof Error ? err.message : "Failed to save brand kit",
      });
    } finally {
      setSaving(false);
    }
  }

  async function handleSaveAll(e: React.FormEvent) {
    e.preventDefault();
    setSaving(true);
    setStatusMsg(null);
    try {
      await apiFetch(`/api/projects/${projectId}`, {
        method: "PUT",
        body: JSON.stringify(meta),
      });

      const updatedBrand: BrandSpec = {
        ...brand,
        brand_rules: rulesText
          .split("\n")
          .map((s) => s.trim())
          .filter(Boolean),
        mandatory_inclusions: mandatoryText
          .split("\n")
          .map((s) => s.trim())
          .filter(Boolean),
        forbidden_treatments: forbiddenText
          .split("\n")
          .map((s) => s.trim())
          .filter(Boolean),
      };

      const res = await apiFetch<{ brand_version: number }>(
        `/api/projects/${projectId}/brand`,
        {
          method: "PUT",
          body: JSON.stringify(updatedBrand),
        }
      );

      onSaved();
      setStatusMsg({
        type: "success",
        text: `Saved immutable Brand Kit v${res.brand_version}. All new generations will use v${res.brand_version}!`,
      });
    } catch (err: unknown) {
      setStatusMsg({
        type: "error",
        text: err instanceof Error ? err.message : "Failed to save brand kit",
      });
    } finally {
      setSaving(false);
    }
  }

  const proposedStylePreset = proposal
    ? DESIGN_STYLES.find((s) => s.number === proposal.recommended_style_number)
    : null;

  return (
    <form
      onSubmit={handleSaveAll}
      className="max-w-6xl mx-auto space-y-6 pb-16"
    >
      {/* ================================================================= */}
      {/* STICKY STUDIO HEADER                                              */}
      {/* ================================================================= */}
      <div className="sticky top-0 z-20 flex flex-col sm:flex-row sm:items-center justify-between gap-4 border-b border-stone-border bg-stone-canvas/95 backdrop-blur-md py-3.5">
        <div>
          <div className="flex items-center gap-2.5">
            <h1 className="text-2xl font-bold text-stone-ink">
              Brand Kit & Design System Studio
            </h1>
            <span className="rounded-full bg-burnt/15 border border-burnt/30 px-2.5 py-0.5 text-xs font-bold text-burnt">
              Snapshot v{brandVersion}
            </span>
          </div>
          <p className="text-xs text-stone-muted mt-0.5">
            Define or let AI compile your Light & Dark color palettes, Web
            Design Style (#1–#20), typography, and brand logo.
          </p>
        </div>

        <div className="flex items-center gap-2">
          <button
            type="submit"
            disabled={saving}
            data-testid="save-brand-kit-btn"
            className="inline-flex items-center gap-2 rounded-xl bg-burnt px-5 py-2.5 text-xs font-bold text-white shadow-sm hover:bg-burnt-hover disabled:opacity-50 transition-colors"
          >
            <Save className="h-4 w-4" />
            <span>
              {saving ? "Saving..." : `Save Brand v${brandVersion + 1}`}
            </span>
          </button>
        </div>
      </div>

      {statusMsg && (
        <div
          role="status"
          className={`rounded-xl border px-4 py-3 text-xs font-medium flex items-center justify-between gap-2 ${
            statusMsg.type === "success"
              ? "border-emerald-200 bg-emerald-50 text-emerald-900"
              : "border-red-200 bg-red-50 text-red-900"
          }`}
        >
          <div className="flex items-center gap-2">
            {statusMsg.type === "success" ? (
              <CheckCircle2 className="h-4 w-4 text-emerald-600 shrink-0" />
            ) : (
              <AlertCircle className="h-4 w-4 text-red-600 shrink-0" />
            )}
            <span>{statusMsg.text}</span>
          </div>
          <button
            type="button"
            onClick={() => setStatusMsg(null)}
            className="opacity-60 hover:opacity-100"
          >
            <X className="h-3.5 w-3.5" />
          </button>
        </div>
      )}

      {/* ================================================================= */}
      {/* MAGIC AI BRAND ARCHITECT HERO CARD (LOGO + DESCRIPTION -> AI)     */}
      {/* ================================================================= */}
      <section
        data-testid="ai-brand-enhancer-card"
        className="rounded-2xl border-2 border-burnt/50 bg-gradient-to-br from-[#20163B] via-[#141827] to-[#0D101B] p-5 text-white shadow-card space-y-4"
      >
        <div className="flex flex-col md:flex-row md:items-center justify-between gap-3">
          <div className="flex items-start gap-3">
            <div className="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-burnt text-white shadow-sm">
              <Wand2 className="h-5 w-5" />
            </div>
            <div>
              <div className="flex items-center gap-2">
                <h2 className="text-sm font-bold text-white">
                  Enhance & Generate Branding with AI
                </h2>
                <span className="rounded-full bg-burnt/30 border border-burnt/50 px-2 py-0.5 text-[10px] font-bold text-amber-200">
                  Logo + Product Brief → Full Light & Dark Palettes
                </span>
              </div>
              <p className="text-xs text-white/70 mt-0.5">
                Let GLM 5.3 Flash inspect your Brand Logo and product
                description to automatically choose the best Web Design Style
                (#1–#50), full Light Mode & Dark Mode color palettes,
                typography, and design rules.
              </p>
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 pt-1">
          {/* Left: Logo Selector / Uploader for AI */}
          <div className="rounded-xl border border-white/15 bg-white/5 p-3.5 flex flex-col justify-between gap-3">
            <div className="flex items-center justify-between">
              <span className="text-xs font-bold text-white/90">
                1. Brand Logo for AI Inspection
              </span>
              <span className="text-[10px] text-white/50">Optional</span>
            </div>

            <div className="flex items-center gap-3">
              <div className="h-14 w-16 shrink-0 rounded-xl border border-white/15 checkerboard-bg flex items-center justify-center overflow-hidden p-1.5">
                {aiLogoDataUrl ? (
                  <img
                    src={aiLogoDataUrl}
                    alt="AI Logo Preview"
                    className="max-h-full max-w-full object-contain"
                  />
                ) : activeLogoAsset ? (
                  <img
                    src={activeLogoAsset.thumbnail_url}
                    alt={activeLogoAsset.label}
                    className="max-h-full max-w-full object-contain"
                  />
                ) : (
                  <span className="text-xs font-bold text-stone-700">
                    {brand.logo.monogram_text ||
                      meta.name.slice(0, 2).toUpperCase()}
                  </span>
                )}
              </div>

              <div className="min-w-0 flex-1 space-y-1.5">
                <div className="truncate text-xs font-semibold text-white">
                  {aiLogoName ||
                    activeLogoAsset?.label ||
                    "No logo uploaded yet"}
                </div>
                <label className="inline-flex items-center gap-1.5 rounded-lg border border-white/20 bg-white/10 px-2.5 py-1 text-[11px] font-semibold text-white hover:bg-white/20 cursor-pointer transition-colors">
                  <Upload className="h-3 w-3 text-burnt" />
                  <span>Upload / Change Logo</span>
                  <input
                    type="file"
                    accept=".png,.jpg,.jpeg,.webp,.svg"
                    onChange={(e) =>
                      handleUploadLogoOrFont(e, "brand_logo", "primary", true)
                    }
                    className="hidden"
                  />
                </label>
              </div>
            </div>
          </div>

          {/* Middle + Right: Product Description & Custom Instructions */}
          <div className="lg:col-span-2 rounded-xl border border-white/15 bg-white/5 p-3.5 flex flex-col justify-between gap-3">
            <div>
              <label className="block text-xs font-bold text-white/90 mb-1">
                2. What is the website/product about & any style preferences?
              </label>
              <textarea
                rows={2}
                data-testid="ai-brand-custom-prompt"
                value={aiCustomPrompt || meta.description}
                onChange={(e) => {
                  setAiCustomPrompt(e.target.value);
                  setMeta({ ...meta, description: e.target.value });
                }}
                placeholder="Describe what your product does, target customers, and any mood or color preferences (e.g. 'AI developer platform, sleek dark mode with warm stone light mode and high-contrast CTAs')..."
                className="w-full rounded-xl border border-white/15 bg-black/35 p-2.5 text-xs text-white placeholder:text-white/40 focus:border-burnt focus:outline-none"
              />
            </div>

            <div className="flex flex-wrap items-center justify-between gap-2">
              <span className="text-[11px] text-white/55">
                Generates Light + Dark Mode palettes, fonts, style #1–#50 &
                brief
              </span>
              <button
                type="button"
                disabled={suggesting}
                data-testid="enhance-brand-with-ai-btn"
                onClick={handleSuggestBrandWithAi}
                className="inline-flex items-center gap-2 rounded-xl bg-burnt px-4 py-2 text-xs font-bold text-white shadow-sm hover:bg-burnt-hover disabled:opacity-50 transition-all"
              >
                {suggesting ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin" />
                    <span>AI Compiling Brand & Palettes...</span>
                  </>
                ) : (
                  <>
                    <Sparkles className="h-4 w-4" />
                    <span>✨ Enhance Brand Kit with AI</span>
                  </>
                )}
              </button>
            </div>
          </div>
        </div>

        {/* =============================================================== */}
        {/* AI PROPOSAL REVIEW PANEL (WHEN GENERATED)                       */}
        {/* =============================================================== */}
        {proposal && (
          <div
            data-testid="ai-brand-proposal-review"
            className="mt-4 rounded-2xl border-2 border-burnt bg-stone-surface p-5 text-stone-ink shadow-modal space-y-4"
          >
            <div className="flex flex-wrap items-center justify-between gap-3 border-b border-stone-border pb-3">
              <div>
                <div className="flex items-center gap-2">
                  <Sparkles className="h-4 w-4 text-burnt" />
                  <h3 className="text-sm font-bold text-stone-ink">
                    AI Brand & Palette Proposal Ready for Review
                  </h3>
                </div>
                <p className="text-xs text-stone-muted mt-0.5">
                  {proposal.design_direction || proposal.caveat_notice}
                </p>
              </div>

              <div className="flex flex-wrap items-center gap-2">
                <button
                  type="button"
                  onClick={() => setProposal(null)}
                  className="rounded-xl border border-stone-border bg-white px-3 py-1.5 text-xs font-medium text-stone-muted hover:text-stone-ink"
                >
                  Dismiss
                </button>
                <button
                  type="button"
                  onClick={applyProposalToEditor}
                  className="rounded-xl border border-burnt bg-white px-3.5 py-1.5 text-xs font-bold text-burnt hover:bg-burnt/10"
                >
                  Apply to Editor
                </button>
                <button
                  type="button"
                  disabled={saving}
                  data-testid="apply-and-save-ai-brand-btn"
                  onClick={applyAndSaveProposalImmediately}
                  className="inline-flex items-center gap-1.5 rounded-xl bg-burnt px-4 py-1.5 text-xs font-bold text-white shadow-sm hover:bg-burnt-hover"
                >
                  <Check className="h-3.5 w-3.5" />
                  <span>
                    Apply & Save Brand v{brandVersion + 1} Immediately
                  </span>
                </button>
              </div>
            </div>

            {/* Proposed Style + Typography Summary */}
            <div className="grid grid-cols-1 md:grid-cols-3 gap-3 text-xs">
              <div className="rounded-xl border border-stone-border bg-white p-3">
                <span className="text-[10px] font-bold uppercase tracking-wider text-stone-muted block mb-1">
                  Recommended Design Style
                </span>
                <div className="font-bold text-stone-ink">
                  #{proposal.recommended_style_number || 1}{" "}
                  {proposedStylePreset?.name || proposal.brand_personality}
                </div>
                <div className="text-[11px] text-stone-muted mt-0.5">
                  {proposal.tone_of_voice} · Density: {proposal.ui_density}
                </div>
              </div>

              <div className="rounded-xl border border-stone-border bg-white p-3">
                <span className="text-[10px] font-bold uppercase tracking-wider text-stone-muted block mb-1">
                  Typography System
                </span>
                <div className="font-bold text-stone-ink">
                  {proposal.typography_heading} / {proposal.typography_body}
                </div>
                <div className="text-[11px] font-mono text-stone-muted mt-0.5">
                  Mono: {proposal.typography_mono}
                </div>
              </div>

              <div className="rounded-xl border border-stone-border bg-white p-3">
                <span className="text-[10px] font-bold uppercase tracking-wider text-stone-muted block mb-1">
                  Audience & Primary CTA
                </span>
                <div className="font-bold text-stone-ink truncate">
                  {proposal.target_audience || meta.target_audience}
                </div>
                <div className="text-[11px] text-burnt font-semibold mt-0.5 truncate">
                  CTA: {proposal.desired_user_action || meta.desired_user_action}
                </div>
              </div>
            </div>

            {/* Side-by-Side Proposed Light & Dark Palettes */}
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="rounded-xl border border-stone-border bg-white p-3 space-y-2">
                <div className="flex items-center justify-between text-xs font-bold text-stone-ink">
                  <span className="flex items-center gap-1.5">
                    <Sun className="h-3.5 w-3.5 text-amber-500" />
                    Proposed Light Mode Palette
                  </span>
                </div>
                <div className="flex flex-wrap gap-1.5">
                  {Object.entries(proposal.colors_light || {}).map(
                    ([k, hex]) => (
                      <div
                        key={`prop-l-${k}`}
                        className="flex items-center gap-1.5 rounded-lg border border-stone-border bg-stone-canvas px-2 py-1 text-[10px]"
                      >
                        <span
                          className="h-4 w-4 rounded border border-black/15 shrink-0"
                          style={{ backgroundColor: hex }}
                        />
                        <span className="font-medium capitalize">
                          {k.replace("_", " ")}
                        </span>
                        <span className="font-mono text-stone-muted">
                          {hex}
                        </span>
                      </div>
                    )
                  )}
                </div>
              </div>

              <div className="rounded-xl border border-stone-border bg-[#1B1A18] p-3 text-white space-y-2">
                <div className="flex items-center justify-between text-xs font-bold">
                  <span className="flex items-center gap-1.5">
                    <Moon className="h-3.5 w-3.5 text-indigo-400" />
                    Proposed Dark Mode Palette
                  </span>
                </div>
                <div className="flex flex-wrap gap-1.5">
                  {Object.entries(proposal.colors_dark || {}).map(
                    ([k, hex]) => (
                      <div
                        key={`prop-d-${k}`}
                        className="flex items-center gap-1.5 rounded-lg border border-white/15 bg-white/5 px-2 py-1 text-[10px]"
                      >
                        <span
                          className="h-4 w-4 rounded border border-white/20 shrink-0"
                          style={{ backgroundColor: hex }}
                        />
                        <span className="font-medium capitalize text-white/90">
                          {k.replace("_", " ")}
                        </span>
                        <span className="font-mono text-white/55">{hex}</span>
                      </div>
                    )
                  )}
                </div>
              </div>
            </div>
          </div>
        )}
      </section>

      {/* ================================================================= */}
      {/* 1. WEB DESIGN STYLE (#1–#50) & SYSTEM MODIFIERS                   */}
      {/* ================================================================= */}
      <section className="rounded-2xl border border-stone-border bg-stone-surface p-5 shadow-card space-y-4">
        <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3">
          <div>
            <div className="flex items-center gap-2">
              <h2 className="text-sm font-bold text-stone-ink">
                1. Active Web Design Style & Visual System
              </h2>
              <span className="rounded-full bg-burnt/20 border border-burnt/40 px-2.5 py-0.5 text-[10px] font-bold text-purple-200">
                20 Curated Design Systems + 13 Modifiers
              </span>
            </div>
            <p className="text-xs text-stone-muted mt-0.5">
              Injected into the AI planner, GPT-5 Image compiler, and
              coding-agent implementation handoff.
            </p>
          </div>
          <button
            type="button"
            data-testid="brandkit-choose-style-btn"
            onClick={() => setStyleModalOpen(true)}
            className="inline-flex items-center gap-1.5 rounded-xl bg-charcoal px-4 py-2 text-xs font-bold text-white hover:bg-charcoal-hover shadow-sm shrink-0 transition-colors"
          >
            <Palette className="h-3.5 w-3.5 text-burnt" />
            <span>
              {brand.design_style?.style_name
                ? "Browse / Change Design Style"
                : "Choose from 20 Design Systems"}
            </span>
          </button>
        </div>

        {brand.design_style?.style_name ? (
          <div className="rounded-xl border border-stone-border bg-white p-4 flex flex-col md:flex-row gap-4 items-start">
            {activeStylePreset && (
              <img
                src={activeStylePreset.heroImage}
                alt={brand.design_style.style_name}
                onClick={() => setStyleModalOpen(true)}
                className="w-full md:w-64 aspect-video rounded-xl object-cover object-top border border-stone-border shrink-0 cursor-pointer hover:opacity-95 transition-opacity"
              />
            )}
            <div className="space-y-2 flex-1 min-w-0">
              <div className="flex flex-wrap items-center gap-2">
                <span className="rounded-md bg-charcoal px-2.5 py-0.5 text-xs font-bold text-white">
                  Style #{brand.design_style.style_number}
                </span>
                <span className="text-base font-bold text-stone-ink">
                  {brand.design_style.style_name}
                </span>
                {brand.design_style.category && (
                  <span className="rounded-md bg-stone-canvas border border-stone-border px-2 py-0.5 text-[11px] font-medium text-stone-muted">
                    {brand.design_style.category}
                  </span>
                )}
                {brand.design_style.modifiers?.length > 0 && (
                  <span className="rounded-full bg-burnt/15 px-2.5 py-0.5 text-[11px] font-bold text-burnt">
                    + {brand.design_style.modifiers.join(" + ")}
                  </span>
                )}
              </div>
              <p className="text-xs text-stone-ink leading-relaxed">
                <strong>Definition:</strong> {brand.design_style.definition}
              </p>
              <p className="text-xs text-stone-muted leading-relaxed">
                <strong>Visual System Rules:</strong>{" "}
                {brand.design_style.visual_rules}
              </p>
            </div>
          </div>
        ) : (
          <div className="rounded-xl border border-dashed border-stone-border bg-white p-5 text-xs text-stone-muted flex items-center justify-between">
            <span>
              No specific design style preset selected yet. Browse the 50
              curated web design styles with visual hero previews or click{" "}
              <strong>Enhance Brand Kit with AI</strong> above.
            </span>
          </div>
        )}
      </section>

      <DesignStylePickerModal
        open={styleModalOpen}
        onClose={() => setStyleModalOpen(false)}
        selectedStyleId={brand.design_style?.style_id || null}
        selectedModifierIds={activeModifierIds}
        onSelectStyle={handleSelectStyleInBrandKit}
      />

      {/* ================================================================= */}
      {/* 2. SIDE-BY-SIDE LIGHT MODE & DARK MODE COLOR PALETTE STUDIO       */}
      {/* ================================================================= */}
      <section className="rounded-2xl border border-stone-border bg-stone-surface p-5 shadow-card space-y-5">
        <div>
          <h2 className="text-sm font-bold text-stone-ink">
            2. Semantic Color Palettes (Light Mode & Dark Mode)
          </h2>
          <p className="text-xs text-stone-muted mt-0.5">
            Click any color swatch to adjust, or use the AI Enhancer above to
            automatically harmonize both Light and Dark mode palettes from your
            logo.
          </p>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
          {/* LIGHT MODE PALETTE + LIVE PREVIEW */}
          <div className="rounded-2xl border border-stone-border bg-white p-4 space-y-4">
            <div className="flex items-center justify-between border-b border-stone-border pb-2.5">
              <div className="flex items-center gap-2">
                <Sun className="h-4 w-4 text-amber-500" />
                <span className="text-xs font-bold text-stone-ink">
                  Light Mode Palette
                </span>
              </div>
              <span className="text-[10px] font-mono text-stone-muted">
                11 Semantic Tokens
              </span>
            </div>

            {/* Live Light Mode Mini UI Card Preview */}
            <div
              style={{
                backgroundColor: brand.colors.light.background,
                color: brand.colors.light.text,
                borderColor: brand.colors.light.border,
              }}
              className="rounded-xl border p-3.5 space-y-2.5 transition-colors"
            >
              <div className="flex items-center justify-between">
                <span className="text-[10px] font-bold uppercase tracking-wider opacity-70">
                  Live Light Preview
                </span>
                <span
                  style={{
                    backgroundColor: brand.colors.light.accent,
                    color: "#FFFFFF",
                  }}
                  className="rounded-full px-2 py-0.5 text-[9px] font-bold"
                >
                  Accent Badge
                </span>
              </div>
              <div
                style={{
                  backgroundColor: brand.colors.light.surface,
                  borderColor: brand.colors.light.border,
                }}
                className="rounded-lg border p-3 flex items-center justify-between gap-2 shadow-sm"
              >
                <div className="min-w-0">
                  <div
                    style={{ color: brand.colors.light.text }}
                    className="text-xs font-bold truncate"
                  >
                    {meta.name || "Brand Component Surface"}
                  </div>
                  <div
                    style={{ color: brand.colors.light.muted_text }}
                    className="text-[10px] truncate"
                  >
                    {meta.desired_user_action ||
                      "High-contrast typography & actions"}
                  </div>
                </div>
                <span
                  style={{
                    backgroundColor: brand.colors.light.primary,
                    color: "#FFFFFF",
                  }}
                  className="rounded-lg px-3 py-1.5 text-[10px] font-bold shrink-0 shadow-sm"
                >
                  Primary CTA
                </span>
              </div>
            </div>

            {/* Swatch Inputs */}
            <div className="grid grid-cols-2 sm:grid-cols-3 gap-2.5">
              {(
                Object.keys(brand.colors.light) as Array<
                  keyof typeof brand.colors.light
                >
              ).map((roleKey) => (
                <div
                  key={`light-${roleKey}`}
                  className="rounded-xl border border-stone-border bg-stone-canvas/50 p-2"
                >
                  <label className="block text-[10px] font-bold text-stone-muted capitalize mb-1 truncate">
                    {roleKey.replace("_", " ")}
                  </label>
                  <div className="flex items-center gap-1.5">
                    <input
                      type="color"
                      value={brand.colors.light[roleKey]}
                      onChange={(e) =>
                        setBrand({
                          ...brand,
                          colors: {
                            ...brand.colors,
                            light: {
                              ...brand.colors.light,
                              [roleKey]: e.target.value,
                            },
                          },
                        })
                      }
                      className="h-7 w-7 rounded-md border border-stone-border cursor-pointer shrink-0 bg-transparent"
                    />
                    <input
                      type="text"
                      value={brand.colors.light[roleKey]}
                      onChange={(e) =>
                        setBrand({
                          ...brand,
                          colors: {
                            ...brand.colors,
                            light: {
                              ...brand.colors.light,
                              [roleKey]: e.target.value,
                            },
                          },
                        })
                      }
                      className="w-full min-w-0 rounded-md border border-stone-border bg-white px-2 py-1 text-[11px] font-mono text-stone-ink"
                    />
                  </div>
                </div>
              ))}
            </div>
          </div>

          {/* DARK MODE PALETTE + LIVE PREVIEW */}
          <div className="rounded-2xl border border-charcoal-border bg-[#1B1A18] p-4 text-white space-y-4">
            <div className="flex items-center justify-between border-b border-white/10 pb-2.5">
              <div className="flex items-center gap-2">
                <Moon className="h-4 w-4 text-indigo-400" />
                <span className="text-xs font-bold text-white">
                  Dark Mode Palette
                </span>
              </div>
              <span className="text-[10px] font-mono text-white/50">
                11 Semantic Tokens
              </span>
            </div>

            {/* Live Dark Mode Mini UI Card Preview */}
            <div
              style={{
                backgroundColor: brand.colors.dark.background,
                color: brand.colors.dark.text,
                borderColor: brand.colors.dark.border,
              }}
              className="rounded-xl border p-3.5 space-y-2.5 transition-colors"
            >
              <div className="flex items-center justify-between">
                <span className="text-[10px] font-bold uppercase tracking-wider opacity-70">
                  Live Dark Preview
                </span>
                <span
                  style={{
                    backgroundColor: brand.colors.dark.accent,
                    color: "#FFFFFF",
                  }}
                  className="rounded-full px-2 py-0.5 text-[9px] font-bold"
                >
                  Accent Badge
                </span>
              </div>
              <div
                style={{
                  backgroundColor: brand.colors.dark.surface,
                  borderColor: brand.colors.dark.border,
                }}
                className="rounded-lg border p-3 flex items-center justify-between gap-2 shadow-sm"
              >
                <div className="min-w-0">
                  <div
                    style={{ color: brand.colors.dark.text }}
                    className="text-xs font-bold truncate"
                  >
                    {meta.name || "Brand Component Surface"}
                  </div>
                  <div
                    style={{ color: brand.colors.dark.muted_text }}
                    className="text-[10px] truncate"
                  >
                    {meta.desired_user_action ||
                      "High-contrast dark mode surface"}
                  </div>
                </div>
                <span
                  style={{
                    backgroundColor: brand.colors.dark.primary,
                    color: "#FFFFFF",
                  }}
                  className="rounded-lg px-3 py-1.5 text-[10px] font-bold shrink-0 shadow-sm"
                >
                  Primary CTA
                </span>
              </div>
            </div>

            {/* Swatch Inputs */}
            <div className="grid grid-cols-2 sm:grid-cols-3 gap-2.5">
              {(
                Object.keys(brand.colors.dark) as Array<
                  keyof typeof brand.colors.dark
                >
              ).map((roleKey) => (
                <div
                  key={`dark-${roleKey}`}
                  className="rounded-xl border border-white/10 bg-white/5 p-2"
                >
                  <label className="block text-[10px] font-bold text-white/65 capitalize mb-1 truncate">
                    {roleKey.replace("_", " ")}
                  </label>
                  <div className="flex items-center gap-1.5">
                    <input
                      type="color"
                      value={brand.colors.dark[roleKey]}
                      onChange={(e) =>
                        setBrand({
                          ...brand,
                          colors: {
                            ...brand.colors,
                            dark: {
                              ...brand.colors.dark,
                              [roleKey]: e.target.value,
                            },
                          },
                        })
                      }
                      className="h-7 w-7 rounded-md border border-white/20 cursor-pointer shrink-0 bg-transparent"
                    />
                    <input
                      type="text"
                      value={brand.colors.dark[roleKey]}
                      onChange={(e) =>
                        setBrand({
                          ...brand,
                          colors: {
                            ...brand.colors,
                            dark: {
                              ...brand.colors.dark,
                              [roleKey]: e.target.value,
                            },
                          },
                        })
                      }
                      className="w-full min-w-0 rounded-md border border-white/15 bg-black/40 px-2 py-1 text-[11px] font-mono text-white"
                    />
                  </div>
                </div>
              ))}
            </div>
          </div>
        </div>
      </section>

      {/* ================================================================= */}
      {/* 3. BRAND LOGO, PERSONALITY & TYPOGRAPHY STUDIO                    */}
      {/* ================================================================= */}
      <section className="rounded-2xl border border-stone-border bg-stone-surface p-5 shadow-card space-y-5">
        <h2 className="text-sm font-bold text-stone-ink">
          3. Brand Logo, Typography & Layout Tokens
        </h2>

        {/* Logo Configuration with Custom Pill Selectors */}
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 rounded-xl border border-stone-border bg-white p-4">
          <div className="space-y-2">
            <label className="block text-xs font-bold text-stone-ink">
              Logo Mode
            </label>
            <div className="flex flex-wrap gap-1.5">
              {[
                { id: "uploaded", label: "Uploaded Logo" },
                { id: "temporary_monogram", label: "Monogram Badge" },
                { id: "none_yet", label: "No Logo" },
              ].map((opt) => (
                <button
                  key={opt.id}
                  type="button"
                  onClick={() =>
                    setBrand({
                      ...brand,
                      logo: { ...brand.logo, mode: opt.id },
                    })
                  }
                  className={`rounded-lg px-3 py-1.5 text-xs font-semibold transition-colors ${
                    brand.logo.mode === opt.id
                      ? "bg-charcoal text-white"
                      : "bg-stone-canvas text-stone-muted border border-stone-border hover:text-stone-ink"
                  }`}
                >
                  {opt.label}
                </button>
              ))}
            </div>
          </div>

          <div className="space-y-2">
            <label className="block text-xs font-bold text-stone-ink">
              Logo Inclusion in Generated UI
            </label>
            <div className="flex flex-wrap gap-1.5">
              {[
                { id: "required", label: "Required" },
                { id: "allowed", label: "Where Appropriate" },
                { id: "never", label: "Never" },
              ].map((opt) => (
                <button
                  key={opt.id}
                  type="button"
                  onClick={() =>
                    setBrand({
                      ...brand,
                      logo: { ...brand.logo, inclusion_policy: opt.id },
                    })
                  }
                  className={`rounded-lg px-3 py-1.5 text-xs font-semibold transition-colors ${
                    brand.logo.inclusion_policy === opt.id
                      ? "bg-charcoal text-white"
                      : "bg-stone-canvas text-stone-muted border border-stone-border hover:text-stone-ink"
                  }`}
                >
                  {opt.label}
                </button>
              ))}
            </div>
          </div>

          <div className="space-y-2">
            <label className="block text-xs font-bold text-stone-ink">
              Active Logo Asset
            </label>
            <div className="flex items-center gap-2 flex-wrap">
              <label className="inline-flex items-center gap-1.5 rounded-lg bg-burnt px-3 py-1.5 text-xs font-bold text-white hover:bg-burnt-hover cursor-pointer shadow-sm">
                <Upload className="h-3.5 w-3.5" />
                <span>Upload Logo</span>
                <input
                  type="file"
                  accept=".png,.jpg,.jpeg,.webp,.svg"
                  onChange={(e) =>
                    handleUploadLogoOrFont(e, "brand_logo", "primary", true)
                  }
                  className="hidden"
                />
              </label>

              {logoAssets.map((la) => (
                <button
                  key={la.id}
                  type="button"
                  onClick={() =>
                    setBrand({
                      ...brand,
                      logo: {
                        ...brand.logo,
                        mode: "uploaded",
                        primary_asset_id: la.id,
                      },
                    })
                  }
                  className={`inline-flex items-center gap-1.5 rounded-lg border px-2.5 py-1 text-xs ${
                    brand.logo.primary_asset_id === la.id
                      ? "border-burnt bg-burnt/10 text-burnt font-bold"
                      : "border-stone-border bg-stone-canvas text-stone-ink"
                  }`}
                >
                  <img
                    src={la.thumbnail_url}
                    alt={la.label}
                    className="h-4 w-4 object-contain"
                  />
                  <span className="truncate max-w-[110px]">{la.label}</span>
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Typography + UI Density */}
        <div className="grid grid-cols-1 sm:grid-cols-4 gap-4">
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Heading Font Family
            </label>
            <input
              type="text"
              value={brand.typography.heading_family}
              onChange={(e) =>
                setBrand({
                  ...brand,
                  typography: {
                    ...brand.typography,
                    heading_family: e.target.value,
                  },
                })
              }
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs font-semibold"
            />
          </div>
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Body Font Family
            </label>
            <input
              type="text"
              value={brand.typography.body_family}
              onChange={(e) =>
                setBrand({
                  ...brand,
                  typography: {
                    ...brand.typography,
                    body_family: e.target.value,
                  },
                })
              }
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs"
            />
          </div>
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Monospace Font Family
            </label>
            <input
              type="text"
              value={brand.typography.mono_family}
              onChange={(e) =>
                setBrand({
                  ...brand,
                  typography: {
                    ...brand.typography,
                    mono_family: e.target.value,
                  },
                })
              }
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs font-mono"
            />
          </div>
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Preferred UI Density
            </label>
            <div className="flex items-center gap-1 pt-0.5">
              {["compact", "comfortable", "spacious"].map((d) => (
                <button
                  key={d}
                  type="button"
                  onClick={() => setBrand({ ...brand, ui_density: d })}
                  className={`flex-1 rounded-lg py-2 text-[11px] font-semibold capitalize transition-colors ${
                    brand.ui_density === d
                      ? "bg-charcoal text-white"
                      : "bg-white border border-stone-border text-stone-muted hover:text-stone-ink"
                  }`}
                >
                  {d}
                </button>
              ))}
            </div>
          </div>
        </div>

        {/* Personality, Tone & Layout Tokens */}
        <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Brand Personality
            </label>
            <input
              type="text"
              value={brand.brand_personality}
              onChange={(e) =>
                setBrand({ ...brand, brand_personality: e.target.value })
              }
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs"
            />
          </div>
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Tone of Voice
            </label>
            <input
              type="text"
              value={brand.tone_of_voice}
              onChange={(e) =>
                setBrand({ ...brand, tone_of_voice: e.target.value })
              }
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs"
            />
          </div>
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Corner Radii & Button Treatment
            </label>
            <input
              type="text"
              value={brand.layout_tokens.radii}
              onChange={(e) =>
                setBrand({
                  ...brand,
                  layout_tokens: {
                    ...brand.layout_tokens,
                    radii: e.target.value,
                  },
                })
              }
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs"
            />
          </div>
        </div>
      </section>

      {/* ================================================================= */}
      {/* 4. PROJECT BRIEF, AUDIENCE & CODING-AGENT HANDOFF RULES           */}
      {/* ================================================================= */}
      <section className="rounded-2xl border border-stone-border bg-stone-surface p-5 shadow-card space-y-4">
        <h2 className="text-sm font-bold text-stone-ink">
          4. Project Brief, Target Audience & Handoff Rules
        </h2>
        <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Project Name
            </label>
            <input
              type="text"
              value={meta.name}
              onChange={(e) => setMeta({ ...meta, name: e.target.value })}
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs"
            />
          </div>
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Industry / Category
            </label>
            <input
              type="text"
              value={meta.industry}
              onChange={(e) => setMeta({ ...meta, industry: e.target.value })}
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs"
            />
          </div>
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Target Audience
            </label>
            <input
              type="text"
              value={meta.target_audience}
              onChange={(e) =>
                setMeta({ ...meta, target_audience: e.target.value })
              }
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs"
            />
          </div>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Project Purpose
            </label>
            <input
              type="text"
              value={meta.purpose}
              onChange={(e) => setMeta({ ...meta, purpose: e.target.value })}
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs"
            />
          </div>
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Audience Needs
            </label>
            <input
              type="text"
              value={meta.audience_needs}
              onChange={(e) =>
                setMeta({ ...meta, audience_needs: e.target.value })
              }
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs"
            />
          </div>
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Primary CTA Action
            </label>
            <input
              type="text"
              value={meta.desired_user_action}
              onChange={(e) =>
                setMeta({ ...meta, desired_user_action: e.target.value })
              }
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs"
            />
          </div>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 pt-2">
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Brand Rules (One per line)
            </label>
            <textarea
              rows={3}
              value={rulesText}
              onChange={(e) => setRulesText(e.target.value)}
              className="w-full rounded-xl border border-stone-border bg-white p-2.5 text-xs"
            />
          </div>
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Forbidden Treatments (One per line)
            </label>
            <textarea
              rows={3}
              value={forbiddenText}
              onChange={(e) => setForbiddenText(e.target.value)}
              className="w-full rounded-xl border border-stone-border bg-white p-2.5 text-xs"
            />
          </div>
        </div>

        <div className="grid grid-cols-1 sm:grid-cols-3 gap-4 pt-2 border-t border-stone-border">
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Target Application Stack
            </label>
            <input
              type="text"
              value={brand.target_context.target_stack}
              onChange={(e) =>
                setBrand({
                  ...brand,
                  target_context: {
                    ...brand.target_context,
                    target_stack: e.target.value,
                  },
                })
              }
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs"
            />
          </div>
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Target Repository Root (For Agent Handoff)
            </label>
            <input
              type="text"
              placeholder="/path/to/target-website-repo"
              value={brand.target_context.repository_root}
              onChange={(e) =>
                setBrand({
                  ...brand,
                  target_context: {
                    ...brand.target_context,
                    repository_root: e.target.value,
                  },
                })
              }
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs font-mono"
            />
          </div>
          <div>
            <label className="block text-xs font-bold text-stone-ink mb-1">
              Preferred Asset Directory
            </label>
            <input
              type="text"
              value={brand.target_context.preferred_asset_dir}
              onChange={(e) =>
                setBrand({
                  ...brand,
                  target_context: {
                    ...brand.target_context,
                    preferred_asset_dir: e.target.value,
                  },
                })
              }
              className="w-full rounded-xl border border-stone-border bg-white px-3 py-2 text-xs font-mono"
            />
          </div>
        </div>
      </section>

      {/* Version History */}
      {brandHistory.length > 0 && (
        <section className="rounded-2xl border border-stone-border bg-stone-surface p-5 space-y-3">
          <div className="flex items-center gap-2 text-xs font-bold text-stone-ink">
            <History className="h-4 w-4 text-burnt" />
            <span>Immutable Brand Version History</span>
          </div>
          <div className="flex flex-wrap gap-2">
            {brandHistory.map((bh) => (
              <button
                key={bh.version}
                type="button"
                onClick={() => {
                  setBrand(bh.brand);
                  setRulesText(bh.brand.brand_rules.join("\n"));
                  setMandatoryText(bh.brand.mandatory_inclusions.join("\n"));
                  setForbiddenText(bh.brand.forbidden_treatments.join("\n"));
                }}
                className={`rounded-xl border px-3 py-1.5 text-xs ${
                  bh.version === brandVersion
                    ? "border-burnt bg-burnt/10 text-burnt font-bold"
                    : "border-stone-border bg-white text-stone-ink hover:bg-stone-canvas"
                }`}
              >
                v{bh.version} · {bh.brand.brand_personality}
              </button>
            ))}
          </div>
        </section>
      )}
    </form>
  );
}

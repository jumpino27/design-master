"use client";

import React, { Suspense, useEffect, useRef, useState } from "react";
import { useRouter, useSearchParams } from "next/navigation";
import {
  FolderPlus,
  Plus,
  Settings,
  Sparkles,
  ArrowRight,
  ArrowLeft,
  CheckCircle2,
  FolderOpen,
  Wand2,
  Search,
  Palette,
  FileText,
  Code2,
  Upload,
  Loader2,
  Sun,
  Moon,
  Sliders,
  X,
  RefreshCw,
  Download,
  ShieldCheck,
  ExternalLink,
} from "lucide-react";
import {
  AiBrandProposalDto,
  apiFetch,
  BrandSpec,
  ProjectSummaryDto,
  SetupStatusDto,
  UpdateCheckReport,
  UpdateInstallReport,
} from "@/lib/api";
import { LightDismissDialog } from "@/components/LightDismissDialog";
import { DesignStylePickerModal } from "@/components/DesignStylePickerModal";
import { ProjectCard } from "@/components/ProjectCard";
import {
  buildCombinedStyleBrief,
  DESIGN_STYLE_MODIFIERS,
  DESIGN_STYLES,
  DesignStylePreset,
  findDesignStyleById,
} from "@/lib/designStyles";

const DEFAULT_BRAND: BrandSpec = {
  brand_personality: "Calm editorial",
  design_direction:
    "Warm architectural minimalism with crisp hierarchy, tactile off-white surfaces, and confident burnt-orange primary actions",
  tone_of_voice: "Clear, direct, confident, and human",
  content_language: "English",
  ui_density: "comfortable",
  logo: {
    mode: "temporary_monogram",
    monogram_text: "F",
    primary_asset_id: null,
    mark_only_asset_id: null,
    light_variant_asset_id: null,
    dark_variant_asset_id: null,
    preferred_usage:
      "Top-left header or card header mark with clear breathing room",
    inclusion_policy: "allowed",
  },
  colors: {
    light: {
      primary: "#D56A3A",
      accent: "#1E1D1B",
      background: "#F4F1EA",
      surface: "#FAF9F6",
      text: "#1E1D1B",
      muted_text: "#6E6A63",
      border: "#E2DED5",
      success: "#2B7A4B",
      warning: "#B7791F",
      danger: "#C53030",
    },
    dark: {
      primary: "#E07A4B",
      accent: "#F4F1EA",
      background: "#181715",
      surface: "#242321",
      text: "#F4F1EA",
      muted_text: "#A09B92",
      border: "#383632",
      success: "#38A169",
      warning: "#D69E2E",
      danger: "#E53E3E",
    },
  },
  typography: {
    heading_family: "Inter",
    body_family: "Inter",
    mono_family: "JetBrains Mono",
    fallback_stack: "-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif",
    weights: [400, 500, 600, 700],
    size_scale: "12px / 14px / 16px / 20px / 28px / 40px",
    line_heights: "Heading 1.15, Body 1.5, Compact UI 1.3",
    font_source: "google_fonts",
    uploaded_font_asset_ids: [],
  },
  layout_tokens: {
    spacing_scale: "4px base (4, 8, 12, 16, 24, 32, 48, 64px)",
    content_width: "1200px max container with 24px padding",
    grid_columns: "12-column responsive grid, 24px gap",
    radii: "8px controls, 12px cards, 9999px pills",
    borders: "1px solid subtle warm border (#E2DED5)",
    shadows: "0 2px 8px rgba(24, 23, 21, 0.05)",
    icon_style: "1.75px stroke rounded line icons (Lucide style)",
    button_treatment:
      "Solid high-contrast primary CTA with subtle arrow affordance",
  },
  brand_rules: [
    "Maintain WCAG AA contrast across all text and CTA controls",
    "Use generous whitespace and clear typographic hierarchy",
  ],
  mandatory_inclusions: [],
  forbidden_treatments: ["Never use neon gradients or sci-fi glowing borders"],
  approved_reference_asset_ids: [],
  target_context: {
    target_stack: "Next.js App Router + TypeScript + Tailwind CSS",
    repository_root: "",
    preferred_asset_dir: "public/assets",
    component_library_notes: "Accessible Radix primitives with Lucide icons",
  },
};

function ProjectsContent() {
  const router = useRouter();
  const searchParams = useSearchParams();
  const [projects, setProjects] = useState<ProjectSummaryDto[]>([]);
  const [status, setStatus] = useState<SetupStatusDto | null>(null);
  const [loading, setLoading] = useState(true);
  const [searchQuery, setSearchQuery] = useState("");

  // Wizard states: "choose" -> ("ai_input" -> "ai_review") OR "manual"
  const [wizardOpen, setWizardOpen] = useState(false);
  const [wizardMode, setWizardMode] = useState<
    "choose" | "ai_input" | "ai_review" | "manual"
  >("choose");
  const [wizardTab, setWizardTab] = useState<1 | 2 | 3>(1);
  const [paletteEditMode, setPaletteEditMode] = useState<"light" | "dark">(
    "light",
  );

  // Logo file & base64 data URL for AI analysis + automatic asset upload
  const [logoFile, setLogoFile] = useState<File | null>(null);
  const [logoDataUrl, setLogoDataUrl] = useState<string | null>(null);
  const logoInputRef = useRef<HTMLInputElement | null>(null);
  const [compilingAi, setCompilingAi] = useState(false);

  const [styleModalOpen, setStyleModalOpen] = useState(false);
  const [selectedStyle, setSelectedStyle] = useState<DesignStylePreset | null>(
    null,
  );
  const [selectedModifierIds, setSelectedModifierIds] = useState<string[]>([]);
  const [name, setName] = useState("Forma");
  const [description, setDescription] = useState(
    "Creative workspace and design system platform for modern product teams",
  );
  const [purpose, setPurpose] = useState(
    "Convert creative teams to Pro subscribers with clear product value",
  );
  const [industry, setIndustry] = useState("Design & Creative SaaS");
  const [targetAudience, setTargetAudience] = useState("Creative teams");
  const [audienceNeeds, setAudienceNeeds] = useState(
    "Fast iteration, unlimited projects, and clear pricing transparency",
  );
  const [desiredAction, setDesiredAction] = useState("Start creating");
  const [brand, setBrand] = useState<BrandSpec>(DEFAULT_BRAND);
  const [creating, setCreating] = useState(false);
  const [createError, setCreateError] = useState<string | null>(null);

  const [deleteTarget, setDeleteTarget] = useState<ProjectSummaryDto | null>(
    null,
  );
  const [deleting, setDeleting] = useState(false);
  const [projectFilter, setProjectFilter] = useState<
    "all" | "custom" | "examples"
  >("all");

  // GitHub Releases Auto-Updater states
  const [updateReport, setUpdateReport] = useState<UpdateCheckReport | null>(
    null,
  );
  const [updateModalOpen, setUpdateModalOpen] = useState(false);
  const [checkingUpdate, setCheckingUpdate] = useState(false);
  const [installingUpdate, setInstallingUpdate] = useState(false);
  const [restartingApp, setRestartingApp] = useState(false);
  const [installResult, setInstallResult] =
    useState<UpdateInstallReport | null>(null);
  const [updateError, setUpdateError] = useState<string | null>(null);
  const [repoInput, setRepoInput] = useState("jumpino/design-master");
  const [autoCheckToggle, setAutoCheckToggle] = useState(true);
  const [autoInstallToggle, setAutoInstallToggle] = useState(false);

  async function checkUpdates(silent = false) {
    if (!silent) setCheckingUpdate(true);
    setUpdateError(null);
    try {
      const rep = await apiFetch<UpdateCheckReport>("/api/system/update/check");
      setUpdateReport(rep);
      setRepoInput(rep.github_repo);
      setAutoCheckToggle(rep.auto_check_enabled);
      setAutoInstallToggle(rep.auto_install_enabled);
    } catch (err: unknown) {
      if (!silent) {
        setUpdateError(
          err instanceof Error ? err.message : "Failed to check for updates",
        );
      }
    } finally {
      if (!silent) setCheckingUpdate(false);
    }
  }

  async function handleSaveUpdateConfig() {
    setCheckingUpdate(true);
    setUpdateError(null);
    try {
      const rep = await apiFetch<UpdateCheckReport>(
        "/api/system/update/config",
        {
          method: "PUT",
          body: JSON.stringify({
            github_repo: repoInput.trim(),
            auto_check_enabled: autoCheckToggle,
            auto_install_enabled: autoInstallToggle,
          }),
        },
      );
      setUpdateReport(rep);
      setRepoInput(rep.github_repo);
    } catch (err: unknown) {
      setUpdateError(
        err instanceof Error ? err.message : "Failed to save update settings",
      );
    } finally {
      setCheckingUpdate(false);
    }
  }

  async function handleInstallUpdate() {
    setInstallingUpdate(true);
    setUpdateError(null);
    setInstallResult(null);
    try {
      const res = await apiFetch<UpdateInstallReport>(
        "/api/system/update/install",
        {
          method: "POST",
        },
      );
      setInstallResult(res);
      setRestartingApp(res.restart_scheduled);
    } catch (err: unknown) {
      setUpdateError(
        err instanceof Error ? err.message : "Failed to install update",
      );
    } finally {
      setInstallingUpdate(false);
    }
  }

  async function handleRestartApp() {
    setRestartingApp(true);
    setUpdateError(null);
    try {
      await apiFetch("/api/system/update/restart", { method: "POST" });
    } catch (err: unknown) {
      setRestartingApp(false);
      setUpdateError(
        err instanceof Error ? err.message : "Could not restart Design Master",
      );
    }
  }

  function openNewProjectWizard() {
    setWizardMode("choose");
    setWizardTab(1);
    setCreateError(null);
    setWizardOpen(true);
  }

  function handleLogoFileChange(file: File | null) {
    if (!file) {
      setLogoFile(null);
      setLogoDataUrl(null);
      return;
    }
    setLogoFile(file);
    const reader = new FileReader();
    reader.onload = () => {
      if (typeof reader.result === "string") {
        setLogoDataUrl(reader.result);
      }
    };
    reader.readAsDataURL(file);
  }

  function handleApplyDesignStyle(
    style: DesignStylePreset,
    modifierIds: string[],
    combinedBrief: string,
  ) {
    setSelectedStyle(style);
    setSelectedModifierIds(modifierIds);
    setCreateError(null);
    const modTerms = DESIGN_STYLE_MODIFIERS.filter((m) =>
      modifierIds.includes(m.id),
    ).map((m) => m.term);

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
      brand_rules: [
        `Strictly follow Web Design Style #${style.number} (${style.name}): ${style.definition}`,
        style.visualRules,
        ...(modTerms.length > 0
          ? [`Combine with layout/system modifiers: ${modTerms.join(" + ")}`]
          : []),
      ],
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
  }

  async function handleCompileWithAi() {
    if (!name.trim()) {
      setCreateError("Please enter your Project / Brand Name first.");
      return;
    }
    if (!description.trim()) {
      setCreateError(
        "Please describe what your website or product is about so AI can craft your brand.",
      );
      return;
    }
    setCompilingAi(true);
    setCreateError(null);
    try {
      const preferredSummary = selectedStyle
        ? `User selected Web Design Style #${selectedStyle.number} (${selectedStyle.name}): ${selectedStyle.definition}`
        : undefined;

      const res = await apiFetch<{ proposal: AiBrandProposalDto }>(
        "/api/brand/generate-ai",
        {
          method: "POST",
          body: JSON.stringify({
            project_name: name.trim(),
            product_description: description.trim(),
            logo_data_url: logoDataUrl || undefined,
            preferred_style_summary: preferredSummary,
          }),
        },
      );

      const p = res.proposal;
      if (p.purpose) setPurpose(p.purpose);
      if (p.industry) setIndustry(p.industry);
      if (p.target_audience) setTargetAudience(p.target_audience);
      if (p.audience_needs) setAudienceNeeds(p.audience_needs);
      if (p.desired_user_action) setDesiredAction(p.desired_user_action);

      // Select recommended style if user hasn't locked one
      const styleNum = p.recommended_style_number || 1;
      const chosenPreset =
        selectedStyle ||
        DESIGN_STYLES.find((s) => s.number === styleNum) ||
        DESIGN_STYLES[0];
      setSelectedStyle(chosenPreset);

      const combinedBrief = buildCombinedStyleBrief(
        chosenPreset,
        selectedModifierIds,
      );

      setBrand((prev) => ({
        ...prev,
        brand_personality: p.brand_personality || chosenPreset.name,
        design_direction:
          p.design_direction ||
          `${chosenPreset.definition} ${chosenPreset.visualRules}`,
        tone_of_voice: p.tone_of_voice || prev.tone_of_voice,
        ui_density: p.ui_density || "comfortable",
        logo: {
          ...prev.logo,
          mode: logoFile ? "uploaded" : "temporary_monogram",
          monogram_text: name.trim().charAt(0).toUpperCase() || "D",
        },
        colors: {
          light: {
            ...prev.colors.light,
            ...(p.colors_light || {}),
          },
          dark: {
            ...prev.colors.dark,
            ...(p.colors_dark || {}),
          },
        },
        typography: {
          ...prev.typography,
          heading_family:
            p.typography_heading || chosenPreset.recommendedBrand.fontHeadline,
          body_family:
            p.typography_body || chosenPreset.recommendedBrand.fontBody,
          mono_family: p.typography_mono || prev.typography.mono_family,
        },
        layout_tokens: {
          ...prev.layout_tokens,
          spacing_scale: p.spacing_scale || prev.layout_tokens.spacing_scale,
          radii: p.radii || prev.layout_tokens.radii,
          shadows: p.shadows || prev.layout_tokens.shadows,
          icon_style: p.icon_style || prev.layout_tokens.icon_style,
        },
        brand_rules:
          p.brand_rules && p.brand_rules.length > 0
            ? p.brand_rules
            : prev.brand_rules,
        forbidden_treatments:
          p.forbidden_treatments && p.forbidden_treatments.length > 0
            ? p.forbidden_treatments
            : prev.forbidden_treatments,
        design_style: {
          style_id: chosenPreset.id,
          style_number: chosenPreset.number,
          style_name: chosenPreset.name,
          definition: chosenPreset.definition,
          category: chosenPreset.category,
          visual_rules: chosenPreset.visualRules,
          keywords: chosenPreset.keywords,
          modifiers: p.recommended_modifiers || [],
          combined_brief: combinedBrief,
        },
      }));

      setWizardMode("ai_review");
    } catch (err: unknown) {
      setCreateError(
        err instanceof Error
          ? err.message
          : "Failed to compile Brand Kit with AI",
      );
    } finally {
      setCompilingAi(false);
    }
  }

  async function loadProjects() {
    setLoading(true);
    try {
      const [sRes, pRes] = await Promise.all([
        apiFetch<SetupStatusDto>("/api/setup/status"),
        apiFetch<{ projects: ProjectSummaryDto[] }>("/api/projects"),
      ]);
      setStatus(sRes);
      setProjects(pRes.projects);
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    loadProjects();
    checkUpdates(true);
  }, []);

  useEffect(() => {
    if (searchParams.get("new") === "1") {
      openNewProjectWizard();
    }
  }, [searchParams]);

  async function handleCreateProject(e?: React.FormEvent) {
    if (e) e.preventDefault();
    if (!name.trim()) {
      setCreateError("Project name is required.");
      return;
    }
    const effectiveStyle = selectedStyle || DESIGN_STYLES[0];
    setCreating(true);
    setCreateError(null);
    try {
      const modTerms = DESIGN_STYLE_MODIFIERS.filter((m) =>
        selectedModifierIds.includes(m.id),
      ).map((m) => m.term);
      const combinedBrief = buildCombinedStyleBrief(
        effectiveStyle,
        selectedModifierIds,
      );

      const finalBrand: BrandSpec = {
        ...brand,
        design_style: {
          style_id: effectiveStyle.id,
          style_number: effectiveStyle.number,
          style_name: effectiveStyle.name,
          definition: effectiveStyle.definition,
          category: effectiveStyle.category,
          visual_rules: effectiveStyle.visualRules,
          keywords: effectiveStyle.keywords,
          modifiers: modTerms,
          combined_brief: combinedBrief,
        },
        logo: {
          ...brand.logo,
          mode: logoFile ? "uploaded" : brand.logo.mode,
          monogram_text:
            brand.logo.monogram_text ||
            name.trim().charAt(0).toUpperCase() ||
            "D",
        },
      };

      const res = await apiFetch<{
        project_id: string;
        default_thread_id: string;
      }>("/api/projects", {
        method: "POST",
        body: JSON.stringify({
          name: name.trim(),
          description,
          purpose,
          industry,
          target_audience: targetAudience,
          audience_needs: audienceNeeds,
          desired_user_action: desiredAction,
          brand: finalBrand,
          initial_thread_title: "Main Design Workspace",
        }),
      });

      // If user uploaded a logo file during creation, upload it as a brand_logo asset and link it!
      if (logoFile) {
        try {
          const fd = new FormData();
          fd.append("role", "brand_logo");
          fd.append("label", `${name.trim()} Brand Logo`);
          fd.append(
            "placement_notes",
            "Official uploaded brand logo — preserve faithfully in headers and brand marks.",
          );
          fd.append("preserve_as_is", "true");
          fd.append("file", logoFile);
          const upRes = await apiFetch<{ asset: { id: string } }>(
            `/api/projects/${res.project_id}/assets`,
            {
              method: "POST",
              body: fd,
            },
          );
          if (upRes?.asset?.id) {
            await apiFetch(`/api/projects/${res.project_id}/brand`, {
              method: "PUT",
              body: JSON.stringify({
                ...finalBrand,
                logo: {
                  ...finalBrand.logo,
                  mode: "uploaded",
                  primary_asset_id: upRes.asset.id,
                },
              }),
            });
          }
        } catch {
          // continue even if optional logo attachment upload fails
        }
      }

      setWizardOpen(false);
      router.push(
        `/workspace?project=${res.project_id}&thread=${res.default_thread_id}`,
      );
    } catch (err: unknown) {
      setCreateError(
        err instanceof Error ? err.message : "Failed to create project",
      );
    } finally {
      setCreating(false);
    }
  }

  async function handleQuickSampleProject() {
    setCreating(true);
    setCreateError(null);
    try {
      const sampleStyle = DESIGN_STYLES[0];
      const res = await apiFetch<{
        project_id: string;
        default_thread_id: string;
      }>("/api/projects", {
        method: "POST",
        body: JSON.stringify({
          name: "Forma",
          description:
            "Warm architectural design system & collaborative product workspace",
          purpose:
            "Convert creative teams to Pro subscribers with clear product value",
          industry: "SaaS / Design Tools",
          target_audience: "Product designers and frontend engineers",
          audience_needs: "Fast side-by-side plan comparison and clear ROI",
          desired_user_action: "Start 14-day Pro trial",
          brand: {
            ...brand,
            design_style: {
              style_id: sampleStyle.id,
              style_number: sampleStyle.number,
              style_name: sampleStyle.name,
              definition: sampleStyle.definition,
              category: sampleStyle.category,
              visual_rules: sampleStyle.visualRules,
              keywords: sampleStyle.keywords,
              modifiers: ["Bento", "Card-based"],
              combined_brief: buildCombinedStyleBrief(sampleStyle, [
                "bento",
                "card-based",
              ]),
            },
            logo: {
              ...brand.logo,
              monogram_text: "F",
            },
          },
          initial_thread_title: "Main Design Workspace",
        }),
      });
      router.push(
        `/workspace?project=${res.project_id}&thread=${res.default_thread_id}`,
      );
    } catch (err: unknown) {
      setCreateError(
        err instanceof Error ? err.message : "Failed to create sample project",
      );
    } finally {
      setCreating(false);
    }
  }

  async function handleConfirmDelete() {
    if (!deleteTarget) return;
    setDeleting(true);
    try {
      await apiFetch(`/api/projects/${deleteTarget.id}`, {
        method: "DELETE",
      });
      setDeleteTarget(null);
      await loadProjects();
    } finally {
      setDeleting(false);
    }
  }

  const showcaseCount = projects.filter((p) => p.is_example).length;
  const customCount = projects.length - showcaseCount;

  const filteredProjects = projects.filter((p) => {
    if (projectFilter === "examples" && !p.is_example) return false;
    if (projectFilter === "custom" && p.is_example) return false;
    if (!searchQuery.trim()) return true;
    const q = searchQuery.toLowerCase();
    return (
      p.name.toLowerCase().includes(q) ||
      p.description.toLowerCase().includes(q) ||
      p.target_audience.toLowerCase().includes(q)
    );
  });

  const activeDataDir = status?.data_dir || "";

  return (
    <div className="h-dvh overflow-y-auto bg-stone-canvas flex flex-col">
      <header className="shrink-0 border-b border-stone-border/70 bg-charcoal">
        <div className="mx-auto flex min-h-16 max-w-[1440px] flex-wrap items-center justify-between gap-3 px-5 py-3 md:px-8">
          <div className="flex items-center gap-3">
            <img
              src="/dm-logo.png"
              alt="Design Master DM Logo"
              className="h-9 w-9 rounded-lg object-cover"
            />
            <span className="font-display text-base font-bold tracking-tight text-stone-ink">
              Design Master
            </span>
            <span className="hidden border-l border-stone-border pl-3 text-xs text-stone-muted sm:block">
              Projects
            </span>
          </div>
          <div className="flex items-center gap-2 sm:gap-4">
            <span className="hidden items-center gap-2 text-[11px] text-stone-muted md:inline-flex">
              <span
                className={`h-1.5 w-1.5 rounded-full ${status?.key_status.configured ? "bg-emerald-400" : "bg-amber-400"}`}
              />
              {status?.key_status.configured
                ? "OpenRouter connected"
                : "API key not set"}
            </span>
            <button
              type="button"
              onClick={() => {
                setUpdateModalOpen(true);
                checkUpdates(false);
              }}
              title="Check for updates"
              aria-label="Check for updates"
              className={`inline-flex items-center gap-1.5 rounded-lg px-2 py-2 text-[11px] transition-colors hover:bg-stone-card ${updateReport?.update_available ? "text-emerald-300" : "text-stone-muted"}`}
            >
              <RefreshCw
                className={`h-3.5 w-3.5 ${checkingUpdate ? "animate-spin" : ""}`}
              />
              <span className="hidden sm:inline">
                {updateReport?.update_available
                  ? `Update v${updateReport.latest_version}`
                  : `v${updateReport?.current_version || "1.1.1"}`}
              </span>
            </button>
            <button
              type="button"
              onClick={() => router.push("/settings")}
              className="inline-flex items-center gap-2 rounded-lg border border-stone-border px-3 py-2 text-xs font-medium text-stone-100 transition-colors hover:bg-stone-card"
            >
              <Settings className="h-3.5 w-3.5" />
              <span>
                Settings <span className="hidden sm:inline">& Usage</span>
              </span>
            </button>
          </div>
        </div>
      </header>

      <main className="mx-auto flex w-full max-w-[1440px] flex-1 flex-col px-5 py-6 md:px-8">
        <div
          className="sticky top-0 z-20 -mx-5 bg-stone-canvas px-5 pb-4 pt-3 md:-mx-8 md:px-8"
          data-testid="project-library-toolbar"
        >
          <div className="flex flex-wrap items-center justify-between gap-4">
            <div>
              <div className="flex items-center gap-3">
                <h1 className="font-display text-2xl font-semibold tracking-tight text-stone-ink">
                  Local Projects
                </h1>
                <span className="rounded-md border border-stone-border bg-stone-surface px-2 py-0.5 text-xs tabular-nums text-stone-muted">
                  {projects.length}
                </span>
              </div>
              <p className="mt-1.5 text-xs leading-5 text-stone-muted">
                Your designs, brand kits, and coding-agent handoffs. Saved
                locally.
              </p>
            </div>
            <div className="flex items-center gap-3">
              <button
                type="button"
                disabled={creating}
                onClick={handleQuickSampleProject}
                className="inline-flex items-center gap-1.5 rounded-lg px-2 py-2.5 text-xs font-medium text-stone-muted transition-colors hover:bg-stone-surface hover:text-stone-ink disabled:opacity-50"
              >
                <Sparkles className="h-3.5 w-3.5" />
                {creating ? "Creating..." : "Try a demo"}
              </button>
              <button
                type="button"
                onClick={openNewProjectWizard}
                className="inline-flex h-10 items-center gap-2 whitespace-nowrap rounded-lg bg-crimson-600 px-4 text-xs font-semibold text-white transition-colors hover:bg-crimson-700"
              >
                <Plus className="h-4 w-4" /> New Project
              </button>
            </div>
          </div>

          {projects.length > 0 && (
            <div className="mt-5 flex flex-wrap items-center justify-between gap-3 border-b border-stone-border/70 pb-4">
              <div
                className="inline-flex items-center gap-1 rounded-lg border border-stone-border bg-stone-surface p-1"
                role="group"
                aria-label="Filter projects"
              >
                {(
                  [
                    { id: "all", label: "All", count: projects.length },
                    { id: "custom", label: "Custom", count: customCount },
                    { id: "examples", label: "Showcase", count: showcaseCount },
                  ] as const
                ).map((tab) => (
                  <button
                    key={tab.id}
                    type="button"
                    aria-pressed={projectFilter === tab.id}
                    onClick={() => setProjectFilter(tab.id)}
                    className={`inline-flex items-center gap-2 rounded-md px-3 py-1.5 text-[11px] font-medium transition-colors ${projectFilter === tab.id ? "bg-stone-card text-stone-ink shadow-sm" : "text-stone-muted hover:text-stone-ink"}`}
                  >
                    {tab.label}
                    <span
                      className={`text-[10px] tabular-nums ${projectFilter === tab.id ? "text-crimson-300" : "text-stone-muted"}`}
                    >
                      {tab.count}
                    </span>
                  </button>
                ))}
              </div>
              <div className="relative w-full sm:w-72">
                <Search className="pointer-events-none absolute left-3 top-1/2 h-3.5 w-3.5 -translate-y-1/2 text-stone-muted" />
                <input
                  type="search"
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  placeholder="Search projects..."
                  aria-label="Search projects"
                  className="h-10 w-full rounded-lg border border-stone-border pl-9 pr-9 text-xs"
                />
                {searchQuery && (
                  <button
                    type="button"
                    onClick={() => setSearchQuery("")}
                    aria-label="Clear search"
                    className="absolute right-2 top-1/2 -translate-y-1/2 rounded p-1 text-stone-muted hover:text-stone-ink"
                  >
                    <X className="h-3.5 w-3.5" />
                  </button>
                )}
              </div>
            </div>
          )}
        </div>

        <div className="mt-1">
          {loading ? (
            <div
              className="flex min-h-[280px] items-center justify-center rounded-2xl border border-stone-border text-sm text-stone-muted"
              role="status"
            >
              Loading projects...
            </div>
          ) : projects.length === 0 ? (
            <div className="flex min-h-[360px] flex-col items-center justify-center rounded-2xl border border-dashed border-stone-border bg-stone-surface p-8 text-center">
              <FolderPlus className="mb-4 h-8 w-8 text-crimson-300" />
              <h2 className="text-lg font-semibold text-stone-ink">
                Create your first project
              </h2>
              <p className="mt-2 max-w-md text-sm leading-6 text-stone-muted">
                Start with a brand kit and a design style, then use chat to
                build your first design.
              </p>
              <button
                type="button"
                onClick={openNewProjectWizard}
                className="mt-5 inline-flex items-center gap-2 rounded-lg bg-crimson-600 px-4 py-2.5 text-xs font-semibold text-white hover:bg-crimson-700"
              >
                <Plus className="h-4 w-4" /> Create first project
              </button>
            </div>
          ) : filteredProjects.length === 0 ? (
            <div className="flex min-h-[280px] flex-col items-center justify-center rounded-2xl border border-dashed border-stone-border text-center">
              <Search className="mb-3 h-6 w-6 text-stone-muted" />
              <h2 className="text-base font-semibold text-stone-ink">
                No matching projects
              </h2>
              <p className="mt-2 text-xs text-stone-muted">
                Try another search or show all projects.
              </p>
              <button
                type="button"
                onClick={() => {
                  setSearchQuery("");
                  setProjectFilter("all");
                }}
                className="mt-4 rounded-lg border border-stone-border px-3 py-2 text-xs text-stone-ink hover:bg-stone-card"
              >
                Clear filters
              </button>
            </div>
          ) : (
            <div
              className="grid grid-cols-1 gap-5 md:grid-cols-2"
              aria-label="Project library"
            >
              {filteredProjects.map((project) => (
                <ProjectCard
                  key={project.id}
                  project={project}
                  onDelete={() => setDeleteTarget(project)}
                  onOpen={() =>
                    router.push(
                      `/workspace?project=${project.id}${project.default_thread_id ? `&thread=${project.default_thread_id}` : ""}`,
                    )
                  }
                />
              ))}
            </div>
          )}
        </div>

        <footer className="mt-auto flex flex-wrap items-center justify-between gap-3 pt-6 text-[10px] text-stone-muted">
          <button
            type="button"
            onClick={() => router.push("/settings")}
            title={activeDataDir}
            className="inline-flex min-w-0 max-w-full items-center gap-2 rounded py-1 text-left hover:text-stone-ink"
          >
            <FolderOpen className="h-3.5 w-3.5 shrink-0" />
            <span className="shrink-0">Local workspace</span>
            <span className="hidden max-w-[520px] truncate font-mono opacity-80 lg:block">
              {activeDataDir}
            </span>
          </button>
          <button
            type="button"
            onClick={() => router.push("/setup")}
            className="inline-flex items-center gap-1.5 rounded py-1 hover:text-stone-ink"
          >
            <Wand2 className="h-3 w-3" /> Setup Wizard
          </button>
        </footer>
      </main>

      {/* Multi-Mode Project Creation Wizard Modal */}
      <LightDismissDialog
        open={wizardOpen && !styleModalOpen}
        onClose={() => setWizardOpen(false)}
        title={
          wizardMode === "choose"
            ? "Create New Project"
            : wizardMode === "ai_input"
              ? "✨ Create Project & Brand Kit with AI"
              : wizardMode === "ai_review"
                ? "✨ Review AI-Compiled Brand Kit & Project"
                : "🛠️ Manual Project & Brand Kit Setup"
        }
        subtitle={
          wizardMode === "choose"
            ? "Choose whether you want AI to craft your Brand Kit, Light & Dark palettes, and Design Style from your logo & description, or configure manually."
            : wizardMode === "ai_input"
              ? "Upload your logo (optional) and tell AI what your website or product is about."
              : wizardMode === "ai_review"
                ? "Review and tweak the AI-compiled Brand Kit, Light & Dark palettes, and Web Design Style before creating."
                : "Configure your project brief, Light & Dark color palettes, and typography step by step."
        }
        maxWidthClass="max-w-2xl"
      >
        {/* STEP 0: Choose AI vs Manual */}
        {wizardMode === "choose" && (
          <div className="space-y-4">
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <button
                type="button"
                onClick={() => {
                  setCreateError(null);
                  setWizardMode("ai_input");
                }}
                className="group text-left rounded-2xl border-2 border-crimson-600 bg-gradient-to-br from-[#341921] via-[#19191C] to-[#111114] p-5 hover:border-crimson-400 hover:shadow-[0_0_24px_rgba(201,39,70,0.25)] transition flex flex-col justify-between gap-4"
              >
                <div className="space-y-2.5">
                  <div className="flex items-center justify-between">
                    <span className="inline-flex h-10 w-10 items-center justify-center rounded-xl bg-crimson-600 text-white shadow-sm">
                      <Sparkles className="h-5 w-5" />
                    </span>
                    <span className="rounded-full bg-crimson-600/25 border border-crimson-600/50 px-2.5 py-0.5 text-[10px] font-bold uppercase tracking-wider text-crimson-300">
                      Recommended
                    </span>
                  </div>
                  <h3 className="text-base font-bold text-white">
                    Create with AI
                  </h3>
                  <p className="text-xs text-stone-200 leading-relaxed">
                    Provide your <strong className="text-white">Logo</strong> +{" "}
                    <strong className="text-white">
                      what your website or game UI is about
                    </strong>
                    . AI will compile every field, craft harmonious{" "}
                    <strong className="text-white">
                      Light & Dark Mode color palettes
                    </strong>
                    , pick fonts, and select the best{" "}
                    <strong className="text-white">Web Design Style</strong> for
                    your quick review.
                  </p>
                </div>
                <div className="inline-flex items-center gap-1.5 text-xs font-bold text-crimson-300 group-hover:text-white group-hover:translate-x-0.5 transition">
                  <span>Start with AI</span>
                  <ArrowRight className="h-4 w-4" />
                </div>
              </button>

              <button
                type="button"
                onClick={() => {
                  setCreateError(null);
                  setWizardMode("manual");
                  setWizardTab(1);
                }}
                className="group text-left rounded-2xl border border-[#333336] bg-[#18181B] p-5 hover:border-crimson-600/60 hover:bg-[#1F1F22] hover:shadow-md transition flex flex-col justify-between gap-4"
              >
                <div className="space-y-2.5">
                  <div className="flex items-center justify-between">
                    <span className="inline-flex h-10 w-10 items-center justify-center rounded-xl bg-[#262629] border border-[#3C3C3F] text-white shadow-sm">
                      <Sliders className="h-5 w-5 text-crimson-300" />
                    </span>
                    <span className="rounded-full bg-[#222225] border border-[#37373A] px-2.5 py-0.5 text-[10px] font-semibold uppercase tracking-wider text-stone-200">
                      Full Control
                    </span>
                  </div>
                  <h3 className="text-base font-bold text-white">
                    Create Manually
                  </h3>
                  <p className="text-xs text-stone-200 leading-relaxed">
                    Set up your Project Brief, pick from the{" "}
                    <strong className="text-white">
                      20 Curated Web Design Systems
                    </strong>
                    , and customize your{" "}
                    <strong className="text-white">
                      Light & Dark Mode color palettes
                    </strong>
                    , typography, and stack manually.
                  </p>
                </div>
                <div className="inline-flex items-center gap-1.5 text-xs font-bold text-white group-hover:translate-x-0.5 transition">
                  <span>Configure Manually</span>
                  <ArrowRight className="h-4 w-4 text-crimson-300" />
                </div>
              </button>
            </div>
          </div>
        )}

        {/* STEP 1A: AI Input (Logo + Description) */}
        {wizardMode === "ai_input" && (
          <div className="space-y-4">
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
              <div>
                <label className="block text-xs font-bold text-stone-ink mb-1">
                  1. Project / Brand Name *
                </label>
                <input
                  type="text"
                  value={name}
                  onChange={(e) => setName(e.target.value)}
                  placeholder="e.g. Jumpino Tech, NovaCloud, Forma"
                  className="w-full rounded-xl border border-stone-border bg-white px-3.5 py-2.5 text-xs font-medium text-stone-ink focus:outline-none focus:border-crimson-600"
                />
              </div>

              <div>
                <label className="block text-xs font-bold text-stone-ink mb-1">
                  2. Upload Brand Logo (Optional)
                </label>
                <input
                  ref={logoInputRef}
                  type="file"
                  accept="image/png,image/jpeg,image/webp,image/svg+xml"
                  className="hidden"
                  onChange={(e) =>
                    handleLogoFileChange(e.target.files?.[0] || null)
                  }
                />
                {logoDataUrl ? (
                  <div className="flex items-center justify-between gap-2 rounded-xl border border-crimson-600/50 bg-stone-surface px-3 py-1.5">
                    <div className="flex items-center gap-2.5 min-w-0">
                      <img
                        src={logoDataUrl}
                        alt="Logo preview"
                        className="h-8 w-8 rounded-lg object-contain bg-white border border-stone-border p-0.5 shrink-0"
                      />
                      <div className="min-w-0">
                        <div className="text-xs font-semibold text-stone-ink truncate">
                          {logoFile?.name || "Uploaded Logo"}
                        </div>
                        <div className="text-[10px] text-emerald-700 font-medium">
                          AI will extract palette & vibe from logo
                        </div>
                      </div>
                    </div>
                    <button
                      type="button"
                      onClick={() => handleLogoFileChange(null)}
                      className="rounded-lg p-1 text-stone-muted hover:bg-stone-canvas hover:text-stone-ink"
                    >
                      <X className="h-3.5 w-3.5" />
                    </button>
                  </div>
                ) : (
                  <button
                    type="button"
                    onClick={() => logoInputRef.current?.click()}
                    className="w-full flex items-center justify-center gap-2 rounded-xl border border-dashed border-stone-border bg-stone-surface hover:border-crimson-600 px-3.5 py-2.5 text-xs font-semibold text-stone-ink transition"
                  >
                    <Upload className="h-3.5 w-3.5 text-crimson-300" />
                    <span>Upload Logo (PNG / SVG / WebP)</span>
                  </button>
                )}
              </div>
            </div>

            <div>
              <label className="block text-xs font-bold text-stone-ink mb-1">
                3. What is your website or product about? *
              </label>
              <p className="text-[11px] text-stone-muted mb-1.5">
                Describe what you are building, who it is for, the vibe you want
                (e.g. dark glassmorphic AI cloud, warm tactile skeuomorphic
                link-in-bio, luxury editorial store), and key actions.
              </p>
              <textarea
                rows={4}
                value={description}
                onChange={(e) => setDescription(e.target.value)}
                placeholder="Example: A link-in-bio and digital storefront platform for creators with warm tactile skeuomorphic leather & brushed brass cards, instant WhatsApp contact, and simple pricing..."
                className="w-full rounded-xl border border-stone-border bg-white p-3 text-xs text-stone-ink leading-relaxed focus:outline-none focus:border-crimson-600"
              />
            </div>

            <div className="rounded-xl border border-stone-border bg-stone-surface p-3 flex items-center justify-between gap-3">
              <div>
                <div className="text-xs font-bold text-stone-ink">
                  4. Web Design Style (Optional Override)
                </div>
                <div className="text-[11px] text-stone-muted">
                  {selectedStyle
                    ? `Locked to #${selectedStyle.number} ${selectedStyle.name}`
                    : "AI will automatically pick the best match from the 20 Curated Web Design Systems."}
                </div>
              </div>
              <button
                type="button"
                onClick={() => setStyleModalOpen(true)}
                className="inline-flex items-center gap-1.5 rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs font-semibold text-stone-ink hover:bg-stone-canvas shrink-0"
              >
                <Palette className="h-3.5 w-3.5 text-crimson-300" />
                <span>
                  {selectedStyle ? "Change Style" : "Pick Style Manually"}
                </span>
              </button>
            </div>

            {createError && (
              <div className="rounded-lg border border-red-200 bg-red-50 px-3 py-2 text-xs text-red-800">
                {createError}
              </div>
            )}

            <div className="flex items-center justify-between pt-2 border-t border-stone-border">
              <button
                type="button"
                onClick={() => setWizardMode("choose")}
                className="inline-flex items-center gap-1.5 rounded-lg border border-stone-border bg-white px-3 py-2 text-xs font-medium text-stone-ink hover:bg-stone-canvas"
              >
                <ArrowLeft className="h-3.5 w-3.5" />
                <span>Back</span>
              </button>

              <button
                type="button"
                disabled={compilingAi}
                onClick={handleCompileWithAi}
                className="inline-flex items-center gap-2 rounded-xl bg-crimson-600 px-5 py-2.5 text-xs font-bold text-white shadow-sm hover:bg-crimson-700 disabled:opacity-60"
              >
                {compilingAi ? (
                  <>
                    <Loader2 className="h-4 w-4 animate-spin" />
                    <span>AI is compiling your Brand Kit & Palettes...</span>
                  </>
                ) : (
                  <>
                    <Sparkles className="h-4 w-4" />
                    <span>Compile Brand & Project with AI →</span>
                  </>
                )}
              </button>
            </div>
          </div>
        )}

        {/* STEP 1B: AI Review & Customize Before Creating */}
        {wizardMode === "ai_review" && (
          <div className="space-y-4 max-h-[72vh] overflow-y-auto pr-1">
            <div className="rounded-xl border border-emerald-500/30 bg-emerald-500/10 px-3.5 py-2.5 flex items-center justify-between gap-3">
              <div className="flex items-center gap-2 text-xs font-semibold text-emerald-900">
                <CheckCircle2 className="h-4 w-4 text-emerald-600 shrink-0" />
                <span>
                  AI compiled your Project Brief, Light & Dark Mode Palettes,
                  Typography, and Web Design Style! Review or tweak anything
                  below before creating.
                </span>
              </div>
              <button
                type="button"
                onClick={() => setWizardMode("ai_input")}
                className="text-[11px] font-semibold text-crimson-300 hover:underline shrink-0"
              >
                Re-prompt AI
              </button>
            </div>

            {/* Selected Design Style Card */}
            {selectedStyle && (
              <div className="rounded-xl border border-stone-border bg-stone-surface p-3.5 flex flex-col sm:flex-row gap-3.5 items-start">
                <img
                  src={selectedStyle.heroImage}
                  alt={selectedStyle.name}
                  className="w-full sm:w-40 aspect-video rounded-lg object-cover object-top border border-stone-border shrink-0"
                />
                <div className="min-w-0 flex-1 space-y-1">
                  <div className="flex items-center justify-between gap-2">
                    <div className="flex items-center gap-1.5">
                      <span className="rounded bg-crimson-600 px-1.5 py-0.5 text-[10px] font-bold text-white">
                        Style #{selectedStyle.number}
                      </span>
                      <span className="text-xs font-bold text-stone-ink">
                        {selectedStyle.name}
                      </span>
                    </div>
                    <button
                      type="button"
                      onClick={() => setStyleModalOpen(true)}
                      className="rounded-lg border border-stone-border bg-white px-2.5 py-1 text-[11px] font-semibold text-stone-ink hover:bg-stone-canvas"
                    >
                      Change Style
                    </button>
                  </div>
                  <p className="text-[11px] text-stone-muted leading-snug">
                    {selectedStyle.definition}
                  </p>
                </div>
              </div>
            )}

            {/* Compiled Light & Dark Palettes Side-by-Side */}
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              {(["light", "dark"] as const).map((modeKey) => {
                const pal = brand.colors[modeKey];
                return (
                  <div
                    key={modeKey}
                    className={`rounded-xl border p-3 space-y-2.5 ${
                      modeKey === "dark"
                        ? "bg-charcoal text-white border-charcoal-border"
                        : "bg-white text-stone-ink border-stone-border"
                    }`}
                  >
                    <div className="flex items-center justify-between">
                      <span className="inline-flex items-center gap-1.5 text-xs font-bold">
                        {modeKey === "light" ? (
                          <Sun className="h-3.5 w-3.5 text-amber-500" />
                        ) : (
                          <Moon className="h-3.5 w-3.5 text-sky-400" />
                        )}
                        {modeKey === "light"
                          ? "Light Mode Palette"
                          : "Dark Mode Palette"}
                      </span>
                    </div>
                    <div className="grid grid-cols-3 gap-2">
                      {(
                        [
                          ["primary", "Primary"],
                          ["accent", "Accent"],
                          ["background", "Bg"],
                          ["surface", "Surface"],
                          ["text", "Text"],
                          ["border", "Border"],
                        ] as const
                      ).map(([cKey, cLabel]) => (
                        <div key={cKey} className="space-y-1">
                          <div className="text-[10px] opacity-75">{cLabel}</div>
                          <div className="flex items-center gap-1">
                            <input
                              type="color"
                              value={pal[cKey]}
                              onChange={(e) =>
                                setBrand((b) => ({
                                  ...b,
                                  colors: {
                                    ...b.colors,
                                    [modeKey]: {
                                      ...b.colors[modeKey],
                                      [cKey]: e.target.value,
                                    },
                                  },
                                }))
                              }
                              className="h-6 w-6 rounded border border-stone-border/50 cursor-pointer shrink-0"
                            />
                            <input
                              type="text"
                              value={pal[cKey]}
                              onChange={(e) =>
                                setBrand((b) => ({
                                  ...b,
                                  colors: {
                                    ...b.colors,
                                    [modeKey]: {
                                      ...b.colors[modeKey],
                                      [cKey]: e.target.value,
                                    },
                                  },
                                }))
                              }
                              className={`w-full rounded px-1.5 py-0.5 text-[10px] font-mono border ${
                                modeKey === "dark"
                                  ? "bg-charcoal-deep border-charcoal-border text-white"
                                  : "bg-stone-canvas border-stone-border text-stone-ink"
                              }`}
                            />
                          </div>
                        </div>
                      ))}
                    </div>
                  </div>
                );
              })}
            </div>

            {/* Compiled Project Identity & Fonts */}
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3 rounded-xl border border-stone-border bg-white p-3.5">
              <div>
                <label className="block text-[11px] font-bold text-stone-ink mb-1">
                  Industry & Purpose
                </label>
                <input
                  type="text"
                  value={industry}
                  onChange={(e) => setIndustry(e.target.value)}
                  className="w-full rounded-lg border border-stone-border px-2.5 py-1.5 text-xs mb-2"
                />
                <input
                  type="text"
                  value={purpose}
                  onChange={(e) => setPurpose(e.target.value)}
                  className="w-full rounded-lg border border-stone-border px-2.5 py-1.5 text-xs"
                />
              </div>
              <div>
                <label className="block text-[11px] font-bold text-stone-ink mb-1">
                  Target Audience & Primary CTA
                </label>
                <input
                  type="text"
                  value={targetAudience}
                  onChange={(e) => setTargetAudience(e.target.value)}
                  className="w-full rounded-lg border border-stone-border px-2.5 py-1.5 text-xs mb-2"
                />
                <input
                  type="text"
                  value={desiredAction}
                  onChange={(e) => setDesiredAction(e.target.value)}
                  className="w-full rounded-lg border border-stone-border px-2.5 py-1.5 text-xs"
                />
              </div>
              <div>
                <label className="block text-[11px] font-bold text-stone-ink mb-1">
                  Heading & Body Fonts
                </label>
                <div className="flex gap-2">
                  <input
                    type="text"
                    value={brand.typography.heading_family}
                    onChange={(e) =>
                      setBrand((b) => ({
                        ...b,
                        typography: {
                          ...b.typography,
                          heading_family: e.target.value,
                        },
                      }))
                    }
                    className="w-1/2 rounded-lg border border-stone-border px-2.5 py-1.5 text-xs"
                  />
                  <input
                    type="text"
                    value={brand.typography.body_family}
                    onChange={(e) =>
                      setBrand((b) => ({
                        ...b,
                        typography: {
                          ...b.typography,
                          body_family: e.target.value,
                        },
                      }))
                    }
                    className="w-1/2 rounded-lg border border-stone-border px-2.5 py-1.5 text-xs"
                  />
                </div>
              </div>
              <div>
                <label className="block text-[11px] font-bold text-stone-ink mb-1">
                  Brand Personality
                </label>
                <input
                  type="text"
                  value={brand.brand_personality}
                  onChange={(e) =>
                    setBrand((b) => ({
                      ...b,
                      brand_personality: e.target.value,
                    }))
                  }
                  className="w-full rounded-lg border border-stone-border px-2.5 py-1.5 text-xs"
                />
              </div>
            </div>

            {createError && (
              <div className="rounded-lg border border-red-200 bg-red-50 px-3 py-2 text-xs text-red-800">
                {createError}
              </div>
            )}

            <div className="flex items-center justify-between pt-2 border-t border-stone-border">
              <button
                type="button"
                onClick={() => setWizardMode("ai_input")}
                className="inline-flex items-center gap-1.5 rounded-lg border border-stone-border bg-white px-3 py-2 text-xs font-medium text-stone-ink hover:bg-stone-canvas"
              >
                <ArrowLeft className="h-3.5 w-3.5" />
                <span>Back to AI Input</span>
              </button>

              <button
                type="button"
                disabled={creating}
                onClick={() => handleCreateProject()}
                className="inline-flex items-center gap-2 rounded-xl bg-crimson-600 px-5 py-2.5 text-xs font-bold text-white shadow-sm hover:bg-crimson-700 disabled:opacity-50"
              >
                <CheckCircle2 className="h-4 w-4" />
                <span>
                  {creating
                    ? "Creating Project..."
                    : "Create Project & Open Workbench →"}
                </span>
              </button>
            </div>
          </div>
        )}

        {/* STEP 2: Manual 3-Tab Setup */}
        {wizardMode === "manual" && (
          <form onSubmit={handleCreateProject} className="space-y-4">
            <div className="flex items-center justify-between gap-2">
              <button
                type="button"
                onClick={() => setWizardMode("choose")}
                className="inline-flex items-center gap-1 text-[11px] font-semibold text-stone-muted hover:text-stone-ink"
              >
                <ArrowLeft className="h-3 w-3" />
                <span>Switch to Create with AI</span>
              </button>
            </div>

            <div className="grid grid-cols-3 gap-1.5 rounded-xl bg-stone-canvas p-1 border border-stone-border">
              <button
                type="button"
                onClick={() => setWizardTab(1)}
                className={`flex items-center justify-center gap-1.5 rounded-lg py-1.5 px-2 text-xs font-semibold transition ${
                  wizardTab === 1
                    ? "bg-white text-stone-ink shadow-sm"
                    : "text-stone-muted hover:text-stone-ink"
                }`}
              >
                <FileText className="h-3.5 w-3.5 text-crimson-300" />
                <span>1. Project Brief</span>
              </button>
              <button
                type="button"
                onClick={() => setWizardTab(2)}
                className={`flex items-center justify-center gap-1.5 rounded-lg py-1.5 px-2 text-xs font-semibold transition ${
                  wizardTab === 2
                    ? "bg-white text-stone-ink shadow-sm"
                    : "text-stone-muted hover:text-stone-ink"
                }`}
              >
                <Palette className="h-3.5 w-3.5 text-crimson-300" />
                <span>2. Light & Dark Palettes</span>
              </button>
              <button
                type="button"
                onClick={() => setWizardTab(3)}
                className={`flex items-center justify-center gap-1.5 rounded-lg py-1.5 px-2 text-xs font-semibold transition ${
                  wizardTab === 3
                    ? "bg-white text-stone-ink shadow-sm"
                    : "text-stone-muted hover:text-stone-ink"
                }`}
              >
                <Code2 className="h-3.5 w-3.5 text-crimson-300" />
                <span>3. Fonts & Stack</span>
              </button>
            </div>

            {wizardTab === 1 && (
              <div className="space-y-3">
                <div
                  className={`rounded-xl border p-3.5 transition-all ${
                    selectedStyle
                      ? "border-crimson-600/50 bg-stone-surface"
                      : "border-dashed border-crimson-600 bg-crimson-600/5"
                  }`}
                >
                  <div className="flex items-center justify-between gap-3">
                    <div>
                      <div className="flex items-center gap-2">
                        <span className="text-xs font-bold text-stone-ink">
                          Web Design Style & System Definition *
                        </span>
                        <span className="rounded-full bg-crimson-600/20 border border-crimson-600/40 px-2 py-0.5 text-[10px] font-semibold text-crimson-300">
                          20 Systems
                        </span>
                      </div>
                      <p className="text-[11px] text-stone-muted mt-0.5">
                        Choose one of the 20 curated web design systems so the
                        AI planner and image generator craft every screen to
                        match your design system.
                      </p>
                    </div>
                    <button
                      type="button"
                      data-testid="choose-style-btn"
                      onClick={() => setStyleModalOpen(true)}
                      className="inline-flex items-center gap-1.5 rounded-lg bg-crimson-600 px-3.5 py-2 text-xs font-semibold text-white hover:bg-crimson-700 shadow-sm shrink-0"
                    >
                      <Palette className="h-3.5 w-3.5" />
                      <span>
                        {selectedStyle ? "Change style" : "Choose style"}
                      </span>
                    </button>
                  </div>

                  {selectedStyle && (
                    <div
                      data-testid="selected-style-preview-card"
                      className="mt-3 pt-3 border-t border-stone-border flex flex-col sm:flex-row gap-3 items-start"
                    >
                      <img
                        src={selectedStyle.heroImage}
                        alt={selectedStyle.name}
                        className="w-full sm:w-36 aspect-video rounded-lg object-cover object-top border border-stone-border shrink-0"
                      />
                      <div className="min-w-0 flex-1 space-y-1">
                        <div className="flex flex-wrap items-center gap-1.5">
                          <span className="rounded bg-charcoal px-1.5 py-0.5 text-[10px] font-bold text-white">
                            #{selectedStyle.number}
                          </span>
                          <span className="text-xs font-bold text-stone-ink">
                            {selectedStyle.name}
                          </span>
                        </div>
                        <p className="text-[11px] text-stone-ink leading-snug">
                          {selectedStyle.definition}
                        </p>
                      </div>
                    </div>
                  )}
                </div>

                <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
                  <div>
                    <label className="block text-xs font-semibold text-stone-ink mb-1">
                      Project / Product Name *
                    </label>
                    <input
                      type="text"
                      required
                      value={name}
                      onChange={(e) => setName(e.target.value)}
                      className="w-full rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs text-stone-ink"
                    />
                  </div>
                  <div>
                    <label className="block text-xs font-semibold text-stone-ink mb-1">
                      Industry / Category
                    </label>
                    <input
                      type="text"
                      value={industry}
                      onChange={(e) => setIndustry(e.target.value)}
                      className="w-full rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs text-stone-ink"
                    />
                  </div>
                </div>

                <div>
                  <label className="block text-xs font-semibold text-stone-ink mb-1">
                    What the Product Does & What the Website Is For
                  </label>
                  <textarea
                    rows={2}
                    value={description}
                    onChange={(e) => setDescription(e.target.value)}
                    className="w-full rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs text-stone-ink"
                  />
                </div>

                <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
                  <div>
                    <label className="block text-xs font-semibold text-stone-ink mb-1">
                      Target Audience
                    </label>
                    <input
                      type="text"
                      value={targetAudience}
                      onChange={(e) => setTargetAudience(e.target.value)}
                      className="w-full rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs text-stone-ink"
                    />
                  </div>
                  <div>
                    <label className="block text-xs font-semibold text-stone-ink mb-1">
                      Desired User Action (Primary CTA)
                    </label>
                    <input
                      type="text"
                      value={desiredAction}
                      onChange={(e) => setDesiredAction(e.target.value)}
                      className="w-full rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs text-stone-ink"
                    />
                  </div>
                </div>
              </div>
            )}

            {wizardTab === 2 && (
              <div className="rounded-xl border border-stone-border bg-white p-4 space-y-3">
                <div className="flex items-center justify-between">
                  <div className="text-xs font-bold text-stone-ink">
                    Color Palette (
                    {paletteEditMode === "light" ? "Light Mode" : "Dark Mode"})
                  </div>
                  <div className="inline-flex rounded-lg bg-stone-canvas p-0.5 border border-stone-border">
                    <button
                      type="button"
                      onClick={() => setPaletteEditMode("light")}
                      className={`inline-flex items-center gap-1 rounded-md px-2.5 py-1 text-[11px] font-semibold ${
                        paletteEditMode === "light"
                          ? "bg-white text-stone-ink shadow-sm"
                          : "text-stone-muted"
                      }`}
                    >
                      <Sun className="h-3 w-3 text-amber-500" />
                      Light
                    </button>
                    <button
                      type="button"
                      onClick={() => setPaletteEditMode("dark")}
                      className={`inline-flex items-center gap-1 rounded-md px-2.5 py-1 text-[11px] font-semibold ${
                        paletteEditMode === "dark"
                          ? "bg-charcoal text-white shadow-sm"
                          : "text-stone-muted"
                      }`}
                    >
                      <Moon className="h-3 w-3 text-sky-400" />
                      Dark
                    </button>
                  </div>
                </div>

                <div className="grid grid-cols-2 sm:grid-cols-3 gap-3 pt-1">
                  {(
                    [
                      ["primary", "Primary CTA"],
                      ["accent", "Accent / Secondary"],
                      ["background", "Page Background"],
                      ["surface", "Card Surface"],
                      ["text", "Primary Text"],
                      ["border", "Border"],
                    ] as const
                  ).map(([key, label]) => (
                    <div key={key}>
                      <label className="block text-[11px] font-medium text-stone-muted mb-1">
                        {label}
                      </label>
                      <div className="flex items-center gap-1.5">
                        <input
                          type="color"
                          value={brand.colors[paletteEditMode][key]}
                          onChange={(e) =>
                            setBrand((b) => ({
                              ...b,
                              colors: {
                                ...b.colors,
                                [paletteEditMode]: {
                                  ...b.colors[paletteEditMode],
                                  [key]: e.target.value,
                                },
                              },
                            }))
                          }
                          className="h-7 w-7 rounded border border-stone-border cursor-pointer shrink-0"
                        />
                        <input
                          type="text"
                          value={brand.colors[paletteEditMode][key]}
                          onChange={(e) =>
                            setBrand((b) => ({
                              ...b,
                              colors: {
                                ...b.colors,
                                [paletteEditMode]: {
                                  ...b.colors[paletteEditMode],
                                  [key]: e.target.value,
                                },
                              },
                            }))
                          }
                          className="w-full rounded border border-stone-border px-2 py-1 text-xs font-mono"
                        />
                      </div>
                    </div>
                  ))}
                </div>
              </div>
            )}

            {wizardTab === 3 && (
              <div className="rounded-xl border border-stone-border bg-white p-4 space-y-3">
                <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
                  <div>
                    <label className="block text-xs font-semibold text-stone-ink mb-1">
                      Heading Font Family
                    </label>
                    <input
                      type="text"
                      value={brand.typography.heading_family}
                      onChange={(e) =>
                        setBrand((b) => ({
                          ...b,
                          typography: {
                            ...b.typography,
                            heading_family: e.target.value,
                          },
                        }))
                      }
                      className="w-full rounded-lg border border-stone-border px-2.5 py-1.5 text-xs"
                    />
                  </div>
                  <div>
                    <label className="block text-xs font-semibold text-stone-ink mb-1">
                      Body Font Family
                    </label>
                    <input
                      type="text"
                      value={brand.typography.body_family}
                      onChange={(e) =>
                        setBrand((b) => ({
                          ...b,
                          typography: {
                            ...b.typography,
                            body_family: e.target.value,
                          },
                        }))
                      }
                      className="w-full rounded-lg border border-stone-border px-2.5 py-1.5 text-xs"
                    />
                  </div>
                </div>

                <div>
                  <label className="block text-xs font-semibold text-stone-ink mb-1">
                    Target Frontend Stack (used in Coding-Agent Handoffs)
                  </label>
                  <input
                    type="text"
                    value={brand.target_context.target_stack}
                    onChange={(e) =>
                      setBrand((b) => ({
                        ...b,
                        target_context: {
                          ...b.target_context,
                          target_stack: e.target.value,
                        },
                      }))
                    }
                    className="w-full rounded-lg border border-stone-border px-2.5 py-1.5 text-xs"
                  />
                </div>
              </div>
            )}

            {createError && (
              <div className="rounded-lg border border-red-200 bg-red-50 px-3 py-2 text-xs text-red-800">
                {createError}
              </div>
            )}

            <div className="flex items-center justify-between gap-3 pt-2 border-t border-stone-border">
              <div className="flex items-center gap-1.5">
                {wizardTab > 1 && (
                  <button
                    type="button"
                    onClick={() => setWizardTab((t) => (t === 3 ? 2 : 1))}
                    className="rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs font-medium text-stone-ink hover:bg-stone-canvas"
                  >
                    Previous tab
                  </button>
                )}
                {wizardTab < 3 && (
                  <button
                    type="button"
                    onClick={() => setWizardTab((t) => (t === 1 ? 2 : 3))}
                    className="rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs font-medium text-stone-ink hover:bg-stone-canvas"
                  >
                    Next tab
                  </button>
                )}
              </div>

              <div className="flex items-center gap-2">
                <button
                  type="button"
                  onClick={() => setWizardOpen(false)}
                  className="rounded-lg border border-stone-border bg-white px-3.5 py-2 text-xs font-medium text-stone-ink"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  disabled={creating}
                  className="inline-flex items-center gap-1.5 rounded-lg bg-crimson-600 px-4 py-2 text-xs font-semibold text-white hover:bg-crimson-700 disabled:opacity-50"
                >
                  <CheckCircle2 className="h-3.5 w-3.5" />
                  <span>
                    {creating
                      ? "Creating..."
                      : "Create project & open workbench"}
                  </span>
                </button>
              </div>
            </div>
          </form>
        )}
      </LightDismissDialog>

      {/* Design Style Picker Modal (50 Styles + 13 Modifiers) */}
      <DesignStylePickerModal
        open={styleModalOpen}
        onClose={() => setStyleModalOpen(false)}
        selectedStyleId={selectedStyle?.id || null}
        selectedModifierIds={selectedModifierIds}
        onSelectStyle={handleApplyDesignStyle}
      />

      {/* Confirm Project Deletion Dialog */}
      <LightDismissDialog
        open={Boolean(deleteTarget)}
        onClose={() => setDeleteTarget(null)}
        title="Delete Local Project"
        subtitle="This permanently deletes the project, threads, generated originals, and exports from disk."
        maxWidthClass="max-w-md"
      >
        {deleteTarget && (
          <div className="space-y-4">
            <p className="text-xs text-stone-ink leading-relaxed">
              Are you sure you want to permanently delete{" "}
              <strong>{deleteTarget.name}</strong>?
            </p>
            <div className="rounded-lg border border-red-200 bg-red-50 p-3 text-xs text-red-900 space-y-1">
              <div className="font-semibold">Affected items to be removed:</div>
              <div>• {deleteTarget.thread_count} threads</div>
              <div>
                • {deleteTarget.run_count} generation runs & saved outputs
              </div>
              <div>• Versioned brand kit (v{deleteTarget.brand_version})</div>
            </div>
            <div className="flex items-center justify-end gap-2">
              <button
                type="button"
                onClick={() => setDeleteTarget(null)}
                className="rounded-lg border border-stone-border bg-white px-4 py-2 text-xs font-medium text-stone-ink"
              >
                Cancel
              </button>
              <button
                type="button"
                disabled={deleting}
                onClick={handleConfirmDelete}
                className="rounded-lg bg-red-600 px-4 py-2 text-xs font-semibold text-white hover:bg-red-700 disabled:opacity-50"
              >
                {deleting ? "Deleting..." : "Confirm permanent delete"}
              </button>
            </div>
          </div>
        )}
      </LightDismissDialog>

      {/* GitHub Releases Auto-Update Modal */}
      <LightDismissDialog
        open={updateModalOpen}
        onClose={() => setUpdateModalOpen(false)}
        title="GitHub Releases Auto-Updater"
        subtitle="Automatically checks and installs the latest Design Master releases from GitHub with zero-breakage safety guarantees."
        maxWidthClass="max-w-xl"
      >
        <div className="space-y-4">
          {/* Status Banner */}
          <div
            className={`rounded-xl border p-4 ${
              updateReport?.update_available
                ? "border-emerald-500/40 bg-emerald-950/30"
                : "border-[#333336] bg-[#17171A]"
            }`}
          >
            <div className="flex items-start justify-between gap-3">
              <div>
                <div className="flex items-center gap-2">
                  <span className="text-sm font-bold text-white">
                    Current Version: v{updateReport?.current_version || "1.1.1"}
                  </span>
                  <span className="rounded-md bg-[#242427] border border-[#3A3A3D] px-2 py-0.5 text-[10px] font-mono text-crimson-300 uppercase">
                    {updateReport?.platform || "desktop"} ·{" "}
                    {updateReport?.arch || "native"}
                  </span>
                  {updateReport?.update_available && (
                    <span className="rounded-full bg-emerald-500/20 border border-emerald-400/50 px-2.5 py-0.5 text-[11px] font-semibold text-emerald-300">
                      New Release: v{updateReport.latest_version}
                    </span>
                  )}
                </div>
                <p className="text-xs text-slate-300 mt-1.5 leading-relaxed">
                  {updateReport?.status_message ||
                    "Checking GitHub Releases for new versions..."}
                </p>
              </div>

              <button
                type="button"
                disabled={checkingUpdate}
                onClick={() => checkUpdates(false)}
                className="shrink-0 inline-flex items-center gap-1.5 rounded-lg border border-[#38383B] bg-[#202023] px-3 py-1.5 text-xs font-semibold text-slate-100 hover:bg-[#2C2C2F] disabled:opacity-50"
              >
                <RefreshCw
                  className={`h-3.5 w-3.5 text-crimson-300 ${
                    checkingUpdate ? "animate-spin" : ""
                  }`}
                />
                <span>{checkingUpdate ? "Checking..." : "Check Now"}</span>
              </button>
            </div>

            {updateReport?.release_notes && (
              <div className="mt-3 rounded-lg border border-[#2F2F32] bg-[#0E0E11] p-3 text-xs text-slate-200 max-h-36 overflow-y-auto whitespace-pre-wrap font-mono">
                {updateReport.release_notes}
              </div>
            )}

            {updateReport?.release_html_url && (
              <div className="mt-2.5 flex items-center gap-3 text-[11px]">
                <a
                  href={updateReport.release_html_url}
                  target="_blank"
                  rel="noreferrer"
                  className="inline-flex items-center gap-1 text-crimson-300 hover:text-crimson-300 underline"
                >
                  <span>View GitHub Releases ({updateReport.github_repo})</span>
                  <ExternalLink className="h-3 w-3" />
                </a>
                {updateReport.download_asset_name && (
                  <span className="text-slate-400">
                    Matched package:{" "}
                    <strong className="text-slate-200">
                      {updateReport.download_asset_name}
                    </strong>
                  </span>
                )}
              </div>
            )}
          </div>

          {/* Repository & Auto-Update Preferences */}
          <div className="rounded-xl border border-[#2F2F32] bg-[#141417] p-4 space-y-3">
            <div>
              <label className="block text-xs font-semibold text-white mb-1">
                GitHub Repository (`owner/repo`)
              </label>
              <div className="flex items-center gap-2">
                <input
                  type="text"
                  value={repoInput}
                  onChange={(e) => setRepoInput(e.target.value)}
                  placeholder="owner/design-master"
                  className="flex-1 h-9 rounded-lg border border-[#333336] bg-[#0E0E11] px-3 text-xs text-white font-mono focus:outline-none focus:border-crimson-400"
                />
                <button
                  type="button"
                  disabled={checkingUpdate}
                  onClick={handleSaveUpdateConfig}
                  className="h-9 rounded-lg bg-[#2B2B2E] border border-[#404043] px-3.5 text-xs font-semibold text-white hover:bg-[#363639] disabled:opacity-50"
                >
                  Save & Check
                </button>
              </div>
            </div>

            <div className="flex flex-col sm:flex-row sm:items-center justify-between gap-3 pt-1">
              <label className="inline-flex items-center gap-2 text-xs text-slate-200 cursor-pointer">
                <input
                  type="checkbox"
                  checked={autoCheckToggle}
                  onChange={(e) => setAutoCheckToggle(e.target.checked)}
                  className="rounded border-slate-600"
                />
                <span>Check GitHub Releases automatically on launch</span>
              </label>

              <label className="inline-flex items-center gap-2 text-xs text-slate-200 cursor-pointer">
                <input
                  type="checkbox"
                  checked={autoInstallToggle}
                  onChange={(e) => setAutoInstallToggle(e.target.checked)}
                  className="rounded border-slate-600"
                />
                <span>Auto-install new releases in background</span>
              </label>
            </div>
          </div>

          {/* Zero-Breakage Safety Guarantees */}
          <div className="rounded-xl border border-[#2E2E31] bg-[#111114] p-3.5 space-y-2">
            <div className="flex items-center gap-2 text-xs font-semibold text-emerald-300">
              <ShieldCheck className="h-4 w-4 shrink-0" />
              <span>Zero-Breakage Update Protection Active</span>
            </div>
            <ul className="space-y-1 text-[11px] text-slate-300">
              {(
                updateReport?.safety_guarantees || [
                  "Automatic pre-update SQLite WAL database snapshot backup (`<data_dir>/backups/`).",
                  "Complete isolation of user projects, brand kits, threads, images, and OS Keychain API keys.",
                  "Isolated temporary download + archive/binary integrity verification before touching any app file.",
                  "Atomic `.bak` bundle backup with automatic instant rollback if installation fails.",
                ]
              ).map((item, idx) => (
                <li key={idx} className="flex items-start gap-1.5">
                  <CheckCircle2 className="h-3.5 w-3.5 text-emerald-400 shrink-0 mt-0.5" />
                  <span>{item}</span>
                </li>
              ))}
            </ul>
          </div>

          {updateError && (
            <div className="rounded-lg border border-red-500/40 bg-red-950/40 p-3 text-xs text-red-200">
              {updateError}
            </div>
          )}

          {installResult && (
            <div className="rounded-lg border border-emerald-500/40 bg-emerald-950/30 p-3 text-xs text-emerald-200 space-y-1.5">
              <div className="font-semibold">{installResult.message}</div>
              <div className="text-[11px] font-mono text-emerald-300/90 break-all">
                SQLite Backup: {installResult.backup_sqlite_path}
              </div>
              {restartingApp && (
                <div role="status" className="flex items-center gap-1.5 pt-1">
                  <Loader2 className="h-3.5 w-3.5 animate-spin" />
                  Opening the updated app…
                </div>
              )}
              {installResult.restart_required && !restartingApp && (
                <div className="pt-1">
                  <button
                    type="button"
                    onClick={handleRestartApp}
                    className="inline-flex items-center gap-1.5 rounded-lg bg-emerald-600 px-3.5 py-1.5 text-xs font-semibold text-white hover:bg-emerald-500"
                  >
                    <RefreshCw className="h-3.5 w-3.5" />
                    <span>Restart Design Master Now</span>
                  </button>
                </div>
              )}
            </div>
          )}

          {/* Modal Footer Actions */}
          <div className="flex items-center justify-between gap-3 pt-1">
            <button
              type="button"
              onClick={() => setUpdateModalOpen(false)}
              className="rounded-lg border border-[#333336] bg-[#1A1A1D] px-4 py-2 text-xs font-semibold text-slate-200 hover:bg-[#252528]"
            >
              Close
            </button>

            <button
              type="button"
              disabled={
                installingUpdate ||
                restartingApp ||
                !updateReport?.update_available ||
                !!installResult?.ok
              }
              onClick={handleInstallUpdate}
              className="inline-flex items-center gap-1.5 rounded-lg bg-crimson-600 px-4 py-2 text-xs font-semibold text-white shadow-sm hover:bg-crimson-700 disabled:opacity-50"
            >
              {installingUpdate ? (
                <Loader2 className="h-3.5 w-3.5 animate-spin" />
              ) : (
                <Download className="h-3.5 w-3.5" />
              )}
              <span>
                {restartingApp
                  ? "Restarting…"
                  : installingUpdate
                    ? "Backing up & Installing..."
                    : updateReport?.update_available
                      ? `Install v${updateReport.latest_version} Update Now`
                      : "Up to date"}
              </span>
            </button>
          </div>
        </div>
      </LightDismissDialog>
    </div>
  );
}

export default function ProjectsPage() {
  return (
    <Suspense
      fallback={
        <div className="h-screen flex items-center justify-center bg-stone-canvas text-stone-muted text-sm">
          Loading projects...
        </div>
      }
    >
      <ProjectsContent />
    </Suspense>
  );
}

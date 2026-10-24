"use client";

import React, { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import {
  ArrowLeft,
  BarChart3,
  CheckCircle2,
  Cpu,
  Database,
  DollarSign,
  FolderOpen,
  KeyRound,
  Layers,
  RefreshCw,
  Save,
  ShieldAlert,
  Sliders,
  Sparkles,
  Trash2,
  Wand2,
} from "lucide-react";
import { apiFetch, SettingsUsageDto, SetupStatusDto } from "@/lib/api";

interface TestModelsResponse {
  capabilities: SetupStatusDto["capabilities"];
  planner_ok: boolean;
  planner_multimodal?: boolean;
  planner_message: string;
  image_ok: boolean;
  image_message: string;
}

const QUALITY_TIER_INFO: Array<{
  id: string;
  label: string;
  badge: string;
  defaultPriceUsd: number;
  desc: string;
}> = [
  {
    id: "low",
    label: "Low Quality",
    badge: "Fastest Drafts",
    defaultPriceUsd: 0.0076,
    desc: "~200 output image tokens · Ideal for rapid layout exploration & style catalog previews (~$0.007/img).",
  },
  {
    id: "medium",
    label: "Medium Quality",
    badge: "Default Recommended",
    defaultPriceUsd: 0.0211,
    desc: "~650 output image tokens · Best balance of crisp UI typography, transparent PNG extraction & speed (~$0.021/img).",
  },
  {
    id: "high",
    label: "High Quality",
    badge: "Production Detail",
    defaultPriceUsd: 0.0541,
    desc: "~1,750 output image tokens · High-density textures, intricate Game HUD filigree & complex landing pages (~$0.054/img).",
  },
  {
    id: "xhigh",
    label: "Max / X-High",
    badge: "Ultra Fidelity",
    defaultPriceUsd: 0.2116,
    desc: "~7,000 output image tokens · Maximum micro-detail for flagship hero showcases (~$0.211/img).",
  },
];

export default function SettingsPage() {
  const router = useRouter();
  const [status, setStatus] = useState<SetupStatusDto | null>(null);
  const [usage, setUsage] = useState<SettingsUsageDto | null>(null);
  const [loading, setLoading] = useState(true);

  const [apiKeyInput, setApiKeyInput] = useState("");
  const [preferSession, setPreferSession] = useState(false);
  const [testingKey, setTestingKey] = useState(false);

  const [plannerModel, setPlannerModel] = useState(
    "google/gemini-3.1-pro-preview"
  );
  const [imageModel, setImageModel] = useState("openai/gpt-5-image");
  const [defaultQuality, setDefaultQuality] = useState("medium");
  const [outputSize, setOutputSize] = useState("provider_default");
  const [includeExtraction, setIncludeExtraction] = useState(true);
  const [maxConcurrent, setMaxConcurrent] = useState(3);
  const [spendLimitUsd, setSpendLimitUsd] = useState("");
  const [customDataDir, setCustomDataDir] = useState("");

  const [feedback, setFeedback] = useState<{
    type: "success" | "error";
    text: string;
  } | null>(null);
  const [refreshingCaps, setRefreshingCaps] = useState(false);
  const [testingModels, setTestingModels] = useState(false);
  const [modelTestReport, setModelTestReport] =
    useState<TestModelsResponse | null>(null);
  const [pickingFolder, setPickingFolder] = useState(false);

  async function loadStatusAndUsage() {
    setLoading(true);
    try {
      const [s, u] = await Promise.all([
        apiFetch<SetupStatusDto>("/api/setup/status"),
        apiFetch<SettingsUsageDto>("/api/settings/usage"),
      ]);
      setStatus(s);
      setUsage(u);
      setPlannerModel(s.settings.planner_model);
      setImageModel(s.settings.image_model);
      setDefaultQuality(s.settings.default_quality || "medium");
      setOutputSize(s.settings.output_size);
      setIncludeExtraction(s.settings.include_extraction_instructions);
      setMaxConcurrent(s.settings.max_concurrent_jobs);
      setSpendLimitUsd(
        s.settings.spend_limit_micros !== null
          ? (s.settings.spend_limit_micros / 1_000_000).toFixed(2)
          : ""
      );
      setCustomDataDir(s.data_dir);
    } finally {
      setLoading(false);
    }
  }

  useEffect(() => {
    loadStatusAndUsage();
  }, []);

  async function handleSaveSettings(e: React.FormEvent) {
    e.preventDefault();
    setFeedback(null);
    try {
      const spendMicros =
        spendLimitUsd.trim() !== ""
          ? Math.round(parseFloat(spendLimitUsd) * 1_000_000)
          : null;
      if (customDataDir.trim() && customDataDir.trim() !== status?.data_dir) {
        const dirRes = await apiFetch<{ data_dir: string }>(
          "/api/setup/data-dir",
          {
            method: "POST",
            body: JSON.stringify({ data_dir: customDataDir.trim() }),
          }
        );
        setCustomDataDir(dirRes.data_dir);
      }
      const res = await apiFetch<{
        settings: SetupStatusDto["settings"];
        capabilities: SetupStatusDto["capabilities"];
      }>("/api/settings", {
        method: "PUT",
        body: JSON.stringify({
          planner_model: plannerModel.trim(),
          image_model: imageModel.trim(),
          default_quality: defaultQuality,
          output_size: outputSize,
          include_extraction_instructions: includeExtraction,
          max_concurrent_jobs: maxConcurrent,
          spend_limit_micros: spendMicros,
          test_adapter_enabled: false,
        }),
      });
      setStatus((prev) =>
        prev
          ? {
              ...prev,
              data_dir: customDataDir.trim() || prev.data_dir,
              settings: res.settings,
              capabilities: res.capabilities,
            }
          : prev
      );
      const refreshedUsage = await apiFetch<SettingsUsageDto>(
        "/api/settings/usage"
      );
      setUsage(refreshedUsage);
      setFeedback({
        type: "success",
        text: "Settings and organized project folder path saved.",
      });
    } catch (err: unknown) {
      setFeedback({
        type: "error",
        text: err instanceof Error ? err.message : "Failed to save settings",
      });
    }
  }

  async function handleTestModels() {
    setTestingModels(true);
    setFeedback(null);
    setModelTestReport(null);
    try {
      const res = await apiFetch<TestModelsResponse>("/api/setup/test-models", {
        method: "POST",
        body: JSON.stringify({
          planner_model: plannerModel.trim(),
          image_model: imageModel.trim(),
        }),
      });
      setModelTestReport(res);
      setStatus((prev) =>
        prev ? { ...prev, capabilities: res.capabilities } : prev
      );
      setUsage((prev) =>
        prev
          ? {
              ...prev,
              capabilities: res.capabilities,
              quality_pricing:
                res.capabilities.quality_pricing_usd || prev.quality_pricing,
            }
          : prev
      );
    } catch (err: unknown) {
      setFeedback({
        type: "error",
        text: err instanceof Error ? err.message : "Failed to test models",
      });
    } finally {
      setTestingModels(false);
    }
  }

  async function handlePickNativeFolder() {
    setPickingFolder(true);
    setFeedback(null);
    try {
      const res = await apiFetch<{ cancelled: boolean; path: string | null }>(
        "/api/setup/pick-folder",
        {
          method: "POST",
          body: JSON.stringify({}),
        }
      );
      if (!res.cancelled && res.path) {
        setCustomDataDir(res.path);
        setStatus((prev) =>
          prev ? { ...prev, data_dir: res.path! } : prev
        );
        setFeedback({
          type: "success",
          text: `Switched active storage folder to: ${res.path}. Organized project & thread folders synced!`,
        });
      }
    } catch (err: unknown) {
      setFeedback({
        type: "error",
        text:
          err instanceof Error
            ? err.message
            : "Could not open native OS folder picker",
      });
    } finally {
      setPickingFolder(false);
    }
  }

  async function handleRefreshCapabilities() {
    setRefreshingCaps(true);
    setFeedback(null);
    try {
      const res = await apiFetch<{
        capabilities: SetupStatusDto["capabilities"];
      }>("/api/capabilities/refresh", {
        method: "POST",
        body: JSON.stringify({
          planner_model: plannerModel.trim(),
          image_model: imageModel.trim(),
        }),
      });
      setStatus((prev) =>
        prev ? { ...prev, capabilities: res.capabilities } : prev
      );
      setUsage((prev) =>
        prev
          ? {
              ...prev,
              capabilities: res.capabilities,
              quality_pricing:
                res.capabilities.quality_pricing_usd || prev.quality_pricing,
            }
          : prev
      );
      setFeedback({
        type: "success",
        text: res.capabilities.live_verified
          ? "Live OpenRouter model & pricing metadata refreshed successfully."
          : "Capabilities updated using fallback metadata.",
      });
    } catch (err: unknown) {
      setFeedback({
        type: "error",
        text:
          err instanceof Error
            ? err.message
            : "Failed to refresh capabilities",
      });
    } finally {
      setRefreshingCaps(false);
    }
  }

  async function handleUpdateKey(clear: boolean) {
    setFeedback(null);
    try {
      const res = await apiFetch<{
        key_status: SetupStatusDto["key_status"];
      }>("/api/setup/key", {
        method: "POST",
        body: JSON.stringify({
          api_key: clear ? undefined : apiKeyInput.trim(),
          prefer_session_only: preferSession,
          clear,
        }),
      });
      setApiKeyInput("");
      setStatus((prev) =>
        prev ? { ...prev, key_status: res.key_status } : prev
      );
      const refreshedUsage = await apiFetch<SettingsUsageDto>(
        "/api/settings/usage"
      );
      setUsage(refreshedUsage);
      setFeedback({
        type: "success",
        text: clear
          ? "Stored OpenRouter API key removed."
          : `OpenRouter API key updated (${res.key_status.masked_hint}).`,
      });
    } catch (err: unknown) {
      setFeedback({
        type: "error",
        text: err instanceof Error ? err.message : "Key update failed",
      });
    }
  }

  async function handleTestApiKey() {
    setTestingKey(true);
    setFeedback(null);
    try {
      const res = await apiFetch<{
        validation: {
          valid: boolean;
          message: string;
          usage_usd?: number | null;
          limit_usd?: number | null;
        };
        capabilities: SetupStatusDto["capabilities"];
      }>("/api/setup/test-key", {
        method: "POST",
        body: JSON.stringify({
          api_key: apiKeyInput.trim() || undefined,
        }),
      });
      if (apiKeyInput.trim() && res.validation.valid) {
        await handleUpdateKey(false);
      } else {
        const refreshedUsage = await apiFetch<SettingsUsageDto>(
          "/api/settings/usage"
        );
        setUsage(refreshedUsage);
      }
      setFeedback({
        type: res.validation.valid ? "success" : "error",
        text: res.validation.message,
      });
    } catch (err: unknown) {
      setFeedback({
        type: "error",
        text: err instanceof Error ? err.message : "API key test failed",
      });
    } finally {
      setTestingKey(false);
    }
  }

  async function handleBackupDb() {
    setFeedback(null);
    try {
      const res = await apiFetch<{ backup_path: string }>(
        "/api/database/backup",
        {
          method: "POST",
          body: JSON.stringify({}),
        }
      );
      setFeedback({
        type: "success",
        text: `Consistent WAL-safe SQLite backup created at: ${res.backup_path}`,
      });
    } catch (err: unknown) {
      setFeedback({
        type: "error",
        text: err instanceof Error ? err.message : "Database backup failed",
      });
    }
  }

  if (loading) {
    return (
      <div className="h-screen flex items-center justify-center bg-stone-canvas text-stone-muted text-sm">
        Loading Design Master settings & usage analytics...
      </div>
    );
  }

  const caps = status?.capabilities;
  const qualityPrices =
    usage?.quality_pricing || caps?.quality_pricing_usd || {};
  const dailySeries = usage?.daily_spend || [];
  const maxDailyUsd = Math.max(
    0.05,
    ...dailySeries.map((d) => d.spend_usd || 0)
  );

  return (
    <div className="h-screen overflow-hidden bg-stone-canvas flex flex-col">
      <header className="h-14 shrink-0 bg-charcoal text-white px-6 flex items-center justify-between border-b border-charcoal-border">
        <div className="flex items-center gap-3">
          <button
            type="button"
            onClick={() => router.back()}
            className="rounded-lg p-1.5 text-stone-300 hover:bg-charcoal-hover hover:text-white"
            aria-label="Go back"
          >
            <ArrowLeft className="h-4 w-4" />
          </button>
          <img
            src="/dm-logo.png"
            alt="Design Master DM Logo"
            className="h-8 w-8 rounded-lg border border-charcoal-border object-cover shadow-sm"
          />
          <span className="font-display font-bold tracking-tight">
            Design Master — Settings & Usage
          </span>
        </div>
        <div className="flex items-center gap-2.5">
          <button
            type="button"
            onClick={() => router.push("/setup")}
            className="inline-flex items-center gap-1.5 rounded-lg border border-[#363639] bg-[#1B1B1E] px-3.5 py-1.5 text-xs font-semibold text-stone-100 hover:bg-[#27272A] hover:text-white transition-colors"
          >
            <Wand2 className="h-3.5 w-3.5 text-crimson-300" />
            <span>3-Step Setup Wizard</span>
          </button>
          <button
            type="button"
            onClick={() => router.push("/projects")}
            className="rounded-lg bg-crimson-600 px-3.5 py-1.5 text-xs font-semibold text-white hover:bg-crimson-700 shadow-sm transition-colors"
          >
            Project Library
          </button>
        </div>
      </header>

      <main className="flex-1 min-h-0 overflow-y-auto max-w-6xl w-full mx-auto py-5 px-6 space-y-5">
        {feedback && (
          <div
            role="status"
            className={`rounded-xl border px-4 py-2.5 text-xs font-medium ${
              feedback.type === "success"
                ? "border-emerald-200 bg-emerald-50 text-emerald-900"
                : "border-red-200 bg-red-50 text-red-900"
            }`}
          >
            {feedback.text}
          </div>
        )}

        {/* ================================================================= */}
        {/* 1. MONEY CONSUMED & SPENDING GRAPH                                */}
        {/* ================================================================= */}
        <section className="rounded-2xl border border-stone-border bg-stone-surface p-5 space-y-4 shadow-sm">
          <div className="flex flex-wrap items-center justify-between gap-3">
            <div className="flex items-center gap-2.5">
              <div className="h-9 w-9 rounded-xl bg-crimson-600/15 border border-crimson-600/40 flex items-center justify-center text-crimson-300">
                <BarChart3 className="h-5 w-5" />
              </div>
              <div>
                <h2 className="font-display text-sm font-bold text-stone-ink">
                  Money Consumed & Generation Spending Graph
                </h2>
                <p className="text-[11px] text-stone-muted">
                  Tracks your local Design Master runs, image renders, and live
                  OpenRouter key usage
                </p>
              </div>
            </div>

            <div className="flex flex-wrap items-center gap-3">
              <div className="rounded-xl border border-stone-border bg-stone-canvas px-3.5 py-2 text-right">
                <div className="text-[10px] uppercase tracking-wider text-stone-muted">
                  Local Workbench Spend
                </div>
                <div className="font-mono text-base font-bold text-stone-ink">
                  ${(usage?.total_effective_usd || 0).toFixed(4)}
                </div>
              </div>

              {usage?.openrouter_account &&
                typeof usage.openrouter_account.usage_usd === "number" && (
                  <div className="rounded-xl border border-crimson-600/40 bg-crimson-600/10 px-3.5 py-2 text-right">
                    <div className="text-[10px] uppercase tracking-wider text-crimson-300">
                      OpenRouter Key Total Usage
                    </div>
                    <div className="font-mono text-base font-bold text-white">
                      ${usage.openrouter_account.usage_usd.toFixed(4)}
                      {typeof usage.openrouter_account.limit_usd === "number"
                        ? ` / $${usage.openrouter_account.limit_usd.toFixed(2)}`
                        : ""}
                    </div>
                  </div>
                )}

              <div className="rounded-xl border border-stone-border bg-stone-canvas px-3.5 py-2 text-right">
                <div className="text-[10px] uppercase tracking-wider text-stone-muted">
                  Completed Images
                </div>
                <div className="font-mono text-base font-bold text-stone-ink">
                  {usage?.total_images || 0}{" "}
                  <span className="text-xs font-normal text-stone-muted">
                    ({usage?.total_runs || 0} runs)
                  </span>
                </div>
              </div>
            </div>
          </div>

          {/* Daily Spending SVG Bar Chart + Project & Quality Breakdown */}
          <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 pt-1">
            <div className="lg:col-span-2 rounded-xl border border-stone-border bg-stone-canvas p-4 flex flex-col justify-between">
              <div className="flex items-center justify-between mb-3">
                <span className="text-xs font-semibold text-stone-ink">
                  Daily Spend Timeline (USD)
                </span>
                <span className="text-[11px] font-mono text-stone-muted">
                  Peak day: ${maxDailyUsd.toFixed(3)}
                </span>
              </div>

              <div className="h-36 flex items-end gap-2 pt-4 pb-1 px-1 border-b border-stone-border">
                {dailySeries.map((day) => {
                  const pct = Math.max(
                    4,
                    Math.min(100, Math.round((day.spend_usd / maxDailyUsd) * 100))
                  );
                  const shortDate = day.date.slice(5);
                  return (
                    <div
                      key={day.date}
                      className="flex-1 flex flex-col items-center gap-1.5 group"
                      title={`${day.date}: $${day.spend_usd.toFixed(4)} (${day.image_count} images, ${day.run_count} runs)`}
                    >
                      <div className="text-[9px] font-mono text-stone-muted opacity-0 group-hover:opacity-100 transition-opacity">
                        ${day.spend_usd.toFixed(3)}
                      </div>
                      <div className="w-full h-24 flex items-end justify-center">
                        <div
                          className={`w-full max-w-[36px] rounded-t-md transition-all ${
                            day.spend_usd > 0
                              ? "bg-gradient-to-t from-[#C92746] to-[#FFA2B2] shadow-[0_0_12px_rgba(201,39,70,0.4)]"
                              : "bg-stone-border/50"
                          }`}
                          style={{ height: `${pct}%` }}
                        />
                      </div>
                      <span className="text-[10px] font-mono text-stone-muted">
                        {shortDate}
                      </span>
                    </div>
                  );
                })}
              </div>

              <div className="flex items-center justify-between pt-2.5 text-[11px] text-stone-muted">
                <span>
                  Includes planner vision tokens + image generation tokens
                </span>
                <span>
                  Local Spend Cap:{" "}
                  <strong className="text-stone-ink">
                    {spendLimitUsd ? `$${spendLimitUsd}` : "Unlimited"}
                  </strong>
                </span>
              </div>
            </div>

            <div className="rounded-xl border border-stone-border bg-stone-canvas p-4 space-y-3 flex flex-col justify-between">
              <div>
                <div className="text-xs font-semibold text-stone-ink mb-2">
                  Spend by Project
                </div>
                <div className="space-y-1.5 max-h-28 overflow-y-auto pr-1">
                  {(usage?.by_project || []).map((proj) => (
                    <div
                      key={proj.project_id}
                      className="flex items-center justify-between text-xs rounded-lg bg-stone-surface px-2.5 py-1.5 border border-stone-border"
                    >
                      <span className="truncate font-medium text-stone-ink max-w-[150px]">
                        {proj.project_name}
                      </span>
                      <span className="font-mono text-[11px] text-stone-muted shrink-0">
                        ${proj.spend_usd.toFixed(3)} · {proj.image_count} imgs
                      </span>
                    </div>
                  ))}
                </div>
              </div>

              <div className="pt-2 border-t border-stone-border">
                <div className="text-xs font-semibold text-stone-ink mb-1.5">
                  Spend by Quality Tier
                </div>
                <div className="grid grid-cols-2 gap-1.5">
                  {(usage?.by_quality || []).map((q) => (
                    <div
                      key={q.tier}
                      className="rounded-lg border border-stone-border bg-stone-surface px-2.5 py-1 flex items-center justify-between text-[11px]"
                    >
                      <span className="uppercase font-mono font-semibold text-stone-ink">
                        {q.tier}
                      </span>
                      <span className="font-mono text-stone-muted">
                        ${q.spend_usd.toFixed(3)}
                      </span>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* ================================================================= */}
        {/* 2. OPENROUTER API KEY & ORGANIZED DISK STORAGE FOLDER             */}
        {/* ================================================================= */}
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-4">
          {/* API Key Management */}
          <section className="rounded-2xl border border-stone-border bg-stone-surface p-5 space-y-3">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <KeyRound className="h-4 w-4 text-crimson-300" />
                <h2 className="text-sm font-bold text-stone-ink">
                  OpenRouter API Key
                </h2>
              </div>
              <span className="text-[11px] text-stone-muted">
                Status:{" "}
                <strong className="text-stone-ink">
                  {status?.key_status.configured
                    ? `${status.key_status.masked_hint} (${status.key_status.storage_kind})`
                    : "Not configured"}
                </strong>
              </span>
            </div>
            <p className="text-[11px] text-stone-muted leading-relaxed">
              Powers multimodal UI planning, post-render asset extraction, and
              image generation. Stored in your OS credential store and never
              committed to Git or exposed to browser localStorage.
            </p>
            <div className="flex flex-col sm:flex-row gap-2">
              <input
                type="password"
                value={apiKeyInput}
                onChange={(e) => setApiKeyInput(e.target.value)}
                placeholder="Enter or replace OpenRouter API key (sk-or-v1-...)"
                className="flex-1 rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs font-mono text-stone-ink"
              />
              <button
                type="button"
                onClick={() => handleUpdateKey(false)}
                disabled={!apiKeyInput.trim()}
                className="rounded-lg bg-crimson-600 px-3.5 py-1.5 text-xs font-semibold text-white hover:bg-crimson-700 disabled:opacity-50"
              >
                Save Key
              </button>
              <button
                type="button"
                disabled={testingKey}
                onClick={handleTestApiKey}
                className="inline-flex items-center gap-1 rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs font-medium text-stone-ink hover:bg-stone-canvas"
              >
                <RefreshCw
                  className={`h-3.5 w-3.5 ${testingKey ? "animate-spin" : ""}`}
                />
                <span>{testingKey ? "Testing..." : "Test Key"}</span>
              </button>
              {status?.key_status.configured && (
                <button
                  type="button"
                  onClick={() => handleUpdateKey(true)}
                  className="inline-flex items-center gap-1 rounded-lg border border-red-200 bg-red-50 px-2.5 py-1.5 text-xs font-medium text-red-800 hover:bg-red-100"
                >
                  <Trash2 className="h-3.5 w-3.5" />
                  Remove
                </button>
              )}
            </div>
            <label className="inline-flex items-center gap-2 text-[11px] text-stone-muted cursor-pointer">
              <input
                type="checkbox"
                checked={preferSession}
                onChange={(e) => setPreferSession(e.target.checked)}
                className="rounded border-stone-border text-crimson-300"
              />
              <span>
                Store in session memory only (cleared when server exits)
              </span>
            </label>
          </section>

          {/* Local Storage Folder & Organized Hierarchy */}
          <section className="rounded-2xl border border-stone-border bg-stone-surface p-5 space-y-3">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-2">
                <Database className="h-4 w-4 text-crimson-300" />
                <h2 className="text-sm font-bold text-stone-ink">
                  Organized Projects & Threads Storage Path
                </h2>
              </div>
              <button
                type="button"
                onClick={handleBackupDb}
                className="inline-flex items-center gap-1 rounded-lg border border-stone-border bg-white px-2.5 py-1 text-[11px] font-medium text-stone-ink hover:bg-stone-canvas"
              >
                <CheckCircle2 className="h-3.5 w-3.5 text-emerald-500" />
                Backup SQLite DB
              </button>
            </div>
            <p className="text-[11px] text-stone-muted leading-relaxed">
              Every project is automatically saved in its own folder with
              threads and tabs inside (`images/*.png` + `prompts.md` per tab):
            </p>
            <div className="flex flex-col sm:flex-row gap-2">
              <input
                type="text"
                value={customDataDir}
                onChange={(e) => setCustomDataDir(e.target.value)}
                className="flex-1 rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs font-mono text-stone-ink"
              />
              <button
                type="button"
                disabled={pickingFolder}
                onClick={handlePickNativeFolder}
                className="inline-flex items-center justify-center gap-1.5 rounded-lg bg-crimson-600 px-3.5 py-1.5 text-xs font-semibold text-white hover:bg-crimson-700 disabled:opacity-50 shrink-0"
              >
                <FolderOpen className="h-3.5 w-3.5" />
                <span>
                  {pickingFolder ? "Opening..." : "Choose Folder..."}
                </span>
              </button>
            </div>
            <div className="rounded-lg border border-stone-border bg-stone-canvas px-3 py-1.5 text-[10px] font-mono text-stone-muted">
              {status?.data_dir}/projects/&lt;project&gt;/threads/&lt;thread&gt;/tabs/&lt;tab&gt;/
              &#123;images/, prompts.md&#125;
            </div>
          </section>
        </div>

        {/* ================================================================= */}
        {/* 3. GENERATION QUALITY PRICING TIERS (WEB-RESEARCHED + LIVE)       */}
        {/* ================================================================= */}
        <section className="rounded-2xl border border-stone-border bg-stone-surface p-5 space-y-3">
          <div className="flex flex-wrap items-center justify-between gap-2">
            <div className="flex items-center gap-2">
              <DollarSign className="h-4 w-4 text-crimson-300" />
              <h2 className="text-sm font-bold text-stone-ink">
                Image Generation Quality Tiers & Per-Image Pricing
              </h2>
            </div>
            <span className="text-[11px] text-stone-muted">
              Click any tier below to set your default generation quality (
              <strong className="text-stone-ink">Medium</strong> is default)
            </span>
          </div>

          <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4 gap-3">
            {QUALITY_TIER_INFO.map((tier) => {
              const livePrice =
                qualityPrices[tier.id] ?? tier.defaultPriceUsd;
              const isSelected = defaultQuality === tier.id;
              return (
                <button
                  key={tier.id}
                  type="button"
                  onClick={() => setDefaultQuality(tier.id)}
                  className={`text-left rounded-xl border p-3.5 transition flex flex-col justify-between gap-2 ${
                    isSelected
                      ? "border-crimson-600 bg-crimson-600/15 shadow-[0_0_20px_rgba(201,39,70,0.2)]"
                      : "border-stone-border bg-stone-canvas hover:border-crimson-600/50"
                  }`}
                >
                  <div>
                    <div className="flex items-center justify-between gap-1">
                      <span className="text-xs font-bold text-stone-ink">
                        {tier.label}
                      </span>
                      <span
                        className={`rounded px-1.5 py-0.5 text-[9px] font-semibold uppercase ${
                          isSelected
                            ? "bg-crimson-600 text-white"
                            : "bg-stone-surface text-stone-muted border border-stone-border"
                        }`}
                      >
                        {tier.badge}
                      </span>
                    </div>
                    <div className="font-mono text-lg font-bold text-white mt-1">
                      ~${livePrice.toFixed(3)}{" "}
                      <span className="text-[11px] font-normal text-stone-muted">
                        / image
                      </span>
                    </div>
                  </div>
                  <p className="text-[11px] text-stone-muted leading-relaxed">
                    {tier.desc}
                  </p>
                </button>
              );
            })}
          </div>
        </section>

        {/* ================================================================= */}
        {/* 4. AI MODELS, LIVE OPENROUTER COMPATIBILITY & CONCURRENT JOBS     */}
        {/* ================================================================= */}
        <form
          onSubmit={handleSaveSettings}
          className="rounded-2xl border border-stone-border bg-stone-surface p-5 space-y-4"
        >
          <div className="flex flex-wrap items-center justify-between gap-2">
            <div className="flex items-center gap-2">
              <Sliders className="h-4 w-4 text-crimson-300" />
              <div>
                <h2 className="text-sm font-bold text-stone-ink">
                  OpenRouter Models, Live Price Discovery & Concurrent Image
                  Jobs
                </h2>
                <p className="text-[11px] text-stone-muted">
                  Enter any OpenRouter model ID and click{" "}
                  <strong>Test & Fetch Model Pricing</strong> to verify if it
                  supports vision/image output
                </p>
              </div>
            </div>
            <div className="flex items-center gap-2">
              <button
                type="button"
                onClick={handleTestModels}
                disabled={testingModels}
                className="inline-flex items-center gap-1.5 rounded-lg border border-crimson-600/50 bg-crimson-600/20 px-3 py-1.5 text-xs font-semibold text-crimson-300 hover:bg-crimson-600/30"
              >
                <Sparkles className="h-3.5 w-3.5 text-crimson-300" />
                <span>
                  {testingModels
                    ? "Verifying on OpenRouter..."
                    : "Test & Fetch Model Pricing"}
                </span>
              </button>
              <button
                type="button"
                onClick={handleRefreshCapabilities}
                disabled={refreshingCaps}
                className="inline-flex items-center gap-1.5 rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs font-medium text-stone-ink hover:bg-stone-canvas"
              >
                <RefreshCw
                  className={`h-3.5 w-3.5 ${
                    refreshingCaps ? "animate-spin" : ""
                  }`}
                />
                Refresh Metadata
              </button>
            </div>
          </div>

          {modelTestReport && (
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-2.5">
              <div
                className={`rounded-xl border p-3 text-xs ${
                  modelTestReport.planner_ok
                    ? "border-emerald-200 bg-emerald-50 text-emerald-900"
                    : "border-red-200 bg-red-50 text-red-800"
                }`}
              >
                <div className="font-bold flex items-center justify-between">
                  <span>
                    Text / Vision Planner:{" "}
                    {modelTestReport.planner_ok ? "Compatible" : "Incompatible"}
                  </span>
                  <span className="font-mono text-[10px]">
                    Prompt: $
                    {(
                      (modelTestReport.capabilities
                        .planner_prompt_usd_per_token || 0) * 1_000_000
                    ).toFixed(2)}
                    /1M tok
                  </span>
                </div>
                <div className="text-[11px] mt-1">
                  {modelTestReport.planner_message}
                </div>
              </div>
              <div
                className={`rounded-xl border p-3 text-xs ${
                  modelTestReport.image_ok
                    ? "border-emerald-200 bg-emerald-50 text-emerald-900"
                    : "border-red-200 bg-red-50 text-red-800"
                }`}
              >
                <div className="font-bold flex items-center justify-between">
                  <span>
                    Image Generator:{" "}
                    {modelTestReport.image_ok ? "Compatible" : "Incompatible"}
                  </span>
                  <span className="font-mono text-[10px]">
                    Medium: ~$
                    {(
                      modelTestReport.capabilities.quality_pricing_usd?.[
                        "medium"
                      ] ?? 0.021
                    ).toFixed(3)}
                    /img
                  </span>
                </div>
                <div className="text-[11px] mt-1">
                  {modelTestReport.image_message}
                </div>
              </div>
            </div>
          )}

          {caps?.blocked_reasons && caps.blocked_reasons.length > 0 && (
            <div className="rounded-lg border border-red-300 bg-red-50 p-3 text-xs text-red-800 space-y-1">
              <div className="flex items-center gap-1.5 font-bold">
                <ShieldAlert className="h-4 w-4" />
                <span>Selected Model Capability Warnings:</span>
              </div>
              <ul className="list-disc pl-5 space-y-0.5">
                {caps.blocked_reasons.map((b, idx) => (
                  <li key={idx}>{b}</li>
                ))}
              </ul>
            </div>
          )}

          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
            <div>
              <label className="block text-xs font-semibold text-stone-ink mb-1">
                1. Multimodal Text & Vision Model ID (must accept image input)
              </label>
              <input
                type="text"
                value={plannerModel}
                onChange={(e) => setPlannerModel(e.target.value)}
                className="w-full rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs font-mono text-stone-ink"
              />
              <p className="text-[11px] text-stone-muted mt-1">
                Default: <code>google/gemini-3.1-pro-preview</code> · Vision
                input:{" "}
                <strong className="text-stone-ink">
                  {caps?.planner_supports_image_input
                    ? "Verified Multimodal"
                    : "Unverified"}
                </strong>{" "}
                · Input rate: $
                {((caps?.planner_prompt_usd_per_token || 0) * 1_000_000).toFixed(
                  2
                )}
                /1M tokens
              </p>
            </div>

            <div>
              <label className="block text-xs font-semibold text-stone-ink mb-1">
                2. Image Generation & Editing Model ID
              </label>
              <input
                type="text"
                value={imageModel}
                onChange={(e) => setImageModel(e.target.value)}
                className="w-full rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs font-mono text-stone-ink"
              />
              <p className="text-[11px] text-stone-muted mt-1">
                Default: <code>openai/gpt-5-image</code> · Max refs:{" "}
                {caps?.max_input_references ?? 16} · Transparent PNG alpha:{" "}
                <strong className="text-stone-ink">
                  {caps?.supports_transparent_background ? "Supported" : "No"}
                </strong>
              </p>
            </div>
          </div>

          <div className="grid grid-cols-1 sm:grid-cols-3 gap-4">
            <div>
              <label className="block text-xs font-semibold text-stone-ink mb-1">
                Default Generation Quality
              </label>
              <select
                value={defaultQuality}
                onChange={(e) => setDefaultQuality(e.target.value)}
                className="w-full rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs text-stone-ink"
              >
                <option value="low">
                  low (~${(qualityPrices["low"] ?? 0.0076).toFixed(3)}/img)
                </option>
                <option value="medium">
                  medium — Default (~$
                  {(qualityPrices["medium"] ?? 0.0211).toFixed(3)}/img)
                </option>
                <option value="high">
                  high (~${(qualityPrices["high"] ?? 0.0541).toFixed(3)}/img)
                </option>
                <option value="xhigh">
                  xhigh / max (~$
                  {(qualityPrices["xhigh"] ?? 0.2116).toFixed(3)}/img)
                </option>
              </select>
            </div>

            <div>
              <label className="block text-xs font-semibold text-stone-ink mb-1">
                Output Resolution Control
              </label>
              <select
                value={outputSize}
                onChange={(e) => setOutputSize(e.target.value)}
                disabled={!caps?.supports_explicit_resolution}
                className="w-full rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs text-stone-ink disabled:opacity-60"
              >
                {(caps?.resolution_options || ["provider_default"]).map((o) => (
                  <option key={o} value={o}>
                    {o === "provider_default"
                      ? "Provider default (lossless original PNG)"
                      : o}
                  </option>
                ))}
              </select>
            </div>

            <div>
              <label className="block text-xs font-semibold text-stone-ink mb-1">
                Concurrent Image Jobs (1 – 6)
              </label>
              <input
                type="number"
                min={1}
                max={6}
                value={maxConcurrent}
                onChange={(e) =>
                  setMaxConcurrent(
                    Math.max(1, Math.min(6, parseInt(e.target.value || "3", 10)))
                  )
                }
                className="w-full rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs font-mono text-stone-ink"
              />
            </div>
          </div>

          {/* Clear Explanation of Concurrent Image Jobs */}
          <div className="rounded-xl border border-stone-border bg-stone-canvas p-3.5 text-xs space-y-1">
            <div className="font-semibold text-stone-ink flex items-center gap-1.5">
              <Layers className="h-3.5 w-3.5 text-crimson-300" />
              <span>How Concurrent Image Jobs Work in Design Master</span>
            </div>
            <p className="text-[11px] text-stone-muted leading-relaxed">
              When you generate a component, landing page, or Game HUD, Design
              Master first renders the primary visual anchor (`item_1`), and
              then launches <strong>parallel Image-to-Image jobs</strong> up to
              your concurrency limit (`{maxConcurrent}` at once) — for example,
              extracting your <strong>Brand Logo</strong> as a transparent PNG
              in one worker while simultaneously extracting a{" "}
              <strong>3D Clay Plant</strong>, <strong>Guild Crest</strong>, or{" "}
              <strong>Icon Sheet</strong> in another parallel worker, or
              rendering multiple dependent webapp pages concurrently.
            </p>
          </div>

          <div className="grid grid-cols-1 sm:grid-cols-2 gap-4 pt-3 border-t border-stone-border items-center">
            <div>
              <label className="block text-xs font-semibold text-stone-ink mb-1">
                Optional Local Spend Limit (USD)
              </label>
              <input
                type="number"
                step="0.01"
                min="0"
                placeholder="No local cap (e.g. 10.00)"
                value={spendLimitUsd}
                onChange={(e) => setSpendLimitUsd(e.target.value)}
                className="w-full rounded-lg border border-stone-border bg-white px-3 py-1.5 text-xs font-mono text-stone-ink"
              />
              <p className="text-[11px] text-stone-muted mt-1">
                Blocks new runs if your cumulative local workbench spend exceeds
                this cap.
              </p>
            </div>

            <div className="space-y-2">
              <label className="flex items-center gap-2 text-xs text-stone-ink cursor-pointer">
                <input
                  type="checkbox"
                  checked={includeExtraction}
                  onChange={(e) => setIncludeExtraction(e.target.checked)}
                  className="rounded border-stone-border text-crimson-300"
                />
                <span>
                  Include{" "}
                  <strong>
                    Automatic Transparent Asset Extraction & SVG Vectorization
                  </strong>{" "}
                  instructions by default in coding-agent handoffs
                </span>
              </label>
            </div>
          </div>

          <div className="flex items-center justify-end pt-2">
            <button
              type="submit"
              className="inline-flex items-center gap-1.5 rounded-xl bg-crimson-600 px-5 py-2.5 text-xs font-bold text-white hover:bg-crimson-700 shadow-sm"
            >
              <Save className="h-3.5 w-3.5" />
              <span>Save All Settings</span>
            </button>
          </div>
        </form>
      </main>
    </div>
  );
}

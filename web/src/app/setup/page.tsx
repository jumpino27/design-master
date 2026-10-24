"use client";

import React, { useEffect, useState } from "react";
import { useRouter } from "next/navigation";
import {
  CheckCircle2,
  Eye,
  EyeOff,
  FolderOpen,
  KeyRound,
  RefreshCw,
  ShieldCheck,
  Sparkles,
  AlertTriangle,
  ClipboardPaste,
  ArrowRight,
  ArrowLeft,
  Cpu,
  Image as ImageIcon,
  FlaskConical,
  FolderSearch,
  ChevronRight,
  CornerLeftUp,
} from "lucide-react";
import { apiFetch, SetupStatusDto } from "@/lib/api";

const RECOMMENDED_TEXT_MODELS = [
  {
    id: "google/gemini-3.1-pro-preview",
    label: "Gemini 3.1 Pro Preview (Recommended)",
    why: "State-of-the-art multimodal vision + UI hierarchy & post-render asset audit",
  },
  {
    id: "google/gemini-2.5-flash",
    label: "Gemini 2.5 Flash",
    why: "Fast, low-cost UI screenshot comprehension & structured output",
  },
  {
    id: "openai/gpt-4o-mini",
    label: "GPT-4o Mini",
    why: "Reliable multimodal vision & layout analysis",
  },
  {
    id: "anthropic/claude-3.5-sonnet",
    label: "Claude 3.5 Sonnet",
    why: "High-precision UI hierarchy & design critique",
  },
];

const RECOMMENDED_IMAGE_MODELS = [
  {
    id: "openai/gpt-5-image",
    label: "GPT-5 Image (Recommended)",
    why: "Default (~$0.021/img on Medium), crisp UI text, multi-image continuity & transparent PNG assets",
  },
  {
    id: "openai/gpt-image-2.5-sunburst",
    label: "GPT Image 2.5 Sunburst",
    why: "High-speed UI layout & transparent asset extraction",
  },
  {
    id: "openai/gpt-image-1",
    label: "GPT Image 1",
    why: "High-detail UI layout & graphic rendering",
  },
  {
    id: "black-forest-labs/flux.2-pro",
    label: "FLUX.2 Pro",
    why: "Editorial illustrations, textures & brand imagery",
  },
];

export default function SetupPage() {
  const router = useRouter();
  const [step, setStep] = useState<1 | 2 | 3>(1);
  const [status, setStatus] = useState<SetupStatusDto | null>(null);
  const [loading, setLoading] = useState(true);

  // Step 1: OpenRouter API Key state
  const [apiKeyInput, setApiKeyInput] = useState("");
  const [showKey, setShowKey] = useState(false);
  const [preferSessionOnly, setPreferSessionOnly] = useState(false);
  const [testingKey, setTestingKey] = useState(false);
  const [keyActionMsg, setKeyActionMsg] = useState<{
    type: "success" | "error" | "info";
    text: string;
  } | null>(null);

  // Step 2: Models state
  const [plannerModel, setPlannerModel] = useState("google/gemini-3.1-pro-preview");
  const [imageModel, setImageModel] = useState("openai/gpt-5-image");
  const [defaultQuality, setDefaultQuality] = useState("medium");
  const [testingModels, setTestingModels] = useState(false);
  const [modelTestResult, setModelTestResult] = useState<{
    planner_ok: boolean;
    planner_multimodal: boolean;
    planner_message: string;
    image_ok: boolean;
    image_message: string;
  } | null>(null);

  // Step 3: Storage Folder state
  const [customDirInput, setCustomDirInput] = useState("");
  const [pickingNativeFolder, setPickingNativeFolder] = useState(false);
  const [showBrowserPicker, setShowBrowserPicker] = useState(false);
  const [browserDirs, setBrowserDirs] = useState<{
    current_dir: string;
    parent_dir: string | null;
    subdirectories: Array<{ name: string; path: string }>;
  } | null>(null);
  const [dirMsg, setDirMsg] = useState<{
    type: "success" | "error";
    text: string;
  } | null>(null);
  const [finishing, setFinishing] = useState(false);

  useEffect(() => {
    apiFetch<SetupStatusDto>("/api/setup/status")
      .then((s) => {
        setStatus(s);
        setPlannerModel(s.settings.planner_model);
        setImageModel(s.settings.image_model);
        setDefaultQuality(s.settings.default_quality);
        setCustomDirInput(s.data_dir);
      })
      .finally(() => setLoading(false));
  }, []);

  async function handleSaveKey() {
    if (!apiKeyInput.trim()) {
      setKeyActionMsg({
        type: "error",
        text: "Please enter an OpenRouter API key first.",
      });
      return;
    }
    setKeyActionMsg(null);
    try {
      const res = await apiFetch<{
        key_status: SetupStatusDto["key_status"];
      }>("/api/setup/key", {
        method: "POST",
        body: JSON.stringify({
          api_key: apiKeyInput.trim(),
          prefer_session_only: preferSessionOnly,
          validate_online: false,
        }),
      });
      setApiKeyInput("");
      setStatus((prev) =>
        prev ? { ...prev, key_status: res.key_status } : prev
      );
      setKeyActionMsg({
        type: "success",
        text: `Key saved (${res.key_status.masked_hint}). ${res.key_status.persistence_note}`,
      });
    } catch (err: unknown) {
      setKeyActionMsg({
        type: "error",
        text: err instanceof Error ? err.message : "Failed to save key",
      });
    }
  }

  async function handleTestConnection() {
    setTestingKey(true);
    setKeyActionMsg(null);
    try {
      const res = await apiFetch<{
        validation: { valid: boolean; message: string };
        capabilities: SetupStatusDto["capabilities"];
      }>("/api/setup/test-key", {
        method: "POST",
        body: JSON.stringify({
          api_key: apiKeyInput.trim() || undefined,
        }),
      });
      if (apiKeyInput.trim() && res.validation.valid) {
        const saved = await apiFetch<{
          key_status: SetupStatusDto["key_status"];
        }>("/api/setup/key", {
          method: "POST",
          body: JSON.stringify({
            api_key: apiKeyInput.trim(),
            prefer_session_only: preferSessionOnly,
            validate_online: false,
          }),
        });
        setApiKeyInput("");
        setStatus((prev) =>
          prev
            ? {
                ...prev,
                key_status: saved.key_status,
                capabilities: res.capabilities,
              }
            : prev
        );
      }
      setKeyActionMsg({
        type: res.validation.valid ? "success" : "error",
        text: res.validation.message,
      });
    } catch (err: unknown) {
      setKeyActionMsg({
        type: "error",
        text: err instanceof Error ? err.message : "Connection test failed",
      });
    } finally {
      setTestingKey(false);
    }
  }

  async function handleTestModels() {
    setTestingModels(true);
    setModelTestResult(null);
    try {
      const res = await apiFetch<{
        planner_ok: boolean;
        planner_multimodal: boolean;
        planner_message: string;
        image_ok: boolean;
        image_message: string;
        capabilities: SetupStatusDto["capabilities"];
      }>("/api/setup/test-models", {
        method: "POST",
        body: JSON.stringify({
          planner_model: plannerModel.trim(),
          image_model: imageModel.trim(),
        }),
      });
      setModelTestResult(res);
      setStatus((prev) =>
        prev ? { ...prev, capabilities: res.capabilities } : prev
      );
    } catch (err: unknown) {
      setModelTestResult({
        planner_ok: false,
        planner_multimodal: false,
        planner_message:
          err instanceof Error ? err.message : "Model check failed",
        image_ok: false,
        image_message: "Could not verify image model.",
      });
    } finally {
      setTestingModels(false);
    }
  }

  async function handlePickNativeFolder() {
    setPickingNativeFolder(true);
    setDirMsg(null);
    try {
      const res = await apiFetch<{
        cancelled: boolean;
        data_dir: string | null;
        note?: string;
      }>("/api/setup/pick-folder", {
        method: "POST",
        body: JSON.stringify({}),
      });
      if (!res.cancelled && res.data_dir) {
        setCustomDirInput(res.data_dir);
        setDirMsg({
          type: "success",
          text: `Selected folder: ${res.data_dir}`,
        });
      }
    } catch (err: unknown) {
      setDirMsg({
        type: "error",
        text:
          err instanceof Error
            ? err.message
            : "Native folder dialog unavailable; use Browse Folders below.",
      });
    } finally {
      setPickingNativeFolder(false);
    }
  }

  async function loadBrowseDirs(targetPath?: string) {
    try {
      const q = targetPath
        ? `?path=${encodeURIComponent(targetPath)}`
        : `?path=${encodeURIComponent(customDirInput)}`;
      const res = await apiFetch<{
        current_dir: string;
        parent_dir: string | null;
        subdirectories: Array<{ name: string; path: string }>;
      }>(`/api/setup/browse-dirs${q}`);
      setBrowserDirs(res);
      setShowBrowserPicker(true);
    } catch (err: unknown) {
      setDirMsg({
        type: "error",
        text: err instanceof Error ? err.message : "Failed to list directories",
      });
    }
  }

  async function handleApplyFolder(folderPath: string) {
    setDirMsg(null);
    try {
      const res = await apiFetch<{
        validated: boolean;
        data_dir: string;
        note: string;
      }>("/api/setup/data-dir", {
        method: "POST",
        body: JSON.stringify({ data_dir: folderPath }),
      });
      setCustomDirInput(res.data_dir);
      setShowBrowserPicker(false);
      setDirMsg({
        type: "success",
        text: `Folder verified & saved: ${res.data_dir}`,
      });
    } catch (err: unknown) {
      setDirMsg({
        type: "error",
        text: err instanceof Error ? err.message : "Invalid folder path",
      });
    }
  }

  async function handleFinishSetup() {
    setFinishing(true);
    try {
      if (customDirInput.trim() && customDirInput.trim() !== status?.data_dir) {
        await apiFetch("/api/setup/data-dir", {
          method: "POST",
          body: JSON.stringify({ data_dir: customDirInput.trim() }),
        });
      }
      await apiFetch("/api/setup/complete", {
        method: "POST",
        body: JSON.stringify({
          planner_model: plannerModel.trim(),
          image_model: imageModel.trim(),
          default_quality: defaultQuality,
          test_adapter_enabled: false,
        }),
      });
      router.push("/projects");
    } finally {
      setFinishing(false);
    }
  }

  if (loading) {
    return (
      <div className="h-screen w-screen flex items-center justify-center bg-stone-canvas text-stone-muted text-sm">
        Loading Design Master setup...
      </div>
    );
  }

  return (
    <div className="h-screen w-screen overflow-hidden bg-stone-canvas flex flex-col">
      {/* Compact Top Bar */}
      <header className="h-12 shrink-0 bg-charcoal text-white px-5 flex items-center justify-between border-b border-charcoal-border">
        <div className="flex items-center gap-2.5">
          <img
            src="/dm-logo.png"
            alt="Design Master DM Logo"
            className="h-7 w-7 rounded-md border border-charcoal-border object-cover"
          />
          <span className="font-display text-sm font-bold tracking-tight">
            Design Master
          </span>
          <span className="rounded bg-burnt/20 px-1.5 py-0.5 text-[10px] font-mono uppercase tracking-wider text-purple-200 border border-burnt/40">
            FIRST-TIME SETUP
          </span>
        </div>

        <button
          type="button"
          onClick={() => router.push("/projects")}
          className="inline-flex items-center gap-1.5 rounded-lg border border-charcoal-border bg-charcoal-surface px-3 py-1.5 text-xs font-semibold text-stone-100 hover:bg-charcoal-hover hover:text-white transition-colors"
        >
          Explore 2 Built-In Showcase Projects →
        </button>
      </header>

      {/* Centered 3-Step Modal Container (Fits cleanly in viewport) */}
      <main className="flex-1 flex flex-col items-center justify-center p-4 overflow-y-auto">
        <div className="w-full max-w-2xl rounded-2xl border border-stone-border bg-stone-surface shadow-modal overflow-hidden flex flex-col">
          {/* Wizard Stepper Header */}
          <div className="bg-charcoal px-6 py-4 text-white border-b border-charcoal-border space-y-3.5">
            <div className="flex items-center gap-3">
              <img
                src="/dm-logo.png"
                alt="DM"
                className="h-10 w-10 rounded-xl border border-burnt/40 object-cover shadow-sm shrink-0"
              />
              <div>
                <h1 className="font-display text-base font-bold tracking-tight text-white">
                  Welcome to Design Master (DM)
                </h1>
                <p className="text-xs text-stone-200 mt-0.5">
                  3-step setup for your dark-mode AI UI, Game HUD & Agent Handoff Studio
                </p>
              </div>
            </div>

            <div className="grid grid-cols-3 gap-2.5">
              {[
                { n: 1, label: "OpenRouter Key" },
                { n: 2, label: "Models & Quality" },
                { n: 3, label: "Save Path" },
              ].map((s) => {
                const isActive = step === s.n;
                const isDone = step > s.n;
                return (
                  <button
                    key={s.n}
                    type="button"
                    onClick={() => setStep(s.n as 1 | 2 | 3)}
                    className={`flex items-center justify-center gap-2 rounded-xl border px-3 py-2 text-xs font-semibold whitespace-nowrap transition-all ${
                      isActive
                        ? "border-burnt bg-burnt text-white shadow-[0_0_18px_rgba(124,58,237,0.38)]"
                        : isDone
                        ? "border-emerald-500/45 bg-emerald-500/15 text-emerald-300 hover:bg-emerald-500/25"
                        : "border-[#2B3350] bg-[#161B2C] text-stone-100 hover:border-burnt/50 hover:bg-[#1E253B] hover:text-white"
                    }`}
                  >
                    <span
                      className={`inline-flex h-5 w-5 items-center justify-center rounded-full text-[11px] font-bold shrink-0 ${
                        isActive
                          ? "bg-white/20 text-white"
                          : isDone
                          ? "bg-emerald-500/25 text-emerald-200"
                          : "bg-white/10 text-stone-200"
                      }`}
                    >
                      {s.n}
                    </span>
                    <span>{s.label}</span>
                  </button>
                );
              })}
            </div>
          </div>

          {/* =============================================================== */}
          {/* MODAL STEP 1: OPENROUTER API KEY                                */}
          {/* =============================================================== */}
          {step === 1 && (
            <div className="p-6 space-y-4">
              <div className="flex items-start justify-between gap-3">
                <div className="flex items-center gap-2.5">
                  <div className="h-9 w-9 rounded-xl bg-burnt/20 border border-burnt/40 text-purple-200 flex items-center justify-center shrink-0">
                    <KeyRound className="h-5 w-5" />
                  </div>
                  <div>
                    <h2 className="text-base font-bold text-white">
                      Step 1: Connect Your OpenRouter API Key
                    </h2>
                    <p className="text-xs text-stone-200">
                      One unified key powers both visual planning and image
                      generation
                    </p>
                  </div>
                </div>
                <span
                  className={`rounded-full px-3 py-1 text-xs font-semibold shrink-0 border ${
                    status?.key_status.configured
                      ? "bg-emerald-500/15 border-emerald-500/40 text-emerald-300"
                      : "bg-amber-500/15 border-amber-500/40 text-amber-200"
                  }`}
                >
                  {status?.key_status.configured
                    ? `Saved (${status.key_status.masked_hint})`
                    : "Optional · 2 Showcase Projects Ready"}
                </span>
              </div>

              {/* Clear Explanation Box */}
              <div className="rounded-xl border border-stone-border bg-white p-3.5 text-xs text-stone-100 space-y-1.5 leading-relaxed">
                <div className="font-bold text-white flex items-center gap-1.5">
                  <ShieldCheck className="h-4 w-4 text-burnt" />
                  <span>What does Design Master use OpenRouter for?</span>
                </div>
                <p className="text-stone-200">
                  1. <strong className="text-white">Multimodal Visual Planning</strong>: Analyzes your
                  pasted UI screenshots, brand colors, and logos to plan
                  component variations, multi-page webapps, or game HUDs.
                </p>
                <p className="text-stone-200">
                  2. <strong className="text-white">High-Craft Image Generation</strong>: Renders the
                  actual design mockups and transparent PNG assets directly to your disk.
                </p>
                <p className="text-[11px] text-stone-200 pt-1.5 border-t border-stone-border">
                  🔒 <strong className="text-white">Privacy guarantee</strong>: Your key is stored only
                  in your OS credential store (never in browser localStorage or
                  exported files).
                </p>
              </div>

              {/* Key Input & Actions */}
              <div className="space-y-2.5">
                <label className="block text-xs font-semibold text-white">
                  OpenRouter API Key (`sk-or-v1-...`)
                </label>
                <div className="flex flex-col sm:flex-row gap-2">
                  <div className="relative flex-1">
                    <input
                      type={showKey ? "text" : "password"}
                      value={apiKeyInput}
                      onChange={(e) => setApiKeyInput(e.target.value)}
                      placeholder={
                        status?.key_status.masked_hint
                          ? `Stored: ${status.key_status.masked_hint} (paste new key to replace)`
                          : "sk-or-v1-..."
                      }
                      className="w-full rounded-xl border border-stone-border bg-[#0C0F1A] px-3 py-2 pr-20 text-xs font-mono text-white focus:border-burnt focus:outline-none"
                    />
                    <div className="absolute right-2 top-1/2 -translate-y-1/2 flex items-center gap-1">
                      <button
                        type="button"
                        onClick={async () => {
                          try {
                            const txt = await navigator.clipboard.readText();
                            if (txt) setApiKeyInput(txt.trim());
                          } catch {
                            // ignore
                          }
                        }}
                        title="Paste from clipboard"
                        className="rounded p-1 text-stone-200 hover:bg-charcoal-hover hover:text-white"
                      >
                        <ClipboardPaste className="h-3.5 w-3.5" />
                      </button>
                      <button
                        type="button"
                        onClick={() => setShowKey((s) => !s)}
                        title={showKey ? "Hide API key" : "Show API key"}
                        className="rounded p-1 text-stone-200 hover:bg-charcoal-hover hover:text-white"
                      >
                        {showKey ? (
                          <EyeOff className="h-3.5 w-3.5" />
                        ) : (
                          <Eye className="h-3.5 w-3.5" />
                        )}
                      </button>
                    </div>
                  </div>

                  <button
                    type="button"
                    onClick={handleSaveKey}
                    className="rounded-xl border border-[#323C5E] bg-[#1A2035] px-4 py-2 text-xs font-semibold text-white hover:bg-[#232B47] hover:border-burnt transition-colors shrink-0"
                  >
                    Save key
                  </button>

                  <button
                    type="button"
                    disabled={testingKey}
                    onClick={handleTestConnection}
                    className="inline-flex items-center justify-center gap-1.5 rounded-xl border border-burnt/50 bg-burnt/20 px-4 py-2 text-xs font-semibold text-purple-100 hover:bg-burnt/30 disabled:opacity-50 transition-colors shrink-0"
                  >
                    <RefreshCw
                      className={`h-3.5 w-3.5 ${
                        testingKey ? "animate-spin" : ""
                      }`}
                    />
                    <span>
                      {testingKey ? "Verifying..." : "Test key (free)"}
                    </span>
                  </button>
                </div>

                <label className="inline-flex items-center gap-2 text-xs text-stone-200 cursor-pointer">
                  <input
                    type="checkbox"
                    checked={preferSessionOnly}
                    onChange={(e) => setPreferSessionOnly(e.target.checked)}
                    className="rounded border-stone-border text-burnt focus:ring-burnt"
                  />
                  <span>
                    Keep key in session-only memory (cleared when server stops)
                  </span>
                </label>

                {keyActionMsg && (
                  <div
                    role="status"
                    className={`rounded-lg px-3 py-2 text-xs font-medium ${
                      keyActionMsg.type === "success"
                        ? "bg-emerald-50 text-emerald-900 border border-emerald-200"
                        : "bg-red-50 text-red-900 border border-red-200"
                    }`}
                  >
                    {keyActionMsg.text}
                  </div>
                )}
              </div>

              {/* Step 1 Footer */}
              <div className="flex items-center justify-between pt-3 border-t border-stone-border">
                <span className="text-[11px] text-stone-200">
                  No key yet? Click <strong className="text-white">Next</strong> to explore the 2 built-in showcase projects first.
                </span>
                <button
                  type="button"
                  onClick={() => setStep(2)}
                  data-testid="setup-next-step-1"
                  className="inline-flex items-center gap-1.5 rounded-xl bg-burnt px-5 py-2.5 text-xs font-bold text-white shadow-sm hover:bg-burnt-hover transition-colors"
                >
                  <span>Next: Choose AI Models</span>
                  <ArrowRight className="h-3.5 w-3.5" />
                </button>
              </div>
            </div>
          )}

          {/* =============================================================== */}
          {/* MODAL STEP 2: MULTIMODAL TEXT MODEL, IMAGE MODEL & TEST MODE    */}
          {/* =============================================================== */}
          {step === 2 && (
            <div className="p-6 space-y-4">
              <div className="flex items-center justify-between gap-2">
                <div className="flex items-center gap-2.5">
                  <div className="h-9 w-9 rounded-xl bg-burnt/20 border border-burnt/40 text-purple-200 flex items-center justify-center shrink-0">
                    <Cpu className="h-5 w-5" />
                  </div>
                  <div>
                    <h2 className="text-base font-bold text-white">
                      Step 2: Pick Your Text & Image Models
                    </h2>
                    <p className="text-xs text-stone-200">
                      Choose from recommended presets or enter any OpenRouter
                      model ID
                    </p>
                  </div>
                </div>

                <button
                  type="button"
                  disabled={testingModels}
                  onClick={handleTestModels}
                  data-testid="test-models-button"
                  className="inline-flex items-center gap-1.5 rounded-xl border border-burnt/50 bg-burnt/20 px-3.5 py-2 text-xs font-semibold text-purple-100 hover:bg-burnt/30 disabled:opacity-50 shrink-0 transition-colors"
                >
                  <RefreshCw
                    className={`h-3.5 w-3.5 ${
                      testingModels ? "animate-spin" : ""
                    }`}
                  />
                  <span>
                    {testingModels
                      ? "Testing models..."
                      : "Test Selected Models"}
                  </span>
                </button>
              </div>

              {/* 1. Multimodal Text / Planner Model */}
              <div className="rounded-xl border border-stone-border bg-white p-3.5 space-y-2.5">
                <div className="flex items-center justify-between gap-2">
                  <label className="text-xs font-bold text-white flex items-center gap-1.5">
                    <Sparkles className="h-3.5 w-3.5 text-burnt" />
                    <span>
                      1. Multimodal Text & Vision Model (Must support image
                      input)
                    </span>
                  </label>
                  <span className="text-[10px] font-semibold uppercase tracking-wider text-purple-200 bg-burnt/20 border border-burnt/40 px-2 py-0.5 rounded shrink-0">
                    Vision + JSON Required
                  </span>
                </div>
                <p className="text-[11px] text-stone-200 leading-relaxed">
                  <strong className="text-white">Why must it be multimodal?</strong> This model looks
                  at your pasted UI screenshots and brand logos to write the
                  design plan. Pick a preset below or type any multimodal
                  OpenRouter model ID:
                </p>
                <div className="flex flex-wrap gap-1.5">
                  {RECOMMENDED_TEXT_MODELS.map((m) => (
                    <button
                      key={m.id}
                      type="button"
                      onClick={() => setPlannerModel(m.id)}
                      title={m.why}
                      className={`rounded-lg border px-2.5 py-1.5 text-[11px] transition-colors ${
                        plannerModel === m.id
                          ? "border-burnt bg-burnt text-white font-semibold shadow-sm"
                          : "border-[#2B3350] bg-[#141829] text-stone-100 hover:border-burnt/60 hover:text-white"
                      }`}
                    >
                      {m.label}
                    </button>
                  ))}
                </div>
                <input
                  type="text"
                  value={plannerModel}
                  onChange={(e) => setPlannerModel(e.target.value)}
                  placeholder="e.g. google/gemini-3.1-pro-preview or google/gemini-2.5-flash"
                  className="w-full rounded-lg border border-stone-border bg-[#0C0F1A] px-3 py-2 text-xs font-mono text-white focus:border-burnt focus:outline-none"
                />
              </div>

              {/* 2. Image Generation Model */}
              <div className="rounded-xl border border-stone-border bg-white p-3.5 space-y-2.5">
                <div className="flex items-center justify-between gap-2">
                  <label className="text-xs font-bold text-white flex items-center gap-1.5">
                    <ImageIcon className="h-3.5 w-3.5 text-burnt" />
                    <span>2. Image Generation & Editing Model</span>
                  </label>
                  <select
                    value={defaultQuality}
                    onChange={(e) => setDefaultQuality(e.target.value)}
                    aria-label="Default Generation Quality"
                    className="rounded-lg border border-stone-border bg-[#0C0F1A] px-2.5 py-1 text-[11px] font-medium text-white"
                  >
                    <option value="low">Quality: Low (~$0.007 / img)</option>
                    <option value="medium">
                      Quality: Medium — Default (~$0.021 / img)
                    </option>
                    <option value="high">Quality: High (~$0.054 / img)</option>
                    <option value="xhigh">
                      Quality: Max / X-High (~$0.211 / img)
                    </option>
                  </select>
                </div>
                <p className="text-[11px] text-stone-200 leading-relaxed">
                  <strong className="text-white">Which image model should I pick?</strong> Use{" "}
                  <code className="text-purple-200 font-mono">openai/gpt-5-image</code> (Default) for crisp UI text,
                  multi-page continuity, and automatic transparent PNG asset
                  extraction, or type any OpenRouter image model ID and click{" "}
                  <strong className="text-white">Test Selected Models</strong> to verify compatibility
                  and fetch live pricing:
                </p>
                <div className="flex flex-wrap gap-1.5">
                  {RECOMMENDED_IMAGE_MODELS.map((m) => (
                    <button
                      key={m.id}
                      type="button"
                      onClick={() => setImageModel(m.id)}
                      title={m.why}
                      className={`rounded-lg border px-2.5 py-1.5 text-[11px] transition-colors ${
                        imageModel === m.id
                          ? "border-burnt bg-burnt text-white font-semibold shadow-sm"
                          : "border-[#2B3350] bg-[#141829] text-stone-100 hover:border-burnt/60 hover:text-white"
                      }`}
                    >
                      {m.label}
                    </button>
                  ))}
                </div>
                <input
                  type="text"
                  value={imageModel}
                  onChange={(e) => setImageModel(e.target.value)}
                  placeholder="e.g. openai/gpt-5-image"
                  className="w-full rounded-lg border border-stone-border bg-[#0C0F1A] px-3 py-2 text-xs font-mono text-white focus:border-burnt focus:outline-none"
                />
              </div>

              {/* Live Model Test Results */}
              {modelTestResult && (
                <div className="rounded-xl border border-stone-border bg-white p-3 space-y-1.5 text-xs">
                  <div
                    className={`flex items-center gap-2 font-medium ${
                      modelTestResult.planner_ok
                        ? "text-emerald-800"
                        : "text-red-800"
                    }`}
                  >
                    {modelTestResult.planner_ok ? (
                      <CheckCircle2 className="h-4 w-4 text-emerald-400 shrink-0" />
                    ) : (
                      <AlertTriangle className="h-4 w-4 text-red-400 shrink-0" />
                    )}
                    <span>{modelTestResult.planner_message}</span>
                  </div>
                  <div
                    className={`flex items-center gap-2 font-medium ${
                      modelTestResult.image_ok
                        ? "text-emerald-800"
                        : "text-red-800"
                    }`}
                  >
                    {modelTestResult.image_ok ? (
                      <CheckCircle2 className="h-4 w-4 text-emerald-400 shrink-0" />
                    ) : (
                      <AlertTriangle className="h-4 w-4 text-red-400 shrink-0" />
                    )}
                    <span>{modelTestResult.image_message}</span>
                  </div>
                </div>
              )}

              {/* 3. Built-In Showcase Example Projects Info */}
              <div className="rounded-xl border border-burnt/40 bg-burnt/15 p-3.5 space-y-1">
                <div className="text-xs font-bold text-white flex items-center gap-1.5">
                  <Sparkles className="h-3.5 w-3.5 text-burnt" />
                  <span>
                    2 Built-In Showcase Projects Pre-Loaded & Ready to Explore
                  </span>
                </div>
                <p className="text-[11px] text-stone-200 leading-relaxed">
                  Design Master comes pre-loaded with{" "}
                  <strong className="text-white">Aetheris Chronicles — RPG Game UI & HUD</strong> and{" "}
                  <strong className="text-white">Jumpino View — Creator Discovery Platform</strong>{" "}
                  (including extracted transparent PNG assets and complete AI
                  coding agent handoff prompts) so you can inspect how it works
                  and continue generating inside them right away.
                </p>
              </div>

              {/* Step 2 Footer */}
              <div className="flex items-center justify-between pt-2 border-t border-stone-border">
                <button
                  type="button"
                  onClick={() => setStep(1)}
                  className="inline-flex items-center gap-1.5 rounded-xl border border-[#323C5E] bg-[#1A2035] px-4 py-2 text-xs font-semibold text-white hover:bg-[#232B47] transition-colors"
                >
                  <ArrowLeft className="h-3.5 w-3.5" />
                  <span>Back</span>
                </button>
                <button
                  type="button"
                  onClick={() => setStep(3)}
                  data-testid="setup-next-step-2"
                  className="inline-flex items-center gap-1.5 rounded-xl bg-burnt px-5 py-2.5 text-xs font-bold text-white shadow-sm hover:bg-burnt-hover transition-colors"
                >
                  <span>Next: Storage Folder</span>
                  <ArrowRight className="h-3.5 w-3.5" />
                </button>
              </div>
            </div>
          )}

          {/* =============================================================== */}
          {/* MODAL STEP 3: STORAGE FOLDER PICKER (FINDER / EXPLORER / LINUX) */}
          {/* =============================================================== */}
          {step === 3 && (
            <div className="p-6 space-y-4">
              <div className="flex items-center gap-2.5">
                <div className="h-9 w-9 rounded-xl bg-burnt/20 border border-burnt/40 text-purple-200 flex items-center justify-center shrink-0">
                  <FolderOpen className="h-5 w-5" />
                </div>
                <div>
                  <h2 className="text-base font-bold text-white">
                    Step 3: Where Should Your Projects & Images Be Saved?
                  </h2>
                  <p className="text-xs text-stone-200">
                    Pick a folder using your system file dialog — no manual path
                    typing required
                  </p>
                </div>
              </div>

              <div className="rounded-xl border border-stone-border bg-white p-4 space-y-3">
                <div className="text-xs text-stone-200 leading-relaxed">
                  Design Master automatically organizes every project, thread,
                  and tab inside your chosen directory with a clean folder
                  structure:
                  <code className="block mt-1.5 rounded-lg border border-stone-border bg-[#0C0F1A] px-3 py-2 font-mono text-[11px] text-purple-200">
                    projects/&lt;project-name&gt;/threads/&lt;thread-name&gt;/tabs/&lt;tab-name&gt;/images/*.png
                    + prompts.md
                  </code>
                </div>

                {/* Current Selected Folder Display */}
                <div className="rounded-lg border border-stone-border bg-[#0C0F1A] px-3.5 py-2.5 font-mono text-xs text-white break-all flex items-center justify-between gap-2">
                  <span>{customDirInput}</span>
                  <span className="rounded-md bg-emerald-500/15 border border-emerald-500/40 px-2 py-0.5 font-sans text-[10px] font-semibold text-emerald-300 shrink-0">
                    Writable
                  </span>
                </div>

                {/* Native OS Folder Picker Button + Visual Folder Browser Button */}
                <div className="flex flex-wrap items-center gap-2.5 pt-1">
                  <button
                    type="button"
                    disabled={pickingNativeFolder}
                    onClick={handlePickNativeFolder}
                    className="inline-flex items-center gap-2 rounded-xl bg-burnt px-4 py-2.5 text-xs font-bold text-white shadow-sm hover:bg-burnt-hover disabled:opacity-50 transition-colors"
                  >
                    <FolderSearch className="h-4 w-4 text-white" />
                    <span>
                      {pickingNativeFolder
                        ? "Waiting for system folder dialog..."
                        : "Choose Folder (Finder / Explorer)..."}
                    </span>
                  </button>

                  <button
                    type="button"
                    onClick={() => loadBrowseDirs()}
                    className="inline-flex items-center gap-1.5 rounded-xl border border-[#323C5E] bg-[#1A2035] px-4 py-2.5 text-xs font-semibold text-stone-100 hover:bg-[#232B47] hover:text-white transition-colors"
                  >
                    <FolderOpen className="h-3.5 w-3.5 text-purple-300" />
                    <span>Browse Folders in Window</span>
                  </button>
                </div>

                {/* Visual In-Window Folder Browser (Fallback / Alternative) */}
                {showBrowserPicker && browserDirs && (
                  <div className="rounded-xl border border-stone-border bg-stone-canvas/70 p-3 space-y-2 text-xs">
                    <div className="flex items-center justify-between gap-2">
                      <span className="font-mono text-[11px] font-semibold text-stone-ink truncate">
                        {browserDirs.current_dir}
                      </span>
                      <div className="flex items-center gap-1.5 shrink-0">
                        {browserDirs.parent_dir && (
                          <button
                            type="button"
                            onClick={() =>
                              loadBrowseDirs(browserDirs.parent_dir!)
                            }
                            className="inline-flex items-center gap-1 rounded border border-stone-border bg-white px-2 py-1 text-[11px] font-medium text-stone-ink hover:bg-stone-surface"
                          >
                            <CornerLeftUp className="h-3 w-3" />
                            <span>Up</span>
                          </button>
                        )}
                        <button
                          type="button"
                          onClick={() =>
                            handleApplyFolder(browserDirs.current_dir)
                          }
                          className="rounded bg-burnt px-2.5 py-1 text-[11px] font-bold text-white hover:bg-burnt-hover"
                        >
                          Use This Folder
                        </button>
                      </div>
                    </div>
                    <div className="max-h-36 overflow-y-auto grid grid-cols-2 sm:grid-cols-3 gap-1.5 pt-1">
                      {browserDirs.subdirectories.map((sub) => (
                        <button
                          key={sub.path}
                          type="button"
                          onClick={() => loadBrowseDirs(sub.path)}
                          className="flex items-center gap-1.5 rounded border border-stone-border bg-white px-2 py-1.5 text-left text-[11px] text-stone-ink hover:border-burnt truncate"
                        >
                          <FolderOpen className="h-3.5 w-3.5 text-burnt shrink-0" />
                          <span className="truncate">{sub.name}</span>
                        </button>
                      ))}
                    </div>
                  </div>
                )}

                {dirMsg && (
                  <div
                    className={`rounded-lg px-3 py-2 text-xs font-medium ${
                      dirMsg.type === "success"
                        ? "bg-emerald-50 text-emerald-900 border border-emerald-200"
                        : "bg-red-50 text-red-900 border border-red-200"
                    }`}
                  >
                    {dirMsg.text}
                  </div>
                )}
              </div>

              {/* Step 3 Footer */}
              <div className="flex items-center justify-between pt-3 border-t border-stone-border">
                <button
                  type="button"
                  onClick={() => setStep(2)}
                  className="inline-flex items-center gap-1.5 rounded-lg border border-stone-border bg-white px-4 py-2 text-xs font-semibold text-stone-ink hover:bg-stone-canvas"
                >
                  <ArrowLeft className="h-3.5 w-3.5" />
                  <span>Back</span>
                </button>

                <button
                  type="button"
                  disabled={finishing}
                  onClick={handleFinishSetup}
                  data-testid="finish-setup-button"
                  className="inline-flex items-center gap-2 rounded-xl bg-burnt px-6 py-2.5 text-xs font-bold text-white shadow-sm hover:bg-burnt-hover disabled:opacity-50"
                >
                  <span>
                    {finishing
                      ? "Saving setup..."
                      : "Finish setup & open project library"}
                  </span>
                  <ArrowRight className="h-4 w-4" />
                </button>
              </div>
            </div>
          )}
        </div>
      </main>
    </div>
  );
}

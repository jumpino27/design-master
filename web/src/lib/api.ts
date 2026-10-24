import { z } from "zod";

export const ColorPaletteSchema = z.object({
  primary: z.string().regex(/^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/),
  accent: z.string().regex(/^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/),
  background: z.string().regex(/^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/),
  surface: z.string().regex(/^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/),
  text: z.string().regex(/^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/),
  muted_text: z.string().regex(/^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/),
  border: z.string().regex(/^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/),
  success: z.string().regex(/^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/),
  warning: z.string().regex(/^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/),
  danger: z.string().regex(/^#([0-9a-fA-F]{3}|[0-9a-fA-F]{6}|[0-9a-fA-F]{8})$/),
});

export type ColorPalette = z.infer<typeof ColorPaletteSchema>;

export const DesignStyleSelectionSchema = z.object({
  style_id: z.string().default(""),
  style_number: z.number().default(1),
  style_name: z.string().default(""),
  definition: z.string().default(""),
  category: z.string().default(""),
  visual_rules: z.string().default(""),
  keywords: z.array(z.string()).default([]),
  modifiers: z.array(z.string()).default([]),
  combined_brief: z.string().default(""),
});

export type DesignStyleSelection = z.infer<typeof DesignStyleSelectionSchema>;

export const BrandSpecSchema = z.object({
  brand_personality: z.string(),
  design_direction: z.string(),
  tone_of_voice: z.string(),
  content_language: z.string(),
  ui_density: z.string(),
  logo: z.object({
    mode: z.string(),
    monogram_text: z.string().nullable().optional(),
    primary_asset_id: z.string().nullable().optional(),
    mark_only_asset_id: z.string().nullable().optional(),
    light_variant_asset_id: z.string().nullable().optional(),
    dark_variant_asset_id: z.string().nullable().optional(),
    preferred_usage: z.string(),
    inclusion_policy: z.string(),
  }),
  colors: z.object({
    light: ColorPaletteSchema,
    dark: ColorPaletteSchema,
  }),
  typography: z.object({
    heading_family: z.string(),
    body_family: z.string(),
    mono_family: z.string(),
    fallback_stack: z.string(),
    weights: z.array(z.number()),
    size_scale: z.string(),
    line_heights: z.string(),
    font_source: z.string(),
    uploaded_font_asset_ids: z.array(z.string()).default([]),
  }),
  layout_tokens: z.object({
    spacing_scale: z.string(),
    content_width: z.string(),
    grid_columns: z.string(),
    radii: z.string(),
    borders: z.string(),
    shadows: z.string(),
    icon_style: z.string(),
    button_treatment: z.string(),
  }),
  brand_rules: z.array(z.string()).default([]),
  mandatory_inclusions: z.array(z.string()).default([]),
  forbidden_treatments: z.array(z.string()).default([]),
  approved_reference_asset_ids: z.array(z.string()).default([]),
  target_context: z.object({
    target_stack: z.string(),
    repository_root: z.string(),
    preferred_asset_dir: z.string(),
    component_library_notes: z.string(),
  }),
  design_style: DesignStyleSelectionSchema.nullable().optional(),
});

export type BrandSpec = z.infer<typeof BrandSpecSchema>;

export const PlanItemSchema = z.object({
  id: z.string(),
  kind: z.string(),
  name: z.string(),
  page_route: z.string().nullable().optional(),
  variant_index: z.number(),
  viewport: z.object({
    intent: z.string(),
    css_width: z.number(),
  }),
  aspect_ratio: z.string(),
  requirements_covered: z.array(z.string()).default([]),
  direction: z.string(),
  exact_copy: z.array(z.string()).default([]),
  reference_asset_ids: z.array(z.string()).default([]),
  depends_on: z.array(z.string()).default([]),
  image_prompt: z.string(),
  implementation_notes: z.array(z.string()).default([]),
  behaviors: z.array(z.string()).default([]),
  responsive_rules: z.array(z.string()).default([]),
  asset_instructions: z.array(z.string()).default([]),
  is_visual_anchor: z.boolean().default(false),
  tab_key: z.string().default(""),
  tab_label: z.string().default(""),
  transparent_background: z.boolean().default(false),
  extracted_from_item_id: z.string().nullable().optional(),
  target_existing_output_id: z.string().nullable().optional(),
  quality_override: z.string().nullable().optional(),
});

export type PlanItem = z.infer<typeof PlanItemSchema>;

export const PlannerPlanSchema = z.object({
  schema_version: z.literal(1),
  mode: z.enum(["component", "full_page", "asset"]),
  request_summary: z.string(),
  requirements: z.array(
    z.object({
      id: z.string(),
      description: z.string(),
      source: z.string(),
    })
  ),
  assumptions: z.array(z.string()).default([]),
  shared_design_spec: z.object({
    colors: z.record(z.string()).default({}),
    typography: z.record(z.string()).default({}),
    spacing: z.record(z.string()).default({}),
    radii: z.record(z.string()).default({}),
    shadows: z.record(z.string()).default({}),
    layout_rules: z.array(z.string()).default([]),
    asset_rules: z.array(z.string()).default([]),
  }),
  items: z.array(PlanItemSchema),
  shared_components: z
    .array(
      z.object({
        id: z.string(),
        name: z.string(),
        canonical_item_id: z.string(),
        used_on_routes: z.array(z.string()),
        description: z.string(),
      })
    )
    .default([]),
  page_composition: z
    .array(
      z.object({
        page_name: z.string(),
        route: z.string(),
        viewport_targets: z.array(z.string()),
        ordered_sections: z.array(z.string()),
        shared_component_ids: z.array(z.string()),
        required_ctas: z.array(z.string()),
        states: z.array(z.string()),
      })
    )
    .default([]),
});

export type PlannerPlan = z.infer<typeof PlannerPlanSchema>;

export type AttachmentRole =
  | "current_screenshot"
  | "brand_logo"
  | "include_asset"
  | "style_reference"
  | "content_reference"
  | "selected_previous_output"
  | "font_file";

export interface SelectedReference {
  asset_id: string;
  role: AttachmentRole;
  label: string;
  placement_notes: string;
  preserve_as_is: boolean;
  source_output_id?: string | null;
}

export interface AssetModeOptions {
  intended_usage: string;
  theme_context: string;
  transparent_background: boolean;
  seamless_tiling: boolean;
  include_logo: string;
  composition: string;
  negative_space: string;
  forbidden_text: string;
  is_decorative: boolean;
  alt_text_intent: string;
}

export interface ModelCapabilitiesReport {
  checked_at: string;
  planner_model: string;
  planner_available: boolean;
  planner_supports_image_input: boolean;
  planner_supports_structured_output: boolean;
  planner_prompt_usd_per_token: number;
  planner_completion_usd_per_token: number;
  image_model: string;
  image_available: boolean;
  image_provider_name: string;
  supported_qualities: string[];
  supports_medium_quality: boolean;
  supported_aspect_ratios: string[];
  max_input_references: number;
  supports_transparent_background: boolean;
  supports_explicit_resolution: boolean;
  resolution_options: string[];
  image_input_text_usd_per_token: number;
  image_input_image_usd_per_token: number;
  image_output_usd_per_token: number;
  quality_pricing_usd?: Record<string, number>;
  blocked_reasons: string[];
  live_verified: boolean;
}

export interface SettingsUsageDto {
  total_actual_micros: number;
  total_estimated_micros: number;
  total_effective_micros: number;
  total_effective_usd: number;
  total_runs: number;
  total_images: number;
  spend_limit_micros: number | null;
  daily_spend: Array<{
    date: string;
    spend_micros: number;
    spend_usd: number;
    run_count: number;
    image_count: number;
  }>;
  by_project: Array<{
    project_id: string;
    project_name: string;
    spend_micros: number;
    spend_usd: number;
    run_count: number;
    image_count: number;
  }>;
  by_quality: Array<{
    tier: string;
    spend_micros: number;
    spend_usd: number;
    run_count: number;
    image_count: number;
  }>;
  openrouter_account?: {
    label?: string | null;
    usage_usd?: number | null;
    limit_usd?: number | null;
  } | null;
  quality_pricing: Record<string, number>;
  capabilities: ModelCapabilitiesReport;
}

export interface SetupStatusDto {
  csrf_token: string;
  setup_completed: boolean;
  key_status: {
    configured: boolean;
    storage_kind: "keyring" | "session" | "env" | "none";
    masked_hint: string | null;
    keyring_available: boolean;
    persistence_note: string;
  };
  data_dir: string;
  data_dir_env_locked: boolean;
  settings: {
    planner_model: string;
    image_model: string;
    default_quality: string;
    output_size: string;
    include_extraction_instructions: boolean;
    max_concurrent_jobs: number;
    spend_limit_micros: number | null;
    test_adapter_enabled: boolean;
    setup_completed_at: string | null;
  };
  capabilities: ModelCapabilitiesReport;
  active_project_count: number;
}

export interface ProjectSummaryDto {
  id: string;
  name: string;
  description: string;
  purpose: string;
  industry: string;
  target_audience: string;
  audience_needs: string;
  desired_user_action: string;
  archive_status: string;
  is_example?: boolean;
  brand_version: number;
  brand: BrandSpec;
  recent_output_id: string | null;
  default_thread_id: string | null;
  thread_count: number;
  run_count: number;
  created_at: string;
  updated_at: string;
}

export interface AssetDto {
  id: string;
  project_id: string;
  role: AttachmentRole;
  label: string;
  mime_type: string;
  width: number;
  height: number;
  byte_size: number;
  checksum_sha256: string;
  has_alpha: boolean;
  placement_notes: string;
  preserve_as_is: boolean;
  metadata: {
    resized_for_model?: boolean;
    svg_sanitized?: boolean;
    rasterized_for_model?: boolean;
    original_dimensions?: { width: number; height: number };
    model_input_dimensions?: { width: number; height: number };
    source_output_id?: string;
  };
  absolute_path: string;
  thumbnail_url: string;
  original_url: string;
  created_at: string;
}

export interface OutputDto {
  id: string;
  run_id: string;
  plan_item_id: string;
  variant_index: number;
  kind: string;
  title: string;
  direction_summary: string;
  role: string;
  page_route: string | null;
  aspect_ratio: string;
  status:
    | "queued"
    | "ready"
    | "rendering"
    | "saving"
    | "completed"
    | "failed"
    | "cancelled"
    | "interrupted"
    | "outcome_unknown";
  mime_type: string | null;
  width: number | null;
  height: number | null;
  byte_size: number | null;
  has_alpha: boolean;
  transparent_background?: boolean;
  tab_key?: string;
  tab_label?: string;
  parent_output_id?: string | null;
  extracted_from_output_id?: string | null;
  compiled_prompt: string;
  reference_asset_ids: string[];
  depends_on: string[];
  item_spec: PlanItem;
  usage: {
    duration_ms?: number;
    provider?: string;
    transparency_verified?: boolean;
    transparency_warning?: string;
    simulated?: boolean;
  };
  provider_request_id: string | null;
  actual_cost_micros: number | null;
  error_message: string | null;
  absolute_path: string | null;
  thumbnail_url: string | null;
  original_url: string | null;
  created_at: string;
  updated_at: string;
}

export interface RunEventDto {
  id: number;
  job_id: string | null;
  output_id: string | null;
  event_type: string;
  status: string;
  message: string;
  created_at: string;
}

export interface RunDto {
  id: string;
  project_id: string;
  thread_id: string;
  mode: "component" | "full_page" | "asset";
  scope_preset: string;
  user_prompt: string;
  brand_version: number;
  brand_snapshot: BrandSpec;
  selected_references: SelectedReference[];
  context_summary: string;
  model_settings: {
    planner_model: string;
    image_model: string;
    quality: string;
    aspect_ratio: string;
    output_size: string;
    scope_preset: string;
    custom_dimensions?: string | null;
    review_plan_first: boolean;
    asset_options?: AssetModeOptions | null;
  };
  plan: PlannerPlan | null;
  consistency_report: {
    checked_at: string;
    overall_consistent: boolean;
    summary: string;
    findings: string[];
    recommendations: string[];
  } | null;
  status:
    | "queued"
    | "planning"
    | "awaiting_plan_review"
    | "ready"
    | "rendering"
    | "saving"
    | "completed"
    | "partial"
    | "failed"
    | "cancelled"
    | "interrupted"
    | "outcome_unknown";
  estimated_cost_micros: number | null;
  actual_cost_micros: number | null;
  cost_is_unknown: boolean;
  is_test_adapter: boolean;
  error_message: string | null;
  target_tab_key?: string | null;
  target_tab_label?: string | null;
  target_output_id?: string | null;
  highlight_annotations?: HighlightAnnotation[];
  outputs: OutputDto[];
  events: RunEventDto[];
  created_at: string;
  updated_at: string;
  completed_at: string | null;
}

export interface HighlightAnnotation {
  output_id: string;
  tab_key?: string;
  tab_label?: string;
  x_pct: number;
  y_pct: number;
  w_pct: number;
  h_pct: number;
  comment: string;
}

export interface ThreadMessageAttachmentDto {
  asset_id: string;
  label: string;
  role: string;
  thumbnail_url: string;
  original_url: string;
  width?: number;
  height?: number;
}

export interface ThreadMessageDto {
  id: string;
  thread_id: string;
  role: "user" | "assistant";
  content: string;
  attachments: ThreadMessageAttachmentDto[];
  highlight_annotations: HighlightAnnotation[];
  quick_replies: string[];
  linked_run_id: string | null;
  target_tab_key: string | null;
  target_tab_label: string | null;
  created_at: string;
}

export interface ThreadChatResponse {
  action: "ask_clarification" | "reply_only" | "trigger_run" | string;
  user_message: ThreadMessageDto;
  assistant_message: ThreadMessageDto;
  target_tab_key: string | null;
  target_tab_label: string | null;
  run: RunDto | null;
}

export interface ThreadDto {
  id: string;
  project_id: string;
  title: string;
  archive_status: "active" | "archived";
  run_count?: number;
  created_at: string;
  updated_at: string;
}

export interface HandoffResponse {
  export_id: string;
  export_kind: string;
  clipboard_text: string;
  manifest_path: string;
  implementation_path: string;
  bundle_path: string | null;
  include_extraction_instructions: boolean;
  is_complete_set: boolean;
  missing_items: string[];
}

export interface AiBrandProposalDto {
  purpose?: string;
  industry?: string;
  target_audience?: string;
  audience_needs?: string;
  desired_user_action?: string;
  recommended_style_number?: number;
  recommended_modifiers?: string[];
  brand_personality: string;
  design_direction: string;
  tone_of_voice: string;
  ui_density: string;
  colors_light: Record<string, string>;
  colors_dark?: Record<string, string>;
  typography_heading: string;
  typography_body: string;
  typography_mono: string;
  spacing_scale?: string;
  radii?: string;
  shadows?: string;
  icon_style?: string;
  button_treatment?: string;
  brand_rules: string[];
  forbidden_treatments?: string[];
  caveat_notice: string;
}

export interface CostEstimateDto {
  per_image_micros: number;
  planner_micros: number;
  reference_input_micros: number;
  total_estimated_micros: number;
  min_range_micros: number;
  max_range_micros: number;
  image_count: number;
  quality: string;
  is_test_adapter: boolean;
  estimate_available: boolean;
  display_label: string;
  disclaimer: string;
}

let cachedCsrfToken: string | null = null;

export async function ensureCsrfToken(): Promise<string> {
  if (cachedCsrfToken) return cachedCsrfToken;
  const res = await fetch("/api/setup/status", { cache: "no-store" });
  if (!res.ok) {
    throw new Error(`Failed to fetch setup status: HTTP ${res.status}`);
  }
  const data: SetupStatusDto = await res.json();
  cachedCsrfToken = data.csrf_token;
  return cachedCsrfToken;
}

export async function apiFetch<T>(
  path: string,
  options: RequestInit = {}
): Promise<T> {
  const method = (options.method || "GET").toUpperCase();
  const headers = new Headers(options.headers || {});

  if (["POST", "PUT", "PATCH", "DELETE"].includes(method)) {
    const token = await ensureCsrfToken();
    headers.set("X-CSRF-Token", token);
    if (!(options.body instanceof FormData) && !headers.has("Content-Type")) {
      headers.set("Content-Type", "application/json");
    }
  }

  const res = await fetch(path, {
    ...options,
    headers,
  });

  if (path === "/api/setup/status" && res.ok) {
    const cloned = res.clone();
    try {
      const d = await cloned.json();
      if (d?.csrf_token) cachedCsrfToken = d.csrf_token;
    } catch {
      // ignore
    }
  }

  if (!res.ok) {
    let errMessage = `Request failed (${res.status})`;
    try {
      const errBody = await res.json();
      if (errBody?.error?.message) {
        errMessage = errBody.error.message;
      }
    } catch {
      // ignore
    }
    throw new Error(errMessage);
  }

  return res.json();
}

export function formatMicrosUsd(micros: number | null | undefined): string {
  if (micros === null || micros === undefined) return "Estimate unavailable";
  const usd = micros / 1_000_000;
  if (usd === 0) return "$0.00";
  if (usd < 0.01) return `$${usd.toFixed(4)}`;
  return `$${usd.toFixed(2)}`;
}

export interface UpdateCheckReport {
  current_version: string;
  github_repo: string;
  auto_check_enabled: boolean;
  auto_install_enabled: boolean;
  platform: string;
  arch: string;
  update_available: boolean;
  latest_version: string | null;
  release_name: string | null;
  release_notes: string | null;
  release_html_url: string | null;
  published_at: string | null;
  download_asset_name: string | null;
  download_asset_url: string | null;
  download_asset_size: number | null;
  status_message: string;
  last_checked_at: string;
  safety_guarantees: string[];
}

export interface UpdateInstallReport {
  ok: boolean;
  previous_version: string;
  installed_version: string;
  backup_sqlite_path: string;
  updated_target_path: string;
  restart_required: boolean;
  restart_scheduled: boolean;
  message: string;
}


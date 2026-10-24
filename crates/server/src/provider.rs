use crate::{
    config::{DEFAULT_IMAGE_MODEL, DEFAULT_PLANNER_MODEL},
    context_builder::{BuiltGenerationContext, ProjectMetaSummary, CHAT_ROUTER_SYSTEM_PROMPT_V1},
    error::{redact_secrets, AppError, AppResult},
    image_util::generate_deterministic_fixture_png,
    models::{
        normalize_tab_key, validate_planner_plan, BrandSpec, ChatRunSpec, ChatTurnDecision,
        ExistingTabSummary, PageCompositionSpec, PlanItem, PlanRequirement, PlannerPlan,
        SharedComponentSpec, SharedDesignSpec, ViewportSpec,
    },
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};
use uuid::Uuid;

pub const OPENROUTER_BASE_URL: &str = "https://openrouter.ai/api/v1";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredVisualAsset {
    pub name: String,
    pub aspect_ratio: String,
    pub direction: String,
    pub image_prompt: String,
    pub asset_instructions: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RenderedUiAuditResult {
    pub missing_assets: Vec<DiscoveredVisualAsset>,
    pub coded_separators_and_backgrounds: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelCapabilitiesReport {
    pub checked_at: String,
    pub planner_model: String,
    pub planner_available: bool,
    pub planner_supports_image_input: bool,
    pub planner_supports_structured_output: bool,
    pub planner_prompt_usd_per_token: f64,
    pub planner_completion_usd_per_token: f64,
    pub image_model: String,
    pub image_available: bool,
    pub image_provider_name: String,
    pub supported_qualities: Vec<String>,
    pub supports_medium_quality: bool,
    pub supported_aspect_ratios: Vec<String>,
    pub max_input_references: u32,
    pub supports_transparent_background: bool,
    pub supports_explicit_resolution: bool,
    pub resolution_options: Vec<String>,
    pub image_input_text_usd_per_token: f64,
    pub image_input_image_usd_per_token: f64,
    pub image_output_usd_per_token: f64,
    #[serde(default = "default_quality_pricing_map")]
    pub quality_pricing_usd: HashMap<String, f64>,
    pub blocked_reasons: Vec<String>,
    pub live_verified: bool,
}

pub fn compute_quality_pricing_map(
    image_output_usd_per_token: f64,
    image_input_text_usd_per_token: f64,
) -> HashMap<String, f64> {
    let text_cost = 320.0 * image_input_text_usd_per_token;
    let mut map = HashMap::new();
    // Official OpenAI / OpenRouter GPT Image token tiers:
    // Low (~196-200 output tokens -> ~$0.006-$0.008)
    // Medium (~650 output tokens -> ~$0.021)
    // High (~1,750 output tokens -> ~$0.054)
    // Max / X-High (~7,000 output tokens -> ~$0.211)
    map.insert(
        "low".into(),
        ((200.0 * image_output_usd_per_token + text_cost) * 10000.0).round() / 10000.0,
    );
    map.insert(
        "medium".into(),
        ((650.0 * image_output_usd_per_token + text_cost) * 10000.0).round() / 10000.0,
    );
    map.insert(
        "auto".into(),
        ((650.0 * image_output_usd_per_token + text_cost) * 10000.0).round() / 10000.0,
    );
    map.insert(
        "high".into(),
        ((1750.0 * image_output_usd_per_token + text_cost) * 10000.0).round() / 10000.0,
    );
    map.insert(
        "xhigh".into(),
        ((7000.0 * image_output_usd_per_token + text_cost) * 10000.0).round() / 10000.0,
    );
    map.insert(
        "max".into(),
        ((7000.0 * image_output_usd_per_token + text_cost) * 10000.0).round() / 10000.0,
    );
    map
}

fn default_quality_pricing_map() -> HashMap<String, f64> {
    compute_quality_pricing_map(0.00003, 0.000005)
}

impl Default for ModelCapabilitiesReport {
    fn default() -> Self {
        Self {
            checked_at: chrono::Utc::now().to_rfc3339(),
            planner_model: DEFAULT_PLANNER_MODEL.to_string(),
            planner_available: true,
            planner_supports_image_input: true,
            planner_supports_structured_output: true,
            planner_prompt_usd_per_token: 0.00000006,
            planner_completion_usd_per_token: 0.00000016,
            image_model: DEFAULT_IMAGE_MODEL.to_string(),
            image_available: true,
            image_provider_name: "OpenAI".to_string(),
            supported_qualities: vec![
                "auto".into(),
                "low".into(),
                "medium".into(),
                "high".into(),
                "xhigh".into(),
                "max".into(),
            ],
            supports_medium_quality: true,
            supported_aspect_ratios: vec![
                "1:1".into(),
                "3:2".into(),
                "2:3".into(),
                "4:3".into(),
                "3:4".into(),
                "16:9".into(),
                "9:16".into(),
                "21:9".into(),
                "auto".into(),
            ],
            max_input_references: 16,
            supports_transparent_background: true,
            supports_explicit_resolution: false,
            resolution_options: vec!["provider_default".into()],
            image_input_text_usd_per_token: 0.000005,
            image_input_image_usd_per_token: 0.000008,
            image_output_usd_per_token: 0.00003,
            quality_pricing_usd: default_quality_pricing_map(),
            blocked_reasons: vec![],
            live_verified: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostEstimate {
    pub per_image_micros: i64,
    pub planner_micros: i64,
    pub reference_input_micros: i64,
    pub total_estimated_micros: i64,
    pub min_range_micros: i64,
    pub max_range_micros: i64,
    pub image_count: u32,
    pub quality: String,
    pub is_test_adapter: bool,
    pub estimate_available: bool,
    pub display_label: String,
    pub disclaimer: String,
}

pub fn compute_cost_estimate(
    caps: &ModelCapabilitiesReport,
    image_count: u32,
    reference_image_count: u32,
    quality: &str,
    is_test_adapter: bool,
) -> CostEstimate {
    if is_test_adapter {
        return CostEstimate {
            per_image_micros: 0,
            planner_micros: 0,
            reference_input_micros: 0,
            total_estimated_micros: 0,
            min_range_micros: 0,
            max_range_micros: 0,
            image_count,
            quality: quality.to_string(),
            is_test_adapter: true,
            estimate_available: true,
            display_label: "Simulated ($0.00)".to_string(),
            disclaimer: "Local test fixture mode is active; no paid OpenRouter calls will be made."
                .to_string(),
        };
    }

    // Official OpenAI / OpenRouter GPT Image token tiers:
    // Low: ~200 output tokens (~$0.006-$0.008/img)
    // Medium (default): ~650 output tokens (~$0.021/img)
    // High: ~1,750 output tokens (~$0.054/img)
    // Max / X-High: ~7,000 output tokens (~$0.211/img)
    let base_output_tokens_per_image: f64 = match quality {
        "low" => 200.0,
        "medium" | "auto" => 650.0,
        "high" => 1750.0,
        "xhigh" | "max" => 7000.0,
        _ => 650.0,
    };
    let text_tokens_per_image = 320.0;
    let tokens_per_ref_image = 280.0;

    let per_img_output_usd = base_output_tokens_per_image * caps.image_output_usd_per_token;
    let per_img_text_usd = text_tokens_per_image * caps.image_input_text_usd_per_token;
    let per_img_refs_usd = (reference_image_count as f64)
        * tokens_per_ref_image
        * caps.image_input_image_usd_per_token;

    let per_image_usd = per_img_output_usd + per_img_text_usd;
    let per_image_micros = (per_image_usd * 1_000_000.0).round() as i64;
    let reference_input_micros =
        (per_img_refs_usd * (image_count as f64) * 1_000_000.0).round() as i64;

    let planner_usd = (1800.0 * caps.planner_prompt_usd_per_token)
        + (1400.0 * caps.planner_completion_usd_per_token);
    let planner_micros = ((planner_usd * 1_000_000.0).round() as i64).max(100);

    let total_estimated_micros =
        (per_image_micros * (image_count as i64)) + reference_input_micros + planner_micros;
    let min_range_micros = ((total_estimated_micros as f64) * 0.80).round() as i64;
    let max_range_micros = ((total_estimated_micros as f64) * 1.35).round() as i64;

    let total_usd = total_estimated_micros as f64 / 1_000_000.0;
    let per_img_usd = per_image_micros as f64 / 1_000_000.0;

    CostEstimate {
        per_image_micros,
        planner_micros,
        reference_input_micros,
        total_estimated_micros,
        min_range_micros,
        max_range_micros,
        image_count,
        quality: quality.to_string(),
        is_test_adapter: false,
        estimate_available: true,
        display_label: format!(
            "~${:.2} est. ({} {} @ ~${:.2}/img + planner)",
            total_usd,
            image_count,
            if image_count == 1 { "image" } else { "images" },
            per_img_usd
        ),
        disclaimer: "Estimated from OpenRouter token rates (~$0.02/image target at medium quality). Actual upstream billing depends on provider token usage.".to_string(),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyValidationResult {
    pub valid: bool,
    pub label: Option<String>,
    pub usage_usd: Option<f64>,
    pub limit_usd: Option<f64>,
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct RenderedImageOutput {
    pub bytes: Vec<u8>,
    pub media_type: Option<String>,
    pub provider_request_id: String,
    pub provider_name: Option<String>,
    pub duration_ms: u64,
    pub usage_json: serde_json::Value,
    pub reported_cost_micros: Option<i64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RenderFailureKind {
    Unauthorized,
    InsufficientCredits,
    RateLimited { retry_after_secs: u64 },
    Validation,
    DefiniteUpstreamFailure,
    OutcomeUnknown,
}

#[derive(Debug, Clone)]
pub struct RenderJobError {
    pub kind: RenderFailureKind,
    pub message: String,
    pub upstream_request_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandSuggestionProposal {
    #[serde(default)]
    pub purpose: String,
    #[serde(default)]
    pub industry: String,
    #[serde(default)]
    pub target_audience: String,
    #[serde(default)]
    pub audience_needs: String,
    #[serde(default)]
    pub desired_user_action: String,
    #[serde(default = "default_recommended_style_num")]
    pub recommended_style_number: u32,
    #[serde(default)]
    pub recommended_modifiers: Vec<String>,
    pub brand_personality: String,
    pub design_direction: String,
    pub tone_of_voice: String,
    pub ui_density: String,
    pub colors_light: HashMap<String, String>,
    #[serde(default)]
    pub colors_dark: HashMap<String, String>,
    pub typography_heading: String,
    pub typography_body: String,
    pub typography_mono: String,
    #[serde(default)]
    pub spacing_scale: String,
    #[serde(default)]
    pub radii: String,
    #[serde(default)]
    pub shadows: String,
    #[serde(default)]
    pub icon_style: String,
    pub brand_rules: Vec<String>,
    #[serde(default)]
    pub forbidden_treatments: Vec<String>,
    pub caveat_notice: String,
}

fn default_recommended_style_num() -> u32 {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsistencyCheckReport {
    pub checked_at: String,
    pub overall_consistent: bool,
    pub summary: String,
    pub findings: Vec<String>,
    pub recommendations: Vec<String>,
}

pub struct OpenRouterClient {
    http: reqwest::Client,
    base_url: String,
}

impl Default for OpenRouterClient {
    fn default() -> Self {
        Self::new()
    }
}

impl OpenRouterClient {
    pub fn new() -> Self {
        let http = reqwest::Client::builder()
            .timeout(Duration::from_secs(180))
            .connect_timeout(Duration::from_secs(15))
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());
        let base_url = std::env::var("OPENROUTER_BASE_URL")
            .unwrap_or_else(|_| OPENROUTER_BASE_URL.to_string());
        Self { http, base_url }
    }

    /// Validates an OpenRouter API key using the non-generation `/auth/key` endpoint.
    pub async fn validate_key(&self, api_key: &str) -> AppResult<KeyValidationResult> {
        let url = format!("{}/auth/key", self.base_url);
        let resp = self
            .http
            .get(&url)
            .header("Authorization", format!("Bearer {}", api_key.trim()))
            .send()
            .await
            .map_err(|e| {
                AppError::ProviderError(format!(
                    "Could not reach OpenRouter key endpoint: {}",
                    redact_secrets(&e.to_string())
                ))
            })?;

        if resp.status() == StatusCode::UNAUTHORIZED || resp.status() == StatusCode::FORBIDDEN {
            return Ok(KeyValidationResult {
                valid: false,
                label: None,
                usage_usd: None,
                limit_usd: None,
                message: "OpenRouter rejected this API key (401 Unauthorized).".to_string(),
            });
        }

        if !resp.status().is_success() {
            return Err(AppError::ProviderError(format!(
                "OpenRouter key validation returned HTTP {}",
                resp.status()
            )));
        }

        let body: serde_json::Value = resp.json().await.unwrap_or_default();
        let data = body.get("data").cloned().unwrap_or_default();
        let label = data
            .get("label")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let usage_usd = data.get("usage").and_then(|v| v.as_f64());
        let limit_usd = data.get("limit").and_then(|v| v.as_f64());

        Ok(KeyValidationResult {
            valid: true,
            label,
            usage_usd,
            limit_usd,
            message: "OpenRouter API key verified successfully (no paid tokens used).".to_string(),
        })
    }

    /// Queries live OpenRouter endpoint metadata for both the planner model and image model.
    pub async fn discover_capabilities(
        &self,
        planner_model: &str,
        image_model: &str,
    ) -> ModelCapabilitiesReport {
        let mut report = ModelCapabilitiesReport {
            checked_at: chrono::Utc::now().to_rfc3339(),
            planner_model: planner_model.to_string(),
            image_model: image_model.to_string(),
            ..Default::default()
        };
        let mut blocked = Vec::new();
        let mut planner_ok = false;
        let mut image_ok = false;

        // 1. Check Planner endpoints
        let planner_url = format!("{}/models/{}/endpoints", self.base_url, planner_model);
        if let Ok(resp) = self.http.get(&planner_url).send().await {
            if resp.status().is_success() {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    if let Some(data) = json.get("data") {
                        planner_ok = true;
                        let input_mods: Vec<String> = data
                            .get("architecture")
                            .and_then(|a| a.get("input_modalities"))
                            .and_then(|m| m.as_array())
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default();
                        report.planner_supports_image_input =
                            input_mods.iter().any(|m| m == "image");
                        if !report.planner_supports_image_input {
                            blocked.push(format!(
                                "Planner model '{}' does not support image input on OpenRouter.",
                                planner_model
                            ));
                        }

                        if let Some(endpoints) = data.get("endpoints").and_then(|e| e.as_array()) {
                            if endpoints.is_empty() {
                                report.planner_available = false;
                                blocked.push(format!(
                                    "Planner model '{}' has no active provider endpoints.",
                                    planner_model
                                ));
                            } else {
                                let ep0 = &endpoints[0];
                                if let Some(pricing) = ep0.get("pricing") {
                                    if let Some(p) = pricing
                                        .get("prompt")
                                        .and_then(|v| v.as_str())
                                        .and_then(|s| s.parse::<f64>().ok())
                                    {
                                        report.planner_prompt_usd_per_token = p;
                                    }
                                    if let Some(c) = pricing
                                        .get("completion")
                                        .and_then(|v| v.as_str())
                                        .and_then(|s| s.parse::<f64>().ok())
                                    {
                                        report.planner_completion_usd_per_token = c;
                                    }
                                }
                                let params: Vec<String> = ep0
                                    .get("supported_parameters")
                                    .and_then(|p| p.as_array())
                                    .map(|arr| {
                                        arr.iter()
                                            .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                            .collect()
                                    })
                                    .unwrap_or_default();
                                report.planner_supports_structured_output = params
                                    .iter()
                                    .any(|p| p == "structured_outputs" || p == "response_format");
                            }
                        }
                    }
                }
            } else {
                report.planner_available = false;
                blocked.push(format!(
                    "Planner model '{}' endpoint metadata returned HTTP {}.",
                    planner_model,
                    resp.status()
                ));
            }
        }

        // 2. Check Image model endpoints (/images/models/{model}/endpoints or /models/{model}/endpoints)
        let image_url = format!("{}/images/models/{}/endpoints", self.base_url, image_model);
        let mut checked_image_endpoint = false;
        if let Ok(resp) = self.http.get(&image_url).send().await {
            if resp.status().is_success() {
                if let Ok(json) = resp.json::<serde_json::Value>().await {
                    if let Some(endpoints) = json.get("endpoints").and_then(|e| e.as_array()) {
                        if let Some(ep) = endpoints.first() {
                            checked_image_endpoint = true;
                            image_ok = true;
                            report.image_available = true;
                            report.image_provider_name = ep
                                .get("provider_name")
                                .and_then(|v| v.as_str())
                                .unwrap_or("OpenRouter")
                                .to_string();

                            if let Some(sp) = ep.get("supported_parameters") {
                                if let Some(q_vals) = sp
                                    .get("quality")
                                    .and_then(|q| q.get("values"))
                                    .and_then(|v| v.as_array())
                                {
                                    report.supported_qualities = q_vals
                                        .iter()
                                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                        .collect();
                                    report.supports_medium_quality =
                                        report.supported_qualities.iter().any(|q| q == "medium");
                                }

                                if let Some(ar_vals) = sp
                                    .get("aspect_ratio")
                                    .and_then(|a| a.get("values"))
                                    .and_then(|v| v.as_array())
                                {
                                    report.supported_aspect_ratios = ar_vals
                                        .iter()
                                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                        .collect();
                                }

                                if let Some(bg_vals) = sp
                                    .get("background")
                                    .and_then(|b| b.get("values"))
                                    .and_then(|v| v.as_array())
                                {
                                    report.supports_transparent_background = bg_vals
                                        .iter()
                                        .filter_map(|v| v.as_str())
                                        .any(|s| s == "transparent");
                                }

                                if let Some(max_refs) = sp
                                    .get("input_references")
                                    .and_then(|r| r.get("max"))
                                    .and_then(|v| v.as_u64())
                                {
                                    report.max_input_references = max_refs as u32;
                                }

                                // Check whether explicit resolution / size is exposed
                                if let Some(res_vals) = sp
                                    .get("resolution")
                                    .or_else(|| sp.get("size"))
                                    .and_then(|r| r.get("values"))
                                    .and_then(|v| v.as_array())
                                {
                                    let opts: Vec<String> = res_vals
                                        .iter()
                                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                        .collect();
                                    if !opts.is_empty() {
                                        report.supports_explicit_resolution = true;
                                        report.resolution_options = opts;
                                    }
                                } else {
                                    report.supports_explicit_resolution = false;
                                    report.resolution_options = vec!["provider_default".into()];
                                }
                            }

                            if let Some(pricing) = ep.get("pricing").and_then(|p| p.as_array()) {
                                for entry in pricing {
                                    let billable = entry
                                        .get("billable")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("");
                                    let cost = entry
                                        .get("cost_usd")
                                        .and_then(|v| v.as_f64())
                                        .unwrap_or(0.0);
                                    match billable {
                                        "input_text" if cost > 0.0 => {
                                            report.image_input_text_usd_per_token = cost
                                        }
                                        "input_image" if cost > 0.0 => {
                                            report.image_input_image_usd_per_token = cost
                                        }
                                        "output_image" if cost > 0.0 => {
                                            report.image_output_usd_per_token = cost
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Fallback: Check /models/{image_model}/endpoints for chat-routed image models (e.g. gemini-2.5-flash-image-preview, flux, gpt-5-image, etc.)
        if !checked_image_endpoint {
            let alt_image_url = format!("{}/models/{}/endpoints", self.base_url, image_model);
            if let Ok(resp) = self.http.get(&alt_image_url).send().await {
                if resp.status().is_success() {
                    if let Ok(json) = resp.json::<serde_json::Value>().await {
                        if let Some(data) = json.get("data") {
                            let out_mods: Vec<String> = data
                                .get("architecture")
                                .and_then(|a| a.get("output_modalities"))
                                .and_then(|m| m.as_array())
                                .map(|arr| {
                                    arr.iter()
                                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                        .collect()
                                })
                                .unwrap_or_default();
                            if out_mods.iter().any(|m| m == "image")
                                || image_model.contains("image")
                                || image_model.contains("flux")
                                || image_model.contains("seedream")
                            {
                                image_ok = true;
                                report.image_available = true;
                                if let Some(ep0) = data
                                    .get("endpoints")
                                    .and_then(|e| e.as_array())
                                    .and_then(|a| a.first())
                                {
                                    report.image_provider_name = ep0
                                        .get("provider_name")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("OpenRouter Image")
                                        .to_string();
                                    if let Some(pricing) = ep0.get("pricing") {
                                        if let Some(p) = pricing
                                            .get("prompt")
                                            .and_then(|v| v.as_str())
                                            .and_then(|s| s.parse::<f64>().ok())
                                            .filter(|&v| v > 0.0)
                                        {
                                            report.image_input_text_usd_per_token = p;
                                        }
                                        if let Some(c) = pricing
                                            .get("completion")
                                            .and_then(|v| v.as_str())
                                            .and_then(|s| s.parse::<f64>().ok())
                                            .filter(|&v| v > 0.0)
                                        {
                                            report.image_output_usd_per_token = c;
                                        } else if let Some(img_cost) = pricing
                                            .get("image")
                                            .and_then(|v| v.as_str())
                                            .and_then(|s| s.parse::<f64>().ok())
                                            .filter(|&v| v > 0.0)
                                        {
                                            // If flat per-image rate is given, normalize to ~650 medium tokens
                                            if img_cost > 0.001 {
                                                report.image_output_usd_per_token =
                                                    img_cost / 650.0;
                                            } else {
                                                report.image_output_usd_per_token = img_cost;
                                            }
                                        }
                                    }
                                } else {
                                    report.image_provider_name = "OpenRouter Image".to_string();
                                }
                            } else {
                                report.image_available = false;
                                blocked.push(format!(
                                    "Model '{}' does not support image generation output.",
                                    image_model
                                ));
                            }
                        }
                    }
                } else {
                    report.image_available = false;
                    blocked.push(format!(
                        "Image model '{}' was not found on OpenRouter (HTTP {}).",
                        image_model,
                        resp.status()
                    ));
                }
            }
        }

        report.quality_pricing_usd = compute_quality_pricing_map(
            report.image_output_usd_per_token,
            report.image_input_text_usd_per_token,
        );
        report.live_verified = planner_ok && image_ok;
        report.blocked_reasons = blocked;
        report
    }

    /// Runs the GLM planner via `POST /chat/completions` with at most 1 bounded repair attempt.
    pub async fn run_planner(
        &self,
        api_key: &str,
        ctx: &BuiltGenerationContext,
        valid_asset_ids: &HashSet<String>,
        supported_aspect_ratios: &[String],
    ) -> AppResult<(PlannerPlan, Option<i64>)> {
        let mut content_parts = vec![serde_json::json!({
            "type": "text",
            "text": ctx.planner_user_prompt
        })];

        if let Some(ref cs) = ctx.current_screenshot {
            content_parts.push(serde_json::json!({
                "type": "image_url",
                "image_url": { "url": cs.data_url }
            }));
        }
        if let Some(ref prev) = ctx.selected_previous_output {
            content_parts.push(serde_json::json!({
                "type": "image_url",
                "image_url": { "url": prev.data_url }
            }));
        }
        for att in &ctx.explicit_attachments {
            if !att.data_url.is_empty() {
                content_parts.push(serde_json::json!({
                    "type": "image_url",
                    "image_url": { "url": att.data_url }
                }));
            }
        }

        let messages = vec![
            serde_json::json!({
                "role": "system",
                "content": ctx.system_prompt
            }),
            serde_json::json!({
                "role": "user",
                "content": content_parts
            }),
        ];

        let (raw_text, cost1) = self
            .call_chat_completion(api_key, &ctx.model_settings.planner_model, messages.clone())
            .await?;

        match Self::parse_and_validate_plan(
            &raw_text,
            &ctx.mode,
            valid_asset_ids,
            supported_aspect_ratios,
        ) {
            Ok(plan) => Ok((plan, cost1)),
            Err(first_err) => {
                tracing::warn!(
                    "Planner first attempt validation failed ({}); attempting 1 bounded repair",
                    first_err
                );
                let mut repair_messages = messages;
                repair_messages.push(serde_json::json!({
                    "role": "assistant",
                    "content": raw_text
                }));
                repair_messages.push(serde_json::json!({
                    "role": "user",
                    "content": format!(
                        "Your previous JSON output failed strict schema validation with error: {}. Return ONLY the corrected valid JSON object matching schema_version=1.",
                        first_err
                    )
                }));
                let (repaired_text, cost2) = self
                    .call_chat_completion(
                        api_key,
                        &ctx.model_settings.planner_model,
                        repair_messages,
                    )
                    .await?;
                let plan = Self::parse_and_validate_plan(
                    &repaired_text,
                    &ctx.mode,
                    valid_asset_ids,
                    supported_aspect_ratios,
                )?;
                let total_cost = match (cost1, cost2) {
                    (Some(a), Some(b)) => Some(a + b),
                    (Some(a), None) => Some(a),
                    (None, Some(b)) => Some(b),
                    (None, None) => None,
                };
                Ok((plan, total_cost))
            }
        }
    }

    fn parse_and_validate_plan(
        raw_text: &str,
        expected_mode: &str,
        valid_asset_ids: &HashSet<String>,
        supported_aspect_ratios: &[String],
    ) -> AppResult<PlannerPlan> {
        let cleaned = Self::extract_json_object(raw_text);
        let mut plan: PlannerPlan = crate::models::parse_planner_plan_tolerant(cleaned)?;
        crate::models::sanitize_planner_plan(
            &mut plan,
            expected_mode,
            valid_asset_ids,
            supported_aspect_ratios,
        );
        validate_planner_plan(
            &plan,
            expected_mode,
            valid_asset_ids,
            supported_aspect_ratios,
        )?;
        Ok(plan)
    }

    fn extract_json_object(raw: &str) -> &str {
        let trimmed = raw.trim();
        if let Some(start) = trimmed.find('{') {
            if let Some(end) = trimmed.rfind('}') {
                if end >= start {
                    return &trimmed[start..=end];
                }
            }
        }
        trimmed
    }

    async fn call_chat_completion(
        &self,
        api_key: &str,
        model: &str,
        messages: Vec<serde_json::Value>,
    ) -> AppResult<(String, Option<i64>)> {
        let url = format!("{}/chat/completions", self.base_url);
        let payload = serde_json::json!({
            "model": model,
            "messages": messages,
            "response_format": { "type": "json_object" },
            "reasoning": { "effort": "low" },
            "temperature": 0.4
        });

        let resp = self
            .http
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key.trim()))
            .json(&payload)
            .send()
            .await
            .map_err(|e| {
                AppError::ProviderError(format!(
                    "Planner request failed: {}",
                    redact_secrets(&e.to_string())
                ))
            })?;

        let status = resp.status();
        let body_text = resp.text().await.unwrap_or_default();

        if status == StatusCode::UNAUTHORIZED {
            return Err(AppError::Unauthorized(
                "OpenRouter API key rejected (401).".into(),
            ));
        }
        if status == StatusCode::PAYMENT_REQUIRED {
            return Err(AppError::PaymentRequired(
                "Insufficient OpenRouter credits (402).".into(),
            ));
        }
        if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(AppError::RateLimited(
                "OpenRouter rate limit reached (429).".into(),
            ));
        }
        if !status.is_success() {
            return Err(AppError::ProviderError(format!(
                "Planner returned HTTP {}: {}",
                status,
                redact_secrets(&body_text)
            )));
        }

        let json: serde_json::Value = serde_json::from_str(&body_text).map_err(|e| {
            AppError::ProviderError(format!("Invalid JSON from planner endpoint: {}", e))
        })?;

        let content = json
            .get("choices")
            .and_then(|c| c.get(0))
            .and_then(|c| c.get("message"))
            .and_then(|m| m.get("content"))
            .and_then(|c| c.as_str())
            .ok_or_else(|| {
                AppError::ProviderError(
                    "Planner response missing choices[0].message.content".into(),
                )
            })?
            .to_string();

        let reported_cost_micros = json
            .get("usage")
            .and_then(|u| u.get("cost").or_else(|| u.get("total_cost")))
            .and_then(|v| v.as_f64())
            .map(|usd| (usd * 1_000_000.0).round() as i64);

        Ok((content, reported_cost_micros))
    }

    /// Calls `POST /images` for `openai/gpt-image-2.5-sunburst` with exact Section 12 schema.
    #[allow(clippy::too_many_arguments)]
    pub async fn render_single_image(
        &self,
        api_key: &str,
        model: &str,
        compiled_prompt: &str,
        quality: &str,
        aspect_ratio: &str,
        input_reference_data_urls: &[String],
        transparent_background: bool,
    ) -> Result<RenderedImageOutput, RenderJobError> {
        let url = format!("{}/images", self.base_url);
        let started = Instant::now();

        let mut payload = serde_json::json!({
            "model": model,
            "prompt": compiled_prompt,
            "quality": quality,
            "aspect_ratio": aspect_ratio,
            "n": 1
        });

        if !input_reference_data_urls.is_empty() {
            let refs: Vec<serde_json::Value> = input_reference_data_urls
                .iter()
                .map(|url| {
                    serde_json::json!({
                        "type": "image_url",
                        "image_url": {
                            "url": url
                        }
                    })
                })
                .collect();
            payload["input_references"] = serde_json::Value::Array(refs);
        }

        if transparent_background {
            payload["background"] = serde_json::Value::String("transparent".to_string());
        }

        let resp = match self
            .http
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key.trim()))
            .json(&payload)
            .send()
            .await
        {
            Ok(r) => r,
            Err(e) => {
                if e.is_timeout() {
                    return Err(RenderJobError {
                        kind: RenderFailureKind::OutcomeUnknown,
                        message: "Request timed out after submission; upstream billing outcome is unknown. Use explicit retry if needed.".to_string(),
                        upstream_request_id: None,
                    });
                }
                return Err(RenderJobError {
                    kind: RenderFailureKind::DefiniteUpstreamFailure,
                    message: format!(
                        "Network error contacting OpenRouter: {}",
                        redact_secrets(&e.to_string())
                    ),
                    upstream_request_id: None,
                });
            }
        };

        let status = resp.status();
        let retry_after = resp
            .headers()
            .get("retry-after")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok())
            .unwrap_or(5);
        let header_req_id = resp
            .headers()
            .get("x-request-id")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string());

        let body_text = match resp.text().await {
            Ok(t) => t,
            Err(e) => {
                return Err(RenderJobError {
                    kind: RenderFailureKind::OutcomeUnknown,
                    message: format!("Interrupted while reading image response body: {}", e),
                    upstream_request_id: header_req_id,
                });
            }
        };

        if status == StatusCode::UNAUTHORIZED || status == StatusCode::FORBIDDEN {
            return Err(RenderJobError {
                kind: RenderFailureKind::Unauthorized,
                message: "OpenRouter rejected the API key (401 Unauthorized).".to_string(),
                upstream_request_id: header_req_id,
            });
        }
        if status == StatusCode::PAYMENT_REQUIRED {
            return Err(RenderJobError {
                kind: RenderFailureKind::InsufficientCredits,
                message: "Insufficient OpenRouter credits (402 Payment Required).".to_string(),
                upstream_request_id: header_req_id,
            });
        }
        if status == StatusCode::TOO_MANY_REQUESTS {
            return Err(RenderJobError {
                kind: RenderFailureKind::RateLimited {
                    retry_after_secs: retry_after.clamp(1, 30),
                },
                message: format!(
                    "Rate limited by OpenRouter (429). Retry after {}s.",
                    retry_after
                ),
                upstream_request_id: header_req_id,
            });
        }
        if status == StatusCode::BAD_REQUEST || status == StatusCode::UNPROCESSABLE_ENTITY {
            return Err(RenderJobError {
                kind: RenderFailureKind::Validation,
                message: format!(
                    "Unsupported parameter or validation error ({}): {}",
                    status,
                    redact_secrets(&body_text)
                ),
                upstream_request_id: header_req_id,
            });
        }
        if !status.is_success() {
            return Err(RenderJobError {
                kind: RenderFailureKind::DefiniteUpstreamFailure,
                message: format!(
                    "OpenRouter image endpoint returned HTTP {}: {}",
                    status,
                    redact_secrets(&body_text)
                ),
                upstream_request_id: header_req_id,
            });
        }

        let json: serde_json::Value =
            serde_json::from_str(&body_text).map_err(|e| RenderJobError {
                kind: RenderFailureKind::DefiniteUpstreamFailure,
                message: format!("Invalid JSON from OpenRouter /images: {}", e),
                upstream_request_id: header_req_id.clone(),
            })?;

        let provider_request_id = json
            .get("id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or(header_req_id)
            .unwrap_or_else(|| format!("or-{}", Uuid::new_v4()));

        let first_item = json
            .get("data")
            .and_then(|d| d.as_array())
            .and_then(|arr| arr.first())
            .ok_or_else(|| RenderJobError {
                kind: RenderFailureKind::DefiniteUpstreamFailure,
                message:
                    "OpenRouter /images succeeded with HTTP 200 but returned no data[0] image."
                        .to_string(),
                upstream_request_id: Some(provider_request_id.clone()),
            })?;

        let b64_str = first_item
            .get("b64_json")
            .and_then(|v| v.as_str())
            .ok_or_else(|| RenderJobError {
                kind: RenderFailureKind::DefiniteUpstreamFailure,
                message: "OpenRouter /images response missing data[0].b64_json.".to_string(),
                upstream_request_id: Some(provider_request_id.clone()),
            })?;

        let bytes = BASE64.decode(b64_str.trim()).map_err(|e| RenderJobError {
            kind: RenderFailureKind::DefiniteUpstreamFailure,
            message: format!("Failed to decode base64 image payload: {}", e),
            upstream_request_id: Some(provider_request_id.clone()),
        })?;

        let media_type = first_item
            .get("media_type")
            .or_else(|| first_item.get("mime_type"))
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let usage = json
            .get("usage")
            .cloned()
            .unwrap_or_else(|| serde_json::json!({}));
        let reported_cost_micros = usage
            .get("cost")
            .or_else(|| usage.get("total_cost"))
            .and_then(|v| v.as_f64())
            .map(|usd| (usd * 1_000_000.0).round() as i64);

        Ok(RenderedImageOutput {
            bytes,
            media_type,
            provider_request_id,
            provider_name: json
                .get("provider")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string()),
            duration_ms: started.elapsed().as_millis() as u64,
            usage_json: usage,
            reported_cost_micros,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn suggest_brand_details(
        &self,
        api_key: &str,
        planner_model: &str,
        project_name: &str,
        project_description: &str,
        project_purpose: &str,
        industry: &str,
        target_audience: &str,
        audience_needs: &str,
        desired_user_action: &str,
        design_style_summary: &str,
        logo_data_url: Option<&str>,
        custom_instructions: Option<&str>,
    ) -> AppResult<BrandSuggestionProposal> {
        let extra_notes = custom_instructions.unwrap_or("").trim();
        let search_context = format!(
            "{} {} {} {} {} {} {} {}",
            project_name,
            project_description,
            project_purpose,
            industry,
            target_audience,
            audience_needs,
            desired_user_action,
            extra_notes
        );
        let shortlisted_styles_block = crate::style_catalog::format_shortlisted_styles_for_ai(
            &search_context,
            if design_style_summary.trim().is_empty() {
                None
            } else {
                Some(design_style_summary)
            },
            14,
        );
        let prompt = format!(
            "You are an elite Brand & UI Design Director. Compile a complete, cohesive, accessible Brand Kit & Project Identity for '{project_name}'.\n\
            Product / App Description: {project_description}\n\
            Project Purpose (if provided): {project_purpose}\n\
            Industry (if provided): {industry}\n\
            Target Audience (if provided): {target_audience} (Needs: {audience_needs})\n\
            Desired User Action (if provided): {desired_user_action}\n\
            Chosen or Preferred Web Design Style & Rules:\n{design_style_summary}\n\
            Additional User / AI Enhancement Instructions: {extra_notes}\n\
            Logo Image Attached: {has_logo}\n\n\
            {shortlisted_styles_block}\n\n\
            CRITICAL INSTRUCTIONS:\n\
            1. If a Logo image is attached, carefully inspect its colors, geometry, contrast, and mood, and harmonize both the Light Mode (`colors_light`) and Dark Mode (`colors_dark`) 7-color hex palettes around the USER'S LOGO and product identity.\n\
            2. Pick the single best `recommended_style_number` (an integer from 1 to 120) from the Shortlisted Web Design Styles above based on what the user wants and what their product is! If their landing page or product fits the structure/flow of a Marketplace (#51–#60), Dropshipping Store (#61–#70), Shop/Ecommerce (#71–#80), Social/Community Platform (#81–#90), Anime Discovery (#91–#100), Film Discovery (#101–#110), or Game Library (#111–#120), pick that exact category-tailored style—while always applying the USER'S OWN BRANDING (their color palette, logo, and typography) rather than generic sample colors.\n\
            3. Fill in every field including `purpose`, `industry`, `target_audience`, `audience_needs`, `desired_user_action`, `recommended_style_number`, `recommended_modifiers` (1-2 modifiers like 'Bento', 'Card-based', 'Dark / Light mode', 'Motion-driven'), full `colors_light` and `colors_dark` hex palettes, typography, layout tokens, `brand_rules`, and `forbidden_treatments`.\n\
            4. Every color in `colors_light` and `colors_dark` MUST be a valid 6-digit `#RRGGBB` hex string.\n\
            Return ONLY JSON with fields:\n\
            {{\n\
              \"purpose\": \"string\",\n\
              \"industry\": \"string\",\n\
              \"target_audience\": \"string\",\n\
              \"audience_needs\": \"string\",\n\
              \"desired_user_action\": \"string\",\n\
              \"recommended_style_number\": 15,\n\
              \"recommended_modifiers\": [\"Subtle Motion\"],\n\
              \"brand_personality\": \"string\",\n\
              \"design_direction\": \"string\",\n\
              \"tone_of_voice\": \"string\",\n\
              \"ui_density\": \"comfortable\",\n\
              \"colors_light\": {{\"primary\": \"#hex\", \"accent\": \"#hex\", \"background\": \"#hex\", \"surface\": \"#hex\", \"text\": \"#hex\", \"muted_text\": \"#hex\", \"border\": \"#hex\"}},\n\
              \"colors_dark\": {{\"primary\": \"#hex\", \"accent\": \"#hex\", \"background\": \"#hex\", \"surface\": \"#hex\", \"text\": \"#hex\", \"muted_text\": \"#hex\", \"border\": \"#hex\"}},\n\
              \"typography_heading\": \"string\",\n\
              \"typography_body\": \"string\",\n\
              \"typography_mono\": \"string\",\n\
              \"spacing_scale\": \"4px / 8px / 16px / 24px / 32px / 48px / 64px\",\n\
              \"radii\": \"12px cards, 8px buttons\",\n\
              \"shadows\": \"0 8px 24px rgba(0,0,0,0.06)\",\n\
              \"icon_style\": \"2px crisp geometric stroke icons\",\n\
              \"brand_rules\": [\"string\"],\n\
              \"forbidden_treatments\": [\"string\"],\n\
              \"caveat_notice\": \"AI-compiled Brand Kit & Style ready for your review.\"\n\
            }}",
            has_logo = if logo_data_url.is_some() { "Yes (inspect attached logo image)" } else { "No" }
        );

        let mut user_parts = vec![serde_json::json!({
            "type": "text",
            "text": prompt
        })];
        if let Some(durl) = logo_data_url {
            if !durl.trim().is_empty() {
                user_parts.push(serde_json::json!({
                    "type": "image_url",
                    "image_url": { "url": durl }
                }));
            }
        }

        let messages = vec![
            serde_json::json!({
                "role": "system",
                "content": "You are Design Gen's multimodal brand system & UI design architect. Return valid JSON only."
            }),
            serde_json::json!({
                "role": "user",
                "content": user_parts
            }),
        ];
        let (text, _) = self
            .call_chat_completion(api_key, planner_model, messages)
            .await?;
        let cleaned = Self::extract_json_object(&text);
        let proposal: BrandSuggestionProposal = serde_json::from_str(cleaned).map_err(|e| {
            AppError::Validation(format!("Failed to parse brand suggestion JSON: {}", e))
        })?;
        Ok(proposal)
    }

    pub async fn check_consistency(
        &self,
        api_key: &str,
        planner_model: &str,
        request_summary: &str,
        image_data_urls: Vec<(String, String)>,
    ) -> AppResult<ConsistencyCheckReport> {
        let mut parts = vec![serde_json::json!({
            "type": "text",
            "text": format!(
                "Review these {} outputs from a coordinated Full-Page design set ('{}'). Check visual hierarchy, color palette consistency, typography feel, and component alignment. Return ONLY JSON: {{\"overall_consistent\": true, \"summary\": \"...\", \"findings\": [\"...\"], \"recommendations\": [\"...\"]}}",
                image_data_urls.len(),
                request_summary
            )
        })];
        for (title, data_url) in image_data_urls {
            parts.push(serde_json::json!({
                "type": "text",
                "text": format!("Output item: {}", title)
            }));
            parts.push(serde_json::json!({
                "type": "image_url",
                "image_url": { "url": data_url }
            }));
        }
        let messages = vec![
            serde_json::json!({
                "role": "system",
                "content": "You are Design Gen's visual consistency reviewer. Return valid JSON only."
            }),
            serde_json::json!({
                "role": "user",
                "content": parts
            }),
        ];
        let (text, _) = self
            .call_chat_completion(api_key, planner_model, messages)
            .await?;
        let cleaned = Self::extract_json_object(&text);
        let mut parsed: serde_json::Value = serde_json::from_str(cleaned).unwrap_or_default();
        Ok(ConsistencyCheckReport {
            checked_at: chrono::Utc::now().to_rfc3339(),
            overall_consistent: parsed
                .get("overall_consistent")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            summary: parsed
                .get("summary")
                .and_then(|v| v.as_str())
                .unwrap_or("Consistency review completed.")
                .to_string(),
            findings: parsed
                .get_mut("findings")
                .and_then(|v| serde_json::from_value(v.take()).ok())
                .unwrap_or_default(),
            recommendations: parsed
                .get_mut("recommendations")
                .and_then(|v| serde_json::from_value(v.take()).ok())
                .unwrap_or_default(),
        })
    }

    pub async fn discover_missing_visual_assets(
        &self,
        api_key: &str,
        planner_model: &str,
        parent_title: &str,
        brand_name: &str,
        rendered_image_data_url: &str,
        existing_asset_titles: &[String],
    ) -> AppResult<RenderedUiAuditResult> {
        let existing_list = if existing_asset_titles.is_empty() {
            "None yet".to_string()
        } else {
            existing_asset_titles
                .iter()
                .map(|t| format!("- {}", t))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let prompt = format!(
            "Inspect this rendered UI design image ('{parent_title}' for '{brand_name}') carefully from top to bottom.\n\n\
            ALREADY PLANNED / EXTRACTED TRANSPARENT ASSET CARDS IN THIS TAB:\n\
            {existing_list}\n\n\
            YOUR MISSION HAS TWO PARTS:\n\
            PART A — `missing_assets` (STANDALONE TRANSPARENT ASSETS TO EXTRACT):\n\
            Identify EVERY distinct TRUE BRAND, DECORATIVE, OR 3D OBJECT VISUAL ASSET — specifically: 3D objects/artifacts/props (such as a 3D crystal/gem/coin, potted plant, 3D megaphone, 3D spark badge/tile, mascot, floating 3D object, or ornate UI crest/emblem), brand logo / spark mark / wordmark, bespoke illustration, or custom icon set — visible anywhere in this rendered UI image that is NOT YET covered as its own separate item in the list above.\n\
            CRITICAL ASSET RULES:\n\
            1. DO EXTRACT TRUE BRAND, DECORATIVE & 3D OBJECT ASSETS: Never assume that a 3D object/prop (like a potted plant, 3D megaphone, 3D artifact, or 3D tile), a brand logo / crest / mark, a bespoke illustration, or an icon sheet should be left embedded! Those true brand, decorative, and 3D object graphics MUST have their own separate transparent-background PNG asset card in this tab.\n\
            2. NEVER EXTRACT PLACEHOLDER USER CONTENT, GAME CHARACTERS, GAMEPLAY BACKGROUNDS, SEPARATORS, BACKGROUNDS, OR UI CARDS/BANNERS: Do NOT extract placeholder user avatars, sample creator portraits, directory photo thumbnail grids, in-game character models, 3D gameplay world backgrounds, sample inventory slot filler items, section separators/dividers, ambient backgrounds, or UI cards/banners (such as a Hero Profile Card Mockup, Directory Card, Pricing Card, or Ad Banner Card)! Sample user photos/avatars and game world/character viewports are placeholder content, and separators, backgrounds, and UI cards/banners/frames are made 1:1 in code.\n\
            3. Do NOT duplicate assets that are already in the 'ALREADY PLANNED / EXTRACTED' list above.\n\
            4. For each missing brand/decorative/3D visual asset (up to 4 items), provide:\n\
               - `name`: Clear title ending with `(Transparent)` (e.g., `Hero 3D Clay Potted Plant & Spark Tile (Transparent)`, `{brand_name} Brand Logo & Wordmark (Transparent)`, `3D Rotating Crystal Relic Object Reference (Transparent)`).\n\
               - `aspect_ratio`: `\"1:1\"`, `\"4:3\"`, or `\"16:9\"`.\n\
               - `direction`: Short art direction summary.\n\
               - `image_prompt`: Detailed extraction prompt instructing `openai/gpt-image-2.5-sunburst` to extract and faithfully reproduce ONLY that specific brand/decorative/3D visual element from the attached reference UI image, isolated on a clean 100% transparent background with all surrounding background surfaces, UI cards, and unrelated text removed.\n\
               - `asset_instructions`: Plain human-language instruction (with zero code snippets) telling the coding agent where to place this asset, whether to convert it into a vectorial SVG graphic (for logos, icons, crests, flat art), or for 3D objects, instructing the coding agent to reconstruct an interactive 3D object from this reference image and animate it with smooth continuous rotation, floating idle motion, and interactive hover tilt.\n\n\
            PART B — `coded_separators_and_backgrounds` (WHAT IN THIS IMAGE MUST BE MADE 1:1 WITH CODE — SEPARATORS, BACKGROUNDS & 3D MOTION):\n\
            Inspect the exact section separators, dividers, backgrounds, ambient glows, and 3D/animated elements visible in THIS rendered image, and write 2 to 4 detailed, pure human-language instructions (with ZERO code snippets, zero HTML/JSX tags, and zero backticks) for the downstream coding AI agent:\n\
            1. **Exact Section Separators & Dividers in This Image (Made 1:1 With Code)**: Describe every horizontal/vertical section separator, glowing rule line, gradient fade divider, curved/angled transition, or ornate frame divider visible between sections in this image, and explicitly state that they must be recreated 1:1 in code to look identical to the image (specifying their exact placement, colors, thickness, glow, or shape).\n\
            2. **Exact Background Layers & Ambient Lighting in This Image (Made 1:1 With Code)**: Describe the exact page/section background canvas, radial or linear ambient glows, color transitions between sections, subtle grid/dot/grain textures, and frosted or elevated surface layers visible in this image, and explicitly state that the background must be recreated 1:1 in code to match the image identically without using a flat background screenshot.\n\
            3. **3D Objects & Component Rotation / Animation in This Image**: If any 3D object, centerpiece prop, or interactive component is visible in this image, specify in pure human language how it should be constructed as an interactive 3D object from its extracted reference image and how it must rotate, float, and animate on idle, scroll, and pointer hover.\n\n\
            Return ONLY valid JSON matching:\n\
            {{\n\
              \"missing_assets\": [\n\
                {{\n\
                  \"name\": \"string\",\n\
                  \"aspect_ratio\": \"1:1\",\n\
                  \"direction\": \"string\",\n\
                  \"image_prompt\": \"string\",\n\
                  \"asset_instructions\": \"string\"\n\
                }}\n\
              ],\n\
              \"coded_separators_and_backgrounds\": [\n\
                \"Pure human-language specification of the exact separators/dividers in this image to recreate 1:1 in code...\",\n\
                \"Pure human-language specification of the exact background layers, gradients, and glows in this image to recreate 1:1 in code...\"\n\
              ]\n\
            }}"
        );

        let messages = vec![
            serde_json::json!({
                "role": "system",
                "content": "You are Design Gen's post-render visual asset & code-split auditor. Inspect rendered UI images to: (1) identify every true brand logo, 3D decorative prop, custom illustration, or icon set that needs a separate transparent PNG, and (2) describe in pure human language (with zero code snippets) how the exact section separators, dividers, backgrounds, and 3D object rotations visible in the image must be recreated 1:1 in code. Return valid JSON only."
            }),
            serde_json::json!({
                "role": "user",
                "content": [
                    { "type": "text", "text": prompt },
                    { "type": "image_url", "image_url": { "url": rendered_image_data_url } }
                ]
            }),
        ];

        let (raw_text, _) = self
            .call_chat_completion(api_key, planner_model, messages)
            .await?;
        let cleaned = Self::extract_json_object(&raw_text);
        let parsed: serde_json::Value = serde_json::from_str(cleaned).unwrap_or_default();
        let mut out = Vec::new();
        if let Some(arr) = parsed.get("missing_assets").and_then(|v| v.as_array()) {
            for item in arr.iter().take(5) {
                let name = item
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string();
                let image_prompt = item
                    .get("image_prompt")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .trim()
                    .to_string();
                if name.is_empty()
                    || image_prompt.is_empty()
                    || crate::models::is_placeholder_or_card_asset_title(&name)
                {
                    continue;
                }
                let aspect_ratio = item
                    .get("aspect_ratio")
                    .and_then(|v| v.as_str())
                    .map(|s| s.trim().to_string())
                    .filter(|s| {
                        matches!(s.as_str(), "1:1" | "4:3" | "3:4" | "16:9" | "3:2" | "2:3")
                    })
                    .unwrap_or_else(|| "1:1".to_string());
                let direction = item
                    .get("direction")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Extracted transparent visual asset from rendered UI")
                    .trim()
                    .to_string();
                let asset_instructions = item
                    .get("asset_instructions")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Place this transparent PNG in public/assets/ and position it in the corresponding UI section.")
                    .trim()
                    .to_string();
                out.push(DiscoveredVisualAsset {
                    name,
                    aspect_ratio,
                    direction,
                    image_prompt,
                    asset_instructions,
                });
            }
        }
        let mut coded_separators_and_backgrounds = Vec::new();
        if let Some(arr) = parsed
            .get("coded_separators_and_backgrounds")
            .and_then(|v| v.as_array())
        {
            for v in arr.iter().take(6) {
                if let Some(s) = v.as_str() {
                    let trimmed = s.trim();
                    if !trimmed.is_empty() {
                        coded_separators_and_backgrounds.push(trimmed.to_string());
                    }
                }
            }
        }
        Ok(RenderedUiAuditResult {
            missing_assets: out,
            coded_separators_and_backgrounds,
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn run_chat_turn(
        &self,
        api_key: &str,
        planner_model: &str,
        project_meta: &ProjectMetaSummary,
        brand: &BrandSpec,
        existing_tabs: &[ExistingTabSummary],
        recent_messages: &[(String, String)], // (role, content)
        user_message: &str,
        attached_image_data_urls: &[String],
        active_tab_key: Option<&str>,
        force_generate: bool,
    ) -> AppResult<(ChatTurnDecision, Option<i64>)> {
        let tabs_block = if existing_tabs.is_empty() {
            "None yet (this thread has no generated tabs yet).".to_string()
        } else {
            existing_tabs
                .iter()
                .map(|t| {
                    format!(
                        "- tab_key: \"{}\" | tab_label: \"{}\" | latest_output_id: \"{}\" | kind: {} | viewport: {} | aspect_ratio: {} | versions: {}",
                        t.tab_key,
                        t.tab_label,
                        t.latest_output_id,
                        t.kind,
                        t.viewport_intent,
                        t.aspect_ratio,
                        t.version_count
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        };

        let history_block = if recent_messages.is_empty() {
            "No prior messages in this thread.".to_string()
        } else {
            recent_messages
                .iter()
                .map(|(role, content)| format!("[{}]: {}", role.to_uppercase(), content))
                .collect::<Vec<_>>()
                .join("\n")
        };

        let style_name = brand
            .design_style
            .as_ref()
            .map(|s| format!("#{} {} ({})", s.style_number, s.style_name, s.definition))
            .unwrap_or_else(|| format!("{} — {}", brand.brand_personality, brand.design_direction));

        let user_prompt_text = format!(
            "--- PROJECT & BRAND CONTEXT (DESIGN.MD) ---\n\
Project Name: {}\n\
Project Description: {}\n\
Project Purpose: {} | Industry: {}\n\
Target Audience: {} | Audience Needs: {}\n\
Primary User / Conversion Action: {}\n\
Chosen Web Design Style: {}\n\
Brand Personality & Direction: {} — {}\n\
Color Palette (Dark/Light): Dark Primary={} Accent={} Bg={} Surface={} | Light Primary={} Accent={} Bg={}\n\
Typography: Heading='{}', Body='{}', Mono='{}'\n\
Icon Style: {} | Button Treatment: {}\n\n\
--- EXISTING CANVAS TABS IN THIS THREAD ---\n\
{}\n\
Currently Active Tab on User's Screen: {}\n\n\
--- RECENT CHAT HISTORY IN THIS THREAD ---\n\
{}\n\n\
--- CURRENT USER MESSAGE ---\n\
User Message: {}\n\
Attached Images in This Turn: {}\n\
force_generate (User clicked Generate Now or confirmed): {}\n\n\
CRITICAL REMINDER: Ask exactly ONE unresolved question with exactly THREE answers in quick_replies. Use the full conversation to adapt the next question after each answer. An answer is not a command to generate. Return trigger_run only when the brief is clear, combining the original request and all answers. Never repeat resolved questions or present a numbered question list.\n\
Respond ONLY with the JSON object matching the ChatTurnDecision schema.",
            project_meta.name,
            project_meta.description,
            project_meta.purpose,
            project_meta.industry,
            project_meta.target_audience,
            project_meta.audience_needs,
            project_meta.desired_user_action,
            style_name,
            brand.brand_personality,
            brand.design_direction,
            brand.colors.dark.primary,
            brand.colors.dark.accent,
            brand.colors.dark.background,
            brand.colors.dark.surface,
            brand.colors.light.primary,
            brand.colors.light.accent,
            brand.colors.light.background,
            brand.typography.heading_family,
            brand.typography.body_family,
            brand.typography.mono_family,
            brand.layout_tokens.icon_style,
            brand.layout_tokens.button_treatment,
            tabs_block,
            active_tab_key.unwrap_or("none"),
            history_block,
            user_message,
            attached_image_data_urls.len(),
            force_generate
        );

        let mut content_parts = vec![serde_json::json!({
            "type": "text",
            "text": user_prompt_text
        })];

        for durl in attached_image_data_urls.iter().take(4) {
            if !durl.is_empty() {
                content_parts.push(serde_json::json!({
                    "type": "image_url",
                    "image_url": { "url": durl }
                }));
            }
        }

        let messages = vec![
            serde_json::json!({
                "role": "system",
                "content": CHAT_ROUTER_SYSTEM_PROMPT_V1
            }),
            serde_json::json!({
                "role": "user",
                "content": content_parts
            }),
        ];

        let (raw_text, cost) = self
            .call_chat_completion(api_key, planner_model, messages)
            .await?;

        let cleaned = Self::extract_json_object(&raw_text);
        let mut decision: ChatTurnDecision = serde_json::from_str(cleaned).map_err(|_| {
            AppError::ProviderError("The AI returned an unreadable reply. Please try again.".into())
        })?;

        if force_generate && decision.action != "trigger_run" {
            decision.action = "trigger_run".to_string();
            if decision.run_spec.is_none() {
                let tk = active_tab_key
                    .map(normalize_tab_key)
                    .unwrap_or_else(|| "component".to_string());
                decision.run_spec = Some(ChatRunSpec {
                    mode: "component".to_string(),
                    scope_preset: "isolated_component".to_string(),
                    target_tab_key: tk.clone(),
                    target_tab_label: tk,
                    target_existing_output_id: None,
                    is_highlight_edit: false,
                    quality_override: None,
                    aspect_ratio: "16:9".to_string(),
                    viewport_intent: "desktop".to_string(),
                    refined_prompt: user_message.to_string(),
                });
            }
        }

        enforce_no_code_in_chat_decision(&mut decision, user_message, force_generate);
        validate_chat_turn(&decision)?;

        Ok((decision, cost))
    }
}

pub fn validate_chat_turn(decision: &ChatTurnDecision) -> AppResult<()> {
    if !matches!(
        decision.action.as_str(),
        "ask_clarification" | "reply_only" | "trigger_run"
    ) {
        return Err(AppError::ProviderError(
            "The AI returned an unknown action. Please try again.".into(),
        ));
    }
    if decision.action != "ask_clarification" {
        return Ok(());
    }
    let options: HashSet<String> = decision
        .quick_replies
        .iter()
        .map(|option| option.trim().to_lowercase())
        .collect();
    let is_single_question = decision.assistant_reply.matches('?').count() == 1
        && decision.assistant_reply.trim_end().ends_with('?');
    if !is_single_question
        || decision.quick_replies.len() != 3
        || options.len() != 3
        || options.contains("")
        || decision.run_spec.is_some()
    {
        return Err(AppError::ProviderError(
            "The AI did not return one question with three distinct answers. Please try again."
                .into(),
        ));
    }
    Ok(())
}

pub fn enforce_no_code_in_chat_decision(
    decision: &mut ChatTurnDecision,
    user_message: &str,
    force_generate: bool,
) {
    let lower_user = user_message.to_ascii_lowercase();
    let asks_for_code = lower_user.contains("write code")
        || lower_user.contains("write the code")
        || lower_user.contains("give me the code")
        || lower_user.contains("give me code")
        || lower_user.contains("write html")
        || lower_user.contains("write css")
        || lower_user.contains("write react")
        || lower_user.contains("tailwind code")
        || lower_user.contains("react code")
        || lower_user.contains("html code")
        || lower_user.contains("code this for me")
        || lower_user.contains("can you code");

    let reply_has_code = decision.assistant_reply.contains("```")
        || decision.assistant_reply.contains("<div")
        || decision.assistant_reply.contains("<section")
        || decision.assistant_reply.contains("<button")
        || decision.assistant_reply.contains("export default function")
        || decision.assistant_reply.contains("import React");

    if reply_has_code || (asks_for_code && !force_generate && decision.action != "trigger_run") {
        decision.assistant_reply = "I cannot write code—my purpose here is strictly to generate visual design images, UI mockups, and transparent assets for an external AI coding agent like Claude, GPT, or Gemini to turn into code (using the 'Copy Handoff' button on the canvas). Would you like me to generate the visual design image for you first?".to_string();
        if asks_for_code && !force_generate {
            decision.action = "reply_only".to_string();
            decision.run_spec = None;
            if decision.quick_replies.is_empty() {
                decision.quick_replies = vec![
                    "Generate Desktop Design (16:9)".to_string(),
                    "Generate Mobile Design (9:16)".to_string(),
                    "Generate Full Website + Assets".to_string(),
                ];
            }
        }
    }
}

// ============================================================================
// Explicitly Labeled Deterministic Test Fixture Adapter (Section 18)
// ============================================================================

pub struct TestFixtureAdapter;

impl TestFixtureAdapter {
    pub fn build_fixture_plan(ctx: &BuiltGenerationContext) -> PlannerPlan {
        let mut all_ref_ids: Vec<String> = Vec::new();
        if let Some(ref cs) = ctx.current_screenshot {
            all_ref_ids.push(cs.asset_id.clone());
        }
        if let Some(ref prev) = ctx.selected_previous_output {
            all_ref_ids.push(prev.asset_id.clone());
        }
        for att in &ctx.explicit_attachments {
            all_ref_ids.push(att.asset_id.clone());
        }

        let mut colors = HashMap::new();
        colors.insert(
            "primary".to_string(),
            ctx.brand_snapshot.colors.light.primary.clone(),
        );
        colors.insert(
            "accent".to_string(),
            ctx.brand_snapshot.colors.light.accent.clone(),
        );
        colors.insert(
            "background".to_string(),
            ctx.brand_snapshot.colors.light.background.clone(),
        );
        colors.insert(
            "surface".to_string(),
            ctx.brand_snapshot.colors.light.surface.clone(),
        );
        colors.insert(
            "text".to_string(),
            ctx.brand_snapshot.colors.light.text.clone(),
        );
        colors.insert(
            "border".to_string(),
            ctx.brand_snapshot.colors.light.border.clone(),
        );

        let mut typography = HashMap::new();
        typography.insert(
            "heading".to_string(),
            ctx.brand_snapshot.typography.heading_family.clone(),
        );
        typography.insert(
            "body".to_string(),
            ctx.brand_snapshot.typography.body_family.clone(),
        );
        typography.insert(
            "mono".to_string(),
            ctx.brand_snapshot.typography.mono_family.clone(),
        );

        let shared_design_spec = SharedDesignSpec {
            colors,
            typography,
            spacing: HashMap::from([(
                "scale".to_string(),
                ctx.brand_snapshot.layout_tokens.spacing_scale.clone(),
            )]),
            radii: HashMap::from([(
                "default".to_string(),
                ctx.brand_snapshot.layout_tokens.radii.clone(),
            )]),
            shadows: HashMap::from([(
                "card".to_string(),
                ctx.brand_snapshot.layout_tokens.shadows.clone(),
            )]),
            layout_rules: vec![
                "Maintain 24px internal card padding and clear typographic rhythm".to_string(),
                "STICKY VS NON-STICKY: Shared top navbar MUST be position: sticky; top: 0; z-index: 50 with scroll-triggered backdrop blur; hero banners, pricing cards, feature grids, and footers MUST remain non-sticky in normal document flow".to_string(),
                "Preserve high-contrast primary action affordance".to_string(),
            ],
            asset_rules: vec![
                "Reuse uploaded original vector/raster brand assets without modification"
                    .to_string(),
                "CHROMA-KEY & PERFECT CROP: Isolate standalone icons, badges, 3D elements, and cutout illustrations on a solid #00FF00 (or #FF00FF) chroma-key backdrop with 12-15% margin, remove chroma key with edge de-spill, and crop tightly to the exact pixel bounding box".to_string(),
            ],
        };

        let requirements = vec![
            PlanRequirement {
                id: "req_1".to_string(),
                description: format!("Fulfill user request: {}", ctx.user_request),
                source: "user".to_string(),
            },
            PlanRequirement {
                id: "req_2".to_string(),
                description: format!(
                    "Follow {} brand direction with {} typography and {} primary accent",
                    ctx.brand_snapshot.brand_personality,
                    ctx.brand_snapshot.typography.heading_family,
                    ctx.brand_snapshot.colors.light.primary
                ),
                source: "brand".to_string(),
            },
        ];

        let aspect = ctx.model_settings.aspect_ratio.clone();
        let default_tab_key = ctx
            .model_settings
            .target_tab_key
            .clone()
            .unwrap_or_else(|| {
                if ctx.mode == "full_page" {
                    "full-website".to_string()
                } else if ctx.mode == "asset" {
                    "icons-assets".to_string()
                } else {
                    normalize_tab_key(&ctx.model_settings.scope_preset)
                }
            });
        let default_tab_label = ctx
            .model_settings
            .target_tab_label
            .clone()
            .unwrap_or_else(|| {
                if ctx.mode == "full_page" {
                    "Full Website".to_string()
                } else if ctx.mode == "asset" {
                    "Icons & Assets".to_string()
                } else {
                    "Component".to_string()
                }
            });

        if ctx.mode == "full_page" {
            // Coordinated set with dependency DAG:
            // 1. Standalone Hero Foreground Asset (transparent, depends on Homepage Anchor or standalone)
            // 2. Shared Navigation Bar Component (no deps)
            // 3. Homepage Desktop Overview [Visual Anchor] (depends on Hero Asset + Navbar)
            // 4. Pricing Page Desktop Overview (depends on Homepage Anchor + Navbar)
            // 5. Signup Modal Interactive State (depends on Homepage Anchor)
            let items = vec![
                PlanItem {
                    id: "asset_hero_bg".to_string(),
                    kind: "asset".to_string(),
                    name: "Architectural Hero Foreground Asset (Transparent)".to_string(),
                    tab_key: default_tab_key.clone(),
                    tab_label: default_tab_label.clone(),
                    page_route: Some("/".to_string()),
                    variant_index: 1,
                    viewport: ViewportSpec {
                        intent: "desktop".to_string(),
                        css_width: 1440,
                    },
                    aspect_ratio: "16:9".to_string(),
                    transparent_background: true,
                    extracted_from_item_id: None,
                    target_existing_output_id: None,
                    quality_override: None,
                    requirements_covered: vec!["req_1".to_string(), "req_2".to_string()],
                    direction: "Sculpted 3D architectural element isolated on a 100% native transparent background".to_string(),
                    exact_copy: vec![],
                    reference_asset_ids: all_ref_ids.clone(),
                    depends_on: vec![],
                    image_prompt: "Sculpted architectural hero element isolated on a 100% transparent background".to_string(),
                    implementation_notes: vec!["Serve as transparent PNG/WebP hero visual layered over CSS background".to_string()],
                    behaviors: vec!["Subtle floating hover transition".to_string()],
                    responsive_rules: vec!["Stack below hero copy on viewports < 768px".to_string()],
                    asset_instructions: vec!["Layer transparent PNG asset over the hero background container".to_string()],
                    is_visual_anchor: false,
                },
                PlanItem {
                    id: "comp_shared_navbar".to_string(),
                    kind: "component".to_string(),
                    name: "Shared Responsive Navbar".to_string(),
                    tab_key: "navbar".to_string(),
                    tab_label: "Navbar".to_string(),
                    page_route: None,
                    variant_index: 2,
                    viewport: ViewportSpec {
                        intent: "desktop".to_string(),
                        css_width: 1280,
                    },
                    aspect_ratio: "16:9".to_string(),
                    transparent_background: false,
                    extracted_from_item_id: None,
                    target_existing_output_id: None,
                    quality_override: None,
                    requirements_covered: vec!["req_1".to_string(), "req_2".to_string()],
                    direction: "Canonical top navigation bar with brand logo on left, links in center, and primary CTA on right".to_string(),
                    exact_copy: vec![
                        ctx.project_meta.name.clone(),
                        "Features".to_string(),
                        "Pricing".to_string(),
                        "Start creating".to_string(),
                    ],
                    reference_asset_ids: all_ref_ids.clone(),
                    depends_on: vec![],
                    image_prompt: "Clean horizontal navigation bar component on warm stone surface".to_string(),
                    implementation_notes: vec!["Use semantic <header> and <nav aria-label='Main'>".to_string()],
                    behaviors: vec!["Sticky top header with keyboard focus rings".to_string()],
                    responsive_rules: vec!["Collapse nav links into accessible mobile menu drawer under 768px".to_string()],
                    asset_instructions: vec!["Reuse original uploaded brand logo SVG/PNG in header".to_string()],
                    is_visual_anchor: false,
                },
                PlanItem {
                    id: "page_home_desktop".to_string(),
                    kind: "page".to_string(),
                    name: "Homepage Overview (Visual Anchor)".to_string(),
                    tab_key: default_tab_key.clone(),
                    tab_label: default_tab_label.clone(),
                    page_route: Some("/".to_string()),
                    variant_index: 3,
                    viewport: ViewportSpec {
                        intent: "desktop".to_string(),
                        css_width: 1280,
                    },
                    aspect_ratio: aspect.clone(),
                    transparent_background: false,
                    extracted_from_item_id: None,
                    target_existing_output_id: None,
                    quality_override: None,
                    requirements_covered: vec!["req_1".to_string(), "req_2".to_string()],
                    direction: "Coordinated editorial homepage combining the shared navbar, hero section with architectural asset, benefits grid, and footer".to_string(),
                    exact_copy: vec![
                        "Ideas to reality, faster.".to_string(),
                        "Start creating".to_string(),
                    ],
                    reference_asset_ids: all_ref_ids.clone(),
                    depends_on: vec!["asset_hero_bg".to_string(), "comp_shared_navbar".to_string()],
                    image_prompt: "Full desktop homepage overview following calm editorial brand system".to_string(),
                    implementation_notes: vec!["12-column CSS grid container with 1200px max width".to_string()],
                    behaviors: vec!["Primary CTA triggers signup modal".to_string()],
                    responsive_rules: vec!["1-column stack on mobile, 3-column benefits grid on desktop".to_string()],
                    asset_instructions: vec!["Embed asset_hero_bg in hero split container".to_string()],
                    is_visual_anchor: true,
                },
                PlanItem {
                    id: "page_pricing_desktop".to_string(),
                    kind: "page".to_string(),
                    name: "Pricing Page Overview".to_string(),
                    tab_key: "pricing".to_string(),
                    tab_label: "Pricing Page".to_string(),
                    page_route: Some("/pricing".to_string()),
                    variant_index: 4,
                    viewport: ViewportSpec {
                        intent: "desktop".to_string(),
                        css_width: 1280,
                    },
                    aspect_ratio: aspect.clone(),
                    transparent_background: false,
                    extracted_from_item_id: None,
                    target_existing_output_id: None,
                    quality_override: None,
                    requirements_covered: vec!["req_1".to_string(), "req_2".to_string()],
                    direction: "Coordinated pricing page reusing shared navbar and matching the homepage visual anchor tokens".to_string(),
                    exact_copy: vec![
                        "Simple, transparent pricing".to_string(),
                        "Monthly".to_string(),
                        "Annual · Save 20%".to_string(),
                        "Pro — $19 / month".to_string(),
                    ],
                    reference_asset_ids: all_ref_ids.clone(),
                    depends_on: vec!["page_home_desktop".to_string(), "comp_shared_navbar".to_string()],
                    image_prompt: "Coordinated desktop pricing page with annual billing toggle and emphasized Pro tier".to_string(),
                    implementation_notes: vec!["Accessible billing period switch updating plan prices".to_string()],
                    behaviors: vec!["Toggle updates displayed monthly/annual price dynamically".to_string()],
                    responsive_rules: vec!["Cards stack vertically under 900px".to_string()],
                    asset_instructions: vec![],
                    is_visual_anchor: false,
                },
                PlanItem {
                    id: "state_signup_modal".to_string(),
                    kind: "state".to_string(),
                    name: "Signup Modal State".to_string(),
                    tab_key: "signup-modal".to_string(),
                    tab_label: "Signup Modal".to_string(),
                    page_route: Some("/".to_string()),
                    variant_index: 5,
                    viewport: ViewportSpec {
                        intent: "desktop".to_string(),
                        css_width: 640,
                    },
                    aspect_ratio: "4:3".to_string(),
                    transparent_background: false,
                    extracted_from_item_id: None,
                    target_existing_output_id: None,
                    quality_override: None,
                    requirements_covered: vec!["req_1".to_string(), "req_2".to_string()],
                    direction: "Interactive modal state triggered by the primary CTA, sharing the exact typography and button tokens".to_string(),
                    exact_copy: vec![
                        "Create your workspace".to_string(),
                        "Continue with email".to_string(),
                    ],
                    reference_asset_ids: all_ref_ids,
                    depends_on: vec!["page_home_desktop".to_string()],
                    image_prompt: "Focused signup modal dialog with email input and burnt-orange submit button".to_string(),
                    implementation_notes: vec!["Use native <dialog> or Radix Dialog with focus trap and Esc close".to_string()],
                    behaviors: vec!["Light-dismiss on backdrop click and Esc key".to_string()],
                    responsive_rules: vec!["Full-width bottom sheet on mobile viewports".to_string()],
                    asset_instructions: vec![],
                    is_visual_anchor: false,
                },
            ];

            return PlannerPlan {
                schema_version: 1,
                mode: "full_page".to_string(),
                request_summary: format!("Coordinated full-page design set for: {}", ctx.user_request),
                requirements,
                assumptions: vec![
                    "Shared navigation component is canonical across / and /pricing.".to_string(),
                    "Homepage desktop overview serves as the visual anchor for dependent pages and states.".to_string(),
                ],
                shared_design_spec,
                items,
                shared_components: vec![SharedComponentSpec {
                    id: "shared_nav".to_string(),
                    name: "Shared Responsive Navbar".to_string(),
                    canonical_item_id: "comp_shared_navbar".to_string(),
                    used_on_routes: vec!["/".to_string(), "/pricing".to_string()],
                    description: "Top navigation header shared across all marketing pages".to_string(),
                }],
                page_composition: vec![
                    PageCompositionSpec {
                        page_name: "Homepage".to_string(),
                        route: "/".to_string(),
                        viewport_targets: vec!["desktop (1280px)".to_string(), "mobile (390px)".to_string()],
                        ordered_sections: vec![
                            "Shared Responsive Navbar".to_string(),
                            "Editorial Hero with Architectural Asset".to_string(),
                            "Core Benefits Grid".to_string(),
                        ],
                        shared_component_ids: vec!["shared_nav".to_string()],
                        required_ctas: vec!["Start creating".to_string()],
                        states: vec!["Signup Modal State".to_string()],
                    },
                    PageCompositionSpec {
                        page_name: "Pricing".to_string(),
                        route: "/pricing".to_string(),
                        viewport_targets: vec!["desktop (1280px)".to_string(), "mobile (390px)".to_string()],
                        ordered_sections: vec![
                            "Shared Responsive Navbar".to_string(),
                            "Billing Period Toggle & Tier Cards".to_string(),
                        ],
                        shared_component_ids: vec!["shared_nav".to_string()],
                        required_ctas: vec!["Start creating".to_string()],
                        states: vec!["Annual billing active state".to_string()],
                    },
                ],
            };
        }

        // Component mode or Asset mode: 4 distinct directions
        let directions = if ctx.mode == "asset" {
            [
                (
                    "direction_1",
                    "Sunlit Architectural Concrete",
                    "Warm editorial daylight casting soft diagonal geometry across travertine stone with generous left negative space",
                ),
                (
                    "direction_2",
                    "Tactile Linen & Terracotta Geometry",
                    "Layered organic paper and terracotta forms with subtle grain and balanced focal composition",
                ),
                (
                    "direction_3",
                    "Minimal Atmospheric Gradient",
                    "Quiet stone-to-ivory tonal transition with delicate micro-texture and high contrast overlay zones",
                ),
                (
                    "direction_4",
                    "Bold Espresso & Burnt Amber Archway",
                    "Dramatic dark architectural archway illuminated by warm golden-hour rim lighting",
                ),
            ]
        } else {
            [
                (
                    "direction_1",
                    "Editorial",
                    "Split two-column editorial card pairing warm ivory typography and burnt-orange CTA on the left with an architectural photography panel on the right",
                ),
                (
                    "direction_2",
                    "Structured",
                    "Clean utility-first layout featuring a top-right Monthly/Annual Save 20% pill switch, bordered feature rows with right-aligned descriptions, and a full-width primary CTA",
                ),
                (
                    "direction_3",
                    "Minimal",
                    "Focused single-column centered hierarchy on a crisp off-white surface with generous whitespace, clear $19/month price display, and prominent CTA",
                ),
                (
                    "direction_4",
                    "Bold",
                    "High-contrast dark charcoal card with warm sunlit archway imagery, serif editorial tagline 'Ideas to reality, faster.', and vibrant burnt-orange CTA",
                ),
            ]
        };

        let mut items: Vec<PlanItem> = directions
            .iter()
            .enumerate()
            .take(if ctx.model_settings.single_focused_output { 1 } else { 4 })
            .map(|(idx, (id, name, dir))| {
                let is_extracted_asset = ctx.mode == "component" && idx > 0;
                PlanItem {
                id: id.to_string(),
                kind: if is_extracted_asset {
                    "asset".to_string()
                } else {
                    ctx.mode.clone()
                },
                name: if ctx.model_settings.single_focused_output {
                    default_tab_label.clone()
                } else {
                    name.to_string()
                },
                tab_key: default_tab_key.clone(),
                tab_label: default_tab_label.clone(),
                page_route: None,
                variant_index: (idx + 1) as u32,
                viewport: ViewportSpec {
                    intent: ctx
                        .model_settings
                        .viewport_intent
                        .clone()
                        .unwrap_or_else(|| "desktop".to_string()),
                    css_width: 1280,
                },
                aspect_ratio: aspect.clone(),
                transparent_background: ctx.mode == "asset" || is_extracted_asset,
                extracted_from_item_id: if is_extracted_asset {
                    Some("direction_1".to_string())
                } else {
                    None
                },
                target_existing_output_id: ctx.model_settings.target_existing_output_id.clone(),
                quality_override: if !ctx.model_settings.highlight_annotations.is_empty() {
                    Some("medium".to_string())
                } else {
                    None
                },
                requirements_covered: vec!["req_1".to_string(), "req_2".to_string()],
                direction: dir.to_string(),
                exact_copy: vec![
                    ctx.project_meta.name.clone(),
                    "Pro".to_string(),
                    "$19 / month".to_string(),
                    "Unlimited projects".to_string(),
                    "Advanced AI tools".to_string(),
                    "Priority support".to_string(),
                    "Start creating".to_string(),
                ],
                reference_asset_ids: all_ref_ids.clone(),
                depends_on: vec![],
                image_prompt: format!("{} — {}", name, dir),
                implementation_notes: vec![
                    "Build with semantic HTML <section> / <article> and accessible ARIA attributes"
                        .to_string(),
                    "POSITIONING (STICKY VS NON-STICKY): Keep card container position: relative in normal flow (non-sticky); only parent page top navbar is sticky top-0 z-50"
                        .to_string(),
                    "ANIMATIONS & MOTION: Transition transform, background-color, border-color, and box-shadow over 180ms cubic-bezier(0.22, 1, 0.36, 1) with @media (prefers-reduced-motion: reduce) fallback"
                        .to_string(),
                ],
                behaviors: vec![
                    "Annual billing toggle switches price state with aria-checked".to_string(),
                    format!(
                        "HOVER & CLICK STATES: Primary CTA ({}) lifts translateY(-2px) with darkened hover color on :hover, depresses scale(0.98) on :active click, and shows 2px solid {} focus ring on :focus-visible",
                        ctx.brand_snapshot.colors.light.primary,
                        ctx.brand_snapshot.colors.light.primary
                    ),
                ],
                responsive_rules: vec![
                    "Collapse side-by-side split panels into a single vertical column below 640px"
                        .to_string(),
                ],
                asset_instructions: vec![
                    "Use original uploaded brand logo file; for standalone icons or foreground cutouts, render with native transparent background (background='transparent')"
                        .to_string(),
                ],
                is_visual_anchor: idx == 0,
                }
            })
            .collect();

        let lower_req = ctx.user_request.to_ascii_lowercase();
        if ctx.model_settings.single_focused_output
            && ctx.mode == "component"
            && (lower_req.contains("hero")
                || lower_req.contains("extract")
                || lower_req.contains("3d")
                || lower_req.contains("illustration"))
        {
            items.push(PlanItem {
                id: "direction_1_extracted_asset".to_string(),
                kind: "asset".to_string(),
                name: format!("{} — Extracted Transparent Foreground Asset", default_tab_label),
                tab_key: default_tab_key.clone(),
                tab_label: default_tab_label.clone(),
                page_route: None,
                variant_index: 2,
                viewport: ViewportSpec {
                    intent: "isolated".to_string(),
                    css_width: 1024,
                },
                aspect_ratio: "1:1".to_string(),
                transparent_background: true,
                extracted_from_item_id: Some("direction_1".to_string()),
                target_existing_output_id: None,
                quality_override: None,
                requirements_covered: vec!["req_1".to_string()],
                direction: "Foreground illustration/3D asset extracted directly from the Hero composite onto a 100% transparent background".to_string(),
                exact_copy: vec![],
                reference_asset_ids: vec![],
                depends_on: vec!["direction_1".to_string()],
                image_prompt: "Extract only the foreground visual artwork from the prerequisite Hero reference image onto a 100% transparent background".to_string(),
                implementation_notes: vec![
                    "Layer this transparent PNG over the coded Hero background container using flex/grid or absolute positioning".to_string(),
                ],
                behaviors: vec!["Subtle float and hover scale(1.02) transition".to_string()],
                responsive_rules: vec!["Scale down proportionally below 768px".to_string()],
                asset_instructions: vec![
                    "Place extracted transparent PNG in public/assets/ and layer over the Hero section background".to_string(),
                ],
                is_visual_anchor: false,
            });
        }

        PlannerPlan {
            schema_version: 1,
            mode: ctx.mode.clone(),
            request_summary: format!(
                "4 distinct {} directions for: {}",
                ctx.mode, ctx.user_request
            ),
            requirements,
            assumptions: vec![
                "Annual billing toggle defaults to active Save 20% state where shown.".to_string(),
            ],
            shared_design_spec,
            items,
            shared_components: vec![],
            page_composition: vec![],
        }
    }

    pub fn render_fixture_item(
        brand: &BrandSpec,
        aspect_ratio: &str,
        variant_index: u32,
        transparent: bool,
        seamless: bool,
    ) -> AppResult<RenderedImageOutput> {
        let bytes = generate_deterministic_fixture_png(
            aspect_ratio,
            variant_index,
            &brand.colors.light.primary,
            &brand.colors.light.background,
            &brand.colors.light.surface,
            transparent,
            seamless,
        )?;
        Ok(RenderedImageOutput {
            bytes,
            media_type: Some("image/png".to_string()),
            provider_request_id: format!("fixture-req-{}", Uuid::new_v4()),
            provider_name: Some("TestFixtureAdapter (Local Deterministic)".to_string()),
            duration_ms: 45,
            usage_json: serde_json::json!({
                "adapter": "test_fixture",
                "simulated": true,
                "input_tokens": 0,
                "output_tokens": 0,
                "cost": 0.0
            }),
            reported_cost_micros: Some(0),
        })
    }

    pub fn build_contextual_clarification_turn(
        project_meta: &ProjectMetaSummary,
        existing_tabs: &[ExistingTabSummary],
        user_request: &str,
        answered_questions: usize,
        force_generate: bool,
    ) -> ChatTurnDecision {
        let lower_user = user_request.to_ascii_lowercase();
        if !force_generate && answered_questions < 4 {
            let is_game = project_meta.industry.to_ascii_lowercase().contains("game");
            let (question, choices) = match answered_questions {
                0 if is_game => (
                    format!(
                        "Which interface should we design first for {}?",
                        project_meta.name
                    ),
                    ["HUD and inventory", "Character loadout", "Main menu"],
                ),
                0 => (
                    format!(
                        "What should the first design for {} include?",
                        project_meta.name
                    ),
                    [
                        "Complete landing page",
                        "Navbar and hero section",
                        "A single product screen",
                    ],
                ),
                1 => (
                    "What should be the main visual focal point?".into(),
                    [
                        "Product preview",
                        "Editorial typography",
                        "A 3D brand object",
                    ],
                ),
                2 => (
                    "Which viewport should the design target?".into(),
                    ["Desktop", "Mobile", "Desktop and mobile"],
                ),
                _ => (
                    "Which reusable assets should accompany the design?".into(),
                    ["Logo and icon sheet", "Logo only", "No separate assets"],
                ),
            };
            return ChatTurnDecision {
                action: "ask_clarification".into(),
                assistant_reply: question,
                quick_replies: choices.into_iter().map(String::from).collect(),
                run_spec: None,
            };
        }

        let mut dec = Self::build_fixture_chat_turn(existing_tabs, user_request, true);
        if lower_user.contains("landing")
            || lower_user.contains("full")
            || lower_user.contains("whole")
        {
            if let Some(ref mut spec) = dec.run_spec {
                spec.mode = "full_page".to_string();
                spec.scope_preset = "full_page".to_string();
                spec.target_tab_key = "full-website".to_string();
                spec.target_tab_label = "Full Website".to_string();
                spec.aspect_ratio = if lower_user.contains("mobile") {
                    "9:16".to_string()
                } else {
                    "2:3".to_string()
                };
            }
        }
        dec
    }

    pub fn build_fixture_chat_turn(
        existing_tabs: &[ExistingTabSummary],
        user_message: &str,
        force_generate: bool,
    ) -> ChatTurnDecision {
        let lower = user_message.to_ascii_lowercase();

        if !force_generate
            && (lower.contains("write code")
                || lower.contains("write the code")
                || lower.contains("give me the code")
                || lower.contains("write html")
                || lower.contains("write react")
                || lower.contains("tailwind code"))
        {
            return ChatTurnDecision {
                action: "reply_only".to_string(),
                assistant_reply: "I cannot write code—my purpose here is strictly to generate visual design images, UI mockups, and transparent assets for an external AI coding agent like Claude, GPT, or Gemini to turn into code (using the 'Copy Handoff' button on the canvas). Would you like me to generate the visual design image for you first?".to_string(),
                quick_replies: vec![
                    "Generate Desktop Design (16:9)".to_string(),
                    "Generate Mobile Design (9:16)".to_string(),
                    "Generate Full Website + Assets".to_string(),
                ],
                run_spec: None,
            };
        }

        // Check if editing an existing tab
        for tab in existing_tabs {
            if lower.contains(&tab.tab_key) || lower.contains(&tab.tab_label.to_ascii_lowercase()) {
                let is_mobile = lower.contains("mobile");
                return ChatTurnDecision {
                    action: "trigger_run".to_string(),
                    assistant_reply: if is_mobile {
                        format!(
                            "Creating Mobile (9:16) layout from your {} tab.",
                            tab.tab_label
                        )
                    } else {
                        format!(
                            "Switching to {} tab and applying your requested edit.",
                            tab.tab_label
                        )
                    },
                    quick_replies: vec![],
                    run_spec: Some(ChatRunSpec {
                        mode: "component".to_string(),
                        scope_preset: tab.tab_key.clone(),
                        target_tab_key: if is_mobile {
                            format!("{}-mobile", tab.tab_key)
                        } else {
                            tab.tab_key.clone()
                        },
                        target_tab_label: if is_mobile {
                            format!("{} (Mobile)", tab.tab_label)
                        } else {
                            tab.tab_label.clone()
                        },
                        target_existing_output_id: Some(tab.latest_output_id.clone()),
                        is_highlight_edit: false,
                        quality_override: None,
                        aspect_ratio: if is_mobile {
                            "9:16".to_string()
                        } else {
                            tab.aspect_ratio.clone()
                        },
                        viewport_intent: if is_mobile {
                            "mobile".to_string()
                        } else {
                            tab.viewport_intent.clone()
                        },
                        refined_prompt: user_message.to_string(),
                    }),
                };
            }
        }

        let specifies_viewport =
            lower.contains("desktop") || lower.contains("mobile") || lower.contains("tablet");
        let specifies_scope = lower.contains("navbar")
            || lower.contains("hero")
            || lower.contains("full website")
            || lower.contains("whole website")
            || lower.contains("icon")
            || lower.contains("pricing")
            || lower.contains("modal")
            || lower.contains("component");

        if !force_generate && (!specifies_viewport || !specifies_scope) {
            return ChatTurnDecision {
                action: "ask_clarification".to_string(),
                assistant_reply: "• Should this be for **Desktop**, **Mobile**, or **Tablet**?\n• Do you want **just this component** or the **whole website + assets**?".to_string(),
                quick_replies: vec![
                    "Desktop Component (16:9)".to_string(),
                    "Mobile Component (9:16)".to_string(),
                    "Full Website + Assets".to_string(),
                    "Transparent Icon Sheet".to_string(),
                    "Generate Now".to_string(),
                ],
                run_spec: None,
            };
        }

        let is_full = lower.contains("full website")
            || lower.contains("whole website")
            || lower.contains("all pages");
        let is_asset =
            lower.contains("icon") && !lower.contains("navbar") && !lower.contains("hero");
        let is_mobile = lower.contains("mobile");
        let (tab_key, tab_label, scope_preset) = if is_full {
            (
                "full-website".to_string(),
                "Full Website".to_string(),
                "full_page".to_string(),
            )
        } else if lower.contains("navbar") {
            (
                "navbar".to_string(),
                "Navbar".to_string(),
                "navbar".to_string(),
            )
        } else if lower.contains("hero") {
            (
                "hero".to_string(),
                "Hero Section".to_string(),
                "section".to_string(),
            )
        } else if is_asset {
            (
                "icons".to_string(),
                "Site Icons (Transparent)".to_string(),
                "icon_set".to_string(),
            )
        } else {
            (
                "component".to_string(),
                "Component".to_string(),
                "isolated_component".to_string(),
            )
        };

        ChatTurnDecision {
            action: "trigger_run".to_string(),
            assistant_reply: format!("Generating {} on its Canvas tab now.", tab_label),
            quick_replies: vec![],
            run_spec: Some(ChatRunSpec {
                mode: if is_full {
                    "full_page".to_string()
                } else if is_asset {
                    "asset".to_string()
                } else {
                    "component".to_string()
                },
                scope_preset,
                target_tab_key: tab_key,
                target_tab_label: tab_label,
                target_existing_output_id: None,
                is_highlight_edit: false,
                quality_override: None,
                aspect_ratio: if is_mobile {
                    "9:16".to_string()
                } else if is_asset {
                    "1:1".to_string()
                } else {
                    "16:9".to_string()
                },
                viewport_intent: if is_mobile {
                    "mobile".to_string()
                } else {
                    "desktop".to_string()
                },
                refined_prompt: user_message.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod chat_question_tests {
    use super::*;

    #[test]
    fn clarification_requires_one_question_and_three_distinct_choices() {
        let mut decision = ChatTurnDecision {
            action: "ask_clarification".into(),
            assistant_reply: "Which screen should we design first?".into(),
            quick_replies: vec!["Home".into(), "Dashboard".into(), "Profile".into()],
            run_spec: None,
        };
        assert!(validate_chat_turn(&decision).is_ok());
        decision.assistant_reply = "Which screen? Which viewport?".into();
        assert!(validate_chat_turn(&decision).is_err());
        decision.assistant_reply = "Which screen?".into();
        decision.quick_replies[2] = " Home ".into();
        assert!(validate_chat_turn(&decision).is_err());
        decision.quick_replies.pop();
        assert!(validate_chat_turn(&decision).is_err());
    }
}

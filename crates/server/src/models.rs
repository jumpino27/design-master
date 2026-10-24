use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn is_valid_hex_color(s: &str) -> bool {
    let trimmed = s.trim();
    if !trimmed.starts_with('#') {
        return false;
    }
    let hex_part = &trimmed[1..];
    (hex_part.len() == 6 || hex_part.len() == 3 || hex_part.len() == 8)
        && hex_part.chars().all(|c| c.is_ascii_hexdigit())
}

pub fn extract_first_hex_color(s: &str) -> Option<String> {
    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() {
        if chars[i] == '#' {
            let mut hex = String::from("#");
            for &ch in chars.iter().skip(i + 1).take(8) {
                if ch.is_ascii_hexdigit() {
                    hex.push(ch);
                } else {
                    break;
                }
            }
            let hlen = hex.len() - 1;
            if hlen == 8 || hlen == 6 {
                return Some(hex[..=6].to_string());
            } else if hlen == 3 {
                return Some(hex);
            }
        }
    }
    None
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub primary: String,
    pub accent: String,
    pub background: String,
    pub surface: String,
    pub text: String,
    pub muted_text: String,
    pub border: String,
    pub success: String,
    pub warning: String,
    pub danger: String,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary: "#D56A3A".to_string(),
            accent: "#1E1D1B".to_string(),
            background: "#F4F1EA".to_string(),
            surface: "#FAF9F6".to_string(),
            text: "#1E1D1B".to_string(),
            muted_text: "#6E6A63".to_string(),
            border: "#E2DED5".to_string(),
            success: "#2B7A4B".to_string(),
            warning: "#B7791F".to_string(),
            danger: "#C53030".to_string(),
        }
    }
}

impl ColorPalette {
    pub fn default_dark() -> Self {
        Self {
            primary: "#E07A4B".to_string(),
            accent: "#F4F1EA".to_string(),
            background: "#181715".to_string(),
            surface: "#242321".to_string(),
            text: "#F4F1EA".to_string(),
            muted_text: "#A09B92".to_string(),
            border: "#383632".to_string(),
            success: "#38A169".to_string(),
            warning: "#D69E2E".to_string(),
            danger: "#E53E3E".to_string(),
        }
    }

    pub fn validate(&self) -> AppResult<()> {
        let fields = [
            ("primary", &self.primary),
            ("accent", &self.accent),
            ("background", &self.background),
            ("surface", &self.surface),
            ("text", &self.text),
            ("muted_text", &self.muted_text),
            ("border", &self.border),
            ("success", &self.success),
            ("warning", &self.warning),
            ("danger", &self.danger),
        ];
        for (name, val) in fields {
            if !is_valid_hex_color(val) {
                return Err(AppError::Validation(format!(
                    "Invalid hex color for '{}': '{}'",
                    name, val
                )));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandColors {
    pub light: ColorPalette,
    pub dark: ColorPalette,
}

impl Default for BrandColors {
    fn default() -> Self {
        Self {
            light: ColorPalette::default(),
            dark: ColorPalette::default_dark(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandLogoConfig {
    pub mode: String, // "uploaded" | "temporary_monogram" | "none_yet"
    pub monogram_text: Option<String>,
    pub primary_asset_id: Option<String>,
    pub mark_only_asset_id: Option<String>,
    pub light_variant_asset_id: Option<String>,
    pub dark_variant_asset_id: Option<String>,
    pub preferred_usage: String,
    pub inclusion_policy: String, // "allowed" | "required" | "never"
}

impl Default for BrandLogoConfig {
    fn default() -> Self {
        Self {
            mode: "temporary_monogram".to_string(),
            monogram_text: Some("D".to_string()),
            primary_asset_id: None,
            mark_only_asset_id: None,
            light_variant_asset_id: None,
            dark_variant_asset_id: None,
            preferred_usage: "Top-left header or card header mark with clear breathing room"
                .to_string(),
            inclusion_policy: "allowed".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandTypography {
    pub heading_family: String,
    pub body_family: String,
    pub mono_family: String,
    pub fallback_stack: String,
    pub weights: Vec<u16>,
    pub size_scale: String,
    pub line_heights: String,
    pub font_source: String, // "google_fonts" | "system" | "uploaded_woff2"
    #[serde(default)]
    pub uploaded_font_asset_ids: Vec<String>,
}

impl Default for BrandTypography {
    fn default() -> Self {
        Self {
            heading_family: "Inter".to_string(),
            body_family: "Inter".to_string(),
            mono_family: "JetBrains Mono".to_string(),
            fallback_stack: "-apple-system, BlinkMacSystemFont, 'Segoe UI', sans-serif".to_string(),
            weights: vec![400, 500, 600, 700],
            size_scale: "12px / 14px / 16px / 20px / 28px / 40px".to_string(),
            line_heights: "Heading 1.15, Body 1.5, Compact UI 1.3".to_string(),
            font_source: "google_fonts".to_string(),
            uploaded_font_asset_ids: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandLayoutTokens {
    pub spacing_scale: String,
    pub content_width: String,
    pub grid_columns: String,
    pub radii: String,
    pub borders: String,
    pub shadows: String,
    pub icon_style: String,
    pub button_treatment: String,
}

impl Default for BrandLayoutTokens {
    fn default() -> Self {
        Self {
            spacing_scale: "4px base (4, 8, 12, 16, 24, 32, 48, 64px)".to_string(),
            content_width: "1200px max container with 24px padding".to_string(),
            grid_columns: "12-column responsive grid, 24px gap".to_string(),
            radii: "8px controls, 12px cards, 9999px pills".to_string(),
            borders: "1px solid subtle warm border (#E2DED5)".to_string(),
            shadows: "0 2px 8px rgba(24, 23, 21, 0.05)".to_string(),
            icon_style: "1.75px stroke rounded line icons (Lucide style)".to_string(),
            button_treatment: "Solid high-contrast primary CTA with subtle arrow affordance"
                .to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TargetContext {
    pub target_stack: String,
    pub repository_root: String,
    pub preferred_asset_dir: String,
    pub component_library_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DesignStyleSelection {
    #[serde(default)]
    pub style_id: String,
    #[serde(default)]
    pub style_number: u32,
    #[serde(default)]
    pub style_name: String,
    #[serde(default)]
    pub definition: String,
    #[serde(default)]
    pub category: String,
    #[serde(default)]
    pub visual_rules: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub modifiers: Vec<String>,
    #[serde(default)]
    pub combined_brief: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrandSpec {
    pub brand_personality: String,
    pub design_direction: String,
    pub tone_of_voice: String,
    pub content_language: String,
    pub ui_density: String,
    pub logo: BrandLogoConfig,
    pub colors: BrandColors,
    pub typography: BrandTypography,
    pub layout_tokens: BrandLayoutTokens,
    #[serde(default)]
    pub brand_rules: Vec<String>,
    #[serde(default)]
    pub mandatory_inclusions: Vec<String>,
    #[serde(default)]
    pub forbidden_treatments: Vec<String>,
    #[serde(default)]
    pub approved_reference_asset_ids: Vec<String>,
    #[serde(default)]
    pub target_context: TargetContext,
    #[serde(default)]
    pub design_style: Option<DesignStyleSelection>,
}

impl Default for BrandSpec {
    fn default() -> Self {
        Self {
            brand_personality: "Calm editorial".to_string(),
            design_direction: "Warm architectural minimalism with crisp hierarchy, tactile off-white surfaces, and confident burnt-orange primary actions".to_string(),
            tone_of_voice: "Clear, direct, confident, and human".to_string(),
            content_language: "English".to_string(),
            ui_density: "comfortable".to_string(),
            logo: BrandLogoConfig::default(),
            colors: BrandColors::default(),
            typography: BrandTypography::default(),
            layout_tokens: BrandLayoutTokens::default(),
            brand_rules: vec![
                "Maintain WCAG AA contrast across all text and CTA controls".to_string(),
                "Use generous whitespace and clear typographic hierarchy".to_string(),
            ],
            mandatory_inclusions: vec![],
            forbidden_treatments: vec![
                "Never use neon gradients or sci-fi glowing borders".to_string(),
                "Avoid cluttered multi-font decorative headers".to_string(),
            ],
            approved_reference_asset_ids: vec![],
            target_context: TargetContext {
                target_stack: "Next.js App Router + TypeScript + Tailwind CSS".to_string(),
                repository_root: String::new(),
                preferred_asset_dir: "public/assets".to_string(),
                component_library_notes: "Accessible Radix primitives with Lucide icons".to_string(),
            },
            design_style: None,
        }
    }
}

impl BrandSpec {
    pub fn validate(&self) -> AppResult<()> {
        self.colors.light.validate()?;
        self.colors.dark.validate()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AttachmentRole {
    CurrentScreenshot,
    BrandLogo,
    IncludeAsset,
    StyleReference,
    ContentReference,
    SelectedPreviousOutput,
    FontFile,
}

impl AttachmentRole {
    pub fn as_str(&self) -> &'static str {
        match self {
            AttachmentRole::CurrentScreenshot => "current_screenshot",
            AttachmentRole::BrandLogo => "brand_logo",
            AttachmentRole::IncludeAsset => "include_asset",
            AttachmentRole::StyleReference => "style_reference",
            AttachmentRole::ContentReference => "content_reference",
            AttachmentRole::SelectedPreviousOutput => "selected_previous_output",
            AttachmentRole::FontFile => "font_file",
        }
    }

    pub fn parse(s: &str) -> AppResult<Self> {
        match s {
            "current_screenshot" => Ok(AttachmentRole::CurrentScreenshot),
            "brand_logo" => Ok(AttachmentRole::BrandLogo),
            "include_asset" => Ok(AttachmentRole::IncludeAsset),
            "style_reference" => Ok(AttachmentRole::StyleReference),
            "content_reference" => Ok(AttachmentRole::ContentReference),
            "selected_previous_output" => Ok(AttachmentRole::SelectedPreviousOutput),
            "font_file" => Ok(AttachmentRole::FontFile),
            _ => Err(AppError::Validation(format!(
                "Unsupported attachment role '{}'",
                s
            ))),
        }
    }

    pub fn planner_instruction(&self) -> &'static str {
        match self {
            AttachmentRole::CurrentScreenshot => {
                "CURRENT / OLD COMPONENT SCREENSHOT (POLISH, ENHANCE & EXTEND MODE): This screenshot is provided so you see what buttons, links, icons, inputs, labels, and controls the current component already has, and make a BETTER one! Creating a component with an old one as reference means polishing, enhancing, and redesigning it strictly in the project's Brand Kit and Chosen Web Design Style — keeping its existing controls while also adding any new buttons, new functionality, layout improvements, or text/icon treatments requested in the user's prompt."
            }
            AttachmentRole::BrandLogo => {
                "BRAND LOGO: Preserve this exact brand mark faithfully. Do not invent a replacement logo."
            }
            AttachmentRole::IncludeAsset => {
                "INCLUDE THIS ASSET: This specific asset must appear inside the generated design."
            }
            AttachmentRole::StyleReference => {
                "STYLE REFERENCE: Use only for visual mood/layout inspiration. Do NOT copy its brand identity or text."
            }
            AttachmentRole::ContentReference => {
                "CONTENT REFERENCE: Use for subject matter and factual context."
            }
            AttachmentRole::SelectedPreviousOutput => {
                "PREVIOUS COMPONENT REFERENCE (POLISH, ENHANCE & EXTEND MODE): See what buttons, controls, and actions this component currently has and make a better version — keeping its core controls while adding any new buttons, new functionality, or text/icon changes requested in the user's prompt, strictly following the project's Brand Kit and Chosen Web Design Style."
            }
            AttachmentRole::FontFile => "FONT FILE: Uploaded licensed typography file for handoff.",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedReference {
    pub asset_id: String,
    pub role: AttachmentRole,
    pub label: String,
    #[serde(default)]
    pub placement_notes: String,
    #[serde(default)]
    pub preserve_as_is: bool,
    /// Optional output ID if role == SelectedPreviousOutput
    #[serde(default)]
    pub source_output_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetModeOptions {
    pub intended_usage: String, // "background" | "texture" | "illustration" | "hero_imagery" | "product_scene" | "decorative_shape" | "icon_logo_concept"
    pub theme_context: String,  // "light" | "dark" | "both"
    pub transparent_background: bool,
    pub seamless_tiling: bool,
    pub include_logo: String, // "auto" | "always" | "never"
    #[serde(default)]
    pub composition: String,
    #[serde(default)]
    pub negative_space: String,
    #[serde(default)]
    pub forbidden_text: String,
    #[serde(default = "default_true")]
    pub is_decorative: bool,
    #[serde(default)]
    pub alt_text_intent: String,
}

fn default_true() -> bool {
    true
}

impl Default for AssetModeOptions {
    fn default() -> Self {
        Self {
            intended_usage: "hero_imagery".to_string(),
            theme_context: "light".to_string(),
            transparent_background: false,
            seamless_tiling: false,
            include_logo: "auto".to_string(),
            composition: "Balanced focal point with clean edges".to_string(),
            negative_space: "Reserve breathing room on the left for typography overlay".to_string(),
            forbidden_text: "No embedded gibberish text or watermarks".to_string(),
            is_decorative: true,
            alt_text_intent: String::new(),
        }
    }
}

impl AssetModeOptions {
    pub fn should_include_logo(&self) -> bool {
        match self.include_logo.as_str() {
            "always" => true,
            "never" => false,
            _ => {
                // "auto" determined from intended_usage: only include logo if it's an icon/logo concept
                self.intended_usage == "icon_logo_concept"
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightAnnotation {
    pub output_id: String,
    #[serde(default)]
    pub tab_key: String,
    #[serde(default)]
    pub tab_label: String,
    pub x_pct: f32,
    pub y_pct: f32,
    pub w_pct: f32,
    pub h_pct: f32,
    pub comment: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExistingTabSummary {
    pub tab_key: String,
    pub tab_label: String,
    pub latest_output_id: String,
    pub kind: String,
    pub title: String,
    pub aspect_ratio: String,
    pub viewport_intent: String,
    pub version_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRunSpec {
    #[serde(default = "default_chat_mode")]
    pub mode: String, // "component" | "full_page" | "asset"
    #[serde(default = "default_chat_scope")]
    pub scope_preset: String,
    #[serde(default)]
    pub target_tab_key: String,
    #[serde(default)]
    pub target_tab_label: String,
    #[serde(default)]
    pub target_existing_output_id: Option<String>,
    #[serde(default)]
    pub is_highlight_edit: bool,
    #[serde(default)]
    pub quality_override: Option<String>,
    #[serde(default = "default_aspect_ratio")]
    pub aspect_ratio: String,
    #[serde(default = "default_viewport_intent")]
    pub viewport_intent: String,
    #[serde(default)]
    pub refined_prompt: String,
}

fn default_chat_mode() -> String {
    "component".to_string()
}

fn default_chat_scope() -> String {
    "isolated_component".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatTurnDecision {
    pub action: String, // "ask_clarification" | "reply_only" | "trigger_run"
    pub assistant_reply: String,
    #[serde(default)]
    pub quick_replies: Vec<String>,
    #[serde(default)]
    pub run_spec: Option<ChatRunSpec>,
}

pub fn normalize_tab_key(raw: &str) -> String {
    let lower = raw.trim().to_ascii_lowercase();
    let mut slug = String::new();
    let mut prev_dash = false;
    for ch in lower.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch);
            prev_dash = false;
        } else if !prev_dash && !slug.is_empty() {
            slug.push('-');
            prev_dash = true;
        }
    }
    let trimmed = slug.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "component".to_string()
    } else {
        trimmed
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunModelSettings {
    pub planner_model: String,
    pub image_model: String,
    pub quality: String,
    pub aspect_ratio: String,
    pub output_size: String,
    pub scope_preset: String,
    pub custom_dimensions: Option<String>,
    pub review_plan_first: bool,
    pub asset_options: Option<AssetModeOptions>,
    #[serde(default)]
    pub target_tab_key: Option<String>,
    #[serde(default)]
    pub target_tab_label: Option<String>,
    #[serde(default)]
    pub target_existing_output_id: Option<String>,
    #[serde(default)]
    pub viewport_intent: Option<String>,
    #[serde(default)]
    pub highlight_annotations: Vec<HighlightAnnotation>,
    #[serde(default)]
    pub single_focused_output: bool,
}

// ============================================================================
// Structured Planner Output Schema (Section 11)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanRequirement {
    #[serde(default = "default_req_id")]
    pub id: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_req_source")]
    pub source: String, // "user" | "brand" | "screenshot_analysis"
}

fn default_req_id() -> String {
    "req_1".to_string()
}

fn default_req_source() -> String {
    "user".to_string()
}

fn default_schema_version() -> u32 {
    1
}

fn default_variant_index() -> u32 {
    1
}

fn default_aspect_ratio() -> String {
    "4:3".to_string()
}

fn default_item_kind() -> String {
    "component".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SharedDesignSpec {
    #[serde(default)]
    pub colors: HashMap<String, String>,
    #[serde(default)]
    pub typography: HashMap<String, String>,
    #[serde(default)]
    pub spacing: HashMap<String, String>,
    #[serde(default)]
    pub radii: HashMap<String, String>,
    #[serde(default)]
    pub shadows: HashMap<String, String>,
    #[serde(default)]
    pub layout_rules: Vec<String>,
    #[serde(default)]
    pub asset_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ViewportSpec {
    #[serde(default = "default_viewport_intent")]
    pub intent: String, // "desktop" | "mobile" | "tablet" | "isolated"
    #[serde(default = "default_viewport_width")]
    pub css_width: u32,
}

fn default_viewport_intent() -> String {
    "desktop".to_string()
}

fn default_viewport_width() -> u32 {
    1280
}

impl Default for ViewportSpec {
    fn default() -> Self {
        Self {
            intent: "desktop".to_string(),
            css_width: 1280,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanItem {
    pub id: String,
    #[serde(default = "default_item_kind")]
    pub kind: String, // "component" | "page" | "section" | "state" | "asset"
    pub name: String,
    #[serde(default)]
    pub tab_key: String,
    #[serde(default)]
    pub tab_label: String,
    #[serde(default)]
    pub page_route: Option<String>,
    #[serde(default = "default_variant_index")]
    pub variant_index: u32,
    #[serde(default)]
    pub viewport: ViewportSpec,
    #[serde(default = "default_aspect_ratio")]
    pub aspect_ratio: String,
    #[serde(default)]
    pub transparent_background: bool,
    #[serde(default)]
    pub extracted_from_item_id: Option<String>,
    #[serde(default)]
    pub target_existing_output_id: Option<String>,
    #[serde(default)]
    pub quality_override: Option<String>,
    #[serde(default)]
    pub requirements_covered: Vec<String>,
    #[serde(default)]
    pub direction: String,
    #[serde(default)]
    pub exact_copy: Vec<String>,
    #[serde(default)]
    pub reference_asset_ids: Vec<String>,
    #[serde(default)]
    pub depends_on: Vec<String>,
    pub image_prompt: String,
    #[serde(default)]
    pub implementation_notes: Vec<String>,
    #[serde(default)]
    pub behaviors: Vec<String>,
    #[serde(default)]
    pub responsive_rules: Vec<String>,
    #[serde(default)]
    pub asset_instructions: Vec<String>,
    #[serde(default)]
    pub is_visual_anchor: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SharedComponentSpec {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub canonical_item_id: String,
    #[serde(default)]
    pub used_on_routes: Vec<String>,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PageCompositionSpec {
    #[serde(default)]
    pub page_name: String,
    #[serde(default)]
    pub route: String,
    #[serde(default)]
    pub viewport_targets: Vec<String>,
    #[serde(default)]
    pub ordered_sections: Vec<String>,
    #[serde(default)]
    pub shared_component_ids: Vec<String>,
    #[serde(default)]
    pub required_ctas: Vec<String>,
    #[serde(default)]
    pub states: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannerPlan {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub request_summary: String,
    #[serde(default)]
    pub requirements: Vec<PlanRequirement>,
    #[serde(default)]
    pub assumptions: Vec<String>,
    #[serde(default)]
    pub shared_design_spec: SharedDesignSpec,
    #[serde(default)]
    pub items: Vec<PlanItem>,
    #[serde(default)]
    pub shared_components: Vec<SharedComponentSpec>,
    #[serde(default)]
    pub page_composition: Vec<PageCompositionSpec>,
}

fn json_val_to_string(v: &serde_json::Value) -> String {
    match v {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Null => String::new(),
        serde_json::Value::Array(arr) => arr
            .iter()
            .map(json_val_to_string)
            .filter(|s| !s.trim().is_empty())
            .collect::<Vec<_>>()
            .join(", "),
        serde_json::Value::Object(map) => {
            for key in ["description", "text", "title", "name", "value", "summary"] {
                if let Some(serde_json::Value::String(s)) = map.get(key) {
                    if !s.trim().is_empty() {
                        return s.clone();
                    }
                }
            }
            map.iter()
                .map(|(k, val)| format!("{}: {}", k, json_val_to_string(val)))
                .collect::<Vec<_>>()
                .join("; ")
        }
    }
}

fn json_val_to_string_vec(v: &serde_json::Value) -> Vec<String> {
    match v {
        serde_json::Value::Array(arr) => arr
            .iter()
            .map(json_val_to_string)
            .filter(|s| !s.trim().is_empty())
            .collect(),
        serde_json::Value::String(s) => {
            if s.trim().is_empty() {
                Vec::new()
            } else {
                vec![s.clone()]
            }
        }
        serde_json::Value::Object(map) => map
            .iter()
            .map(|(k, val)| format!("{}: {}", k, json_val_to_string(val)))
            .collect(),
        _ => Vec::new(),
    }
}

fn json_val_to_string_map(v: &serde_json::Value) -> serde_json::Map<String, serde_json::Value> {
    let mut out = serde_json::Map::new();
    match v {
        serde_json::Value::Object(map) => {
            for (k, val) in map {
                out.insert(
                    k.clone(),
                    serde_json::Value::String(json_val_to_string(val)),
                );
            }
        }
        serde_json::Value::Array(arr) => {
            for (idx, val) in arr.iter().enumerate() {
                out.insert(
                    format!("rule_{}", idx + 1),
                    serde_json::Value::String(json_val_to_string(val)),
                );
            }
        }
        _ => {}
    }
    out
}

fn json_val_to_u32(v: &serde_json::Value, default: u32) -> u32 {
    match v {
        serde_json::Value::Number(n) => n.as_u64().map(|x| x as u32).unwrap_or(default),
        serde_json::Value::String(s) => {
            let digits: String = s.chars().filter(|c| c.is_ascii_digit()).collect();
            digits.parse::<u32>().unwrap_or(default)
        }
        _ => default,
    }
}

fn json_val_to_bool(v: &serde_json::Value, default: bool) -> bool {
    match v {
        serde_json::Value::Bool(b) => *b,
        serde_json::Value::String(s) => match s.trim().to_ascii_lowercase().as_str() {
            "true" | "1" | "yes" => true,
            "false" | "0" | "no" => false,
            _ => default,
        },
        serde_json::Value::Number(n) => n.as_i64().map(|x| x != 0).unwrap_or(default),
        _ => default,
    }
}

fn json_val_to_opt_string(v: &serde_json::Value) -> serde_json::Value {
    match v {
        serde_json::Value::Null => serde_json::Value::Null,
        serde_json::Value::String(s) => {
            let t = s.trim();
            if t.is_empty() || t.eq_ignore_ascii_case("null") || t.eq_ignore_ascii_case("none") {
                serde_json::Value::Null
            } else {
                serde_json::Value::String(s.clone())
            }
        }
        other => {
            let s = json_val_to_string(other);
            if s.trim().is_empty() {
                serde_json::Value::Null
            } else {
                serde_json::Value::String(s)
            }
        }
    }
}

/// Parses a `PlannerPlan` from raw JSON text while gracefully coercing common LLM structural variations
/// (such as strings instead of structs in `page_composition`, `shared_components`, `requirements`, or `viewport`,
/// string numbers like `"1440px"`, or single strings instead of `Vec<String>`).
pub fn parse_planner_plan_tolerant(raw_json_str: &str) -> AppResult<PlannerPlan> {
    let mut val: serde_json::Value = serde_json::from_str(raw_json_str).map_err(|e| {
        AppError::Validation(format!("Planner output was not valid schema JSON: {}", e))
    })?;
    coerce_planner_plan_value(&mut val);
    serde_json::from_value(val).map_err(|e| {
        AppError::Validation(format!("Planner output was not valid schema JSON: {}", e))
    })
}

pub fn coerce_planner_plan_value(val: &mut serde_json::Value) {
    let Some(obj) = val.as_object_mut() else {
        return;
    };

    obj.insert("schema_version".to_string(), serde_json::json!(1));

    let mode_str = obj
        .get("mode")
        .map(json_val_to_string)
        .unwrap_or_else(|| "component".to_string());
    obj.insert("mode".to_string(), serde_json::Value::String(mode_str));

    let req_summary = obj
        .get("request_summary")
        .map(json_val_to_string)
        .unwrap_or_default();
    obj.insert(
        "request_summary".to_string(),
        serde_json::Value::String(req_summary),
    );

    // requirements
    let reqs_val = obj
        .get("requirements")
        .cloned()
        .unwrap_or(serde_json::Value::Array(Vec::new()));
    let mut normalized_reqs = Vec::new();
    match reqs_val {
        serde_json::Value::Array(arr) => {
            for (idx, r) in arr.iter().enumerate() {
                match r {
                    serde_json::Value::String(s) => {
                        normalized_reqs.push(serde_json::json!({
                            "id": format!("req_{}", idx + 1),
                            "description": s,
                            "source": "user"
                        }));
                    }
                    serde_json::Value::Object(m) => {
                        let id = m
                            .get("id")
                            .map(json_val_to_string)
                            .filter(|s| !s.trim().is_empty())
                            .unwrap_or_else(|| format!("req_{}", idx + 1));
                        let desc = m
                            .get("description")
                            .or_else(|| m.get("text"))
                            .or_else(|| m.get("requirement"))
                            .map(json_val_to_string)
                            .unwrap_or_default();
                        let source = m
                            .get("source")
                            .map(json_val_to_string)
                            .filter(|s| !s.trim().is_empty())
                            .unwrap_or_else(|| "user".to_string());
                        normalized_reqs.push(serde_json::json!({
                            "id": id,
                            "description": desc,
                            "source": source
                        }));
                    }
                    other => {
                        let s = json_val_to_string(other);
                        if !s.trim().is_empty() {
                            normalized_reqs.push(serde_json::json!({
                                "id": format!("req_{}", idx + 1),
                                "description": s,
                                "source": "user"
                            }));
                        }
                    }
                }
            }
        }
        serde_json::Value::String(s) if !s.trim().is_empty() => {
            normalized_reqs.push(serde_json::json!({
                "id": "req_1",
                "description": s,
                "source": "user"
            }));
        }
        _ => {}
    }
    obj.insert(
        "requirements".to_string(),
        serde_json::Value::Array(normalized_reqs),
    );

    // assumptions
    let assumptions = obj
        .get("assumptions")
        .map(json_val_to_string_vec)
        .unwrap_or_default();
    obj.insert("assumptions".to_string(), serde_json::json!(assumptions));

    // shared_design_spec
    let sds_val = obj
        .get("shared_design_spec")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));
    let mut sds_out = serde_json::Map::new();
    if let Some(sds_obj) = sds_val.as_object() {
        for map_key in ["colors", "typography", "spacing", "radii", "shadows"] {
            let map_val = sds_obj
                .get(map_key)
                .map(json_val_to_string_map)
                .unwrap_or_default();
            sds_out.insert(map_key.to_string(), serde_json::Value::Object(map_val));
        }
        for vec_key in ["layout_rules", "asset_rules"] {
            let vec_val = sds_obj
                .get(vec_key)
                .map(json_val_to_string_vec)
                .unwrap_or_default();
            sds_out.insert(vec_key.to_string(), serde_json::json!(vec_val));
        }
    }
    obj.insert(
        "shared_design_spec".to_string(),
        serde_json::Value::Object(sds_out),
    );

    // items
    let items_val = obj
        .get("items")
        .cloned()
        .unwrap_or(serde_json::Value::Array(Vec::new()));
    let mut normalized_items = Vec::new();
    if let Some(arr) = items_val.as_array() {
        for (idx, raw_item) in arr.iter().enumerate() {
            if let Some(m) = raw_item.as_object() {
                let id = m
                    .get("id")
                    .map(json_val_to_string)
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| format!("item_{}", idx + 1));
                let kind = m
                    .get("kind")
                    .map(json_val_to_string)
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "component".to_string());
                let name = m
                    .get("name")
                    .or_else(|| m.get("title"))
                    .map(json_val_to_string)
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| format!("Design Item {}", idx + 1));
                let tab_key = m.get("tab_key").map(json_val_to_string).unwrap_or_default();
                let tab_label = m
                    .get("tab_label")
                    .map(json_val_to_string)
                    .unwrap_or_default();
                let page_route = m
                    .get("page_route")
                    .map(json_val_to_opt_string)
                    .unwrap_or(serde_json::Value::Null);
                let variant_index = m
                    .get("variant_index")
                    .map(|v| json_val_to_u32(v, 1))
                    .unwrap_or(1);

                let viewport = match m.get("viewport") {
                    Some(serde_json::Value::String(s)) => {
                        let lower = s.trim().to_ascii_lowercase();
                        let width = if lower.contains("mobile") {
                            390
                        } else if lower.contains("tablet") {
                            834
                        } else {
                            1440
                        };
                        serde_json::json!({
                            "intent": lower,
                            "css_width": width
                        })
                    }
                    Some(serde_json::Value::Object(vp)) => {
                        let intent = vp
                            .get("intent")
                            .map(json_val_to_string)
                            .filter(|s| !s.trim().is_empty())
                            .unwrap_or_else(|| "desktop".to_string());
                        let default_w = if intent == "mobile" {
                            390
                        } else if intent == "tablet" {
                            834
                        } else {
                            1440
                        };
                        let css_width = vp
                            .get("css_width")
                            .or_else(|| vp.get("width"))
                            .map(|v| json_val_to_u32(v, default_w))
                            .unwrap_or(default_w);
                        serde_json::json!({
                            "intent": intent,
                            "css_width": css_width
                        })
                    }
                    _ => serde_json::json!({
                        "intent": "desktop",
                        "css_width": 1440
                    }),
                };

                let aspect_ratio = m
                    .get("aspect_ratio")
                    .map(json_val_to_string)
                    .filter(|s| !s.trim().is_empty())
                    .unwrap_or_else(|| "16:9".to_string());
                let transparent_background = m
                    .get("transparent_background")
                    .map(|v| json_val_to_bool(v, false))
                    .unwrap_or(false);
                let extracted_from_item_id = m
                    .get("extracted_from_item_id")
                    .map(json_val_to_opt_string)
                    .unwrap_or(serde_json::Value::Null);
                let target_existing_output_id = m
                    .get("target_existing_output_id")
                    .map(json_val_to_opt_string)
                    .unwrap_or(serde_json::Value::Null);
                let quality_override = m
                    .get("quality_override")
                    .map(json_val_to_opt_string)
                    .unwrap_or(serde_json::Value::Null);

                let requirements_covered = m
                    .get("requirements_covered")
                    .map(json_val_to_string_vec)
                    .unwrap_or_default();
                let direction = m
                    .get("direction")
                    .map(json_val_to_string)
                    .unwrap_or_default();
                let exact_copy = m
                    .get("exact_copy")
                    .map(json_val_to_string_vec)
                    .unwrap_or_default();
                let reference_asset_ids = m
                    .get("reference_asset_ids")
                    .map(json_val_to_string_vec)
                    .unwrap_or_default();
                let depends_on = m
                    .get("depends_on")
                    .map(json_val_to_string_vec)
                    .unwrap_or_default();
                let image_prompt = m
                    .get("image_prompt")
                    .or_else(|| m.get("prompt"))
                    .map(json_val_to_string)
                    .unwrap_or_default();
                let implementation_notes = m
                    .get("implementation_notes")
                    .map(json_val_to_string_vec)
                    .unwrap_or_default();
                let behaviors = m
                    .get("behaviors")
                    .map(json_val_to_string_vec)
                    .unwrap_or_default();
                let responsive_rules = m
                    .get("responsive_rules")
                    .map(json_val_to_string_vec)
                    .unwrap_or_default();
                let asset_instructions = m
                    .get("asset_instructions")
                    .map(json_val_to_string_vec)
                    .unwrap_or_default();
                let is_visual_anchor = m
                    .get("is_visual_anchor")
                    .map(|v| json_val_to_bool(v, false))
                    .unwrap_or(false);

                normalized_items.push(serde_json::json!({
                    "id": id,
                    "kind": kind,
                    "name": name,
                    "tab_key": tab_key,
                    "tab_label": tab_label,
                    "page_route": page_route,
                    "variant_index": variant_index,
                    "viewport": viewport,
                    "aspect_ratio": aspect_ratio,
                    "transparent_background": transparent_background,
                    "extracted_from_item_id": extracted_from_item_id,
                    "target_existing_output_id": target_existing_output_id,
                    "quality_override": quality_override,
                    "requirements_covered": requirements_covered,
                    "direction": direction,
                    "exact_copy": exact_copy,
                    "reference_asset_ids": reference_asset_ids,
                    "depends_on": depends_on,
                    "image_prompt": image_prompt,
                    "implementation_notes": implementation_notes,
                    "behaviors": behaviors,
                    "responsive_rules": responsive_rules,
                    "asset_instructions": asset_instructions,
                    "is_visual_anchor": is_visual_anchor
                }));
            }
        }
    }
    obj.insert(
        "items".to_string(),
        serde_json::Value::Array(normalized_items),
    );

    // shared_components
    let sc_val = obj
        .get("shared_components")
        .cloned()
        .unwrap_or(serde_json::Value::Array(Vec::new()));
    let mut normalized_sc = Vec::new();
    match sc_val {
        serde_json::Value::Array(arr) => {
            for (idx, elem) in arr.iter().enumerate() {
                match elem {
                    serde_json::Value::String(s) => {
                        normalized_sc.push(serde_json::json!({
                            "id": format!("shared_{}", idx + 1),
                            "name": s,
                            "canonical_item_id": "",
                            "used_on_routes": ["/"],
                            "description": s
                        }));
                    }
                    serde_json::Value::Object(m) => {
                        let id = m
                            .get("id")
                            .map(json_val_to_string)
                            .filter(|s| !s.trim().is_empty())
                            .unwrap_or_else(|| format!("shared_{}", idx + 1));
                        let name = m
                            .get("name")
                            .or_else(|| m.get("title"))
                            .map(json_val_to_string)
                            .unwrap_or_default();
                        let canonical_item_id = m
                            .get("canonical_item_id")
                            .or_else(|| m.get("item_id"))
                            .map(json_val_to_string)
                            .unwrap_or_default();
                        let used_on_routes = m
                            .get("used_on_routes")
                            .or_else(|| m.get("used_in_items"))
                            .or_else(|| m.get("routes"))
                            .map(json_val_to_string_vec)
                            .unwrap_or_default();
                        let description = m
                            .get("description")
                            .or_else(|| m.get("role"))
                            .or_else(|| m.get("notes"))
                            .map(json_val_to_string)
                            .unwrap_or_default();
                        normalized_sc.push(serde_json::json!({
                            "id": id,
                            "name": name,
                            "canonical_item_id": canonical_item_id,
                            "used_on_routes": used_on_routes,
                            "description": description
                        }));
                    }
                    other => {
                        let s = json_val_to_string(other);
                        if !s.trim().is_empty() {
                            normalized_sc.push(serde_json::json!({
                                "id": format!("shared_{}", idx + 1),
                                "name": s,
                                "canonical_item_id": "",
                                "used_on_routes": ["/"],
                                "description": s
                            }));
                        }
                    }
                }
            }
        }
        serde_json::Value::String(s) if !s.trim().is_empty() => {
            normalized_sc.push(serde_json::json!({
                "id": "shared_1",
                "name": s,
                "canonical_item_id": "",
                "used_on_routes": ["/"],
                "description": s
            }));
        }
        _ => {}
    }
    obj.insert(
        "shared_components".to_string(),
        serde_json::Value::Array(normalized_sc),
    );

    // page_composition
    let pc_val = obj
        .get("page_composition")
        .cloned()
        .unwrap_or(serde_json::Value::Array(Vec::new()));
    let mut normalized_pc = Vec::new();
    match pc_val {
        serde_json::Value::Array(arr) => {
            for elem in arr {
                match elem {
                    serde_json::Value::String(s) => {
                        normalized_pc.push(serde_json::json!({
                            "page_name": s,
                            "route": "/",
                            "viewport_targets": ["desktop"],
                            "ordered_sections": [s],
                            "shared_component_ids": [],
                            "required_ctas": [],
                            "states": []
                        }));
                    }
                    serde_json::Value::Object(m) => {
                        let page_name = m
                            .get("page_name")
                            .or_else(|| m.get("title"))
                            .or_else(|| m.get("name"))
                            .map(json_val_to_string)
                            .unwrap_or_else(|| "Full Page".to_string());
                        let route = m
                            .get("route")
                            .or_else(|| m.get("page_route"))
                            .map(json_val_to_string)
                            .unwrap_or_else(|| "/".to_string());
                        let viewport_targets = m
                            .get("viewport_targets")
                            .or_else(|| m.get("viewports"))
                            .map(json_val_to_string_vec)
                            .unwrap_or_default();
                        let ordered_sections = m
                            .get("ordered_sections")
                            .or_else(|| m.get("ordered_item_ids"))
                            .or_else(|| m.get("sections"))
                            .or_else(|| m.get("items"))
                            .map(json_val_to_string_vec)
                            .unwrap_or_default();
                        let shared_component_ids = m
                            .get("shared_component_ids")
                            .or_else(|| m.get("shared_components"))
                            .map(json_val_to_string_vec)
                            .unwrap_or_default();
                        let required_ctas = m
                            .get("required_ctas")
                            .or_else(|| m.get("ctas"))
                            .map(json_val_to_string_vec)
                            .unwrap_or_default();
                        let states = m
                            .get("states")
                            .or_else(|| m.get("layout_notes"))
                            .map(json_val_to_string_vec)
                            .unwrap_or_default();
                        normalized_pc.push(serde_json::json!({
                            "page_name": page_name,
                            "route": route,
                            "viewport_targets": viewport_targets,
                            "ordered_sections": ordered_sections,
                            "shared_component_ids": shared_component_ids,
                            "required_ctas": required_ctas,
                            "states": states
                        }));
                    }
                    other => {
                        let s = json_val_to_string(&other);
                        if !s.trim().is_empty() {
                            normalized_pc.push(serde_json::json!({
                                "page_name": s,
                                "route": "/",
                                "viewport_targets": ["desktop"],
                                "ordered_sections": [s],
                                "shared_component_ids": [],
                                "required_ctas": [],
                                "states": []
                            }));
                        }
                    }
                }
            }
        }
        serde_json::Value::String(s) if !s.trim().is_empty() => {
            normalized_pc.push(serde_json::json!({
                "page_name": s,
                "route": "/",
                "viewport_targets": ["desktop"],
                "ordered_sections": [s],
                "shared_component_ids": [],
                "required_ctas": [],
                "states": []
            }));
        }
        serde_json::Value::Object(m) => {
            let page_name = m
                .get("page_name")
                .or_else(|| m.get("title"))
                .or_else(|| m.get("name"))
                .map(json_val_to_string)
                .unwrap_or_else(|| "Full Page".to_string());
            let route = m
                .get("route")
                .or_else(|| m.get("page_route"))
                .map(json_val_to_string)
                .unwrap_or_else(|| "/".to_string());
            let ordered_sections = m
                .get("ordered_sections")
                .or_else(|| m.get("ordered_item_ids"))
                .or_else(|| m.get("sections"))
                .map(json_val_to_string_vec)
                .unwrap_or_default();
            normalized_pc.push(serde_json::json!({
                "page_name": page_name,
                "route": route,
                "viewport_targets": ["desktop"],
                "ordered_sections": ordered_sections,
                "shared_component_ids": [],
                "required_ctas": [],
                "states": []
            }));
        }
        _ => {}
    }
    obj.insert(
        "page_composition".to_string(),
        serde_json::Value::Array(normalized_pc),
    );
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProductArchetype {
    GameUiInterface,
    MultiPageWebapp,
    LandingPage,
    ReusableUiComponent,
}

pub fn is_game_ui_text(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("game ui")
        || lower.contains("game interface")
        || lower.contains("in-game")
        || lower.contains("ingame")
        || lower.contains("game hud")
        || lower.contains("hud overlay")
        || lower.contains("health bar")
        || lower.contains("mana bar")
        || lower.contains("stamina bar")
        || lower.contains("skill tree")
        || lower.contains("inventory grid")
        || lower.contains("inventory screen")
        || lower.contains("inventory modal")
        || lower.contains("quest log")
        || lower.contains("minimap")
        || lower.contains("character loadout")
        || lower.contains("loadout screen")
        || lower.contains("main menu")
        || lower.contains("pause menu")
        || lower.contains("boss bar")
        || lower.contains("battle pass")
        || lower.contains("gacha")
        || lower.contains("rpg ui")
        || lower.contains("fps hud")
        || lower.contains("moba hud")
        || lower.contains("rts ui")
        || lower.contains("unity ui")
        || lower.contains("unreal umg")
        || lower.contains("godot ui")
}

pub fn detect_product_archetype(
    project_name: &str,
    project_description: &str,
    project_purpose: &str,
    project_industry: &str,
    mode: &str,
    scope_preset: &str,
    user_prompt: &str,
) -> ProductArchetype {
    let combined = format!(
        "{} {} {} {} {} {}",
        project_name,
        project_description,
        project_purpose,
        project_industry,
        scope_preset,
        user_prompt
    );
    if is_game_ui_text(&combined) {
        return ProductArchetype::GameUiInterface;
    }
    let lower = combined.to_ascii_lowercase();
    if mode == "full_page" {
        if lower.contains("multi-page")
            || lower.contains("multipage")
            || lower.contains("webapp")
            || lower.contains("web app")
            || lower.contains("dashboard")
            || lower.contains("marketplace")
            || lower.contains("directory")
            || lower.contains("platform")
            || lower.contains("saas")
            || lower.contains("store")
            || lower.contains("ecommerce")
            || lower.contains("e-commerce")
            || lower.contains("portal")
        {
            ProductArchetype::MultiPageWebapp
        } else {
            ProductArchetype::LandingPage
        }
    } else if lower.contains("landing page") || scope_preset == "full_page" {
        ProductArchetype::LandingPage
    } else {
        ProductArchetype::ReusableUiComponent
    }
}

pub fn build_product_archetype_guidance(archetype: ProductArchetype) -> &'static str {
    match archetype {
        ProductArchetype::GameUiInterface => {
            "DETECTED PRODUCT ARCHETYPE: GAME UI / HUD / GAME INTERFACE COMPONENTS\n\
- Game UI Component Scope Mandate: You are designing Game UI / HUD / Game Interface Components (such as HUD overlays, health/mana/XP bars, inventory grids, skill trees, quest logs, minimap frames, loadout panels, main menus, or dialog boxes).\n\
- NEVER Invent Game Characters or 3D Game Worlds as Required Assets: Do NOT generate separate asset cutouts for in-game characters, 3D gameplay worlds, environmental lore scenes, or sample item icons inside inventory slots! Any background gameplay scene, character model stage/pedestal, character portrait inside a frame, or sample item slot content shown in the reference image is strictly a PLACEHOLDER VIEWPORT / SLOT.\n\
- Image-to-Code Mandate for Game UI: Explicitly specify in the plan and handoff instructions that ONLY the UI interface components, HUD frames, ornate/sci-fi borders, bars, panels, buttons, badges, and empty/reusable slot frames are to be implemented via image-to-code, while any background game world, character model viewport, or dynamic slot content is a placeholder where the user's actual game engine or runtime state will be rendered."
        }
        ProductArchetype::MultiPageWebapp => {
            "DETECTED PRODUCT ARCHETYPE: MULTI-PAGE WEBAPP / PLATFORM FRONTEND\n\
- Multi-Page & Dashboard Coverage: Generate a dedicated UI page image for every core route, view, and dashboard of the application (`item_1` as the Visual Anchor Landing/Home Page, and `item_2..N` with `depends_on: [\"item_1\"]` using Image-to-Image conditioning so the sticky top navigation bar, brand logo, color palette, and footer remain 100% identical across all pages).\n\
- Placeholder Data vs. 1:1 Coded UI Cards: Dynamic sample profiles, directory listings, table rows, and metrics are placeholder sample content for the user's future database, while every card, widget, banner, and table layout must be implemented 1:1 in code."
        }
        ProductArchetype::LandingPage => {
            "DETECTED PRODUCT ARCHETYPE: HIGH-CONVERSION LANDING PAGE\n\
- Vertical Top-to-Bottom Scroll Architecture: Design a cohesive vertical top-to-bottom landing page (`2:3` desktop or `9:16` mobile) starting with a sticky top navigation bar, followed by the hero section, core value/feature sections, social proof or pricing, and global footer.\n\
- Placeholder Data vs. 1:1 Coded UI Cards: Sample user profiles, mockup cards, testimonials, and banners shown in the reference image are placeholder sample content for future database integration, while every card, banner, and section layout must be implemented 1:1 in code."
        }
        ProductArchetype::ReusableUiComponent => {
            "DETECTED PRODUCT ARCHETYPE: REUSABLE UI COMPONENT / SECTION\n\
- Self-Contained Modular Architecture: Design a clean, reusable UI component or section that integrates into the host application with configurable data inputs, interactive states, animations, and extracted transparent brand/decorative assets."
        }
    }
}

pub fn is_placeholder_or_card_asset_title(name: &str) -> bool {
    let name_lower = name.to_ascii_lowercase();
    if name_lower.contains("logo")
        || name_lower.contains("wordmark")
        || name_lower.contains("icon sheet")
        || name_lower.contains("icon pack")
        || name_lower.contains("icon set")
        || name_lower.contains("plant")
        || name_lower.contains("megaphone")
        || name_lower.contains("mascot")
        || name_lower.contains("crest")
        || name_lower.contains("emblem")
    {
        return false;
    }
    name_lower.contains("avatar stack")
        || name_lower.contains("creator avatar")
        || name_lower.contains("user avatar")
        || name_lower.contains("avatars")
        || name_lower.contains("photo thumbnail")
        || name_lower.contains("thumbnails grid")
        || name_lower.contains("profile-card mockup")
        || name_lower.contains("profile card mockup")
        || name_lower.contains("card mockup")
        || name_lower.contains("ui card")
        || name_lower.contains("banner card")
        || name_lower.contains("directory card")
        || name_lower.contains("pricing card")
        || name_lower.contains("placeholder")
        || name_lower.contains("sample user")
        || name_lower.contains("game character")
        || name_lower.contains("character model")
        || name_lower.contains("character portrait")
        || name_lower.contains("gameplay background")
        || name_lower.contains("game world")
        || name_lower.contains("in-game scene")
        || name_lower.contains("background scene")
}

/// Sanitizes and normalizes LLM-generated `PlannerPlan` fields so minor formatting quirks
/// (such as descriptive text after `#hex` colors in `shared_design_spec.colors` or unknown asset IDs)
/// never cause a valid generation run to fail.
pub fn sanitize_planner_plan(
    plan: &mut PlannerPlan,
    expected_mode: &str,
    valid_asset_ids: &HashSet<String>,
    supported_aspect_ratios: &[String],
) {
    plan.schema_version = 1;
    plan.mode = expected_mode.to_string();

    if plan.request_summary.trim().is_empty() {
        plan.request_summary = "Coordinated UI design generation plan".to_string();
    } else if plan.request_summary.len() > 1900 {
        plan.request_summary = plan.request_summary.chars().take(1900).collect();
    }

    if plan.requirements.is_empty() {
        plan.requirements.push(PlanRequirement {
            id: "req_1".to_string(),
            description: plan.request_summary.clone(),
            source: "user_prompt".to_string(),
        });
    }

    // Sanitize shared_design_spec.colors so values like "#FFEDE4 soft peach clay, #EFE9FB lavender"
    // have their primary #hex extracted and descriptive notes preserved in layout_rules.
    let mut cleaned_colors = HashMap::new();
    for (k, v) in &plan.shared_design_spec.colors {
        let trimmed = v.trim();
        if is_valid_hex_color(trimmed) {
            cleaned_colors.insert(k.clone(), trimmed.to_string());
        } else if let Some(first_hex) = extract_first_hex_color(trimmed) {
            cleaned_colors.insert(k.clone(), first_hex);
            plan.shared_design_spec
                .layout_rules
                .push(format!("Color token '{}': {}", k, trimmed));
        } else if !trimmed.is_empty() {
            plan.shared_design_spec
                .layout_rules
                .push(format!("Color rule '{}': {}", k, trimmed));
        }
    }
    plan.shared_design_spec.colors = cleaned_colors;

    // Never generate separate asset images for placeholder user photos/avatars or UI card/banner mockups
    // (those are coded 1:1 in HTML/CSS and wired to a real database in the future).
    if expected_mode != "asset" && plan.items.len() > 1 {
        plan.items.retain(|it| {
            let is_asset = it.kind == "asset"
                || it.transparent_background
                || it.extracted_from_item_id.is_some();
            if is_asset && is_placeholder_or_card_asset_title(&it.name) {
                false
            } else {
                true
            }
        });
    }

    // Never generate multiple variations of a component: keep at most 1 non-asset primary UI item in component mode,
    // while keeping all separate transparent-background asset items (`kind == "asset"` or `transparent_background == true`).
    if expected_mode == "component" {
        let mut seen_primary_ui = false;
        plan.items.retain(|it| {
            let is_asset = it.kind == "asset"
                || it.transparent_background
                || it.extracted_from_item_id.is_some();
            if is_asset {
                true
            } else if !seen_primary_ui {
                seen_primary_ui = true;
                true
            } else {
                false
            }
        });
    } else if expected_mode == "full_page" {
        // In full_page mode, keep ALL distinct pages/routes/dashboards (`kind == "page"`, `"section"`, `"state"`),
        // while deduplicating only if the LLM accidentally emitted 2 variations of the exact same page route & title.
        let mut seen_page_signatures: HashSet<String> = HashSet::new();
        plan.items.retain(|it| {
            let is_asset = it.kind == "asset"
                || it.transparent_background
                || it.extracted_from_item_id.is_some();
            if is_asset {
                true
            } else {
                let sig = format!(
                    "{}::{}",
                    it.page_route
                        .as_deref()
                        .unwrap_or("")
                        .trim()
                        .to_ascii_lowercase(),
                    normalize_tab_key(&it.name)
                );
                seen_page_signatures.insert(sig)
            }
        });
    }

    let max_items = if expected_mode == "full_page" { 24 } else { 12 };
    if plan.items.len() > max_items {
        plan.items.truncate(max_items);
    }

    let mut seen_ids = HashSet::new();
    for (idx, item) in plan.items.iter_mut().enumerate() {
        if item.id.trim().is_empty() || !seen_ids.insert(item.id.clone()) {
            let fallback_id = format!("item_{}", idx + 1);
            seen_ids.insert(fallback_id.clone());
            item.id = fallback_id;
        }
        if item.name.trim().is_empty() {
            item.name = format!("Design Item {}", idx + 1);
        } else if item.name.len() > 190 {
            item.name = item.name.chars().take(190).collect();
        }
        if item.image_prompt.trim().is_empty() {
            item.image_prompt = if !item.direction.trim().is_empty() {
                item.direction.clone()
            } else {
                plan.request_summary.clone()
            };
        } else if item.image_prompt.len() > 7800 {
            item.image_prompt = item.image_prompt.chars().take(7800).collect();
        }

        let item_text_lower = format!(
            "{} {} {} {}",
            item.name, item.direction, item.tab_key, item.viewport.intent
        )
        .to_ascii_lowercase();

        let is_asset_item = item.kind == "asset"
            || expected_mode == "asset"
            || item.transparent_background
            || item.extracted_from_item_id.is_some();

        let is_full_page_item = !is_asset_item
            && (item.kind == "page"
                || (expected_mode == "full_page" && item.kind != "component")
                || item_text_lower.contains("landing page")
                || item_text_lower.contains("full website")
                || item_text_lower.contains("full desktop")
                || item_text_lower.contains("full mobile")
                || item_text_lower.contains("homepage")
                || item_text_lower.contains("entire page")
                || item_text_lower.contains("full page"));

        // Normalize viewport intent & internal CSS width based on requested breakpoint
        if is_asset_item {
            if item.viewport.intent.trim().is_empty() {
                item.viewport.intent = "isolated".to_string();
            }
        } else if item.viewport.intent == "mobile" || item_text_lower.contains("mobile") {
            item.viewport.intent = "mobile".to_string();
            item.viewport.css_width = 390;
        } else if item.viewport.intent == "tablet" || item_text_lower.contains("tablet") {
            item.viewport.intent = "tablet".to_string();
            item.viewport.css_width = 834;
        } else {
            item.viewport.intent = "desktop".to_string();
            if item.viewport.css_width < 1024 {
                item.viewport.css_width = 1440;
            }
        }

        let is_game_hud_or_menu = is_game_ui_text(&item_text_lower)
            || is_game_ui_text(&plan.request_summary)
            || is_game_ui_text(&item.image_prompt);

        // Enforce vertical aspect ratios for full top-to-bottom web pages and mobile/tablet breakpoints
        // so a landing page NEVER looks like a horizontal 16:9 PowerPoint slide!
        // (For desktop PC/Console Game UI HUDs & Game Menus, allow 16:9 widescreen viewport if selected.)
        if is_full_page_item {
            if item.viewport.intent == "mobile" {
                item.aspect_ratio = "9:16".to_string();
            } else if item.viewport.intent == "tablet" {
                if item.aspect_ratio != "2:3"
                    && item.aspect_ratio != "3:4"
                    && item.aspect_ratio != "9:16"
                {
                    item.aspect_ratio = "2:3".to_string();
                }
            } else if is_game_hud_or_menu
                && !item_text_lower.contains("landing page")
                && !item_text_lower.contains("website")
            {
                if !matches!(
                    item.aspect_ratio.as_str(),
                    "16:9" | "21:9" | "2:3" | "9:16" | "4:3"
                ) {
                    item.aspect_ratio = "16:9".to_string();
                }
            } else {
                // Desktop Full-Page / Landing Page: MUST be vertical ("2:3" or "9:16") from top to bottom!
                if item.aspect_ratio != "2:3" && item.aspect_ratio != "9:16" {
                    item.aspect_ratio = "2:3".to_string();
                }
            }
        } else if !is_asset_item {
            if item.viewport.intent == "mobile" {
                if matches!(
                    item.aspect_ratio.as_str(),
                    "16:9" | "21:9" | "3:2" | "4:3" | ""
                ) {
                    item.aspect_ratio = "9:16".to_string();
                }
            } else if item.viewport.intent == "tablet" {
                if matches!(item.aspect_ratio.as_str(), "16:9" | "21:9" | "") {
                    item.aspect_ratio = "3:4".to_string();
                }
            }
        }

        if !supported_aspect_ratios.is_empty()
            && !supported_aspect_ratios.contains(&item.aspect_ratio)
        {
            let lower = item.aspect_ratio.to_lowercase();
            let mapped = if lower.contains("9:16") || lower.contains("mobile") {
                "9:16"
            } else if lower.contains("2:3") || is_full_page_item {
                "2:3"
            } else if lower.contains("3:4")
                || lower.contains("portrait")
                || lower.contains("tablet")
            {
                "3:4"
            } else if lower.contains("1:1") || lower.contains("square") || item.kind == "asset" {
                "1:1"
            } else {
                "16:9"
            };
            if supported_aspect_ratios.iter().any(|s| s == mapped) {
                item.aspect_ratio = mapped.to_string();
            } else if let Some(first) = supported_aspect_ratios.first() {
                item.aspect_ratio = first.clone();
            }
        }

        item.reference_asset_ids
            .retain(|ref_id| valid_asset_ids.contains(ref_id));
    }

    let valid_item_ids: HashSet<String> = plan.items.iter().map(|i| i.id.clone()).collect();
    let item_index_map: HashMap<String, usize> = plan
        .items
        .iter()
        .enumerate()
        .map(|(idx, i)| (i.id.clone(), idx))
        .collect();

    // In full_page mode, identify the primary Visual Anchor page (first non-asset item, typically `item_1`)
    // and ensure every subsequent page (`item_2`, `item_3`, `item_4`, ...) depends on that Visual Anchor
    // so `worker.rs` passes Page 1 as an Image-to-Image reference to keep the Navbar, Logo, and Chrome 100% consistent!
    let anchor_item_info: Option<(usize, String)> = if expected_mode == "full_page" {
        plan.items
            .iter()
            .enumerate()
            .find(|(_, it)| {
                it.kind != "asset"
                    && !it.transparent_background
                    && it.extracted_from_item_id.is_none()
            })
            .map(|(idx, it)| (idx, it.id.clone()))
    } else {
        None
    };

    for (idx, item) in plan.items.iter_mut().enumerate() {
        if let Some((anchor_idx, ref anchor_id)) = anchor_item_info {
            let is_asset = item.kind == "asset"
                || item.transparent_background
                || item.extracted_from_item_id.is_some();
            if idx == anchor_idx {
                item.is_visual_anchor = true;
                item.depends_on.clear();
                if item
                    .page_route
                    .as_deref()
                    .map(|r| r.trim().is_empty())
                    .unwrap_or(true)
                {
                    item.page_route = Some("/".to_string());
                }
            } else if !is_asset {
                item.is_visual_anchor = false;
                if !item.depends_on.contains(anchor_id) {
                    item.depends_on.insert(0, anchor_id.clone());
                }
                if item
                    .page_route
                    .as_deref()
                    .map(|r| r.trim().is_empty())
                    .unwrap_or(true)
                {
                    let slug = normalize_tab_key(&item.name);
                    let route_slug = slug
                        .trim_start_matches("page-")
                        .trim_start_matches("full-")
                        .trim_end_matches("-page");
                    item.page_route = Some(format!(
                        "/{}",
                        if route_slug.is_empty() {
                            slug.as_str()
                        } else {
                            route_slug
                        }
                    ));
                }
            }
        }

        if let Some(ref ext_id) = item.extracted_from_item_id {
            if !valid_item_ids.contains(ext_id) || ext_id == &item.id {
                item.extracted_from_item_id = None;
            } else if !item.depends_on.contains(ext_id) {
                item.depends_on.push(ext_id.clone());
            }
        }
        // Keep only dependencies that exist and appear earlier (or do not form a cycle)
        item.depends_on.retain(|dep| {
            dep != &item.id
                && valid_item_ids.contains(dep)
                && item_index_map.get(dep).copied().unwrap_or(usize::MAX) < idx
        });
    }

    // Ensure `page_composition` and `shared_components` reflect all generated pages in `full_page` mode
    if expected_mode == "full_page" {
        let mut all_routes: Vec<String> = Vec::new();
        for item in &plan.items {
            let is_asset = item.kind == "asset"
                || item.transparent_background
                || item.extracted_from_item_id.is_some();
            if !is_asset && item.kind == "page" {
                let route = item.page_route.clone().unwrap_or_else(|| "/".to_string());
                if !all_routes.contains(&route) {
                    all_routes.push(route.clone());
                }
                let exists_in_pc = plan
                    .page_composition
                    .iter()
                    .any(|pc| pc.route == route || pc.ordered_sections.contains(&item.id));
                if !exists_in_pc {
                    plan.page_composition.push(PageCompositionSpec {
                        page_name: item.name.clone(),
                        route,
                        viewport_targets: vec![item.viewport.intent.clone()],
                        ordered_sections: vec![item.id.clone()],
                        shared_component_ids: vec!["shared_navbar".to_string()],
                        required_ctas: item.exact_copy.iter().take(2).cloned().collect(),
                        states: vec![
                            "default".to_string(),
                            "hover".to_string(),
                            "active".to_string(),
                        ],
                    });
                }
            }
        }
        if !all_routes.is_empty() {
            if plan.shared_components.is_empty() {
                if let Some((_, ref anchor_id)) = anchor_item_info {
                    plan.shared_components.push(SharedComponentSpec {
                        id: "shared_navbar".to_string(),
                        name: "Shared Sticky Top Navigation Bar & Global Footer".to_string(),
                        canonical_item_id: anchor_id.clone(),
                        used_on_routes: all_routes,
                        description: "Global sticky top navigation bar (with Framer Motion layoutId active route indicator) and footer shared identically across all pages via Image-to-Image anchor.".to_string(),
                    });
                }
            } else {
                for sc in &mut plan.shared_components {
                    for r in &all_routes {
                        if !sc.used_on_routes.contains(r) {
                            sc.used_on_routes.push(r.clone());
                        }
                    }
                }
            }
        }
    }
}

/// Validates a `PlannerPlan` against Section 11 rules and returns the topological execution order of item IDs.
pub fn validate_planner_plan(
    plan: &PlannerPlan,
    expected_mode: &str,
    valid_asset_ids: &HashSet<String>,
    supported_aspect_ratios: &[String],
) -> AppResult<Vec<String>> {
    if plan.schema_version != 1 {
        return Err(AppError::Validation(format!(
            "Unsupported planner schema_version: {} (expected 1)",
            plan.schema_version
        )));
    }
    if plan.mode != expected_mode {
        return Err(AppError::Validation(format!(
            "Planner mode '{}' does not match requested mode '{}'",
            plan.mode, expected_mode
        )));
    }
    if plan.request_summary.trim().is_empty() || plan.request_summary.len() > 2000 {
        return Err(AppError::Validation(
            "Planner request_summary must be non-empty and <= 2000 chars".into(),
        ));
    }
    if plan.requirements.is_empty() {
        return Err(AppError::Validation(
            "Planner must include at least one requirement.".into(),
        ));
    }

    match expected_mode {
        "component" | "asset" => {
            if plan.items.is_empty() || plan.items.len() > 12 {
                return Err(AppError::Validation(format!(
                    "{} mode requires between 1 and 12 items, got {}",
                    expected_mode,
                    plan.items.len()
                )));
            }
        }
        "full_page" => {
            if plan.items.is_empty() || plan.items.len() > 24 {
                return Err(AppError::Validation(format!(
                    "full_page mode requires between 1 and 24 planned items, got {}",
                    plan.items.len()
                )));
            }
        }
        _ => {
            return Err(AppError::Validation(format!(
                "Invalid generation mode '{}'",
                expected_mode
            )))
        }
    }

    // Validate shared design spec hex colors when provided as #hex
    for (k, v) in &plan.shared_design_spec.colors {
        if !v.trim().is_empty() && !is_valid_hex_color(v) {
            return Err(AppError::Validation(format!(
                "Invalid hex color '{}' for key '{}' in shared_design_spec",
                v, k
            )));
        }
    }

    let mut item_ids = HashSet::new();
    for item in &plan.items {
        if item.id.trim().is_empty() || !item_ids.insert(item.id.clone()) {
            return Err(AppError::Validation(format!(
                "Duplicate or empty plan item id '{}'",
                item.id
            )));
        }
        if item.name.trim().is_empty() || item.name.len() > 200 {
            return Err(AppError::Validation(format!(
                "Invalid name for plan item '{}'",
                item.id
            )));
        }
        if item.image_prompt.trim().is_empty() || item.image_prompt.len() > 8000 {
            return Err(AppError::Validation(format!(
                "Invalid or excessive image_prompt for plan item '{}'",
                item.id
            )));
        }
        if !supported_aspect_ratios.is_empty()
            && !supported_aspect_ratios.contains(&item.aspect_ratio)
        {
            return Err(AppError::Validation(format!(
                "Unsupported aspect_ratio '{}' in item '{}'. Supported: {:?}",
                item.aspect_ratio, item.id, supported_aspect_ratios
            )));
        }
        for ref_id in &item.reference_asset_ids {
            if !valid_asset_ids.contains(ref_id) {
                return Err(AppError::Validation(format!(
                    "Plan item '{}' references unknown attachment ID '{}'",
                    item.id, ref_id
                )));
            }
        }
    }

    // Validate dependency graph and detect cycles using Kahn's algorithm
    let mut in_degree: HashMap<String, usize> = HashMap::new();
    let mut adj: HashMap<String, Vec<String>> = HashMap::new();
    for item in &plan.items {
        in_degree.insert(item.id.clone(), 0);
        adj.insert(item.id.clone(), Vec::new());
    }
    for item in &plan.items {
        for dep in &item.depends_on {
            if !item_ids.contains(dep) {
                return Err(AppError::Validation(format!(
                    "Plan item '{}' depends on non-existent item '{}'",
                    item.id, dep
                )));
            }
            if dep == &item.id {
                return Err(AppError::Validation(format!(
                    "Plan item '{}' cannot depend on itself",
                    item.id
                )));
            }
            adj.entry(dep.clone()).or_default().push(item.id.clone());
            *in_degree.entry(item.id.clone()).or_insert(0) += 1;
        }
    }

    let mut queue: VecDeque<String> = VecDeque::new();
    // Preserve stable declaration order for items with 0 in-degree
    for item in &plan.items {
        if in_degree.get(&item.id).copied().unwrap_or(0) == 0 {
            queue.push_back(item.id.clone());
        }
    }

    let mut topo_order = Vec::with_capacity(plan.items.len());
    while let Some(curr) = queue.pop_front() {
        topo_order.push(curr.clone());
        if let Some(neighbors) = adj.get(&curr) {
            for next in neighbors {
                if let Some(deg) = in_degree.get_mut(next) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(next.clone());
                    }
                }
            }
        }
    }

    if topo_order.len() != plan.items.len() {
        return Err(AppError::Validation(
            "Cyclic dependency detected in planner items.".into(),
        ));
    }

    Ok(topo_order)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_color_validation() {
        assert!(is_valid_hex_color("#D56A3A"));
        assert!(is_valid_hex_color("#fff"));
        assert!(!is_valid_hex_color("D56A3A"));
        assert!(!is_valid_hex_color("#GGGGGG"));
    }

    #[test]
    fn test_validate_planner_plan_rejects_unknown_assets_and_cycles() {
        let mut valid_assets = HashSet::new();
        valid_assets.insert("asset_1".to_string());
        let supported_ratios = vec![
            "4:3".to_string(),
            "16:9".to_string(),
            "1:1".to_string(),
            "2:3".to_string(),
            "9:16".to_string(),
        ];

        let item_template = |id: &str, v: u32, deps: Vec<String>, refs: Vec<String>| PlanItem {
            id: id.to_string(),
            kind: "component".to_string(),
            name: format!("Dir {}", v),
            page_route: None,
            variant_index: v,
            viewport: ViewportSpec::default(),
            aspect_ratio: "4:3".to_string(),
            requirements_covered: vec!["req_1".to_string()],
            direction: "Editorial".to_string(),
            exact_copy: vec!["$19 / month".to_string()],
            reference_asset_ids: refs,
            depends_on: deps,
            image_prompt: "A clean pricing card".to_string(),
            implementation_notes: vec![],
            behaviors: vec![],
            responsive_rules: vec![],
            asset_instructions: vec![],
            is_visual_anchor: false,
            tab_key: "pricing-card".to_string(),
            tab_label: "Pricing Card".to_string(),
            transparent_background: false,
            extracted_from_item_id: None,
            target_existing_output_id: None,
            quality_override: None,
        };

        // Valid 4-item component plan
        let valid_plan = PlannerPlan {
            schema_version: 1,
            mode: "component".to_string(),
            request_summary: "Polish pricing card".to_string(),
            requirements: vec![PlanRequirement {
                id: "req_1".to_string(),
                description: "Add annual toggle".to_string(),
                source: "user".to_string(),
            }],
            assumptions: vec![],
            shared_design_spec: SharedDesignSpec::default(),
            items: vec![
                item_template("d1", 1, vec![], vec!["asset_1".to_string()]),
                item_template("d2", 2, vec![], vec![]),
                item_template("d3", 3, vec![], vec![]),
                item_template("d4", 4, vec![], vec![]),
            ],
            shared_components: vec![],
            page_composition: vec![],
        };
        assert!(
            validate_planner_plan(&valid_plan, "component", &valid_assets, &supported_ratios)
                .is_ok()
        );

        // Unknown attachment reference should fail
        let mut bad_ref_plan = valid_plan.clone();
        bad_ref_plan.items[0].reference_asset_ids = vec!["unknown_asset_999".to_string()];
        assert!(validate_planner_plan(
            &bad_ref_plan,
            "component",
            &valid_assets,
            &supported_ratios
        )
        .is_err());

        // Cyclic dependency in full_page should fail
        let cyclic_plan = PlannerPlan {
            schema_version: 1,
            mode: "full_page".to_string(),
            request_summary: "Full site".to_string(),
            requirements: valid_plan.requirements.clone(),
            assumptions: vec![],
            shared_design_spec: SharedDesignSpec::default(),
            items: vec![
                item_template("p1", 1, vec!["p2".to_string()], vec![]),
                item_template("p2", 2, vec!["p1".to_string()], vec![]),
            ],
            shared_components: vec![],
            page_composition: vec![],
        };
        assert!(
            validate_planner_plan(&cyclic_plan, "full_page", &valid_assets, &supported_ratios)
                .is_err()
        );

        // Tolerant parser test with string page_composition, string shared_components, and string viewport
        let raw_llm_json = r##"{
            "schema_version": "1",
            "mode": "full_page",
            "request_summary": "Full desktop landing page",
            "requirements": ["Sticky navbar", "Hero section"],
            "assumptions": "Clean dark theme",
            "shared_design_spec": {
                "colors": {"primary": "#FF5500 warm orange"},
                "layout_rules": "Sticky top navbar"
            },
            "items": [
                {
                    "id": "item_1",
                    "kind": "page",
                    "name": "Full desktop landing page",
                    "tab_key": "full-website",
                    "tab_label": "Full Website",
                    "viewport": {"intent": "desktop", "css_width": "1440px"},
                    "aspect_ratio": "16:9",
                    "image_prompt": "Vertical top-to-bottom landing page",
                    "implementation_notes": "Sticky top-0 navbar"
                }
            ],
            "shared_components": ["Sticky top navbar"],
            "page_composition": ["item_1 — Full desktop landing page (sticky navbar → hero → how-it-works → features → advertising → pricing → footer)"]
        }"##;
        let mut parsed =
            parse_planner_plan_tolerant(raw_llm_json).expect("tolerant parse should succeed");
        sanitize_planner_plan(&mut parsed, "full_page", &valid_assets, &supported_ratios);
        assert_eq!(parsed.items[0].aspect_ratio, "2:3");
        assert_eq!(parsed.items[0].viewport.css_width, 1440);
        assert_eq!(parsed.page_composition.len(), 1);
        assert!(
            validate_planner_plan(&parsed, "full_page", &valid_assets, &supported_ratios).is_ok()
        );

        // Multi-page full_page plan test: verifies all distinct pages are kept, Page 1 is Visual Anchor,
        // and Pages 2 & 3 automatically depend on Page 1 (`item_1`) for Image-to-Image Navbar/Chrome consistency.
        let multi_page_json = r##"{
            "schema_version": 1,
            "mode": "full_page",
            "request_summary": "Full multi-page webapp frontend",
            "requirements": ["Landing page", "Discovery directory", "Creator dashboard"],
            "items": [
                {
                    "id": "item_1",
                    "kind": "page",
                    "name": "Primary Landing Page",
                    "page_route": "/",
                    "viewport": {"intent": "desktop", "css_width": 1440},
                    "aspect_ratio": "2:3",
                    "image_prompt": "Landing page with sticky navbar, hero, features, pricing, footer"
                },
                {
                    "id": "item_2",
                    "kind": "page",
                    "name": "Discovery Directory Page",
                    "page_route": "/discover",
                    "viewport": {"intent": "desktop", "css_width": 1440},
                    "aspect_ratio": "2:3",
                    "image_prompt": "Directory page keeping exact navbar from item_1 with filter sidebar and card grid"
                },
                {
                    "id": "item_3",
                    "kind": "page",
                    "name": "Analytics & Admin Dashboard Page",
                    "viewport": {"intent": "desktop", "css_width": 1440},
                    "aspect_ratio": "2:3",
                    "image_prompt": "Dashboard keeping exact navbar from item_1 with KPI cards, chart, and table"
                },
                {
                    "id": "item_4",
                    "kind": "asset",
                    "name": "Brand Logo & Wordmark (Transparent)",
                    "transparent_background": true,
                    "extracted_from_item_id": "item_1",
                    "aspect_ratio": "1:1",
                    "image_prompt": "Isolated logo on transparent background"
                }
            ]
        }"##;
        let mut mp_plan =
            parse_planner_plan_tolerant(multi_page_json).expect("multi-page parse should succeed");
        sanitize_planner_plan(&mut mp_plan, "full_page", &valid_assets, &supported_ratios);
        assert_eq!(mp_plan.items.len(), 4);
        assert!(mp_plan.items[0].is_visual_anchor);
        assert!(mp_plan.items[0].depends_on.is_empty());
        assert_eq!(mp_plan.items[1].depends_on, vec!["item_1".to_string()]);
        assert_eq!(mp_plan.items[2].depends_on, vec!["item_1".to_string()]);
        assert_eq!(
            mp_plan.items[2].page_route.as_deref(),
            Some("/analytics-admin-dashboard")
        );
        assert_eq!(mp_plan.page_composition.len(), 3);
        let topo = validate_planner_plan(&mp_plan, "full_page", &valid_assets, &supported_ratios)
            .expect("multi-page topological sort should succeed");
        assert_eq!(topo[0], "item_1");
    }
}

use crate::models::{AttachmentRole, BrandSpec, PlannerPlan, RunModelSettings, SelectedReference};
use serde::{Deserialize, Serialize};

pub const PLANNER_SYSTEM_PROMPT_V1: &str = include_str!("../../../prompts/v1/planner_system.txt");
pub const CHAT_ROUTER_SYSTEM_PROMPT_V1: &str =
    include_str!("../../../prompts/v1/chat_router_system.txt");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetaSummary {
    pub id: String,
    pub name: String,
    pub description: String,
    pub purpose: String,
    pub industry: String,
    pub target_audience: String,
    pub audience_needs: String,
    pub desired_user_action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResolvedReferenceInput {
    pub asset_id: String,
    pub role: AttachmentRole,
    pub label: String,
    pub placement_notes: String,
    pub preserve_as_is: bool,
    pub mime_type: String,
    pub width: u32,
    pub height: u32,
    /// Data URL (`data:image/png;base64,...`) prepared from bounded model-input bytes
    pub data_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnchorReferenceInput {
    pub plan_item_id: String,
    pub title: String,
    pub data_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuiltGenerationContext {
    pub system_prompt: String,
    pub output_schema_description: String,
    pub project_meta: ProjectMetaSummary,
    pub brand_version: i64,
    pub brand_snapshot: BrandSpec,
    pub mode: String,
    pub user_request: String,
    pub model_settings: RunModelSettings,
    pub current_screenshot: Option<ResolvedReferenceInput>,
    pub explicit_attachments: Vec<ResolvedReferenceInput>,
    pub selected_previous_output: Option<ResolvedReferenceInput>,
    pub approved_plan: Option<PlannerPlan>,
    pub shared_anchor_images: Vec<AnchorReferenceInput>,
    pub compact_summary: String,
    pub planner_user_prompt: String,
}

/// Pure, isolated context builder function (Section 6).
/// Never queries thread history, unrelated runs, or unselected project files.
#[allow(clippy::too_many_arguments)]
pub fn build_generation_context(
    system_prompt: &str,
    project_meta: ProjectMetaSummary,
    brand_version: i64,
    brand_snapshot: BrandSpec,
    mode: &str,
    user_request: &str,
    model_settings: RunModelSettings,
    resolved_references: Vec<ResolvedReferenceInput>,
    approved_plan: Option<PlannerPlan>,
    shared_anchor_images: Vec<AnchorReferenceInput>,
) -> BuiltGenerationContext {
    let mut current_screenshot = None;
    let mut selected_previous_output = None;
    let mut explicit_attachments = Vec::new();

    for r in resolved_references {
        match r.role {
            AttachmentRole::CurrentScreenshot if current_screenshot.is_none() => {
                current_screenshot = Some(r);
            }
            AttachmentRole::SelectedPreviousOutput if selected_previous_output.is_none() => {
                selected_previous_output = Some(r);
            }
            _ => {
                explicit_attachments.push(r);
            }
        }
    }

    let total_selected_images = (if current_screenshot.is_some() { 1 } else { 0 })
        + (if selected_previous_output.is_some() {
            1
        } else {
            0
        })
        + explicit_attachments.len();

    let compact_summary = format!(
        "Brand kit + current request + {} selected {}",
        total_selected_images,
        if total_selected_images == 1 {
            "image"
        } else {
            "images"
        }
    );

    let output_schema_description = r##"Return ONLY valid JSON matching schema_version=1 (IMPORTANT: All text inside implementation_notes, behaviors, responsive_rules, and asset_instructions MUST be pure human language prose with zero code snippets, zero HTML/JSX tags, and zero CSS syntax blocks):
{
  "schema_version": 1,
  "mode": "component" | "full_page" | "asset",
  "request_summary": "string",
  "requirements": [{"id": "req_1", "description": "string", "source": "user"}],
  "assumptions": ["string"],
  "shared_design_spec": {
    "colors": {"primary": "#hex", "background": "#hex", "surface": "#hex", "text": "#hex"},
    "typography": {"heading": "string", "body": "string", "mono": "string"},
    "spacing": {"base": "string"},
    "radii": {"card": "string", "button": "string"},
    "shadows": {"card": "string"},
    "layout_rules": [
      "STICKY VS NON-STICKY: Specify in plain human language which global components stay anchored on scroll (such as the top navigation bar or bottom action dock) versus which sections scroll naturally in normal page flow"
    ],
    "asset_rules": [
      "NATIVE TRANSPARENT PNG ASSETS: All standalone icons, icon sheets, badges, 3D object references, and extracted foreground illustrations must set transparent_background=true so Sunburst 2.5 renders them on a 100% native transparent alpha background"
    ]
  },
  "items": [
    {
      "id": "item_1",
      "kind": "component" | "page" | "section" | "state" | "asset",
      "name": "Descriptive title (e.g. Primary Navbar, Hero Section, Hero 3D Foreground Object Reference (Transparent), Site Icon Sheet)",
      "tab_key": "navbar | hero | full-website | icons | ...",
      "tab_label": "Navbar | Hero Section | Full Website | Site Icons | ...",
      "page_route": null,
      "variant_index": 1,
      "viewport": {"intent": "desktop" | "mobile" | "tablet" | "isolated", "css_width": 1440},
      "aspect_ratio": "2:3 (for full_page/landing page) | 9:16 (for mobile) | 16:9 (for wide desktop section/component or PC/Console Game HUD) | 1:1 (for isolated icon/asset)",
      "transparent_background": false,
      "extracted_from_item_id": null,
      "target_existing_output_id": null,
      "quality_override": null,
      "requirements_covered": ["req_1"],
      "direction": "Art direction summary",
      "exact_copy": ["Exact text strings to display"],
      "reference_asset_ids": ["only supplied asset_ids"],
      "depends_on": [],
      "image_prompt": "Detailed visual prompt (for transparent_background=true assets/icons/extracted foreground graphics, state that the subject or icon grid is isolated on a clean 100% transparent background with zero UI text or background clutter)",
      "implementation_notes": [
        "POSITIONING (STICKY VS NON-STICKY): Explain in plain human language whether this component, header, sidebar, or HUD bar stays anchored at the top/bottom of the viewport or scrolls naturally in normal flow",
        "PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: Explain in plain human language which elements in this image are placeholder content (such as sample user profiles, avatars, directory listings, banners, or for Game UI: background gameplay scenes, in-game character models, and sample inventory slot contents) and mandate that ONLY the UI interface components, cards, banners, HUD frames, bars, and controls are to be built 1:1 in code right now",
        "CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Specify in plain human language what must be built purely with code (section separators, glowing dividers, ambient background gradients, particle/grid backdrops, cards, and frames), what must be converted into vectorial SVG graphics (logos, icons, emblems, flat marks), and what must be reconstructed as an interactive 3D object from its extracted reference image (Image-to-3D or procedural 3D) along with its required continuous rotation, floating idle motion, and interactive hover tilt animation",
        "FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Describe in plain human language how page route transitions, shared active navigation indicators, staggered scroll reveals, tactile spring button and card hover/press animations, filter/tab layout reflows, 3D object rotation, and reduced-motion accessibility work",
        "REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): If a reference of the user's current hero, header, navbar, or component was attached, state in plain human language that the old component in the codebase must be replaced and updated to match this newly generated design 1:1"
      ],
      "behaviors": [
        "HOVER STATE: Describe in plain human language the exact hover background hex color, text hex color, border hex color, shadow elevation, and subtle upward lift or glow",
        "CLICK / ACTIVE STATE: Describe in plain human language the exact pressed background hex color, tactile press compression, and active border or indicator color",
        "FOCUS & DISABLED STATES: Describe in plain human language the keyboard focus ring hex color and disabled low-opacity state"
      ],
      "responsive_rules": ["Describe in plain human language how the layout adapts across desktop, tablet, and mobile viewports"],
      "asset_instructions": ["Describe in plain human language how to use this extracted transparent asset: whether to convert it into a scalable vectorial SVG graphic (for logos, icons, crests, flat art), layer it as an optimized alpha cutout, or reconstruct it as an interactive rotating 3D object from the image"],
      "is_visual_anchor": false
    }
  ],
  "shared_components": [
    {"id": "shared_1", "name": "Sticky Top Navbar", "canonical_item_id": "item_1", "used_on_routes": ["/"], "description": "Global navigation header"}
  ],
  "page_composition": [
    {"page_name": "Full Landing Page", "route": "/", "viewport_targets": ["desktop"], "ordered_sections": ["item_1"], "shared_component_ids": ["shared_1"], "required_ctas": ["Primary CTA"], "states": ["default"]}
  ]
}"##
    .to_string();

    // Build structured untrusted data block for the planner
    let mut all_refs_desc = Vec::new();
    if let Some(ref cs) = current_screenshot {
        all_refs_desc.push(format!(
            "- ID: {} | Role: current_screenshot | Label: {} | Dimensions: {}x{} | Instruction: {}",
            cs.asset_id,
            cs.label,
            cs.width,
            cs.height,
            cs.role.planner_instruction()
        ));
    }
    if let Some(ref prev) = selected_previous_output {
        all_refs_desc.push(format!(
            "- ID: {} | Role: selected_previous_output | Label: {} | Dimensions: {}x{} | Instruction: {}",
            prev.asset_id,
            prev.label,
            prev.width,
            prev.height,
            prev.role.planner_instruction()
        ));
    }
    for att in &explicit_attachments {
        all_refs_desc.push(format!(
            "- ID: {} | Role: {} | Label: {} | Preserve-as-is: {} | Notes: {} | Instruction: {}",
            att.asset_id,
            att.role.as_str(),
            att.label,
            att.preserve_as_is,
            att.placement_notes,
            att.role.planner_instruction()
        ));
    }

    let style_block = if let Some(ref ds) = brand_snapshot.design_style {
        if !ds.style_name.trim().is_empty() {
            let num = if ds.style_number > 0 {
                ds.style_number
            } else {
                1
            };
            let catalog_entry = crate::style_catalog::find_style_by_number(num)
                .or_else(|| crate::style_catalog::find_style_by_id_or_name(&ds.style_id))
                .or_else(|| crate::style_catalog::find_style_by_id_or_name(&ds.style_name));
            let best_fit = catalog_entry
                .map(|c| c.best_fit)
                .unwrap_or("Tailored to the project's product category and user flows.");
            let essential_screens = catalog_entry
                .map(|c| c.essential_screens)
                .unwrap_or("Hero value proposition, core interactive cards, and conversion flow.");
            format!(
                "Selected Style: #{num} {name} (ID: {id}, Category: {cat})\n\
Exact Style Definition: {def}\n\
Concrete Visual & CSS System Rules: {rules}\n\
Best Product Fit: {best_fit}\n\
Essential Category Screens & Component Hierarchy: {essential_screens}\n\
Combinable Layout / System Modifiers: {mods}\n\
Combined Design System Brief: {brief}\n\
Design System Search Keywords: {kw}\n\
CRITICAL BRANDING OVERRIDE RULE: Follow the layout hierarchy, component patterns, borders, shadows, and surface treatments of Style #{num} ({name}) above, while ALWAYS using the USER'S OWN BRAND KIT (User's Light/Dark Color Palette hex tokens, Uploaded Logo, and Brand Typography) — never copy generic sample colors from the style catalog preview!",
                num = num,
                name = ds.style_name,
                id = ds.style_id,
                cat = if ds.category.is_empty() {
                    catalog_entry.map(|c| c.category).unwrap_or("General")
                } else {
                    &ds.category
                },
                def = if ds.definition.is_empty() {
                    catalog_entry.map(|c| c.definition).unwrap_or("")
                } else {
                    &ds.definition
                },
                rules = if ds.visual_rules.is_empty() {
                    catalog_entry.map(|c| c.visual_rules).unwrap_or("")
                } else {
                    &ds.visual_rules
                },
                best_fit = best_fit,
                essential_screens = essential_screens,
                mods = if ds.modifiers.is_empty() {
                    "None".to_string()
                } else {
                    ds.modifiers.join(" + ")
                },
                brief = ds.combined_brief,
                kw = if ds.keywords.is_empty() {
                    catalog_entry
                        .map(|c| c.keywords.join(", "))
                        .unwrap_or_else(|| "None".to_string())
                } else {
                    ds.keywords.join(", ")
                },
            )
        } else {
            let auto_matched = crate::style_catalog::search_top_design_styles(
                &format!(
                    "{} {} {}",
                    project_meta.description, project_meta.purpose, user_request
                ),
                Some(&brand_snapshot.brand_personality),
                2,
            );
            let matched_note = auto_matched
                .first()
                .map(|s| {
                    format!(
                        "\nAuto-Matched Category Style (#{} {} — {}): {} | Visual Rules: {} | Essential Screens: {}",
                        s.number, s.name, s.category, s.definition, s.visual_rules, s.essential_screens
                    )
                })
                .unwrap_or_default();
            format!(
                "Selected Style: {} — {}{}\nCRITICAL BRANDING RULE: Apply these style rules strictly using the User's Brand Kit colors, logo, and typography.",
                brand_snapshot.brand_personality, brand_snapshot.design_direction, matched_note
            )
        }
    } else {
        let auto_matched = crate::style_catalog::search_top_design_styles(
            &format!(
                "{} {} {}",
                project_meta.description, project_meta.purpose, user_request
            ),
            Some(&brand_snapshot.brand_personality),
            2,
        );
        let matched_note = auto_matched
            .first()
            .map(|s| {
                format!(
                    "\nAuto-Matched Category Style (#{} {} — {}): {} | Visual Rules: {} | Essential Screens: {}",
                    s.number, s.name, s.category, s.definition, s.visual_rules, s.essential_screens
                )
            })
            .unwrap_or_default();
        format!(
            "Selected Style: {} — {}{}\nCRITICAL BRANDING RULE: Apply these style rules strictly using the User's Brand Kit colors, logo, and typography.",
            brand_snapshot.brand_personality, brand_snapshot.design_direction, matched_note
        )
    };

    let asset_opts_summary = if let Some(ref ao) = model_settings.asset_options {
        format!(
            "usage={}, theme_context={}, transparent_background={}, seamless_tiling={}, include_logo={}, composition='{}', negative_space='{}'",
            ao.intended_usage,
            ao.theme_context,
            ao.transparent_background,
            ao.seamless_tiling,
            ao.include_logo,
            ao.composition,
            ao.negative_space
        )
    } else {
        "N/A (UI mode)".to_string()
    };

    let screenshot_workflow_mandate = if let Some(ref cs) = current_screenshot {
        format!(
            "WORKFLOW MODE: REPLACE & UPGRADE EXISTING HERO / HEADER / COMPONENT (Current Screenshot Attached: `{}` — {})\n\
- SEE WHAT THE CURRENT COMPONENT HAS: Inspect this screenshot so you know EVERY button, link, CTA, toggle, input, icon button, badge, and label that currently exists.\n\
- MAKE A BETTER ONE IN THE BRAND & DESIGN STYLE: Creating a component with an old/current one as reference means polishing, enhancing, and redesigning it strictly respecting the project's Brand Kit and Chosen Web Design Style (do NOT copy outdated or unbranded colors from the old screenshot).\n\
- ADD NEW FUNCTIONALITY & BUTTONS WHEN REQUESTED: If the user's prompt asks for new functionality, extra buttons, different layout structure, or specific changes, combine what the current component has with ALL new buttons, features, and layout improvements requested by the user.\n\
- MANDATORY REPLACEMENT NOTE IN `implementation_notes`: Explicitly state in `implementation_notes` that the coding AI agent must locate the old hero, header, navbar, or component shown in `{}` inside the codebase and REPLACE / CHANGE that old component to match this newly generated design 1:1 while preserving existing routes, event handlers, and data bindings!\n\
- TEXTS VS. ICONS BASED ON STYLE & PROMPT: Render buttons, navigation items, and actions using text labels, icons, or icon+text combinations based on what the user asks for and what best fits the Chosen Web Design Style and Brand Icon Style (`{}`).",
            cs.asset_id, cs.label, cs.label, brand_snapshot.layout_tokens.icon_style
        )
    } else if let Some(ref prev) = selected_previous_output {
        format!(
            "WORKFLOW MODE: REPLACE & UPGRADE PREVIOUS COMPONENT REFERENCE (`{}` — {})\n\
- See what buttons, controls, and actions this reference currently has, keep its core functionality, add any new buttons/features/layout changes requested in the user's prompt, render texts/icons strictly according to the Brand Kit (`{}`), Chosen Web Design Style, and user instructions, and state in `implementation_notes` that the old component must be changed/updated to this new generated version.",
            prev.asset_id, prev.label, brand_snapshot.layout_tokens.icon_style
        )
    } else {
        format!(
            "WORKFLOW MODE: GENERATE BRAND-NEW COMPONENT / DESIGN FROM SCRATCH (No Current Screenshot Attached)\n\
- No existing UI screenshot was pasted. Generate the component, buttons, controls, texts, icons (`{}`), layout, and copy directly from the user's prompt, what the user wants, what we are building inside this project, the Brand Kit, and the Chosen Web Design Style.",
            brand_snapshot.layout_tokens.icon_style
        )
    };

    let detected_archetype = crate::models::detect_product_archetype(
        &project_meta.name,
        &project_meta.description,
        &project_meta.purpose,
        &project_meta.industry,
        mode,
        &model_settings.scope_preset,
        user_request,
    );
    let product_archetype_mandate =
        crate::models::build_product_archetype_guidance(detected_archetype);

    let tab_and_edit_context = {
        let mut lines = Vec::new();
        if let Some(ref tk) = model_settings.target_tab_key {
            let tl = model_settings
                .target_tab_label
                .as_deref()
                .unwrap_or(tk.as_str());
            lines.push(format!(
                "- TARGET CANVAS TAB: Set `tab_key: \"{}\"` and `tab_label: \"{}\"` on the primary item (and on any dependent extracted transparent asset items belonging to this component).",
                tk, tl
            ));
        }
        if let Some(ref existing_out_id) = model_settings.target_existing_output_id {
            lines.push(format!(
                "- EDITING / ADAPTING EXISTING CANVAS TAB OUTPUT (`target_existing_output_id`: `{}`): Set `target_existing_output_id: \"{}\"` on the primary item so the worker automatically passes that existing output image in `input_references` to `openai/gpt-image-2.5-sunburst`! Preserve everything in that existing output that should stay the same while applying the user's requested edits or viewport adaptation.",
                existing_out_id, existing_out_id
            ));
        }
        if let Some(ref vp) = model_settings.viewport_intent {
            lines.push(format!(
                "- TARGET VIEWPORT INTENT: `{}` (CRITICAL: For any Full Page / Landing Page / Multi-Section Website, ALWAYS use a vertical aspect ratio `2:3` or `9:16` from top to bottom — NEVER horizontal `16:9` like a PowerPoint slide! And INSIDE the image, strictly render `{}` breakpoint dimensions: `1440px` multi-column desktop grid for `desktop`, `834px` 2-column grid for `tablet`, or `390px` single-column smartphone layout for `mobile`).",
                vp, vp
            ));
        }
        if !model_settings.highlight_annotations.is_empty() {
            lines.push("- HIGHLIGHTED REGION COMMENTS FOR TARGETED EDIT (MUST use `quality_override: \"medium\"` and produce 1 focused edited item):".to_string());
            for (idx, h) in model_settings.highlight_annotations.iter().enumerate() {
                lines.push(format!(
                    "  {}. Target Output `{}` (Tab `{}`): Highlight Box [x={:.1}%, y={:.1}%, width={:.1}%, height={:.1}%] -> User Comment: \"{}\". Modify ONLY the highlighted element/region as requested while keeping the rest of the component/page 100% consistent!",
                    idx + 1,
                    h.output_id,
                    h.tab_label,
                    h.x_pct,
                    h.y_pct,
                    h.w_pct,
                    h.h_pct,
                    h.comment
                ));
            }
        }
        if mode == "full_page" {
            lines.push(
                "- MULTI-PAGE FULL-WEBSITE / WEBAPP GENERATION & IMAGE-TO-IMAGE CROSS-PAGE CONSISTENCY (`mode: \"full_page\"`): \
                1) IF THIS PRODUCT IS MULTI-PAGE OR A FULL WEBAPP FRONTEND (inspect the Product Description, Project Purpose, Chosen Style's Essential Category Screens, and User Request): You MUST generate a separate vertical UI page item (`kind: \"page\"`, `aspect_ratio: \"2:3\"` for desktop or `\"9:16\"` for mobile, `transparent_background: false`) for EVERY distinct page, route, and dashboard of the product (e.g., `item_1` Route `/` Primary Landing Page, `item_2` Route `/discover` or `/explore` Directory/Marketplace Page, `item_3` Route `/dashboard` or `/admin` Analytics/User Dashboard, `item_4` Route `/request` or `/submit` or `/detail` Workflow/Form Page) inside the SAME `\"full-website\"` tab! \
                2) IMAGE-TO-IMAGE VISUAL ANCHOR (`item_1` -> `item_2`, `item_3`, `item_4`...): Set `is_visual_anchor: true` and `depends_on: []` on Page 1 (`id: \"item_1\"`). On EVERY subsequent page (`item_2`, `item_3`, `item_4`, ..., `kind: \"page\"`), you MUST set `depends_on: [\"item_1\"]`! The backend worker automatically passes Page 1's rendered image as an Image-to-Image reference to `openai/gpt-image-2.5-sunburst`. \
                3) EXPLICIT NAVBAR CONTINUITY & PAGE CONTENT SPEC IN `image_prompt`: In the `image_prompt` of every secondary page (`item_2`, `item_3`, ...), explicitly instruct the image model to keep the EXACT SAME sticky top navigation bar (same brand logo on the left, same horizontal nav links in the center with the active pill/underline on the current route, same CTA button on the right, same header height/surface/border) and same footer/design tokens as the attached `item_1` reference image, AND specify in complete detail what sections, dashboard KPI cards, charts, filter rails, tables, forms, or card grids are displayed inside THIS specific page from top to bottom! \
                4) NO DUPLICATE VARIATIONS OF THE SAME PAGE, & EXTRACT TRUE BRAND/DECORATIVE ASSETS: Never generate 2 variations of the same page. After the distinct `kind: \"page\"` items, include the shared transparent-background assets (`Brand Logo / Wordmark`, `3D Decorative Props / Plants / Mascots`, `Organized Custom Icon Sheet` with `kind: \"asset\"`, `transparent_background: true`, `depends_on: [\"item_1\"]`, `extracted_from_item_id: \"item_1\"`). Do NOT generate separate asset images for placeholder user photos/avatars or UI cards/banners — document them under `PLACEHOLDER CONTENT & 1:1 CODED CARDS/BANNERS:` in `implementation_notes`, alongside `FRAMER MOTION, ANIMATIONS & EFFECTS:` best practices!".to_string()
            );
        } else {
            lines.push(
                "- NO MULTIPLE VARIATIONS & TRUE BRAND/DECORATIVE TRANSPARENT ASSET EXTRACTION (NEVER GENERATE PLACEHOLDER IMAGES): \
                1) NEVER generate 2 or more variations of a component, hero, or landing page! Always create 1 single canonical UI item (`transparent_background: false`) for this tab. \
                2) Extract ONLY true brand & decorative visual assets (Brand Logo/Wordmark/3D Spark Badge, 3D Decorative Props such as potted plants/megaphones/mascots/floating 3D shapes, custom illustrations, and organized Custom Icon Sheets) as separate `kind: \"asset\"`, `transparent_background: true` items in the SAME tab (`tab_key` and `tab_label`). \
                3) NEVER generate separate `kind: \"asset\"` images for placeholder user photos/avatars (like creator avatar stacks or directory photo thumbnails) or UI cards/banners (like a profile card mockup or ad banner card)! Those are placeholder sample content for a future database, and their cards/banners must be coded 1:1 in HTML/CSS by the coding agent. \
                4) In the primary UI item's `implementation_notes`, ALWAYS include a `PLACEHOLDER CONTENT & 1:1 CODED CARDS/BANNERS:` note specifying what in the image is placeholder data for a future database and mandating 1:1 HTML/CSS implementation of all cards, mockups, and banners right now, plus a `FRAMER MOTION, ANIMATIONS & EFFECTS:` note!".to_string()
            );
        }
        lines.join("\n")
    };

    let planner_user_prompt = format!(
        "--- USER-SELECTED RUN OPTIONS ---\n\
MODE: {mode}\n\
SCOPE PRESET: {scope}\n\
DEFAULT ASPECT RATIO: {aspect}\n\
IMAGE QUALITY TIER: {quality}\n\
ASSET MODE OPTIONS: {asset_opts}\n\n\
--- CANVAS TAB, EDIT & TRANSPARENT EXTRACTION DIRECTIVES ---\n\
{tab_and_edit}\n\n\
--- SCREENSHOT & COMPONENT CREATION WORKFLOW MANDATE ---\n\
{screenshot_workflow}\n\n\
--- DETECTED PRODUCT ARCHETYPE & SCOPE MANDATE ---\n\
{product_archetype}\n\n\
--- WHAT WE ARE WORKING ON INSIDE THIS PROJECT ---\n\
Project Name: {proj_name}\n\
Product / App Description (What it is & how it works): {proj_desc}\n\
Primary Project Purpose: {proj_purpose}\n\
Industry / Domain: {proj_industry}\n\
Target Audience: {proj_audience}\n\
Audience Needs & Pain Points: {proj_needs}\n\
Desired User Action (Primary Conversion / Goal): {proj_action}\n\
Target Tech Stack & Component Conventions: {target_stack} ({comp_notes})\n\n\
--- CHOSEN WEB DESIGN STYLE & DESIGN SYSTEM MANDATE ---\n\
{style_block}\n\n\
--- BRAND KIT SNAPSHOT (v{brand_version}) ---\n\
Brand Personality: {personality}\n\
Design Direction: {direction}\n\
Tone of Voice: {tone} (Language: {lang}, Density: {density})\n\
Colors (Light): primary={c_pri}, accent={c_acc}, bg={c_bg}, surface={c_surf}, text={c_txt}, muted={c_mut}, border={c_brd}\n\
Colors (Dark): primary={d_pri}, accent={d_acc}, bg={d_bg}, surface={d_surf}, text={d_txt}, muted={d_mut}, border={d_brd}\n\
Typography: heading={f_head}, body={f_body}, mono={f_mono}, scale={f_scale}\n\
Layout Tokens: spacing={l_space}, radii={l_rad}, borders={l_brd}, shadows={l_shd}, buttons={l_btn}\n\
Logo Policy: mode={logo_mode}, inclusion={logo_inc}, usage={logo_use}\n\
Brand Rules: {rules}\n\
Mandatory Inclusions: {mandatory}\n\
Forbidden Treatments: {forbidden}\n\n\
--- EXPLICITLY SELECTED ATTACHMENTS ({ref_count}) ---\n\
{refs_block}\n\n\
--- CURRENT USER REQUEST ---\n\
{user_request}\n\n\
--- SCHEMA REQUIREMENT ---\n\
{schema}",
        mode = mode,
        scope = model_settings.scope_preset,
        aspect = model_settings.aspect_ratio,
        quality = model_settings.quality,
        asset_opts = asset_opts_summary,
        tab_and_edit = tab_and_edit_context,
        screenshot_workflow = screenshot_workflow_mandate,
        product_archetype = product_archetype_mandate,
        brand_version = brand_version,
        proj_name = project_meta.name,
        proj_desc = project_meta.description,
        proj_purpose = project_meta.purpose,
        proj_industry = project_meta.industry,
        proj_audience = project_meta.target_audience,
        proj_needs = project_meta.audience_needs,
        proj_action = project_meta.desired_user_action,
        target_stack = brand_snapshot.target_context.target_stack,
        comp_notes = brand_snapshot.target_context.component_library_notes,
        style_block = style_block,
        personality = brand_snapshot.brand_personality,
        direction = brand_snapshot.design_direction,
        tone = brand_snapshot.tone_of_voice,
        lang = brand_snapshot.content_language,
        density = brand_snapshot.ui_density,
        c_pri = brand_snapshot.colors.light.primary,
        c_acc = brand_snapshot.colors.light.accent,
        c_bg = brand_snapshot.colors.light.background,
        c_surf = brand_snapshot.colors.light.surface,
        c_txt = brand_snapshot.colors.light.text,
        c_mut = brand_snapshot.colors.light.muted_text,
        c_brd = brand_snapshot.colors.light.border,
        d_pri = brand_snapshot.colors.dark.primary,
        d_acc = brand_snapshot.colors.dark.accent,
        d_bg = brand_snapshot.colors.dark.background,
        d_surf = brand_snapshot.colors.dark.surface,
        d_txt = brand_snapshot.colors.dark.text,
        d_mut = brand_snapshot.colors.dark.muted_text,
        d_brd = brand_snapshot.colors.dark.border,
        f_head = brand_snapshot.typography.heading_family,
        f_body = brand_snapshot.typography.body_family,
        f_mono = brand_snapshot.typography.mono_family,
        f_scale = brand_snapshot.typography.size_scale,
        l_space = brand_snapshot.layout_tokens.spacing_scale,
        l_rad = brand_snapshot.layout_tokens.radii,
        l_brd = brand_snapshot.layout_tokens.borders,
        l_shd = brand_snapshot.layout_tokens.shadows,
        l_btn = brand_snapshot.layout_tokens.button_treatment,
        logo_mode = brand_snapshot.logo.mode,
        logo_inc = brand_snapshot.logo.inclusion_policy,
        logo_use = brand_snapshot.logo.preferred_usage,
        rules = brand_snapshot.brand_rules.join("; "),
        mandatory = brand_snapshot.mandatory_inclusions.join("; "),
        forbidden = brand_snapshot.forbidden_treatments.join("; "),
        ref_count = total_selected_images,
        refs_block = if all_refs_desc.is_empty() {
            "None".to_string()
        } else {
            all_refs_desc.join("\n")
        },
        user_request = user_request,
        schema = output_schema_description,
    );

    BuiltGenerationContext {
        system_prompt: system_prompt.to_string(),
        output_schema_description,
        project_meta,
        brand_version,
        brand_snapshot,
        mode: mode.to_string(),
        user_request: user_request.to_string(),
        model_settings,
        current_screenshot,
        explicit_attachments,
        selected_previous_output,
        approved_plan,
        shared_anchor_images,
        compact_summary,
        planner_user_prompt,
    }
}

/// Helper to summarize selected references without loading full bytes.
pub fn summarize_context_badge(selected_refs: &[SelectedReference]) -> String {
    let count = selected_refs.len();
    format!(
        "Brand kit + current request + {} selected {}",
        count,
        if count == 1 { "image" } else { "images" }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_builder_isolates_only_explicit_inputs() {
        let meta = ProjectMetaSummary {
            id: "proj_1".into(),
            name: "Forma".into(),
            description: "Design platform".into(),
            purpose: "SaaS marketing & product".into(),
            industry: "Creative tools".into(),
            target_audience: "Creative teams".into(),
            audience_needs: "Clarity and fast iteration".into(),
            desired_user_action: "Start creating".into(),
        };
        let settings = RunModelSettings {
            planner_model: "z-ai/glm-5.3-flash".into(),
            image_model: "openai/gpt-image-2.5-sunburst".into(),
            quality: "medium".into(),
            aspect_ratio: "4:3".into(),
            output_size: "provider_default".into(),
            scope_preset: "isolated_component".into(),
            custom_dimensions: None,
            review_plan_first: false,
            asset_options: None,
            target_tab_key: None,
            target_tab_label: None,
            target_existing_output_id: None,
            viewport_intent: None,
            highlight_annotations: vec![],
            single_focused_output: false,
        };
        let refs = vec![
            ResolvedReferenceInput {
                asset_id: "shot_1".into(),
                role: AttachmentRole::CurrentScreenshot,
                label: "Pricing card v1".into(),
                placement_notes: "".into(),
                preserve_as_is: false,
                mime_type: "image/png".into(),
                width: 800,
                height: 600,
                data_url: "data:image/png;base64,AAAA".into(),
            },
            ResolvedReferenceInput {
                asset_id: "logo_1".into(),
                role: AttachmentRole::BrandLogo,
                label: "Forma mark".into(),
                placement_notes: "Top left".into(),
                preserve_as_is: true,
                mime_type: "image/svg+xml".into(),
                width: 200,
                height: 60,
                data_url: "data:image/png;base64,BBBB".into(),
            },
        ];

        let ctx = build_generation_context(
            PLANNER_SYSTEM_PROMPT_V1,
            meta,
            1,
            BrandSpec::default(),
            "component",
            "Polish this pricing card.",
            settings,
            refs,
            None,
            vec![],
        );

        assert_eq!(
            ctx.compact_summary,
            "Brand kit + current request + 2 selected images"
        );
        assert!(ctx.current_screenshot.is_some());
        assert_eq!(ctx.explicit_attachments.len(), 1);
        assert!(ctx.selected_previous_output.is_none());
        assert!(ctx.planner_user_prompt.contains("shot_1"));
        assert!(ctx.planner_user_prompt.contains("logo_1"));
    }
}

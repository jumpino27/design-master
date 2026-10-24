use crate::{
    config::AppConfig,
    error::{AppError, AppResult},
    image_util::atomic_write_file,
    models::{BrandSpec, PlannerPlan, RunModelSettings, SelectedReference},
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use std::{
    io::{Seek, Write},
    path::Path,
};
use uuid::Uuid;
use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

pub const HANDOFF_SINGLE_TEMPLATE_V1: &str = include_str!("../../../prompts/v1/handoff_single.txt");
pub const HANDOFF_SET_TEMPLATE_V1: &str = include_str!("../../../prompts/v1/handoff_set.txt");

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandoffResponse {
    pub export_id: String,
    pub export_kind: String,
    pub clipboard_text: String,
    pub manifest_path: String,
    pub implementation_path: String,
    pub bundle_path: Option<String>,
    pub include_extraction_instructions: bool,
    pub is_complete_set: bool,
    pub missing_items: Vec<String>,
}

/// Formats a filesystem path safely for agent prompts, quoting paths with spaces or special characters
/// while preserving Unicode and OS-native separators (including Windows backslashes).
pub fn format_host_path(path: &Path) -> String {
    let raw = path.to_string_lossy().to_string();
    if raw.contains(' ') || raw.contains('(') || raw.contains(')') || raw.contains('&') {
        format!("\"{}\"", raw)
    } else {
        raw
    }
}

fn build_extraction_steps(include_extraction: bool) -> String {
    if include_extraction {
        "5. Inspect the reference for required visual assets. Prefer the original provided files. If an image contains a usable embedded illustration, photograph, decorative shape, or background without a separate source file, extract or crop it into its own asset when feasible. Remove a background only with suitable tooling, preserve alpha, avoid UI/text fragments in the crop, and save the asset under the target project's asset convention.\n\
6. Do not claim a cropped image recovers pixels hidden behind text or UI. When a clean extraction is impossible, report the missing asset and use an explicitly documented recreation or request a separate asset. Do not invent a high-resolution source or silently replace a required subject."
            .to_string()
    } else {
        "5. Reuse the provided original source files directly. Asset extraction from generated UI screenshots is disabled for this handoff.\n\
6. Do not crop behind text or UI overlays; if a standalone asset is missing, request a dedicated Asset Mode generation."
            .to_string()
    }
}

fn format_brand_tokens(brand: &BrandSpec) -> String {
    format!(
        "- Brand Personality & Direction: {} — {}\n\
- Tone & UI Density: {} | Density: {} | Language: {}\n\
- Light Palette: primary={}, accent={}, background={}, surface={}, text={}, muted_text={}, border={}, success={}, warning={}, danger={}\n\
- Dark Palette: primary={}, accent={}, background={}, surface={}, text={}, muted_text={}, border={}\n\
- Typography: Heading='{}', Body='{}', Mono='{}', Fallbacks='{}', Scale='{}', Line Heights='{}', Source='{}'\n\
- Layout & Component Tokens: Spacing='{}', Content Width='{}', Grid='{}', Radii='{}', Borders='{}', Shadows='{}', Icons='{}', Buttons='{}'\n\
- Brand Rules: {}\n\
- Mandatory Inclusions: {}\n\
- Forbidden Treatments: {}",
        brand.brand_personality,
        brand.design_direction,
        brand.tone_of_voice,
        brand.ui_density,
        brand.content_language,
        brand.colors.light.primary,
        brand.colors.light.accent,
        brand.colors.light.background,
        brand.colors.light.surface,
        brand.colors.light.text,
        brand.colors.light.muted_text,
        brand.colors.light.border,
        brand.colors.light.success,
        brand.colors.light.warning,
        brand.colors.light.danger,
        brand.colors.dark.primary,
        brand.colors.dark.accent,
        brand.colors.dark.background,
        brand.colors.dark.surface,
        brand.colors.dark.text,
        brand.colors.dark.muted_text,
        brand.colors.dark.border,
        brand.typography.heading_family,
        brand.typography.body_family,
        brand.typography.mono_family,
        brand.typography.fallback_stack,
        brand.typography.size_scale,
        brand.typography.line_heights,
        brand.typography.font_source,
        brand.layout_tokens.spacing_scale,
        brand.layout_tokens.content_width,
        brand.layout_tokens.grid_columns,
        brand.layout_tokens.radii,
        brand.layout_tokens.borders,
        brand.layout_tokens.shadows,
        brand.layout_tokens.icon_style,
        brand.layout_tokens.button_treatment,
        if brand.brand_rules.is_empty() {
            "None specified".to_string()
        } else {
            brand.brand_rules.join("; ")
        },
        if brand.mandatory_inclusions.is_empty() {
            "None specified".to_string()
        } else {
            brand.mandatory_inclusions.join("; ")
        },
        if brand.forbidden_treatments.is_empty() {
            "None specified".to_string()
        } else {
            brand.forbidden_treatments.join("; ")
        }
    )
}

fn format_target_context(brand: &BrandSpec) -> String {
    let tc = &brand.target_context;
    let mut parts = Vec::new();
    if !tc.target_stack.trim().is_empty() {
        parts.push(format!("Stack: {}", tc.target_stack.trim()));
    }
    if !tc.repository_root.trim().is_empty() {
        parts.push(format!("Repository root: {}", tc.repository_root.trim()));
    }
    if !tc.preferred_asset_dir.trim().is_empty() {
        parts.push(format!(
            "Preferred asset directory: {}",
            tc.preferred_asset_dir.trim()
        ));
    }
    if !tc.component_library_notes.trim().is_empty() {
        parts.push(format!(
            "Component library notes: {}",
            tc.component_library_notes.trim()
        ));
    }
    if parts.is_empty() {
        "Inspect the existing target project repository to follow its framework and styling conventions.".to_string()
    } else {
        parts.join(" | ")
    }
}

/// Converts any incidental code-like fragments (backticks, HTML/JSX tags, or inline code tokens)
/// that an LLM planner might have emitted in its free-text notes into clean, natural human language
/// so the copied handoff prompt contains zero code snippets.
pub fn clean_code_syntax_to_human_prose(text: &str) -> String {
    let mut s = text.replace('`', "");
    for (pat, rep) in [
        (
            "<AnimatePresence mode=\"wait\">",
            "a route exit-and-enter presence transition",
        ),
        (
            "<AnimatePresence mode=\"popLayout\">",
            "a layout-aware exit-and-enter transition",
        ),
        ("<AnimatePresence>", "an exit-and-enter presence transition"),
        ("</AnimatePresence>", ""),
        ("<LayoutGroup>", "a coordinated layout group"),
        ("</LayoutGroup>", ""),
        ("<Navbar />", "the top navigation bar component"),
        ("<Footer />", "the global footer component"),
        ("<AppShell>", "the shared application shell"),
        ("<svg>", "a scalable vectorial SVG graphic"),
        ("</svg>", ""),
        ("<img>", "a responsive image element"),
        ("<Image>", "a framework image component"),
        ("<picture>", "a responsive picture container"),
        ("<header>", "the header landmark"),
        ("<nav>", "the navigation landmark"),
        ("<main>", "the main content area"),
        ("<dialog>", "a modal dialog overlay"),
        ("\"use client\"", "client-side interactive rendering"),
    ] {
        s = s.replace(pat, rep);
    }
    s
}

async fn build_existing_ui_replacement_spec(
    pool: &SqlitePool,
    config: &AppConfig,
    project_id: &str,
    selected_refs: &[SelectedReference],
    new_target_title: &str,
    new_output_path: &str,
    portable_relative: bool,
) -> AppResult<String> {
    let mut ref_entries = Vec::new();
    for sr in selected_refs {
        let is_existing_ui_ref = matches!(
            sr.role,
            crate::models::AttachmentRole::CurrentScreenshot
                | crate::models::AttachmentRole::SelectedPreviousOutput
                | crate::models::AttachmentRole::ContentReference
        ) || sr.placement_notes.to_ascii_lowercase().contains("current")
            || sr.placement_notes.to_ascii_lowercase().contains("old")
            || sr.placement_notes.to_ascii_lowercase().contains("replace")
            || sr.placement_notes.to_ascii_lowercase().contains("hero")
            || sr.placement_notes.to_ascii_lowercase().contains("header")
            || sr.placement_notes.to_ascii_lowercase().contains("navbar");

        if !is_existing_ui_ref {
            continue;
        }

        if let Some(row) = sqlx::query(
            "SELECT label, original_rel_path FROM assets WHERE id = ? AND project_id = ?",
        )
        .bind(&sr.asset_id)
        .bind(project_id)
        .fetch_optional(pool)
        .await?
        {
            let label: String = row.get("label");
            let orig_rel: String = row.get("original_rel_path");
            let path_str = if portable_relative {
                let fname = Path::new(&orig_rel)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("original");
                format!("assets/{}/{}", sr.asset_id, fname)
            } else {
                let abs = config.resolve_rel_path(&orig_rel)?;
                format_host_path(&abs)
            };
            let notes_part = if sr.placement_notes.trim().is_empty() {
                String::new()
            } else {
                format!(
                    " (User notes: {})",
                    clean_code_syntax_to_human_prose(&sr.placement_notes)
                )
            };
            ref_entries.push(format!(
                "'{}' (Role: {}, Reference Path: {}){}",
                label,
                sr.role.as_str(),
                path_str,
                notes_part
            ));
        }
    }

    if ref_entries.is_empty() {
        Ok("- Implementation Mode (New Design From Scratch): No prior UI screenshot or old hero/header reference was attached. Build this component or page fresh in the codebase following the newly generated reference image, the Project Brand Kit, and the Chosen Design Style.".to_string())
    } else {
        Ok(format!(
            "- CRITICAL UI REPLACEMENT MANDATE — CHANGE THE OLD HERO / HEADER / COMPONENT TO THIS NEW GENERATED DESIGN:\n  \
            * Attached Old / Current UI Reference(s) to Replace: {}\n  \
            * Newly Generated Replacement Design: '{}' at {}\n  \
            * What You Must Do in the Codebase: Locate the existing hero, header, navigation bar, or UI section in the project codebase that corresponds to the attached old reference above, and CHANGE / REPLACE that old component completely with the newly generated design shown at {}! Keep and reconnect all existing functional navigation routes, click actions, event handlers, state bindings, and backend data connections from the old version, while completely replacing its outdated visual layout, structure, styling, and assets with the new generated design 1:1.",
            ref_entries.join("; "),
            new_target_title,
            new_output_path,
            new_output_path
        ))
    }
}

fn format_design_style_block(brand: &BrandSpec) -> String {
    if let Some(ref ds) = brand.design_style {
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
                .unwrap_or("Sticky top navigation, hero value proposition, core interactive cards, and conversion flow.");
            return format!(
                "- Selected Design Style: #{num} {name} (Category: {cat})\n\
- Exact Style Definition: {def}\n\
- Concrete Visual System Rules: {rules}\n\
- Best Product Fit: {best_fit}\n\
- Category Essential Screens & Component Hierarchy: {essential_screens}\n\
- Combinable Layout & System Modifiers: {mods}\n\
- Combined Style Brief: {brief}\n\
- Design System Keywords: {kw}\n\
- USER BRAND KIT AUTHORITY RULE: Execute all layout structures, card hierarchies, borders, corner radii, and surface treatments of Style #{num} ({name}) above using the USER'S BRAND TOKENS AND LOGO below (Light Primary {lp}, Accent {la}, Surface {ls} / Dark Primary {dp}, Surface {ds}) as the authoritative color and brand identity!",
                num = num,
                name = ds.style_name,
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
                lp = brand.colors.light.primary,
                la = brand.colors.light.accent,
                ls = brand.colors.light.surface,
                dp = brand.colors.dark.primary,
                ds = brand.colors.dark.surface,
            );
        }
    }
    format!(
        "- Selected Design Style: {} — {}\n\
- Visual System Rules: Follow the Brand Personality, Design Direction, and Layout Tokens below strictly.",
        brand.brand_personality, brand.design_direction
    )
}

fn build_placeholder_and_card_mandate(
    brand: &BrandSpec,
    plan: Option<&PlannerPlan>,
    item: Option<&crate::models::PlanItem>,
    archetype: crate::models::ProductArchetype,
) -> String {
    let l = &brand.colors.light;
    let d = &brand.colors.dark;
    let style_name = brand
        .design_style
        .as_ref()
        .map(|s| s.style_name.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or(&brand.brand_personality);

    let archetype_header = match archetype {
        crate::models::ProductArchetype::GameUiInterface => {
            "0. DETECTED PRODUCT ARCHETYPE — GAME UI / HUD / GAME INTERFACE COMPONENTS (CRITICAL BOUNDARY):\n   \
            - Only Implement the UI Interface Components & Frames (Image-to-Code): This design is for a Game UI, HUD, or Game Interface. Do NOT invent or bake in static in-game characters, 3D gameplay worlds, or lore scenes as screenshots! Any background 3D gameplay scene, character model stage or pedestal, character portrait inside a frame, or sample item inside an inventory slot shown in the reference image is strictly a PLACEHOLDER VIEWPORT OR SLOT where the live game engine or runtime state will render.\n   \
            - Build Every HUD Frame, Bar, Panel, Slot & Button 1:1 in Code: You MUST implement all UI interface components—including health/mana/XP bars, ornate or sci-fi frames, inventory slot grids, skill tree nodes, minimap frames, dialog boxes, badges, and interactive buttons—1:1 from the reference image using code plus the extracted transparent UI emblems, icons, and 3D UI objects.\n"
        }
        crate::models::ProductArchetype::MultiPageWebapp => {
            "0. DETECTED PRODUCT ARCHETYPE — MULTI-PAGE WEB APPLICATION & DASHBOARD FRONTEND:\n   \
            - Coordinated Multi-Route Application: This design represents a multi-page web application with a persistent shared navigation bar, shared footer, and dedicated route views and dashboards.\n"
        }
        crate::models::ProductArchetype::LandingPage => {
            "0. DETECTED PRODUCT ARCHETYPE — VERTICAL FULL-PAGE LANDING PAGE:\n   \
            - Top-to-Bottom Marketing & Conversion Flow: This design represents a full-page vertical scroll layout from the sticky top navigation bar through the hero, core value sections, cards, and footer.\n"
        }
        crate::models::ProductArchetype::ReusableUiComponent => {
            "0. DETECTED PRODUCT ARCHETYPE — MODULAR REUSABLE UI COMPONENT:\n   \
            - Self-Contained Component: This design represents a modular, reusable interface component ready to integrate into the host application.\n"
        }
    };

    let mut lines = vec![
        archetype_header.to_string(),
        "1. WHAT IN THE REFERENCE IMAGE IS PLACEHOLDER CONTENT (FOR FUTURE REAL DATABASE OR LIVE GAME RUNTIME):".to_string(),
        "   - Dynamic Sample Data, User Profiles & Runtime Viewports: Any sample user or creator profiles (such as a featured profile card mockup, sample names, handles, bios, category tags, follower or view counts, or ratings), creator avatar stacks, directory listing cards, sample banner ad slots, testimonials, live statistics, or in-game character/world background viewports and sample inventory slot contents visible in the reference image are PLACEHOLDER SAMPLE CONTENT.".to_string(),
        "   - Future Database & Runtime Integration: The user will connect these dynamic cards, profiles, avatars, directory listings, banner slots, and game UI slots to a real database, backend API, or live game engine state in the future.".to_string(),
        "   - Why Placeholder Photos & Characters Are Not Separate Asset Files: Standalone transparent assets in this tab are generated ONLY for true brand, decorative, and 3D interface graphics (brand logos, wordmarks, UI crests, 3D objects/props/plants/artifacts, mascots, custom illustrations, and icon sheets). Placeholder user photos, avatars, in-game characters, and UI cards are intentionally NOT generated as standalone image assets.".to_string(),
        format!(
            "\n2. CRUCIAL 1:1 MANDATE FOR RIGHT NOW — BUILD EVERY UI COMPONENT, CARD, HUD FRAME & BANNER 1:1 IN CODE AS SHOWN IN THE IMAGE (Style: {}):",
            style_name
        ),
        format!(
            "   - Build Every Card, Frame & Banner 1:1 in Code: Right now, you MUST build every single UI component, Hero Profile Card Mockup, Directory Listing Card, Ad Banner Card, Pricing Tier Card, HUD Frame, Inventory Slot, and Stats Pill 1:1 in code matching the exact layout, proportions, internal spacing, corner radius ({}), border style ({}), shadow depth ({}), surface colors ({} in light mode / {} in dark mode), primary and accent highlights ({} / {}), status badges, and buttons shown in the reference image!",
            brand.layout_tokens.radii,
            brand.layout_tokens.borders,
            brand.layout_tokens.shadows,
            l.surface,
            d.surface,
            l.primary,
            l.accent
        ),
        "   - Never Flatten or Skip UI Components, Cards, or Frames: Do NOT replace a profile card mockup, directory card, HUD panel, or banner with a static screenshot image, and do NOT omit or simplify its internal elements. Build every inner badge, avatar or slot frame, tag pill, progress bar, metric counter, and action button inside the component 1:1 in code.".to_string(),
        "   - How to Populate Placeholder Avatars, Thumbnails & Slots Right Now: For sample user portraits, avatar stacks, directory card thumbnails, or inventory/character slots shown in the reference image, render clean, styled placeholder containers matching the image's exact frame geometry and color palette, backed by a structured mock-data collection containing the exact sample names, titles, tags, and numbers from the reference image so the interface renders 1:1 immediately and is ready for real database records or live game state later.".to_string(),
        "   - Pure Code vs. 2D Vector SVG vs. Interactive 3D Objects (Image-to-3D & Rotation Mandate): Build all section separators, glowing dividers, ambient background gradients, and UI frames purely in code; convert brand logos, icons, and flat graphical illustrations into crisp scalable vectorial SVG graphics; and for any volumetric 3D centerpiece, 3D orb, 3D artifact, or 3D relic shown in the design, construct it as an interactive 3D object from its isolated reference image (using WebGL, Three.js, React Three Fiber, or an image-to-3D mesh pipeline) rather than a flat static image, and animate the 3D object or component to smoothly rotate, float, and respond to pointer movement.".to_string(),
    ];

    let mut specific_notes = Vec::new();
    if let Some(it) = item {
        if !it.exact_copy.is_empty() {
            specific_notes.push(format!(
                "   - Exact Sample / Placeholder Copy & Labels in '{}' to Populate 1:1 Now: {}",
                it.name,
                clean_code_syntax_to_human_prose(&it.exact_copy.join(" | "))
            ));
        }
        for note in &it.implementation_notes {
            let lower = note.to_ascii_lowercase();
            if lower.contains("placeholder")
                || lower.contains("database")
                || lower.contains("1:1")
                || lower.contains("card")
                || lower.contains("banner")
                || lower.contains("mockup")
                || lower.contains("avatar")
                || lower.contains("directory")
                || lower.contains("game")
                || lower.contains("hud")
                || lower.contains("slot")
                || lower.contains("separator")
                || lower.contains("divider")
                || lower.contains("background")
                || lower.contains("gradient")
                || lower.contains("glow")
                || lower.contains("code")
                || lower.contains("3d")
                || lower.contains("rotate")
            {
                specific_notes.push(format!(
                    "   - [{}] {}",
                    it.name,
                    clean_code_syntax_to_human_prose(note)
                ));
            }
        }
    } else if let Some(p) = plan {
        for it in &p.items {
            if it.kind == "asset" {
                continue;
            }
            if !it.exact_copy.is_empty() {
                specific_notes.push(format!(
                    "   - Exact Sample / Placeholder Copy & Labels in '{}' ({}) to Populate 1:1 Now: {}",
                    it.name,
                    it.id,
                    clean_code_syntax_to_human_prose(&it.exact_copy.join(" | "))
                ));
            }
            for note in &it.implementation_notes {
                let lower = note.to_ascii_lowercase();
                if lower.contains("placeholder")
                    || lower.contains("database")
                    || lower.contains("1:1")
                    || lower.contains("card")
                    || lower.contains("banner")
                    || lower.contains("mockup")
                    || lower.contains("avatar")
                    || lower.contains("directory")
                    || lower.contains("game")
                    || lower.contains("hud")
                    || lower.contains("slot")
                    || lower.contains("separator")
                    || lower.contains("divider")
                    || lower.contains("background")
                    || lower.contains("gradient")
                    || lower.contains("glow")
                    || lower.contains("code")
                    || lower.contains("3d")
                    || lower.contains("rotate")
                {
                    specific_notes.push(format!(
                        "   - [{}] {}",
                        it.name,
                        clean_code_syntax_to_human_prose(note)
                    ));
                }
            }
        }
    }

    if !specific_notes.is_empty() {
        lines.push("\n3. ITEM-SPECIFIC PLACEHOLDER DATA, CODED SEPARATORS/BACKGROUNDS & 1:1 COMPONENT DETAILS FROM THIS DESIGN:".to_string());
        lines.extend(specific_notes);
    }

    lines.join("\n")
}

fn build_sticky_positioning_spec(
    mode: &str,
    kind: &str,
    scope_preset: &str,
    title: &str,
    brand: &BrandSpec,
    plan: Option<&PlannerPlan>,
    item: Option<&crate::models::PlanItem>,
) -> String {
    let c_surf = &brand.colors.light.surface;
    let c_brd = &brand.colors.light.border;
    let d_surf = &brand.colors.dark.surface;
    let d_brd = &brand.colors.dark.border;

    let mut lines = Vec::new();
    let lower_title = title.to_ascii_lowercase();
    let is_nav = scope_preset == "navbar"
        || lower_title.contains("navbar")
        || lower_title.contains("navigation")
        || lower_title.contains("header");
    let is_modal = scope_preset == "modal"
        || kind == "state"
        || lower_title.contains("modal")
        || lower_title.contains("dialog")
        || lower_title.contains("drawer");
    let is_player_or_dock = lower_title.contains("player")
        || lower_title.contains("dock")
        || lower_title.contains("bottom bar")
        || lower_title.contains("toolbar")
        || lower_title.contains("hud");

    lines.push("WHAT MUST STAY ANCHORED (STICKY OR FIXED ON SCROLL):".to_string());
    if mode == "full_page" || is_nav {
        lines.push(format!(
            "- Top Navigation Header: Must remain anchored at the very top edge of the viewport across 100 percent width on the highest navigation layer (z-index level 50). At the top of the page (zero scroll offset), render it with the solid surface color {c_surf} (or {d_surf} in dark mode); once the user scrolls past 12 pixels, smoothly transition it to a translucent frosted-glass surface at 88 percent opacity with a 12-pixel background blur and a 1-pixel bottom border in {c_brd} ({d_brd} in dark mode)."
        ));
    }
    if mode == "full_page" || is_player_or_dock {
        lines.push(
            "- Persistent Bottom Action Dock, HUD Bar, or Mobile Sticky Action Bar (when present in the layout): Must remain anchored at the bottom edge of the viewport on layer level 40, with matching bottom padding (around 88 pixels) on the main scrolling container so the dock never covers footer or card content.".to_string()
        );
    }
    if mode == "full_page" {
        lines.push(
            "- Desktop Sidebar, Filter Rail, or Table of Contents (when a multi-column application shell or filter sidebar is present): Keep it sticky at 84 pixels below the top viewport edge on desktop screens (1024 pixels and wider) with independent vertical scrolling inside its maximum viewport height, collapsing into a normal stacked block or slide-over drawer on mobile screens (under 1024 pixels).".to_string()
        );
    }
    if is_modal || mode == "full_page" {
        lines.push(
            "- Modal Dialogs, Inventory Overlays, and Slide-Over Drawers: Must be fixed across the full viewport on the top overlay layer (level 100) with a dimmed, blurred backdrop and background page scroll locking while open.".to_string()
        );
    }
    if !is_nav && !is_modal && !is_player_or_dock && mode != "full_page" {
        lines.push(format!(
            "- Component '{}' (Scope: {}): This component itself is not a global sticky header or fixed modal; keep its outer root container in normal relative positioning so the parent page or game screen layout controls where it sits.",
            title, scope_preset
        ));
    }

    lines.push("\nWHAT MUST NOT BE STICKY (KEEP IN NORMAL DOCUMENT FLOW):".to_string());
    lines.push(
        "- Hero Banners, Feature Cards, Pricing Tiers, Bento Grid Cells, Data Tables, Testimonial Blocks, and Footers must remain in normal vertical document flow. Never make content cards or hero sections sticky or fixed, as that traps viewport height and obscures scrolling content.".to_string()
    );
    lines.push(
        "- Scroll Container Rule: Never clip or hide overflow on an ancestor wrapper of a sticky element unless that ancestor is the intended scroll container.".to_string()
    );

    if let Some(it) = item {
        for note in &it.implementation_notes {
            if note.to_ascii_lowercase().contains("sticky")
                || note.to_ascii_lowercase().contains("fixed")
                || note.to_ascii_lowercase().contains("scroll")
                || note.to_ascii_lowercase().contains("z-index")
            {
                lines.push(format!(
                    "- Component Positioning Note: {}",
                    clean_code_syntax_to_human_prose(note)
                ));
            }
        }
    }
    if let Some(p) = plan {
        for lr in &p.shared_design_spec.layout_rules {
            lines.push(format!(
                "- Shared Layout & Positioning Rule: {}",
                clean_code_syntax_to_human_prose(lr)
            ));
        }
    }

    lines.join("\n")
}

fn build_interactive_states_and_colors_spec(
    brand: &BrandSpec,
    plan: Option<&PlannerPlan>,
    item: Option<&crate::models::PlanItem>,
) -> String {
    let l = &brand.colors.light;
    let d = &brand.colors.dark;
    let style_name = brand
        .design_style
        .as_ref()
        .map(|s| s.style_name.as_str())
        .filter(|s| !s.is_empty())
        .unwrap_or(&brand.brand_personality);
    let is_brutalist = style_name.to_ascii_lowercase().contains("brutal");

    let hover_effect_cta = if is_brutalist {
        format!(
            "shift 2 pixels left and 2 pixels up with a crisp hard-edge offset shadow of 6 pixels by 6 pixels in {}",
            l.text
        )
    } else {
        format!(
            "lift 2 pixels upward with a soft elevated glow shadow tinted with {} at 40 percent opacity",
            l.primary
        )
    };

    let active_effect_cta = if is_brutalist {
        format!(
            "press 2 pixels right and 2 pixels down while collapsing the hard shadow to 1 pixel by 1 pixel in {}",
            l.text
        )
    } else {
        "return to zero vertical offset and compress slightly to 98 percent scale with a tight subtle shadow".to_string()
    };

    let mut lines = vec![
        format!(
            "1. PRIMARY CALL-TO-ACTION BUTTONS & MAIN ACTIONS (Style: {}):\n\
   - Default Resting State: Background fill {} (or {} in dark mode), crisp white text #FFFFFF, corner radius {}, border {}, and resting shadow {}.\n\
   - Pointer Hover State: Deepen the primary background fill by mixing 88 percent {} with 12 percent black (or brighten {} slightly in dark mode), keep white text #FFFFFF, show a pointer cursor, and {}.\n\
   - Click / Pressed State: Deepen the background fill further to 76 percent {} mixed with black, and {}.\n\
   - Keyboard Focus State: Display a clear 2-pixel focus ring in {} with a 2-pixel offset and a soft 4-pixel outer halo in {} at 24 percent opacity.\n\
   - Disabled State: Reduce opacity to 45 percent, show a not-allowed cursor, disable pointer interactions, and remove hover elevation.",
            style_name,
            l.primary,
            d.primary,
            brand.layout_tokens.radii,
            brand.layout_tokens.borders,
            brand.layout_tokens.shadows,
            l.primary,
            d.primary,
            hover_effect_cta,
            l.primary,
            active_effect_cta,
            l.primary,
            l.primary
        ),
        format!(
            "2. SECONDARY, OUTLINE & GHOST BUTTONS AND ICON CONTROLS:\n\
   - Default Resting State: Surface fill {} (or transparent for ghost controls), text color {}, and a 1-pixel border in {} (Dark mode surface {}, text {}, border {}).\n\
   - Pointer Hover State: Tint the surface background with 10 percent {} over {}, transition the border and text color to {}, and lift 1 pixel upward.\n\
   - Click / Pressed State: Increase the primary surface tint to 18 percent {} over {}, keep the border in {}, and compress gently to 97 percent scale.\n\
   - Keyboard Focus State: Display a 2-pixel focus outline in {} with a 2-pixel offset.",
            l.surface,
            l.text,
            l.border,
            d.surface,
            d.text,
            d.border,
            l.primary,
            l.surface,
            l.primary,
            l.primary,
            l.surface,
            l.primary,
            l.primary
        ),
        format!(
            "3. INTERACTIVE CARDS, BENTO TILES, HUD PANELS & LIST ROWS:\n\
   - Default Resting State: Surface fill {} (Dark mode: {}), 1-pixel border in {} (Dark mode: {}), and resting shadow {}.\n\
   - Pointer Hover State: Transition the border color to a 50 percent blend of {} and {}, tint the surface subtly with 3 percent {} over {}, elevate the drop shadow, and lift the card 2 to 4 pixels upward.\n\
   - Selected / Active State: Highlight with a 2-pixel border in {} and a 7 percent tint of {} over {}.",
            l.surface,
            d.surface,
            l.border,
            d.border,
            brand.layout_tokens.shadows,
            l.primary,
            l.border,
            l.primary,
            l.surface,
            l.primary,
            l.primary,
            l.surface
        ),
        format!(
            "4. NAVIGATION LINKS, TABS & SEGMENTED PILL SWITCHES:\n\
   - Default Resting State: Muted text color {} (Dark mode: {}) on a transparent background.\n\
   - Pointer Hover State: Primary text color {} (Dark mode: {}) with a subtle 6 percent {} pill highlight.\n\
   - Active Route or Selected Tab State: Bold/semibold text in {} with the active pill or underline indicator colored in {} (or accent {}).",
            l.muted_text,
            d.muted_text,
            l.text,
            d.text,
            l.primary,
            l.text,
            l.primary,
            l.accent
        ),
        format!(
            "5. FORM INPUTS, SEARCH BARS & TOGGLES:\n\
   - Default Resting State: Surface fill {}, text color {}, placeholder text in {}, and a 1-pixel border in {}.\n\
   - Pointer Hover State: Darken the border slightly by blending 35 percent {} into {}.\n\
   - Focus State: Highlight the border in {} with a 3-pixel outer focus glow in {} at 18 percent opacity.\n\
   - Validation Feedback Colors: Error state border and helper text in {}; Success state in {}; Warning state in {}.",
            l.surface,
            l.text,
            l.muted_text,
            l.border,
            l.text,
            l.border,
            l.primary,
            l.primary,
            l.danger,
            l.success,
            l.warning
        ),
    ];

    if let Some(it) = item {
        if !it.behaviors.is_empty() {
            lines.push("\n6. ITEM-SPECIFIC INTERACTIVE BEHAVIORS FROM DESIGN PLAN:".to_string());
            for b in &it.behaviors {
                lines.push(format!("   - {}", clean_code_syntax_to_human_prose(b)));
            }
        }
    } else if let Some(p) = plan {
        let mut added_header = false;
        for it in &p.items {
            for b in &it.behaviors {
                if !added_header {
                    lines.push(
                        "\n6. ITEM-SPECIFIC INTERACTIVE BEHAVIORS FROM DESIGN PLAN:".to_string(),
                    );
                    added_header = true;
                }
                lines.push(format!(
                    "   - [{}] {}",
                    it.name,
                    clean_code_syntax_to_human_prose(b)
                ));
            }
        }
    }

    lines.join("\n")
}

fn build_animations_and_motion_spec(
    brand: &BrandSpec,
    plan: Option<&PlannerPlan>,
    item: Option<&crate::models::PlanItem>,
) -> String {
    let style_motion_note = if let Some(ref ds) = brand.design_style {
        if !ds.style_name.trim().is_empty() {
            format!(
                "Match the motion character of Style #{num} {name} — enforce its visual system rules ({rules}) across all hover, press, scroll-reveal, 3D object rotation, and route transitions.",
                num = if ds.style_number > 0 { ds.style_number } else { 1 },
                name = ds.style_name,
                rules = ds.visual_rules
            )
        } else {
            format!(
                "Match brand personality '{}' ({})",
                brand.brand_personality, brand.design_direction
            )
        }
    } else {
        format!(
            "Match brand personality '{}' ({})",
            brand.brand_personality, brand.design_direction
        )
    };

    let mut lines = vec![
        format!(
            "1. STYLE MOTION SIGNATURE & FRAMER MOTION ARCHITECTURE (PURE HUMAN LANGUAGE SPEC):\n   \
            - Aesthetic Motion Identity: {}\n   \
            - Animation Architecture Best Practices: Implement smooth, physics-driven UI animations using Framer Motion (or the target engine's native spring/tween animation system). Keep static layout containers lightweight and isolate interactive animated wrappers cleanly so page load performance stays instant.",
            style_motion_note
        ),
        "2. MULTI-PAGE ROUTE TRANSITIONS & SHARED NAVIGATION CONTINUITY:\n   \
        - Shared Sticky Navigation Active Indicator Glide: Keep the sticky top navigation bar mounted persistently across all pages. Attach a shared layout animation identifier to the active route pill or underline indicator so that switching between routes (such as Home, Discover, Dashboard, or Request) smoothly glides the indicator across navigation links using a snappy spring (stiffness around 420, damping around 32) without remounting the header.\n   \
        - Page Enter and Exit Choreography: Wrap route page views below the persistent navigation bar in an exit-and-enter presence transition so outgoing pages softly fade out and slide slightly upward while incoming pages fade in from 10 pixels below with a subtle 4-pixel-to-0-pixel unblur over roughly 280 milliseconds.\n   \
        - Shared Element Card-to-Detail Morphing: When clicking a directory card, profile card, inventory item, or dashboard widget opens a detail modal or drawer, link the card container and title with shared layout identifiers so the card smoothly expands into the detail view.".to_string(),
        "3. TACTILE SPRING PHYSICS & MICRO-INTERACTIONS:\n   \
        - Primary and Secondary Buttons: Apply responsive spring physics (stiffness around 450, damping around 26, mass around 0.7) so buttons lift 2 pixels upward and scale to 101 percent on pointer hover, and compress crisply to 97 percent scale on click or tap, paired with smooth 180-millisecond color and shadow transitions.\n   \
        - Interactive Cards, Bento Tiles, HUD Panels & Directory Rows: Apply spring hover elevation (stiffness around 340, damping around 24) that lifts cards 4 pixels upward on hover and compresses to 99 percent scale on press. For cards containing thumbnails or artwork, smoothly zoom the inner artwork to 103 percent scale over 320 milliseconds inside its clipped frame on hover.".to_string(),
        "4. SCROLL-TRIGGERED STAGGERED REVEALS:\n   \
        - Section & Grid Entrance Cascade: As each page section, card grid, HUD panel group, or dashboard widget row enters the viewport (triggering once when roughly 15 percent visible), stagger its child elements into view with a 60-millisecond delay between items, transitioning each item from invisible and 16 pixels lower to full opacity and resting position using a smooth spring (stiffness around 300, damping around 26).".to_string(),
        "5. DYNAMIC LAYOUT REFLOWS FOR FILTERS, SEARCH, TABS & GRIDS:\n   \
        - Smooth Grid & Tab Rearrangement: Coordinate filterable directory grids, search results, segmented tab panels, billing toggles, inventory slots, and dashboard boards inside a shared layout group with exit-and-enter transitions. Enable automatic layout reflow on each item (spring stiffness around 360, damping around 30) so filtering categories, searching, or switching tabs smoothly glides remaining cards into their new positions with zero abrupt layout jumps.".to_string(),
        "6. INTERACTIVE 3D OBJECT ROTATION, FLOATING PROPS & COMPONENT ANIMATIONS (CRUCIAL):\n   \
        - 3D Object Continuous Rotation & Idle Floating: Whenever the design includes a 3D object, 3D artifact, 3D emblem/crest, 3D crystal/gem/coin, 3D trophy/shield, 3D device centerpiece, or 3D decorative prop (reconstructed as an interactive 3D object from its reference image or layered as a 3D rig), animate it with a smooth continuous 3D rotation around its vertical axis (or gentle orbital spin) combined with a soft vertical floating and bobbing oscillation (gliding 6 pixels up and down over a 4.5-second ease-in-out loop).\n   \
        - Pointer-Tracking 3D Tilt & Hover Spin: Add interactive cursor-tracking 3D tilt and parallax rotation when the user moves their pointer across the 3D object or hero centerpiece, and trigger an accelerated spin, subtle scale boost, and intensified rim-light or emissive glow when hovered or clicked.".to_string(),
        "7. SCROLL-LINKED HEADER BLUR, ANIMATED METRICS & REDUCED-MOTION ACCESSIBILITY:\n   \
        - Scroll-Aware Frosted Glass Header & Spring Modals: Smoothly transition the sticky top header into a translucent frosted-glass surface over 200 milliseconds as soon as scroll offset exceeds 12 pixels. Animate modal dialogs and drawers with a 180-millisecond backdrop fade and a spring scale-and-slide entrance.\n   \
        - Animated KPI Counters & Progress Bars: Use spring-interpolated number counters for dashboard metrics and smooth spring fill animations for Game UI health, mana, XP, or progress bars.\n   \
        - 60/120fps Compositor Rule & Reduced-Motion Safeguard: Animate only hardware-accelerated transform, opacity, and filter properties—never animate raw layout height, width, or margins directly. Always check the user's operating system reduced-motion accessibility preference and automatically disable spatial translations, continuous 3D spins/bobbing loops, and stagger delays when reduced motion is requested.".to_string(),
    ];

    if let Some(it) = item {
        for note in &it.implementation_notes {
            let lower = note.to_ascii_lowercase();
            if lower.contains("animat")
                || lower.contains("transition")
                || lower.contains("motion")
                || lower.contains("framer")
                || lower.contains("rotate")
                || lower.contains("rotation")
                || lower.contains("3d")
                || lower.contains("effect")
            {
                lines.push(format!(
                    "- Item-Specific Motion & 3D Animation Note: {}",
                    clean_code_syntax_to_human_prose(note)
                ));
            }
        }
    } else if let Some(p) = plan {
        for it in &p.items {
            for note in &it.implementation_notes {
                let lower = note.to_ascii_lowercase();
                if lower.contains("animat")
                    || lower.contains("transition")
                    || lower.contains("motion")
                    || lower.contains("framer")
                    || lower.contains("rotate")
                    || lower.contains("rotation")
                    || lower.contains("3d")
                    || lower.contains("effect")
                {
                    lines.push(format!(
                        "- [{}] Motion & 3D Animation Note: {}",
                        it.name,
                        clean_code_syntax_to_human_prose(note)
                    ));
                }
            }
        }
    }

    lines.join("\n")
}

#[allow(clippy::too_many_arguments)]
fn build_chroma_and_asset_cropping_spec(
    brand: &BrandSpec,
    plan: Option<&PlannerPlan>,
    item: Option<&crate::models::PlanItem>,
    model_settings: Option<&RunModelSettings>,
    primary_composite_path: Option<&str>,
    extracted_transparent_assets: &[(String, String, String, i64, i64)], // (item_id, title, formatted_path, width, height)
) -> String {
    let preferred_dir = if brand.target_context.preferred_asset_dir.trim().is_empty() {
        "public/assets"
    } else {
        brand.target_context.preferred_asset_dir.trim()
    };

    let mut lines = vec![
        "1. WHAT MUST BE BUILT PURELY WITH CODE (SEPARATORS, BACKGROUNDS, GRADIENTS & UI FRAMES — NEVER USE FLAT IMAGES):".to_string(),
        format!(
            "   - Section Separators, Dividers & Ornate Rule Lines: Build all horizontal and vertical section separators, glowing energy lines, gradient fade dividers, angled or curved section transitions, and decorative corner brackets purely with code and scalable vector paths (using brand border token {} and primary/accent highlights {} / {}) so they stay razor-sharp across any screen width.",
            brand.colors.light.border,
            brand.colors.light.primary,
            brand.colors.light.accent
        ),
        format!(
            "   - Ambient Backgrounds, Lighting & Surface Depth: Build all page and section backgrounds ({} in light mode / {} in dark mode), radial ambient glows, mesh gradients, subtle noise or grain overlays, dot or grid patterns, particle/ember fields, and frosted glass panels ({} / {}) purely with code rather than using a static flat background image.",
            brand.colors.light.background,
            brand.colors.dark.background,
            brand.colors.light.surface,
            brand.colors.dark.surface
        ),
        "   - UI Cards, HUD Bars, Badges & Interactive Controls: Construct every card container, banner, HUD health/mana/XP bar, inventory slot frame, button, input, and badge 1:1 in code.".to_string(),
        "\n2. WHAT MUST BE CONVERTED TO VECTORIAL SVG GRAPHICS (LOGOS, ICONS, CRESTS & FLAT ART — QUALITY & SPEED MANDATE):".to_string(),
        format!(
            "   - Convert Logos, Wordmarks, UI Icons, Badges & Geometric Art to Vectorial SVG: Whenever an asset or icon sheet contains brand logos, wordmarks, UI icons (Style: {}), HUD emblems/crests, badges, or flat/geometric illustrations, convert, trace, or reconstruct them into clean, scalable vectorial SVG graphics using exact brand hex colors ({} and {}). Vectorial SVG graphics guarantee infinite Retina and 4K sharpness at under 2 kilobytes with instant rendering speed!",
            brand.layout_tokens.icon_style,
            brand.colors.light.primary,
            brand.colors.light.accent
        ),
        "\n3. WHAT MUST BE IMPLEMENTED AS AN INTERACTIVE 3D OBJECT FROM THE REFERENCE IMAGE (IMAGE-TO-3D / PROCEDURAL 3D + ROTATION & ANIMATION):".to_string(),
        "   - Make 3D Objects From the Reference Image Instead of a Flat Static Picture: Whenever the design features a 3D object, 3D artifact, 3D emblem/crest, 3D crystal/gem/coin, 3D trophy/shield, 3D device centerpiece, or 3D decorative prop (provided as an isolated transparent reference image below), do NOT simply paste it as a static flat 2D image when 3D or interactive rendering is appropriate! Instead, use the extracted reference image to build or reconstruct a real interactive 3D object (using procedural 3D geometry, materials, and lighting in Three.js, React Three Fiber, WebGL, Spline, or an Image-to-3D mesh in the target engine).\n   \
        - Required 3D Object Rotation & Animation: Animate the reconstructed 3D object (or high-depth 3D layered rig) so it continuously rotates smoothly on its vertical axis, floats gently up and down in idle state, tilts interactively in 3D space as the user moves their pointer, and performs a responsive spin or emissive glow pulse on hover and click.\n   \
        - Fallback / 2D Textured Cutout Optimization: For painterly 2D illustrations or tactile clay cutouts layered as 2D images, tightly crop transparent dead space around the non-transparent silhouette, serve as compressed next-gen alpha WebP or AVIF with explicit width and height dimensions (zero layout shift), prioritize above-the-fold hero visuals, and lazy-load below-the-fold assets.".to_string(),
    ];

    if !extracted_transparent_assets.is_empty() {
        lines.push("\n4. EXTRACTED TRANSPARENT FOREGROUND & 3D OBJECT REFERENCE ASSETS FOR THIS DESIGN (MANDATORY IMPLEMENTATION SPEC):".to_string());
        if let Some(comp_p) = primary_composite_path {
            lines.push(format!(
                "  * Primary Composite Design Reference Image: {} (shows how the complete layout looks with its coded background, typography, buttons, and foreground/3D assets in place).",
                comp_p
            ));
        }
        for (idx, (aid, atitle, apath, aw, ah)) in extracted_transparent_assets.iter().enumerate() {
            let lower_t = atitle.to_ascii_lowercase();
            let asset_type_guidance = if lower_t.contains("icon")
                || lower_t.contains("logo")
                || lower_t.contains("wordmark")
                || lower_t.contains("badge")
                || lower_t.contains("crest")
            {
                "Convert/reconstruct this asset into a crisp, scalable vectorial SVG graphic (or copy the original into your assets folder) and place it in the exact position shown in the composite reference."
            } else if lower_t.contains("3d")
                || lower_t.contains("crystal")
                || lower_t.contains("gem")
                || lower_t.contains("coin")
                || lower_t.contains("orb")
                || lower_t.contains("relic")
                || lower_t.contains("artifact")
                || lower_t.contains("shield")
                || lower_t.contains("trophy")
                || lower_t.contains("megaphone")
                || lower_t.contains("plant")
            {
                "Use this isolated transparent reference image to construct an interactive 3D object (via Three.js / WebGL / Image-to-3D mesh, or a tightly-cropped alpha WebP/AVIF 3D rig stored in your assets folder), place it in the exact position shown in the composite reference, and animate it with smooth continuous 3D rotation, gentle floating idle motion, and interactive pointer-tracking hover tilt!"
            } else {
                "Copy this native transparent asset into your project's assets directory, optimize it as vectorial SVG or tightly-cropped alpha WebP/AVIF, and layer it over your coded background in the exact position shown in the composite reference."
            };
            lines.push(format!(
                "  {}. Extracted Transparent Asset '{}' ({}):\n     - Local Image Path: {}\n     - Dimensions: {}x{}px (100% Native Transparent Background PNG)\n     - Target Asset Folder: {}\n     - How to Implement & Animate: {}",
                idx + 1,
                atitle,
                aid,
                apath,
                aw,
                ah,
                preferred_dir,
                asset_type_guidance
            ));
        }
    }

    if let Some(it) = item {
        for ai in &it.asset_instructions {
            lines.push(format!(
                "- Item Asset, Vector SVG & 3D Object Instruction: {}",
                clean_code_syntax_to_human_prose(ai)
            ));
        }
    } else if let Some(p) = plan {
        for ar in &p.shared_design_spec.asset_rules {
            lines.push(format!(
                "- Shared Asset, Vector SVG & 3D Object Rule: {}",
                clean_code_syntax_to_human_prose(ar)
            ));
        }
        for it in &p.items {
            for ai in &it.asset_instructions {
                lines.push(format!(
                    "- [{}] Asset, Vector SVG & 3D Object Rule: {}",
                    it.name,
                    clean_code_syntax_to_human_prose(ai)
                ));
            }
        }
    }

    if let Some(ms) = model_settings {
        if let Some(ref ao) = ms.asset_options {
            lines.push(format!(
                "- Asset Mode Configuration: intended usage = {}, theme context = {}, native transparent background = {}, seamless tiling = {}",
                ao.intended_usage, ao.theme_context, ao.transparent_background, ao.seamless_tiling
            ));
        }
    }

    lines.join("\n")
}

pub async fn build_single_output_handoff(
    pool: &SqlitePool,
    config: &AppConfig,
    output_id: &str,
    include_extraction_override: Option<bool>,
) -> AppResult<HandoffResponse> {
    let out_row = sqlx::query(
        "SELECT id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, mime_type, width, height, has_alpha, item_spec_json, tab_key FROM outputs WHERE id = ?"
    )
    .bind(output_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Output '{}' not found", output_id)))?;

    let status: String = out_row.get("status");
    let orig_rel: Option<String> = out_row.get("original_rel_path");
    let orig_rel = orig_rel.ok_or_else(|| {
        AppError::Conflict(format!(
            "Output '{}' does not have a saved original image yet (status: {}).",
            output_id, status
        ))
    })?;

    let run_id: String = out_row.get("run_id");
    let project_id: String = out_row.get("project_id");
    let tab_key: String = out_row
        .try_get::<String, _>("tab_key")
        .unwrap_or_else(|_| "main".to_string());
    let title: String = out_row.get("title");
    let kind: String = out_row.get("kind");
    let role: String = out_row.get("role");
    let page_route: Option<String> = out_row.get("page_route");
    let width: i64 = out_row.get::<Option<i64>, _>("width").unwrap_or(0);
    let height: i64 = out_row.get::<Option<i64>, _>("height").unwrap_or(0);
    let has_alpha: i64 = out_row.get("has_alpha");
    let item_spec_str: String = out_row.get("item_spec_json");
    let plan_item: Option<crate::models::PlanItem> = serde_json::from_str(&item_spec_str).ok();

    let run_row = sqlx::query(
        "SELECT mode, scope_preset, user_prompt, brand_version, brand_snapshot_json, selected_references_json, model_settings_json, plan_json FROM runs WHERE id = ?"
    )
    .bind(&run_id)
    .fetch_one(pool)
    .await?;

    let proj_row = sqlx::query(
        "SELECT name, description, purpose, industry, target_audience, audience_needs, desired_user_action FROM projects WHERE id = ?",
    )
    .bind(&project_id)
    .fetch_one(pool)
    .await?;

    let settings_row =
        sqlx::query("SELECT include_extraction_instructions FROM settings WHERE id = 1")
            .fetch_one(pool)
            .await?;
    let default_extract: i64 = settings_row.get("include_extraction_instructions");
    let include_extraction = include_extraction_override.unwrap_or(default_extract != 0);

    let project_name: String = proj_row.get("name");
    let project_description: String = proj_row.get("description");
    let project_purpose: String = proj_row.get("purpose");
    let project_industry: String = proj_row.get("industry");
    let target_audience: String = proj_row.get("target_audience");
    let audience_needs: String = proj_row.get("audience_needs");
    let desired_user_action: String = proj_row.get("desired_user_action");

    let mode: String = run_row.get("mode");
    let scope_preset: String = run_row.get("scope_preset");
    let user_prompt: String = run_row.get("user_prompt");
    let brand_snapshot_str: String = run_row.get("brand_snapshot_json");
    let brand: BrandSpec = serde_json::from_str(&brand_snapshot_str).unwrap_or_default();
    let selected_refs_str: String = run_row.get("selected_references_json");
    let selected_refs: Vec<SelectedReference> =
        serde_json::from_str(&selected_refs_str).unwrap_or_default();
    let model_settings_str: String = run_row.get("model_settings_json");
    let model_settings: Option<RunModelSettings> = serde_json::from_str(&model_settings_str).ok();
    let plan_str: Option<String> = run_row.get("plan_json");
    let plan: Option<PlannerPlan> = plan_str.and_then(|s| serde_json::from_str(&s).ok());

    let abs_output_path = config.resolve_rel_path(&orig_rel)?;
    let formatted_output_path = format_host_path(&abs_output_path);

    // Resolve original source assets (logos, uploaded fonts, included assets)
    let source_assets_info =
        resolve_source_assets_list(pool, config, &project_id, &brand, &selected_refs, false)
            .await?;

    // Check if there are sibling completed outputs in this tab or run (e.g. extracted transparent foreground assets or sibling Desktop/Mobile views in the same tab)
    let sibling_rows = sqlx::query(
        "SELECT id, plan_item_id, kind, title, role, original_rel_path, width, height, has_alpha, transparent_background, extracted_from_output_id FROM outputs WHERE project_id = ? AND (tab_key = ? OR run_id = ?) AND id != ? AND status = 'completed' ORDER BY created_at ASC, variant_index ASC"
    )
    .bind(&project_id)
    .bind(&tab_key)
    .bind(&run_id)
    .bind(output_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    let mut extracted_transparent_assets: Vec<(String, String, String, i64, i64)> = Vec::new();
    for srow in &sibling_rows {
        let skind: String = srow.get("kind");
        let stitle: String = srow.get("title");
        let sitem_id: String = srow.get("plan_item_id");
        let srel: Option<String> = srow.get("original_rel_path");
        let sw: i64 = srow.get::<Option<i64>, _>("width").unwrap_or(0);
        let sh: i64 = srow.get::<Option<i64>, _>("height").unwrap_or(0);
        let shas_alpha: i64 = srow.try_get::<i64, _>("has_alpha").unwrap_or(0);
        let strans: i64 = srow
            .try_get::<i64, _>("transparent_background")
            .unwrap_or(0);
        let sext_from: Option<String> = srow.try_get("extracted_from_output_id").ok().flatten();

        let is_linked_transparent_asset = sext_from.as_deref() == Some(output_id)
            || (skind == "asset" && (shas_alpha != 0 || strans != 0));
        if is_linked_transparent_asset {
            if let Some(rel_p) = srel {
                if let Ok(abs_p) = config.resolve_rel_path(&rel_p) {
                    extracted_transparent_assets.push((
                        sitem_id,
                        stitle,
                        format_host_path(&abs_p),
                        sw,
                        sh,
                    ));
                }
            }
        }
    }

    let mut design_references_lines = vec![format!(
        "1. [PRIMARY GENERATED DESIGN REFERENCE — {kind}/{role}]\n   Title: {title}\n   Path: {formatted_output_path}\n   Dimensions: {width}x{height}px (Alpha transparency: {alpha})\n   Treatment: Generated visual design reference (implement as real semantic UI/code; layer any extracted transparent assets listed below over your coded background).",
        kind = kind,
        role = role,
        title = title,
        formatted_output_path = formatted_output_path,
        width = width,
        height = height,
        alpha = if has_alpha != 0 { "yes" } else { "no" }
    )];

    if !extracted_transparent_assets.is_empty() && mode != "full_page" {
        design_references_lines.push("\nEXTRACTED TRANSPARENT FOREGROUND ASSETS FOR THIS COMPONENT (use these native transparent PNG files to layer the foreground artwork/icons over the coded background):".to_string());
        for (idx, (aid, atitle, apath, aw, ah)) in extracted_transparent_assets.iter().enumerate() {
            design_references_lines.push(format!(
                "   {}. [EXTRACTED TRANSPARENT ASSET — {}]\n      Item ID: {}\n      Path: {}\n      Dimensions: {}x{}px (Alpha transparency: yes — Native Transparent PNG)\n      Implementation Role: Copy into your project's assets folder and layer over the coded component background to match the primary composite reference above.",
                idx + 2,
                atitle,
                aid,
                apath,
                aw,
                ah
            ));
        }
    } else if mode == "full_page" && !sibling_rows.is_empty() {
        design_references_lines.push("\nCOORDINATED SIBLING OUTPUTS IN THIS FULL-PAGE SET (use these local image paths for reusable components & embedded transparent assets):".to_string());
        for (idx, srow) in sibling_rows.iter().enumerate() {
            let skind: String = srow.get("kind");
            let srole: String = srow.get("role");
            let stitle: String = srow.get("title");
            let sitem_id: String = srow.get("plan_item_id");
            let srel: Option<String> = srow.get("original_rel_path");
            let sw: i64 = srow.get::<Option<i64>, _>("width").unwrap_or(0);
            let sh: i64 = srow.get::<Option<i64>, _>("height").unwrap_or(0);
            let shas_alpha: i64 = srow.try_get::<i64, _>("has_alpha").unwrap_or(0);
            if let Some(rel_p) = srel {
                if let Ok(abs_p) = config.resolve_rel_path(&rel_p) {
                    design_references_lines.push(format!(
                        "   {}. [SIBLING {} ({}) — {}]\n      Item ID: {}\n      Path: {}\n      Dimensions: {}x{}px (Alpha transparency: {})",
                        idx + 2,
                        skind.to_uppercase(),
                        srole,
                        stitle,
                        sitem_id,
                        format_host_path(&abs_p),
                        sw,
                        sh,
                        if shas_alpha != 0 { "yes" } else { "no" }
                    ));
                }
            }
        }
    }

    let design_references_section = design_references_lines.join("\n");

    let archetype = crate::models::detect_product_archetype(
        &project_name,
        &project_description,
        &project_purpose,
        &project_industry,
        &mode,
        &scope_preset,
        &user_prompt,
    );
    let archetype_guidance = crate::models::build_product_archetype_guidance(archetype);
    let existing_ui_replacement_spec = build_existing_ui_replacement_spec(
        pool,
        config,
        &project_id,
        &selected_refs,
        &title,
        &formatted_output_path,
        false,
    )
    .await?;

    let handoff_mission_header = match (mode.as_str(), kind.as_str()) {
        (_, "component") | ("component", _) => format!(
            "Implement and integrate the reusable UI Component '{}' (Scope: {}) inside the '{}' application codebase using the reference image path below.",
            title, scope_preset, project_name
        ),
        (_, "asset") | ("asset", _) => format!(
            "Integrate and optimize the standalone visual asset '{}' into the '{}' application codebase using the file path below.",
            title, project_name
        ),
        _ => format!(
            "Implement the Full-Page View / Section '{}' (Route: {}) and its coordinated UI components inside the '{}' application codebase using the reference image paths below.",
            title,
            page_route.as_deref().unwrap_or("/"),
            project_name
        ),
    };

    let has_current_screenshot_ref = selected_refs.iter().any(|r| {
        r.role == crate::models::AttachmentRole::CurrentScreenshot
            || r.role == crate::models::AttachmentRole::SelectedPreviousOutput
    });

    let screenshot_workflow_line = if has_current_screenshot_ref {
        "- Screenshot Polish, Enhance & Replace Mode: This design was created from an existing UI screenshot or prior component plus the user's prompt. Locate the old hero, header, navigation bar, or component in the codebase and replace its visual structure with this newly generated design while keeping all existing functional buttons, controls, handlers, routes, and callbacks — strictly styled in the Project Brand Kit and Chosen Web Design Style."
    } else {
        "- New Component From Scratch Mode: This design was generated fresh from the user's prompt (no prior UI screenshot attached). Implement all buttons, controls, texts, icons, properties, and interactive states shown in the reference image from scratch following the Project Brand Kit and Chosen Web Design Style."
    };

    let base_component_architecture_spec = match (mode.as_str(), kind.as_str()) {
        (_, "component") | ("component", _) => format!(
            "- Component Role: Reusable, self-contained UI component ({title}, scope {scope_preset}) designed to slot cleanly into the {project_name} application.\n\
{screenshot_workflow_line}\n\
- Properties & State Contract: Expose clean, typed component properties for all dynamic data, labels, action callbacks, and active, loading, or disabled states shown in {formatted_output_path}.\n\
- Encapsulation & Layout: Do not hardcode full-viewport width or height wrappers around an isolated component; let the parent container control outer placement while preserving internal padding, grid or flex alignment, corner radii, and elevation shadows from the reference image.\n\
- Interactive Fidelity: Implement hover, keyboard focus ring, active pressed, and keyboard navigation states matching the Chosen Web Design Style rules above."
        ),
        (_, "asset") | ("asset", _) => format!(
            "- Asset Role: Standalone production graphic ({title}) at {formatted_output_path}.\n\
- Integration: Copy and optimize into {} and reference via a responsive image element, vector SVG, or interactive 3D object scene with descriptive accessibility text and aspect-ratio preservation.",
            if brand.target_context.preferred_asset_dir.trim().is_empty() {
                "public/assets"
            } else {
                brand.target_context.preferred_asset_dir.trim()
            }
        ),
        _ => {
            let route_str = page_route.as_deref().unwrap_or("/");
            let what_is_shown = if let Some(ref pi) = plan_item {
                format!(
                    "- What Is Shown in This Page Image ({title}, Route {route_str}) & How It Works:\n  * Visual Direction & Layout: {}\n  * Detailed Page Content & Sections (Top-to-Bottom): {}\n  * Exact Copy & Labels Displayed: {}",
                    clean_code_syntax_to_human_prose(&pi.direction),
                    clean_code_syntax_to_human_prose(&pi.image_prompt),
                    if pi.exact_copy.is_empty() {
                        "Follow visible headings, metrics, and calls to action in the reference image.".to_string()
                    } else {
                        clean_code_syntax_to_human_prose(&pi.exact_copy.join(" | "))
                    }
                )
            } else {
                format!(
                    "- What Is Shown in This Page Image ({title}, Route {route_str}): Full top-to-bottom UI layout for route {route_str}."
                )
            };
            format!(
                "- Full-Page & Multi-Route Composition: Implement route {route_str} matching {formatted_output_path}.\n\
{screenshot_workflow_line}\n\
- Shared Persistent App Shell (Navigation Bar & Footer via Image-to-Image Consistency): The sticky top navigation bar (brand logo on the left, route navigation links in the center with a smooth shared-layout sliding active indicator, and primary call to action on the right) and global footer MUST be implemented once as shared layout components so they remain 100% identical across all pages and routes of this application, matching the Visual Anchor (item_1).\n\
{what_is_shown}\n\
- Modular Decomposition: Extract repeating cards, HUD widgets, KPI panels, filter bars, navigation bars, tables, and modal overlays into clean reusable components rather than one monolithic file, wiring up any sibling page, component, and transparent asset references listed above."
            )
        }
    };

    let component_architecture_spec = format!(
        "{}\n\n{}\n\nCOMPONENT ARCHITECTURE & INTEGRATION RULES:\n{}",
        clean_code_syntax_to_human_prose(archetype_guidance),
        existing_ui_replacement_spec,
        base_component_architecture_spec
    );

    let (reqs_copy, behaviors_responsive, asset_rules) =
        format_item_and_plan_details(&plan, plan_item.as_ref(), model_settings.as_ref());

    let audience_combined = if audience_needs.trim().is_empty() {
        if target_audience.trim().is_empty() {
            "General web users".to_string()
        } else {
            target_audience.clone()
        }
    } else {
        format!("{} (Needs: {})", target_audience, audience_needs)
    };

    let handoff_text = HANDOFF_SINGLE_TEMPLATE_V1
        .replace("{{handoff_mission_header}}", &handoff_mission_header)
        .replace("{{primary_image_path}}", &formatted_output_path)
        .replace("{{project_name}}", &project_name)
        .replace(
            "{{project_description}}",
            if project_description.trim().is_empty() {
                "Interactive web application"
            } else {
                &project_description
            },
        )
        .replace(
            "{{project_purpose}}",
            if project_purpose.trim().is_empty() {
                "Web application interface"
            } else {
                &project_purpose
            },
        )
        .replace(
            "{{project_industry}}",
            if project_industry.trim().is_empty() {
                "Software & Digital Product"
            } else {
                &project_industry
            },
        )
        .replace("{{target_audience}}", &audience_combined)
        .replace(
            "{{desired_user_action}}",
            if desired_user_action.trim().is_empty() {
                "Engage with primary workflow"
            } else {
                &desired_user_action
            },
        )
        .replace(
            "{{scope}}",
            &format!("{} ({}) — {}", mode, scope_preset, title),
        )
        .replace(
            "{{target_stack_and_repository_or_inspect_existing_project}}",
            &format_target_context(&brand),
        )
        .replace(
            "{{chosen_design_style_block}}",
            &format_design_style_block(&brand),
        )
        .replace(
            "{{ordered_reference_list_with_exact_absolute_paths_roles_and_dimensions}}",
            &design_references_section,
        )
        .replace(
            "{{component_architecture_and_integration_spec}}",
            &component_architecture_spec,
        )
        .replace(
            "{{placeholder_and_one_to_one_card_mandate}}",
            &build_placeholder_and_card_mandate(
                &brand,
                plan.as_ref(),
                plan_item.as_ref(),
                archetype,
            ),
        )
        .replace(
            "{{sticky_and_positioning_rules}}",
            &build_sticky_positioning_spec(
                &mode,
                &kind,
                &scope_preset,
                &title,
                &brand,
                plan.as_ref(),
                plan_item.as_ref(),
            ),
        )
        .replace(
            "{{hover_click_states_and_color_effects}}",
            &build_interactive_states_and_colors_spec(&brand, plan.as_ref(), plan_item.as_ref()),
        )
        .replace(
            "{{animations_and_motion_spec}}",
            &build_animations_and_motion_spec(&brand, plan.as_ref(), plan_item.as_ref()),
        )
        .replace(
            "{{chroma_and_asset_cropping_rules}}",
            &build_chroma_and_asset_cropping_spec(
                &brand,
                plan.as_ref(),
                plan_item.as_ref(),
                model_settings.as_ref(),
                Some(&formatted_output_path),
                &extracted_transparent_assets,
            ),
        )
        .replace(
            "{{brand_tokens_and_typography}}",
            &format_brand_tokens(&brand),
        )
        .replace(
            "{{original_logo_font_and_required_asset_paths}}",
            &source_assets_info,
        )
        .replace("{{asset_usage_rules}}", &asset_rules)
        .replace("{{original_user_request}}", &user_prompt)
        .replace("{{requirements_and_exact_copy}}", &reqs_copy)
        .replace(
            "{{interactions_states_and_responsive_rules}}",
            &behaviors_responsive,
        )
        .replace(
            "{{extraction_steps}}",
            &build_extraction_steps(include_extraction),
        );

    let export_id = Uuid::new_v4().to_string();
    let export_dir_rel = format!("projects/{}/runs/{}/exports", project_id, run_id);
    let manifest_rel = format!("{}/manifest-{}.json", export_dir_rel, output_id);
    let impl_rel = format!("{}/implementation-{}.md", export_dir_rel, output_id);

    let manifest_json = serde_json::json!({
        "manifest_version": 1,
        "export_id": export_id,
        "export_kind": "single_handoff",
        "project_id": project_id,
        "project_name": project_name,
        "run_id": run_id,
        "output_id": output_id,
        "title": title,
        "mode": mode,
        "brand_version": run_row.get::<i64, _>("brand_version"),
        "generated_reference_path": abs_output_path.to_string_lossy(),
        "dimensions": { "width": width, "height": height },
        "include_extraction_instructions": include_extraction,
        "created_at": chrono::Utc::now().to_rfc3339()
    });

    let abs_manifest = config.resolve_rel_path(&manifest_rel)?;
    let abs_impl = config.resolve_rel_path(&impl_rel)?;
    atomic_write_file(
        &abs_manifest,
        serde_json::to_string_pretty(&manifest_json)
            .unwrap_or_default()
            .as_bytes(),
    )?;
    atomic_write_file(&abs_impl, handoff_text.as_bytes())?;

    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO exports (id, project_id, run_id, output_id, export_kind, manifest_version, manifest_rel_path, implementation_rel_path, bundle_rel_path, include_extraction, handoff_text, created_at) VALUES (?, ?, ?, ?, 'single_handoff', 1, ?, ?, NULL, ?, ?, ?)"
    )
    .bind(&export_id)
    .bind(&project_id)
    .bind(&run_id)
    .bind(output_id)
    .bind(&manifest_rel)
    .bind(&impl_rel)
    .bind(if include_extraction { 1 } else { 0 })
    .bind(&handoff_text)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(HandoffResponse {
        export_id,
        export_kind: "single_handoff".to_string(),
        clipboard_text: handoff_text,
        manifest_path: abs_manifest.to_string_lossy().to_string(),
        implementation_path: abs_impl.to_string_lossy().to_string(),
        bundle_path: None,
        include_extraction_instructions: include_extraction,
        is_complete_set: true,
        missing_items: vec![],
    })
}

pub async fn build_set_handoff(
    pool: &SqlitePool,
    config: &AppConfig,
    run_id: &str,
    include_extraction_override: Option<bool>,
    create_zip_bundle: bool,
    explicit_output_ids: Option<&[String]>,
) -> AppResult<HandoffResponse> {
    let run_row = sqlx::query(
        "SELECT project_id, mode, scope_preset, user_prompt, brand_version, brand_snapshot_json, selected_references_json, model_settings_json, plan_json, status FROM runs WHERE id = ?"
    )
    .bind(run_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Run '{}' not found", run_id)))?;

    let project_id: String = run_row.get("project_id");
    let mode: String = run_row.get("mode");
    let scope_preset: String = run_row.get("scope_preset");
    let user_prompt: String = run_row.get("user_prompt");
    let brand_version: i64 = run_row.get("brand_version");
    let brand_snapshot_str: String = run_row.get("brand_snapshot_json");
    let brand: BrandSpec = serde_json::from_str(&brand_snapshot_str).unwrap_or_default();
    let selected_refs_str: String = run_row.get("selected_references_json");
    let selected_refs: Vec<SelectedReference> =
        serde_json::from_str(&selected_refs_str).unwrap_or_default();
    let model_settings_str: String = run_row.get("model_settings_json");
    let model_settings: Option<RunModelSettings> = serde_json::from_str(&model_settings_str).ok();
    let plan_str: Option<String> = run_row.get("plan_json");
    let plan: Option<PlannerPlan> = plan_str.and_then(|s| serde_json::from_str(&s).ok());

    let proj_row = sqlx::query(
        "SELECT name, description, purpose, industry, target_audience, audience_needs, desired_user_action FROM projects WHERE id = ?",
    )
    .bind(&project_id)
    .fetch_one(pool)
    .await?;

    let settings_row =
        sqlx::query("SELECT include_extraction_instructions FROM settings WHERE id = 1")
            .fetch_one(pool)
            .await?;
    let default_extract: i64 = settings_row.get("include_extraction_instructions");
    let include_extraction = include_extraction_override.unwrap_or(default_extract != 0);

    let project_name: String = proj_row.get("name");
    let project_description: String = proj_row.get("description");
    let project_purpose: String = proj_row.get("purpose");
    let project_industry: String = proj_row.get("industry");
    let target_audience: String = proj_row.get("target_audience");
    let audience_needs: String = proj_row.get("audience_needs");
    let desired_user_action: String = proj_row.get("desired_user_action");
    let audience_combined = if audience_needs.trim().is_empty() {
        target_audience.clone()
    } else {
        format!("{} (Needs: {})", target_audience, audience_needs)
    };

    let out_rows = if let Some(ids) = explicit_output_ids.filter(|s| !s.is_empty()) {
        let mut rows = Vec::new();
        for oid in ids {
            if let Some(r) = sqlx::query(
                "SELECT id, run_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, status, original_rel_path, width, height, has_alpha, depends_on_json, item_spec_json FROM outputs WHERE id = ? AND project_id = ?"
            )
            .bind(oid)
            .bind(&project_id)
            .fetch_optional(pool)
            .await?
            {
                rows.push(r);
            }
        }
        rows
    } else {
        sqlx::query(
            "SELECT id, run_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, status, original_rel_path, width, height, has_alpha, depends_on_json, item_spec_json FROM outputs WHERE run_id = ? ORDER BY variant_index ASC"
        )
        .bind(run_id)
        .fetch_all(pool)
        .await?
    };

    if out_rows.is_empty() {
        return Err(AppError::Conflict(
            "No outputs exist for this run or tab yet.".into(),
        ));
    }

    let mut completed_items = Vec::new();
    let mut missing_items = Vec::new();
    let mut seen_item_ids = std::collections::HashSet::new();
    let mut active_plan_items: Vec<crate::models::PlanItem> = Vec::new();
    let mut distinct_run_ids: Vec<String> = Vec::new();

    for r in &out_rows {
        let out_id: String = r.get("id");
        let row_run_id: String = r
            .try_get::<String, _>("run_id")
            .unwrap_or_else(|_| run_id.to_string());
        if !distinct_run_ids.contains(&row_run_id) {
            distinct_run_ids.push(row_run_id);
        }
        let raw_plan_item_id: String = r.get("plan_item_id");
        let plan_item_id = if seen_item_ids.contains(&raw_plan_item_id) {
            let short_id: String = out_id.chars().take(6).collect();
            format!("{}_{}", raw_plan_item_id, short_id)
        } else {
            raw_plan_item_id.clone()
        };
        seen_item_ids.insert(plan_item_id.clone());
        let title: String = r.get("title");
        let kind: String = r.get("kind");
        let role: String = r.get("role");
        let page_route: Option<String> = r.get("page_route");
        let status: String = r.get("status");
        let orig_rel: Option<String> = r.get("original_rel_path");
        let width: i64 = r.get::<Option<i64>, _>("width").unwrap_or(0);
        let height: i64 = r.get::<Option<i64>, _>("height").unwrap_or(0);
        let deps_str: String = r.get("depends_on_json");
        let deps: Vec<String> = serde_json::from_str(&deps_str).unwrap_or_default();
        let item_spec_str: String = r.try_get("item_spec_json").unwrap_or_default();

        if status == "completed" {
            if let Some(rel) = orig_rel {
                if let Ok(mut parsed_item) =
                    serde_json::from_str::<crate::models::PlanItem>(&item_spec_str)
                {
                    parsed_item.id = plan_item_id.clone();
                    parsed_item.name = title.clone();
                    active_plan_items.push(parsed_item);
                }
                completed_items.push((
                    out_id,
                    plan_item_id,
                    title,
                    kind,
                    role,
                    page_route,
                    rel,
                    width,
                    height,
                    deps,
                ));
                continue;
            }
        }
        missing_items.push(format!("{} ({}) — status: {}", title, plan_item_id, status));
    }

    // Merge PlannerPlan across all runs represented in `out_rows` and keep only active (non-deleted) items
    let mut merged_plan = plan.clone();
    for rid in &distinct_run_ids {
        if rid == run_id {
            continue;
        }
        if let Ok(Some(rr)) = sqlx::query("SELECT plan_json FROM runs WHERE id = ?")
            .bind(rid)
            .fetch_optional(pool)
            .await
        {
            if let Some(other_plan) = rr
                .get::<Option<String>, _>("plan_json")
                .and_then(|s| serde_json::from_str::<PlannerPlan>(&s).ok())
            {
                if let Some(ref mut mp) = merged_plan {
                    for pc in other_plan.page_composition {
                        if !mp
                            .page_composition
                            .iter()
                            .any(|x| x.route == pc.route && x.page_name == pc.page_name)
                        {
                            mp.page_composition.push(pc);
                        }
                    }
                    for sc in other_plan.shared_components {
                        if !mp.shared_components.iter().any(|x| x.id == sc.id) {
                            mp.shared_components.push(sc);
                        }
                    }
                } else {
                    merged_plan = Some(other_plan);
                }
            }
        }
    }
    if let Some(ref mut mp) = merged_plan {
        if !active_plan_items.is_empty() {
            mp.items = active_plan_items;
        }
    }
    let plan = merged_plan;

    if completed_items.is_empty() {
        return Err(AppError::Conflict(
            "Cannot build set handoff because zero outputs have completed.".into(),
        ));
    }

    let is_complete_set = missing_items.is_empty();
    let export_dir_rel = format!("projects/{}/runs/{}/exports", project_id, run_id);
    let manifest_rel = format!("{}/manifest.json", export_dir_rel);
    let impl_rel = format!("{}/implementation.md", export_dir_rel);
    let abs_manifest = config.resolve_rel_path(&manifest_rel)?;
    let abs_impl = config.resolve_rel_path(&impl_rel)?;

    // Build local absolute-path handoff text for clipboard
    let local_handoff_text = render_set_handoff_markdown(
        pool,
        config,
        &project_id,
        &project_name,
        &project_description,
        &project_purpose,
        &project_industry,
        &audience_combined,
        &desired_user_action,
        &mode,
        &scope_preset,
        &user_prompt,
        &brand,
        &selected_refs,
        plan.as_ref(),
        model_settings.as_ref(),
        &completed_items,
        &missing_items,
        is_complete_set,
        include_extraction,
        &format_host_path(&abs_manifest),
        false,
    )
    .await?;

    // Build manifest JSON (local absolute paths)
    let manifest_outputs: Vec<serde_json::Value> = completed_items
        .iter()
        .map(
            |(out_id, item_id, title, kind, role, route, rel, w, h, deps)| {
                let abs = config
                    .resolve_rel_path(rel)
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| rel.clone());
                serde_json::json!({
                    "output_id": out_id,
                    "plan_item_id": item_id,
                    "title": title,
                    "kind": kind,
                    "role": role,
                    "page_route": route,
                    "path": abs,
                    "relative_bundle_path": format!("outputs/{}/{}", item_id, Path::new(rel).file_name().and_then(|s| s.to_str()).unwrap_or("original.png")),
                    "dimensions": { "width": w, "height": h },
                    "depends_on": deps
                })
            },
        )
        .collect();

    let manifest_json = serde_json::json!({
        "manifest_version": 1,
        "project_id": project_id,
        "project_name": project_name,
        "run_id": run_id,
        "mode": mode,
        "brand_version": brand_version,
        "status": if is_complete_set { "complete" } else { "partial_incomplete" },
        "missing_items": missing_items,
        "outputs": manifest_outputs,
        "created_at": chrono::Utc::now().to_rfc3339()
    });

    atomic_write_file(
        &abs_manifest,
        serde_json::to_string_pretty(&manifest_json)
            .unwrap_or_default()
            .as_bytes(),
    )?;
    atomic_write_file(&abs_impl, local_handoff_text.as_bytes())?;

    let mut bundle_rel_opt = None;
    let mut bundle_abs_opt = None;

    if create_zip_bundle {
        let bundle_rel = format!("{}/portable-bundle.zip", export_dir_rel);
        let abs_bundle = config.resolve_rel_path(&bundle_rel)?;

        // Build portable handoff markdown using bundle-relative paths
        let portable_handoff_text = render_set_handoff_markdown(
            pool,
            config,
            &project_id,
            &project_name,
            &project_description,
            &project_purpose,
            &project_industry,
            &audience_combined,
            &desired_user_action,
            &mode,
            &scope_preset,
            &user_prompt,
            &brand,
            &selected_refs,
            plan.as_ref(),
            model_settings.as_ref(),
            &completed_items,
            &missing_items,
            is_complete_set,
            include_extraction,
            "manifest.json",
            true,
        )
        .await?;

        let portable_outputs: Vec<serde_json::Value> = completed_items
            .iter()
            .map(
                |(out_id, item_id, title, kind, role, route, rel, w, h, deps)| {
                    let fname = Path::new(rel)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("original.png");
                    serde_json::json!({
                        "output_id": out_id,
                        "plan_item_id": item_id,
                        "title": title,
                        "kind": kind,
                        "role": role,
                        "page_route": route,
                        "path": format!("outputs/{}/{}", item_id, fname),
                        "dimensions": { "width": w, "height": h },
                        "depends_on": deps
                    })
                },
            )
            .collect();

        let portable_manifest = serde_json::json!({
            "manifest_version": 1,
            "portable_bundle": true,
            "project_id": project_id,
            "project_name": project_name,
            "run_id": run_id,
            "mode": mode,
            "brand_version": brand_version,
            "status": if is_complete_set { "complete" } else { "partial_incomplete" },
            "missing_items": missing_items,
            "outputs": portable_outputs,
            "created_at": chrono::Utc::now().to_rfc3339()
        });

        write_portable_zip_bundle(
            pool,
            config,
            &abs_bundle,
            &project_id,
            &brand,
            &selected_refs,
            &completed_items,
            &portable_handoff_text,
            &portable_manifest,
        )
        .await?;

        bundle_rel_opt = Some(bundle_rel);
        bundle_abs_opt = Some(abs_bundle.to_string_lossy().to_string());
    }

    let export_id = Uuid::new_v4().to_string();
    let export_kind = if create_zip_bundle {
        "portable_bundle"
    } else {
        "set_handoff"
    };
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO exports (id, project_id, run_id, output_id, export_kind, manifest_version, manifest_rel_path, implementation_rel_path, bundle_rel_path, include_extraction, handoff_text, created_at) VALUES (?, ?, ?, NULL, ?, 1, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&export_id)
    .bind(&project_id)
    .bind(run_id)
    .bind(export_kind)
    .bind(&manifest_rel)
    .bind(&impl_rel)
    .bind(&bundle_rel_opt)
    .bind(if include_extraction { 1 } else { 0 })
    .bind(&local_handoff_text)
    .bind(&now)
    .execute(pool)
    .await?;

    Ok(HandoffResponse {
        export_id,
        export_kind: export_kind.to_string(),
        clipboard_text: local_handoff_text,
        manifest_path: abs_manifest.to_string_lossy().to_string(),
        implementation_path: abs_impl.to_string_lossy().to_string(),
        bundle_path: bundle_abs_opt,
        include_extraction_instructions: include_extraction,
        is_complete_set,
        missing_items,
    })
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
async fn render_set_handoff_markdown(
    pool: &SqlitePool,
    config: &AppConfig,
    project_id: &str,
    project_name: &str,
    project_description: &str,
    project_purpose: &str,
    project_industry: &str,
    target_audience: &str,
    desired_user_action: &str,
    mode: &str,
    scope_preset: &str,
    user_prompt: &str,
    brand: &BrandSpec,
    selected_refs: &[SelectedReference],
    plan: Option<&PlannerPlan>,
    model_settings: Option<&RunModelSettings>,
    completed_items: &[(
        String,
        String,
        String,
        String,
        String,
        Option<String>,
        String,
        i64,
        i64,
        Vec<String>,
    )],
    missing_items: &[String],
    is_complete_set: bool,
    include_extraction: bool,
    manifest_path_str: &str,
    portable_relative: bool,
) -> AppResult<String> {
    let set_status = if is_complete_set {
        format!(
            "COMPLETE SET ({} of {} outputs ready)",
            completed_items.len(),
            completed_items.len()
        )
    } else {
        format!(
            "INCOMPLETE / PARTIAL SET ({} available, {} missing — see missing items below)",
            completed_items.len(),
            missing_items.len()
        )
    };

    let missing_section = if missing_items.is_empty() {
        "Missing items: None (all planned items succeeded).".to_string()
    } else {
        format!(
            "MISSING / INCOMPLETE ITEMS (DO NOT ASSUME COMPLETE):\n{}",
            missing_items
                .iter()
                .map(|m| format!("- {}", m))
                .collect::<Vec<_>>()
                .join("\n")
        )
    };

    let mut ordered_refs = Vec::new();
    let mut role_assets = Vec::new();
    let mut role_components = Vec::new();
    let mut role_pages_states = Vec::new();

    for (idx, (_out_id, item_id, title, kind, role, route, rel, w, h, deps)) in
        completed_items.iter().enumerate()
    {
        let display_path = if portable_relative {
            let fname = Path::new(rel)
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("original.png");
            format!("outputs/{}/{}", item_id, fname)
        } else {
            let abs = config.resolve_rel_path(rel)?;
            format_host_path(&abs)
        };
        let summary_line = format!(
            "   - {} ({}, {}x{}px): {}",
            title, item_id, w, h, display_path
        );
        match kind.as_str() {
            "asset" => role_assets.push(summary_line),
            "component" => role_components.push(summary_line),
            _ => role_pages_states.push(summary_line),
        }

        ordered_refs.push(format!(
            "{}. [{} | kind={} | role={} | route={}]\n   Item ID: {}\n   Path: {}\n   Dimensions: {}x{}px\n   Depends on: {}",
            idx + 1,
            title,
            kind,
            role,
            route.as_deref().unwrap_or("n/a"),
            item_id,
            display_path,
            w,
            h,
            if deps.is_empty() {
                "none (root/prerequisite)".to_string()
            } else {
                deps.join(", ")
            }
        ));
    }

    let grouped_image_paths_by_role = format!(
        "1. STANDALONE VISUAL ASSETS (place or optimize in asset directory FIRST):\n{}\n\
2. SHARED REUSABLE UI COMPONENTS (build as modular components SECOND):\n{}\n\
3. FULL-PAGE VIEWS, SECTIONS & INTERACTIVE STATES (compose routes THIRD):\n{}",
        if role_assets.is_empty() {
            "   - None in this set.".to_string()
        } else {
            role_assets.join("\n")
        },
        if role_components.is_empty() {
            "   - None in this set.".to_string()
        } else {
            role_components.join("\n")
        },
        if role_pages_states.is_empty() {
            "   - None in this set.".to_string()
        } else {
            role_pages_states.join("\n")
        }
    );

    let archetype = crate::models::detect_product_archetype(
        project_name,
        project_description,
        project_purpose,
        project_industry,
        mode,
        scope_preset,
        user_prompt,
    );
    let archetype_guidance = crate::models::build_product_archetype_guidance(archetype);
    let primary_set_path = completed_items
        .first()
        .and_then(|(_, item_id, _, _, _, _, rel, _, _, _)| {
            if portable_relative {
                let fname = Path::new(rel)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("original.png");
                Some(format!("outputs/{}/{}", item_id, fname))
            } else {
                config
                    .resolve_rel_path(rel)
                    .ok()
                    .map(|abs| format_host_path(&abs))
            }
        })
        .unwrap_or_else(|| manifest_path_str.to_string());
    let existing_ui_replacement_spec = build_existing_ui_replacement_spec(
        pool,
        config,
        project_id,
        selected_refs,
        project_name,
        &primary_set_path,
        portable_relative,
    )
    .await?;

    let mut page_routes_lines = vec![
        clean_code_syntax_to_human_prose(archetype_guidance),
        existing_ui_replacement_spec,
    ];
    let mut shared_comp_lines = Vec::new();
    let mut state_variant_lines = Vec::new();
    let mut dep_order_lines = Vec::new();

    if let Some(p) = plan {
        for pc in &p.page_composition {
            page_routes_lines.push(format!(
                "- Route {} — {}: Viewports [{}] | Ordered Sections: {} | Shared Components: {} | Required CTAs: {} | States: {}",
                pc.route,
                clean_code_syntax_to_human_prose(&pc.page_name),
                pc.viewport_targets.join(", "),
                pc.ordered_sections.join(" -> "),
                pc.shared_component_ids.join(", "),
                pc.required_ctas.join(", "),
                pc.states.join(", ")
            ));
        }

        // Also append a rich per-page visual specification ("What Is Shown in Each Page Image & How It Works")
        // for every non-asset page/section item so the coding agent knows the exact contents of every generated page!
        let mut added_per_page_header = false;
        for item in &p.items {
            if item.kind == "asset" || item.transparent_background {
                continue;
            }
            if !added_per_page_header {
                page_routes_lines.push(
                    "\nPER-PAGE UI SPECIFICATION (WHAT IS SHOWN IN EACH GENERATED PAGE IMAGE & IMAGE-TO-IMAGE NAVBAR CONTINUITY):".to_string()
                );
                page_routes_lines.push(
                    "- Shared Navigation Bar & Application Shell Rule: Page 1 (item_1, Visual Anchor) establishes the canonical sticky top navigation bar (brand logo on the left, route links in the center with a smooth shared-layout sliding active indicator, and primary call to action button on the right) and global footer. All subsequent pages (item_2, item_3, and onward) were generated using Image-to-Image conditioning from item_1 so the navigation bar and design system tokens stay 100% identical across routes while the page body changes per route.".to_string()
                );
                added_per_page_header = true;
            }
            let route_label = item.page_route.as_deref().unwrap_or("/");
            let anchor_badge = if item.is_visual_anchor || item.depends_on.is_empty() {
                "VISUAL ANCHOR PAGE (Root Reference)"
            } else {
                "IMAGE-TO-IMAGE DEPENDENT PAGE (Shares Navigation Bar & Tokens with item_1)"
            };
            page_routes_lines.push(format!(
                "- {} ({} | Route {} | {} | Viewport {} {}px):\n  * Art Direction & Purpose: {}\n  * What Is Shown Inside This Page Image (Top-to-Bottom): {}\n  * Exact Visible Copy & Labels: {}",
                clean_code_syntax_to_human_prose(&item.name),
                item.id,
                route_label,
                anchor_badge,
                item.viewport.intent,
                item.viewport.css_width,
                clean_code_syntax_to_human_prose(&item.direction),
                clean_code_syntax_to_human_prose(&item.image_prompt),
                if item.exact_copy.is_empty() {
                    "See reference image headings, cards, and controls.".to_string()
                } else {
                    clean_code_syntax_to_human_prose(&item.exact_copy.join(" | "))
                }
            ));
        }

        for sc in &p.shared_components {
            shared_comp_lines.push(format!(
                "- {} ({}): Canonical output {} | Reused across routes: {} | {}",
                clean_code_syntax_to_human_prose(&sc.name),
                sc.id,
                sc.canonical_item_id,
                sc.used_on_routes.join(", "),
                clean_code_syntax_to_human_prose(&sc.description)
            ));
        }
        for item in &p.items {
            if item.kind == "state" || item.viewport.intent != "desktop" {
                state_variant_lines.push(format!(
                    "- {} ({}): kind={}, viewport={} ({}px), route={}, parent dependencies=[{}]",
                    clean_code_syntax_to_human_prose(&item.name),
                    item.id,
                    item.kind,
                    item.viewport.intent,
                    item.viewport.css_width,
                    item.page_route.as_deref().unwrap_or("shared"),
                    item.depends_on.join(", ")
                ));
            }
            dep_order_lines.push(format!(
                "- Step {} ({} — {}): depends_on=[{}]{}",
                item.id,
                clean_code_syntax_to_human_prose(&item.name),
                item.kind,
                item.depends_on.join(", "),
                if item.is_visual_anchor {
                    " [VISUAL ANCHOR]"
                } else {
                    ""
                }
            ));
        }
    }

    let source_assets_info = resolve_source_assets_list(
        pool,
        config,
        project_id,
        brand,
        selected_refs,
        portable_relative,
    )
    .await?;

    let (reqs_copy, behaviors_responsive, asset_rules) =
        format_item_and_plan_details(&plan.cloned(), None, model_settings);

    let mut set_extracted_transparent_assets: Vec<(String, String, String, i64, i64)> = Vec::new();
    for (_out_id, item_id, title, kind, _role, _route, rel, w, h, _deps) in completed_items {
        if kind == "asset" {
            let display_p = if portable_relative {
                let fname = Path::new(rel)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("original.png");
                format!("outputs/{}/{}", item_id, fname)
            } else if let Ok(abs_p) = config.resolve_rel_path(rel) {
                format_host_path(&abs_p)
            } else {
                rel.clone()
            };
            set_extracted_transparent_assets.push((
                item_id.clone(),
                title.clone(),
                display_p,
                *w,
                *h,
            ));
        }
    }

    Ok(HANDOFF_SET_TEMPLATE_V1
        .replace(
            "{{grouped_image_paths_by_role}}",
            &grouped_image_paths_by_role,
        )
        .replace("{{project_name}}", project_name)
        .replace(
            "{{project_description}}",
            if project_description.trim().is_empty() {
                "Interactive web application"
            } else {
                project_description
            },
        )
        .replace(
            "{{project_purpose}}",
            if project_purpose.trim().is_empty() {
                "Web application interface"
            } else {
                project_purpose
            },
        )
        .replace(
            "{{project_industry}}",
            if project_industry.trim().is_empty() {
                "Software & Digital Product"
            } else {
                project_industry
            },
        )
        .replace(
            "{{target_audience}}",
            if target_audience.trim().is_empty() {
                "General web users"
            } else {
                target_audience
            },
        )
        .replace(
            "{{desired_user_action}}",
            if desired_user_action.trim().is_empty() {
                "Engage with primary workflow"
            } else {
                desired_user_action
            },
        )
        .replace(
            "{{scope}}",
            &format!("{} set ({}) — {} outputs", mode, scope_preset, completed_items.len()),
        )
        .replace(
            "{{target_stack_and_repository_or_inspect_existing_project}}",
            &format_target_context(brand),
        )
        .replace(
            "{{chosen_design_style_block}}",
            &format_design_style_block(brand),
        )
        .replace("{{manifest_path}}", manifest_path_str)
        .replace("{{set_completion_status}}", &set_status)
        .replace("{{missing_items_section}}", &missing_section)
        .replace(
            "{{ordered_reference_list_with_exact_absolute_paths_roles_and_dimensions}}",
            &ordered_refs.join("\n\n"),
        )
        .replace(
            "{{page_routes_and_sections_table}}",
            &if page_routes_lines.is_empty() {
                "Single view / component set (no multi-route composition table).".to_string()
            } else {
                page_routes_lines.join("\n")
            },
        )
        .replace(
            "{{shared_components_table}}",
            &if shared_comp_lines.is_empty() {
                "No separate cross-route shared component entries.".to_string()
            } else {
                shared_comp_lines.join("\n")
            },
        )
        .replace(
            "{{placeholder_and_one_to_one_card_mandate}}",
            &build_placeholder_and_card_mandate(brand, plan, None, archetype),
        )
        .replace(
            "{{sticky_and_positioning_rules}}",
            &build_sticky_positioning_spec(
                mode,
                "page",
                scope_preset,
                project_name,
                brand,
                plan,
                None,
            ),
        )
        .replace(
            "{{hover_click_states_and_color_effects}}",
            &build_interactive_states_and_colors_spec(brand, plan, None),
        )
        .replace(
            "{{animations_and_motion_spec}}",
            &build_animations_and_motion_spec(brand, plan, None),
        )
        .replace(
            "{{chroma_and_asset_cropping_rules}}",
            &build_chroma_and_asset_cropping_spec(
                brand,
                plan,
                None,
                model_settings,
                None,
                &set_extracted_transparent_assets,
            ),
        )
        .replace(
            "{{state_and_viewport_variants}}",
            &if state_variant_lines.is_empty() {
                "Standard desktop viewport outputs.".to_string()
            } else {
                state_variant_lines.join("\n")
            },
        )
        .replace(
            "{{asset_dependencies_and_order}}",
            &if dep_order_lines.is_empty() {
                "Implement prerequisite assets first, then shared components, then pages and modal states.".to_string()
            } else {
                dep_order_lines.join("\n")
            },
        )
        .replace("{{brand_tokens_and_typography}}", &format_brand_tokens(brand))
        .replace(
            "{{original_logo_font_and_required_asset_paths}}",
            &source_assets_info,
        )
        .replace("{{asset_usage_rules}}", &asset_rules)
        .replace("{{original_user_request}}", user_prompt)
        .replace("{{requirements_and_exact_copy}}", &reqs_copy)
        .replace(
            "{{interactions_states_and_responsive_rules}}",
            &behaviors_responsive,
        )
        .replace(
            "{{extraction_steps}}",
            &build_extraction_steps(include_extraction),
        ))
}

async fn resolve_source_assets_list(
    pool: &SqlitePool,
    config: &AppConfig,
    project_id: &str,
    brand: &BrandSpec,
    selected_refs: &[SelectedReference],
    portable_relative: bool,
) -> AppResult<String> {
    let mut asset_ids = Vec::new();
    if let Some(ref id) = brand.logo.primary_asset_id {
        asset_ids.push((
            id.clone(),
            "Primary Brand Logo (Reuse Original)".to_string(),
        ));
    }
    if let Some(ref id) = brand.logo.mark_only_asset_id {
        asset_ids.push((
            id.clone(),
            "Mark-Only Brand Logo (Reuse Original)".to_string(),
        ));
    }
    for font_id in &brand.typography.uploaded_font_asset_ids {
        asset_ids.push((
            font_id.clone(),
            "Uploaded Brand Font File (Reuse Original)".to_string(),
        ));
    }
    for sr in selected_refs {
        asset_ids.push((
            sr.asset_id.clone(),
            format!(
                "Selected Reference ({} | preserve_as_is={} | notes={})",
                sr.role.as_str(),
                sr.preserve_as_is,
                sr.placement_notes
            ),
        ));
    }

    let mut lines = Vec::new();
    let mut seen = std::collections::HashSet::new();

    for (aid, desc) in asset_ids {
        if !seen.insert(aid.clone()) {
            continue;
        }
        if let Some(row) = sqlx::query(
            "SELECT label, role, original_rel_path, mime_type, width, height FROM assets WHERE id = ? AND project_id = ?"
        )
        .bind(&aid)
        .bind(project_id)
        .fetch_optional(pool)
        .await?
        {
            let label: String = row.get("label");
            let orig_rel: String = row.get("original_rel_path");
            let mime: String = row.get("mime_type");
            let w: i64 = row.get("width");
            let h: i64 = row.get("height");

            let path_str = if portable_relative {
                let fname = Path::new(&orig_rel)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("original");
                format!("assets/{}/{}", aid, fname)
            } else {
                let abs = config.resolve_rel_path(&orig_rel)?;
                format_host_path(&abs)
            };

            lines.push(format!(
                "- Original Source Asset [{} — {}]: {} (MIME: {}, Dimensions: {}x{}px) -> REUSE THIS ORIGINAL FILE directly instead of extracting from generated raster.",
                label, desc, path_str, mime, w, h
            ));
        }
    }

    if lines.is_empty() {
        Ok("No separate uploaded logo/font/source asset files attached (use brand typography and tokens above).".to_string())
    } else {
        Ok(lines.join("\n"))
    }
}

fn format_item_and_plan_details(
    plan: &Option<PlannerPlan>,
    item: Option<&crate::models::PlanItem>,
    model_settings: Option<&RunModelSettings>,
) -> (String, String, String) {
    let mut reqs = Vec::new();
    let mut behaviors = Vec::new();
    let mut asset_rules = Vec::new();

    if let Some(p) = plan {
        for r in &p.requirements {
            reqs.push(format!(
                "- [{}] ({}) {}",
                r.id,
                r.source,
                clean_code_syntax_to_human_prose(&r.description)
            ));
        }
        for rule in &p.shared_design_spec.layout_rules {
            behaviors.push(format!(
                "- Shared layout rule: {}",
                clean_code_syntax_to_human_prose(rule)
            ));
        }
        for ar in &p.shared_design_spec.asset_rules {
            asset_rules.push(format!("- {}", clean_code_syntax_to_human_prose(ar)));
        }
    }

    if let Some(it) = item {
        if !it.exact_copy.is_empty() {
            reqs.push(format!(
                "Exact copy for '{}': {}",
                clean_code_syntax_to_human_prose(&it.name),
                clean_code_syntax_to_human_prose(&it.exact_copy.join(" | "))
            ));
        }
        for b in &it.behaviors {
            behaviors.push(format!(
                "- Interactive behavior: {}",
                clean_code_syntax_to_human_prose(b)
            ));
        }
        for rr in &it.responsive_rules {
            behaviors.push(format!(
                "- Responsive rule: {}",
                clean_code_syntax_to_human_prose(rr)
            ));
        }
        for note in &it.implementation_notes {
            behaviors.push(format!(
                "- Implementation note: {}",
                clean_code_syntax_to_human_prose(note)
            ));
        }
        for ai in &it.asset_instructions {
            asset_rules.push(format!("- {}", clean_code_syntax_to_human_prose(ai)));
        }
    } else if let Some(p) = plan {
        for it in &p.items {
            if !it.exact_copy.is_empty() {
                reqs.push(format!(
                    "- Copy for '{}' ({}): {}",
                    clean_code_syntax_to_human_prose(&it.name),
                    it.id,
                    clean_code_syntax_to_human_prose(&it.exact_copy.join(" | "))
                ));
            }
            for b in &it.behaviors {
                behaviors.push(format!(
                    "- [{}] Behavior: {}",
                    clean_code_syntax_to_human_prose(&it.name),
                    clean_code_syntax_to_human_prose(b)
                ));
            }
            for rr in &it.responsive_rules {
                behaviors.push(format!(
                    "- [{}] Responsive: {}",
                    clean_code_syntax_to_human_prose(&it.name),
                    clean_code_syntax_to_human_prose(rr)
                ));
            }
        }
    }

    if let Some(ms) = model_settings {
        if let Some(ref ao) = ms.asset_options {
            asset_rules.push(format!(
                "- Asset Mode Specs: usage={}, theme={}, transparent_background={}, seamless_tiling={}, decorative={}, alt_text_intent='{}'",
                ao.intended_usage,
                ao.theme_context,
                ao.transparent_background,
                ao.seamless_tiling,
                ao.is_decorative,
                ao.alt_text_intent
            ));
        }
    }

    (
        if reqs.is_empty() {
            "Implement all user requirements in the prompt above.".to_string()
        } else {
            reqs.join("\n")
        },
        if behaviors.is_empty() {
            "Implement accessible keyboard focus, responsive layout, and semantic interface controls."
                .to_string()
        } else {
            behaviors.join("\n")
        },
        if asset_rules.is_empty() {
            "Prefer original uploaded vector or raster assets over cropped raster regions."
                .to_string()
        } else {
            asset_rules.join("\n")
        },
    )
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
async fn write_portable_zip_bundle(
    pool: &SqlitePool,
    config: &AppConfig,
    zip_target: &Path,
    project_id: &str,
    brand: &BrandSpec,
    selected_refs: &[SelectedReference],
    completed_items: &[(
        String,
        String,
        String,
        String,
        String,
        Option<String>,
        String,
        i64,
        i64,
        Vec<String>,
    )],
    portable_handoff_md: &str,
    portable_manifest_json: &serde_json::Value,
) -> AppResult<()> {
    if let Some(parent) = zip_target.parent() {
        std::fs::create_dir_all(parent)?;
    }

    let mut asset_files_to_pack: Vec<(String, Vec<u8>)> = Vec::new();
    let mut seen_assets = std::collections::HashSet::new();

    let mut candidate_asset_ids = Vec::new();
    if let Some(ref id) = brand.logo.primary_asset_id {
        candidate_asset_ids.push(id.clone());
    }
    if let Some(ref id) = brand.logo.mark_only_asset_id {
        candidate_asset_ids.push(id.clone());
    }
    for fid in &brand.typography.uploaded_font_asset_ids {
        candidate_asset_ids.push(fid.clone());
    }
    for sr in selected_refs {
        candidate_asset_ids.push(sr.asset_id.clone());
    }

    for aid in candidate_asset_ids {
        if !seen_assets.insert(aid.clone()) {
            continue;
        }
        if let Some(row) =
            sqlx::query("SELECT original_rel_path FROM assets WHERE id = ? AND project_id = ?")
                .bind(&aid)
                .bind(project_id)
                .fetch_optional(pool)
                .await?
        {
            let orig_rel: String = row.get("original_rel_path");
            if let Ok(abs) = config.resolve_rel_path(&orig_rel) {
                if let Ok(bytes) = std::fs::read(&abs) {
                    let fname = Path::new(&orig_rel)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or("original");
                    asset_files_to_pack.push((format!("assets/{}/{}", aid, fname), bytes));
                }
            }
        }
    }

    let mut output_files_to_pack: Vec<(String, Vec<u8>)> = Vec::new();
    for (_out_id, item_id, _title, _kind, _role, _route, rel, _w, _h, _deps) in completed_items {
        if let Ok(abs) = config.resolve_rel_path(rel) {
            if let Ok(bytes) = std::fs::read(&abs) {
                let fname = Path::new(rel)
                    .file_name()
                    .and_then(|s| s.to_str())
                    .unwrap_or("original.png");
                output_files_to_pack.push((format!("outputs/{}/{}", item_id, fname), bytes));
            }
        }
    }

    let brand_bytes = serde_json::to_vec_pretty(brand).unwrap_or_default();
    let manifest_bytes = serde_json::to_vec_pretty(portable_manifest_json).unwrap_or_default();
    let impl_bytes = portable_handoff_md.as_bytes().to_vec();
    let target_buf = zip_target.to_path_buf();

    tokio::task::spawn_blocking(move || -> AppResult<()> {
        let mut cursor = std::io::Cursor::new(Vec::new());
        {
            let mut zip = ZipWriter::new(&mut cursor);
            let options =
                SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);

            zip.start_file("implementation.md", options)
                .map_err(|e| AppError::Internal(format!("ZIP error: {}", e)))?;
            zip.write_all(&impl_bytes)?;

            zip.start_file("manifest.json", options)
                .map_err(|e| AppError::Internal(format!("ZIP error: {}", e)))?;
            zip.write_all(&manifest_bytes)?;

            zip.start_file("brand.json", options)
                .map_err(|e| AppError::Internal(format!("ZIP error: {}", e)))?;
            zip.write_all(&brand_bytes)?;

            for (rel_name, bytes) in asset_files_to_pack {
                zip.start_file(rel_name, options)
                    .map_err(|e| AppError::Internal(format!("ZIP error: {}", e)))?;
                zip.write_all(&bytes)?;
            }

            for (rel_name, bytes) in output_files_to_pack {
                zip.start_file(rel_name, options)
                    .map_err(|e| AppError::Internal(format!("ZIP error: {}", e)))?;
                zip.write_all(&bytes)?;
            }

            let w = zip
                .finish()
                .map_err(|e| AppError::Internal(format!("ZIP finish error: {}", e)))?;
            w.seek(std::io::SeekFrom::Start(0))?;
        }
        atomic_write_file(&target_buf, &cursor.into_inner())?;
        Ok(())
    })
    .await
    .map_err(|e| AppError::Internal(format!("ZIP worker task failed: {}", e)))??;

    Ok(())
}

pub fn slugify_folder_name(raw: &str, fallback: &str) -> String {
    let mut slug = String::new();
    let mut last_dash = false;
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash && !slug.is_empty() {
            slug.push('-');
            last_dash = true;
        }
        if slug.len() >= 48 {
            break;
        }
    }
    let trimmed = slug.trim_matches('-').to_string();
    if trimmed.is_empty() {
        fallback.to_string()
    } else {
        trimmed
    }
}

/// Synchronizes a human-readable, organized folder hierarchy on disk inside `<active_data_dir>/projects/`:
/// `<active_data_dir>/projects/<project-slug>-<short-id>/`
///   ├── `project.json`
///   ├── `brand.json`
///   └── `threads/<thread-slug>/`
///         ├── `thread.json`
///         └── `tabs/<tab-key>/`
///               ├── `images/01-<item-slug>.png`
///               └── `prompts.md` (complete AI coding agent handoff prompt + per-image generation prompts)
pub async fn sync_project_organized_folders(
    pool: &SqlitePool,
    config: &AppConfig,
    project_id: &str,
) -> AppResult<std::path::PathBuf> {
    let proj_row = sqlx::query(
        "SELECT id, name, description, purpose, industry, target_audience, audience_needs, desired_user_action, is_example, created_at, updated_at FROM projects WHERE id = ? AND archive_status = 'active'"
    )
    .bind(project_id)
    .fetch_optional(pool)
    .await?;

    let Some(pr) = proj_row else {
        return Ok(config.active_data_dir().join("projects"));
    };

    let proj_name: String = pr.get("name");
    let proj_desc: String = pr.get("description");
    let proj_purpose: String = pr.get("purpose");
    let proj_industry: String = pr.get("industry");
    let proj_audience: String = pr.get("target_audience");
    let proj_needs: String = pr.get("audience_needs");
    let proj_action: String = pr.get("desired_user_action");
    let is_example: i64 = pr.try_get("is_example").unwrap_or(0);
    let created_at: String = pr.get("created_at");
    let updated_at: String = pr.get("updated_at");

    let short_id: String = project_id
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .take(8)
        .collect::<String>()
        .to_ascii_lowercase();
    let proj_slug = format!(
        "{}-{}",
        slugify_folder_name(&proj_name, "project"),
        if short_id.is_empty() { "p1" } else { &short_id }
    );

    let active_root = config.active_data_dir();
    let organized_proj_dir = active_root.join("projects").join(&proj_slug);
    std::fs::create_dir_all(&organized_proj_dir)?;

    let brand_row = sqlx::query(
        "SELECT version, brand_json FROM brand_versions WHERE project_id = ? ORDER BY version DESC LIMIT 1"
    )
    .bind(project_id)
    .fetch_optional(pool)
    .await?;

    let (brand_ver, brand): (i64, BrandSpec) = match brand_row {
        Some(br) => (
            br.get("version"),
            serde_json::from_str(br.get::<&str, _>("brand_json")).unwrap_or_default(),
        ),
        None => (1, BrandSpec::default()),
    };

    let project_meta = serde_json::json!({
        "id": project_id,
        "name": proj_name,
        "description": proj_desc,
        "purpose": proj_purpose,
        "industry": proj_industry,
        "target_audience": proj_audience,
        "audience_needs": proj_needs,
        "desired_user_action": proj_action,
        "is_example": is_example != 0,
        "brand_version": brand_ver,
        "created_at": created_at,
        "updated_at": updated_at
    });
    atomic_write_file(
        &organized_proj_dir.join("project.json"),
        serde_json::to_string_pretty(&project_meta)
            .unwrap_or_default()
            .as_bytes(),
    )?;
    atomic_write_file(
        &organized_proj_dir.join("brand.json"),
        serde_json::to_string_pretty(&brand)
            .unwrap_or_default()
            .as_bytes(),
    )?;
    let design_md_content = render_project_design_md(
        &proj_name,
        &proj_desc,
        &proj_purpose,
        &proj_industry,
        &proj_audience,
        &proj_needs,
        &proj_action,
        &brand,
        brand_ver,
    );
    atomic_write_file(
        &organized_proj_dir.join("DESIGN.md"),
        design_md_content.as_bytes(),
    )?;

    let threads = sqlx::query(
        "SELECT id, title, created_at, updated_at FROM threads WHERE project_id = ? AND archive_status = 'active' ORDER BY created_at ASC"
    )
    .bind(project_id)
    .fetch_all(pool)
    .await?;

    for (t_idx, tr) in threads.iter().enumerate() {
        let tid: String = tr.get("id");
        let t_title: String = tr.get("title");
        let t_created: String = tr.get("created_at");
        let t_updated: String = tr.get("updated_at");

        let thread_slug = format!(
            "{:02}-{}",
            t_idx + 1,
            slugify_folder_name(&t_title, "thread")
        );
        let thread_dir = organized_proj_dir.join("threads").join(&thread_slug);
        std::fs::create_dir_all(&thread_dir)?;

        let thread_meta = serde_json::json!({
            "id": tid,
            "project_id": project_id,
            "title": t_title,
            "created_at": t_created,
            "updated_at": t_updated
        });
        atomic_write_file(
            &thread_dir.join("thread.json"),
            serde_json::to_string_pretty(&thread_meta)
                .unwrap_or_default()
                .as_bytes(),
        )?;

        let out_rows = sqlx::query(
            "SELECT o.id, o.run_id, o.plan_item_id, o.title, o.kind, o.role, o.page_route, o.tab_key, o.tab_label, o.transparent_background, o.compiled_prompt, o.original_rel_path, o.width, o.height, o.depends_on_json, o.item_spec_json, o.created_at \
             FROM outputs o \
             JOIN runs r ON r.id = o.run_id \
             WHERE r.thread_id = ? AND o.status = 'completed' \
             ORDER BY o.created_at ASC, o.variant_index ASC"
        )
        .bind(&tid)
        .fetch_all(pool)
        .await?;

        let mut tabs_map: std::collections::BTreeMap<String, Vec<sqlx::sqlite::SqliteRow>> =
            std::collections::BTreeMap::new();
        for row in out_rows {
            let tk: String = row
                .try_get::<String, _>("tab_key")
                .unwrap_or_else(|_| "main".to_string());
            let clean_tk = slugify_folder_name(&tk, "main");
            tabs_map.entry(clean_tk).or_default().push(row);
        }

        let tabs_root_dir = thread_dir.join("tabs");
        if tabs_root_dir.exists() {
            if let Ok(entries) = std::fs::read_dir(&tabs_root_dir) {
                for entry in entries.flatten() {
                    if let Ok(ft) = entry.file_type() {
                        if ft.is_dir() {
                            let fname = entry.file_name().to_string_lossy().to_string();
                            if !tabs_map.contains_key(&fname) {
                                let _ = std::fs::remove_dir_all(entry.path());
                            }
                        }
                    }
                }
            }
        }

        for (tab_key, tab_outputs) in tabs_map {
            let tab_dir = thread_dir.join("tabs").join(&tab_key);
            let images_dir = tab_dir.join("images");
            if images_dir.exists() {
                let _ = std::fs::remove_dir_all(&images_dir);
            }
            std::fs::create_dir_all(&images_dir)?;

            let tab_label: String = tab_outputs
                .first()
                .and_then(|r| r.try_get::<String, _>("tab_label").ok())
                .unwrap_or_else(|| tab_key.clone());

            let latest_run_id: String = tab_outputs
                .last()
                .map(|r| r.get::<String, _>("run_id"))
                .unwrap_or_default();

            let run_row = sqlx::query(
                "SELECT mode, scope_preset, user_prompt, selected_references_json, plan_json, model_settings_json FROM runs WHERE id = ?"
            )
            .bind(&latest_run_id)
            .fetch_optional(pool)
            .await?;

            let (mode, scope_preset, user_prompt, selected_refs, mut plan_opt, model_settings_opt) =
                if let Some(rr) = run_row {
                    let m: String = rr.get("mode");
                    let sp: String = rr.get("scope_preset");
                    let up: String = rr.get("user_prompt");
                    let sr: Vec<SelectedReference> =
                        serde_json::from_str(rr.get::<&str, _>("selected_references_json"))
                            .unwrap_or_default();
                    let pl: Option<PlannerPlan> = rr
                        .get::<Option<String>, _>("plan_json")
                        .and_then(|s| serde_json::from_str(&s).ok());
                    let ms: Option<RunModelSettings> =
                        serde_json::from_str(rr.get::<&str, _>("model_settings_json")).ok();
                    (m, sp, up, sr, pl, ms)
                } else {
                    (
                        "full_page".to_string(),
                        "page_view".to_string(),
                        String::new(),
                        Vec::new(),
                        None,
                        None,
                    )
                };

            let mut completed_items = Vec::new();
            let mut per_image_prompt_sections = Vec::new();
            let mut seen_tab_item_ids = std::collections::HashSet::new();
            let mut active_tab_plan_items = Vec::new();

            for (idx, row) in tab_outputs.iter().enumerate() {
                let out_id: String = row.get("id");
                let raw_item_id: String = row.get("plan_item_id");
                let item_id = if seen_tab_item_ids.contains(&raw_item_id) {
                    let short_id: String = out_id.chars().take(6).collect();
                    format!("{}_{}", raw_item_id, short_id)
                } else {
                    raw_item_id.clone()
                };
                seen_tab_item_ids.insert(item_id.clone());
                let title: String = row.get("title");
                let kind: String = row.get("kind");
                let role: String = row.get("role");
                let route: Option<String> = row.get("page_route");
                let orig_rel: String = row.get("original_rel_path");
                let w: i64 = row.get("width");
                let h: i64 = row.get("height");
                let transparent: i64 = row.try_get("transparent_background").unwrap_or(0);
                let prompt_used: String = row.try_get("compiled_prompt").unwrap_or_default();
                let item_spec_str: String = row.try_get("item_spec_json").unwrap_or_default();
                if let Ok(mut parsed_item) =
                    serde_json::from_str::<crate::models::PlanItem>(&item_spec_str)
                {
                    parsed_item.id = item_id.clone();
                    parsed_item.name = title.clone();
                    active_tab_plan_items.push(parsed_item);
                }
                let deps: Vec<String> =
                    serde_json::from_str(row.try_get::<&str, _>("depends_on_json").unwrap_or("[]"))
                        .unwrap_or_default();

                let file_slug = slugify_folder_name(&title, &item_id);
                let suffix = if transparent != 0 && !file_slug.contains("transparent") {
                    "-transparent"
                } else {
                    ""
                };
                let organized_filename = format!("{:02}-{}{}.png", idx + 1, file_slug, suffix);
                let organized_img_abs = images_dir.join(&organized_filename);

                if let Ok(src_abs) = config.resolve_rel_path(&orig_rel) {
                    if src_abs.exists() {
                        let _ = std::fs::copy(&src_abs, &organized_img_abs);
                    }
                }

                let organized_rel = organized_img_abs
                    .strip_prefix(&active_root)
                    .map(|p| p.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| orig_rel.clone());

                completed_items.push((
                    out_id,
                    item_id.clone(),
                    title.clone(),
                    kind.clone(),
                    role.clone(),
                    route.clone(),
                    organized_rel,
                    w,
                    h,
                    deps,
                ));

                per_image_prompt_sections.push(format!(
                    "### {}. {} ({})\n- **Organized Image File**: images/{}\n- **Item ID / Role**: {} ({} / {})\n- **Dimensions**: {}x{}px\n- **Transparent Background**: {}\n- **Generation Prompt**:\n  {}\n",
                    idx + 1,
                    clean_code_syntax_to_human_prose(&title),
                    kind,
                    organized_filename,
                    item_id,
                    kind,
                    role,
                    w,
                    h,
                    if transparent != 0 { "Yes (Alpha PNG)" } else { "No (Full Canvas)" },
                    clean_code_syntax_to_human_prose(&prompt_used)
                ));
            }

            let audience_combined = if proj_needs.trim().is_empty() {
                proj_audience.clone()
            } else {
                format!("{} (Needs: {})", proj_audience, proj_needs)
            };

            if let Some(ref mut mp) = plan_opt {
                if !active_tab_plan_items.is_empty() {
                    mp.items = active_tab_plan_items;
                }
            }

            let manifest_path_str = format_host_path(&tab_dir.join("prompts.md"));
            let handoff_md = render_set_handoff_markdown(
                pool,
                config,
                project_id,
                &proj_name,
                &proj_desc,
                &proj_purpose,
                &proj_industry,
                &audience_combined,
                &proj_action,
                &mode,
                &scope_preset,
                &user_prompt,
                &brand,
                &selected_refs,
                plan_opt.as_ref(),
                model_settings_opt.as_ref(),
                &completed_items,
                &[],
                true,
                true,
                &manifest_path_str,
                false,
            )
            .await?;

            let full_prompts_md = format!(
                "# Design Master — Tab Handoff & Image Prompts ({})\n\n\
                - **Project**: {}\n\
                - **Thread**: {}\n\
                - **Tab**: {} ({})\n\
                - **Images Folder**: {}\n\n\
                ---\n\n\
                ## 1. Ready-to-Paste AI Coding Agent Handoff Prompt\n\n\
                {}\n\n\
                ---\n\n\
                ## 2. Per-Image Generation Prompts & Asset Inventory\n\n\
                {}\n",
                clean_code_syntax_to_human_prose(&tab_label),
                clean_code_syntax_to_human_prose(&proj_name),
                clean_code_syntax_to_human_prose(&t_title),
                clean_code_syntax_to_human_prose(&tab_label),
                tab_key,
                format_host_path(&images_dir),
                handoff_md,
                per_image_prompt_sections.join("\n")
            );

            atomic_write_file(&tab_dir.join("prompts.md"), full_prompts_md.as_bytes())?;
        }
    }

    Ok(organized_proj_dir)
}

#[allow(clippy::too_many_arguments)]
pub fn render_project_design_md(
    project_name: &str,
    project_description: &str,
    project_purpose: &str,
    project_industry: &str,
    target_audience: &str,
    audience_needs: &str,
    desired_user_action: &str,
    brand: &BrandSpec,
    brand_version: i64,
) -> String {
    let style_title = brand
        .design_style
        .as_ref()
        .filter(|s| !s.style_name.trim().is_empty())
        .map(|s| format!("#{} {}", s.style_number, s.style_name))
        .unwrap_or_else(|| brand.brand_personality.clone());
    let style_def = brand
        .design_style
        .as_ref()
        .filter(|s| !s.definition.trim().is_empty())
        .map(|s| s.definition.clone())
        .unwrap_or_else(|| brand.design_direction.clone());
    let style_rules = brand
        .design_style
        .as_ref()
        .filter(|s| !s.visual_rules.trim().is_empty())
        .map(|s| s.visual_rules.clone())
        .unwrap_or_else(|| "Enforce crisp architectural borders, high-contrast scannable typography, and purposeful negative space.".to_string());
    let style_modifiers = brand
        .design_style
        .as_ref()
        .filter(|s| !s.modifiers.is_empty())
        .map(|s| s.modifiers.join(" + "))
        .unwrap_or_else(|| "None".to_string());

    let l = &brand.colors.light;
    let d = &brand.colors.dark;
    let t = &brand.typography;
    let lt = &brand.layout_tokens;

    let do_rules = if brand.brand_rules.is_empty() {
        "- Maintain strict visual consistency across all routes, components, and interactive states.\n- Ensure WCAG AA/AAA text contrast on every surface.".to_string()
    } else {
        brand
            .brand_rules
            .iter()
            .map(|r| format!("- {}", r))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let inclusions = if brand.mandatory_inclusions.is_empty() {
        "- Canonical brand mark / monogram and consistent primary action buttons.".to_string()
    } else {
        brand
            .mandatory_inclusions
            .iter()
            .map(|r| format!("- {}", r))
            .collect::<Vec<_>>()
            .join("\n")
    };
    let forbidden = if brand.forbidden_treatments.is_empty() {
        "- No generic AI slop gradients, nested card-in-card clutter, or low-contrast gray text on colored surfaces.".to_string()
    } else {
        brand
            .forbidden_treatments
            .iter()
            .map(|r| format!("- {}", r))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "# DESIGN.md — {project_name}\n\
> Canonical AI Design System & Brand Schema (`nexu-io/open-design` + `pbakaus/impeccable` Anti-AI-Slop Standard · Brand Version v{brand_version})\n\n\
## 1. Visual Theme, Atmosphere & Product Identity\n\
- **Project Name**: {project_name}\n\
- **Product Description**: {project_description}\n\
- **Purpose & Industry**: {project_purpose} ({project_industry})\n\
- **Target Audience & Needs**: {target_audience} — {audience_needs}\n\
- **Primary Conversion / User Action**: {desired_user_action}\n\
- **Chosen Design Style**: **{style_title}**\n\
- **Style Definition & Philosophy**: {style_def}\n\
- **Visual System Rules**: {style_rules}\n\
- **Combinable Modifiers**: {style_modifiers}\n\
- **Brand Personality & Direction**: {brand_personality} — {design_direction}\n\
- **Tone of Voice & UI Density**: {tone} | Density: {density} | Language: {lang}\n\n\
## 2. Color Palette & Component Semantic Roles\n\
Colors are mapped by functional UI role so coding agents never guess surface polarity or contrast.\n\n\
### 2.1 Core Atmospheric Palette (Dark & Light Polarity)\n\
| Semantic Token | Dark Mode (Primary Studio Polarity) | Light Mode Polarity | Functional Usage & Role |\n\
| :--- | :--- | :--- | :--- |\n\
| **Primary Brand Action** | `{d_pri}` | `{l_pri}` | Primary CTA buttons, active route indicators, focus rings, key highlights |\n\
| **Secondary / Accent** | `{d_acc}` | `{l_acc}` | Interactive badges, secondary highlights, telemetry accents, gradient spark nodes |\n\
| **Canvas Background** | `{d_bg}` | `{l_bg}` | Root page background (`body` / main viewport canvas) — tinted neutral, never dead `#000000` |\n\
| **Elevated Surface / Card** | `{d_surf}` | `{l_surf}` | Elevated panels, navigation shells, cards, modal dialogs, inspector drawers |\n\
| **Primary Ink (Text)** | `{d_txt}` | `{l_txt}` | Headlines, primary body copy, active navigation labels, input text |\n\
| **Secondary / Muted Ink** | `{d_muted}` | `{l_muted}` | Supporting descriptions, metadata captions, timestamps, axis labels |\n\
| **Architectural Hairline Border** | `{d_bdr}` | `{l_bdr}` | `1px` structural grid dividers, card borders, input outlines, navbar bottom border |\n\
| **Success Semantic** | `{d_suc}` | `{l_suc}` | Completed states, verified badges, positive delta indicators |\n\
| **Warning Semantic** | `{d_warn}` | `{l_warn}` | Caution banners, pending/in-progress indicators |\n\
| **Danger / Destructive** | `{d_err}` | `{l_err}` | Error states, destructive actions, validation alerts |\n\n\
### 2.2 Component-Specific Color Allocation Matrix (Buttons, Navbar, Cards, Inputs)\n\
| UI Component | Background / Fill | Text / Icon Color | Border Token | Hover / Active State Treatment |\n\
| :--- | :--- | :--- | :--- | :--- |\n\
| **Sticky Top Navbar** | `{d_surf}` (85% opacity + `backdrop-filter: blur(16px)`) | Active link: `{d_txt}` · Inactive link: `{d_muted}` | Bottom `1px solid {d_bdr}` | Links transition to `{d_txt}` with shared active pill/underline in `{d_pri}` |\n\
| **Primary Action Button** | Solid `{d_pri}` (or subtle linear sheen `{d_pri}` → `{d_acc}`) | High-contrast `#FFFFFF` (or crisp inverse ink) | `1px solid {d_pri}` | Elevate `-1px`, brighten fill by 8%, glow shadow `0 8px 24px -6px {d_pri}66`, active press scale `0.98` |\n\
| **Secondary / Outline Button** | `{d_surf}` | `{d_txt}` | `1px solid {d_bdr}` | Border shifts to `{d_pri}`, background tints with 10% `{d_pri}`, text stays `{d_txt}` |\n\
| **Ghost / Toolbar Button** | Transparent | `{d_muted}` | Transparent (`1px solid transparent`) | Subtle surface fill `{d_surf}`, text brightens to `{d_txt}` |\n\
| **Destructive Button** | `{d_err}` (15% surface tint at rest, solid on confirm) | `{d_err}` (or `#FFFFFF` on solid fill) | `1px solid {d_err}` | Solid `{d_err}` fill with `#FFFFFF` text |\n\
| **Feature / Bento Card** | `{d_surf}` | Title: `{d_txt}` · Body: `{d_muted}` | `1px solid {d_bdr}` | Border highlights toward `{d_pri}` (40% alpha) with subtle surface lift |\n\
| **Form Input / Search Box** | `{d_bg}` | Value: `{d_txt}` · Placeholder: `{d_muted}` | `1px solid {d_bdr}` | Focus ring `2px solid {d_pri}`, border `{d_pri}` |\n\
| **Status / Category Badge** | 15% tinted `{d_pri}` or `{d_acc}` | `{d_txt}` or `{d_acc}` | `1px solid {d_pri}4D` | Crisp pill geometry with tabular numerals |\n\n\
## 3. Typography & Scannable Hierarchy (`/typeset`)\n\
- **Heading / Display Font Family**: `{h_font}` (Weights: `600` SemiBold, `700` Bold; tight optical tracking `-0.025em` to `-0.04em`; line-height `1.08`–`1.18`)\n\
- **Body / UI Font Family**: `{b_font}` (Weights: `400` Regular for prose, `500` Medium for navigation/labels, `600` SemiBold for emphasis; line-height `1.55`–`1.65`; max line width `60ch–70ch`)\n\
- **Monospace / Code / Telemetry Font**: `{m_font}` (`font-variant-numeric: tabular-nums` required for all numbers, counters, prices, and coordinates)\n\
- **Fallback Stack**: `{fallbacks}`\n\
- **Type Scale System**: `{scale}`\n\
- **Line Heights**: `{lh}`\n\
- **Font Source**: `{f_source}`\n\n\
## 4. Layout, Spatial Architecture & Component Specifications\n\
- **Spacing Scale**: `{spacing}`\n\
- **Content Max Width**: `{c_width}`\n\
- **Grid System**: `{grid}`\n\
- **Corner Radii**: `{radii}`\n\
- **Border Treatment**: `{borders}`\n\
- **Shadow & Elevation System**: `{shadows}`\n\
- **Iconography Style**: `{icons}` (Convert all icons, logos, and geometric marks into crisp vectorial SVG graphics)\n\
- **Button Treatment**: `{buttons}`\n\n\
## 5. Anti-AI-Slop Engineering & Design Guardrails (`pbakaus/impeccable` Standard)\n\
1. **Pass the Squint Test**: Every viewport must have one unmistakable #1 primary focal point, a clear #2 secondary action, and generous negative space.\n\
2. **Ban Card-in-Card Hell (\"Cardocalypse\")**: Never nest cards inside cards with redundant borders and shadows. Use proximity, whitespace, `1px` architectural hairlines (`{d_bdr}`), and surface luminance contrast (`{d_bg}` vs `{d_surf}`).\n\
3. **Ban Cliché AI Gradients & Floating Junk**: Never use generic purple-to-blue SaaS gradients, random floating neon blobs, or unrequested decorative clutter. Use tinted neutrals and purposeful brand accents.\n\
4. **High-Contrast Legibility (`/colorize`)**: Never place low-contrast gray text on colored, gradient, or translucent backgrounds. Left-align body paragraphs and data tables.\n\
5. **Authentic Domain Copy**: Never use AI filler copy (\"Revolutionize your workflow\", \"Lorem ipsum\") or mismatched sections (never add SaaS pricing tables to open-source tools).\n\
6. **Physical Motion Discipline**: Use crisp spring physics (`stiffness: 320, damping: 28`) or exponential ease-out (`cubic-bezier(0.22, 1, 0.36, 1)`). Never use dated bounce or elastic easing.\n\n\
### Project-Specific Rules & Guardrails\n\
**Do Rules**:\n\
{do_rules}\n\n\
**Mandatory Inclusions**:\n\
{inclusions}\n\n\
**Forbidden Treatments (Never Do)**:\n\
{forbidden}\n",
        project_name = project_name,
        brand_version = brand_version,
        project_description = project_description,
        project_purpose = project_purpose,
        project_industry = project_industry,
        target_audience = target_audience,
        audience_needs = audience_needs,
        desired_user_action = desired_user_action,
        style_title = style_title,
        style_def = style_def,
        style_rules = style_rules,
        style_modifiers = style_modifiers,
        brand_personality = brand.brand_personality,
        design_direction = brand.design_direction,
        tone = brand.tone_of_voice,
        density = brand.ui_density,
        lang = brand.content_language,
        d_pri = d.primary,
        l_pri = l.primary,
        d_acc = d.accent,
        l_acc = l.accent,
        d_bg = d.background,
        l_bg = l.background,
        d_surf = d.surface,
        l_surf = l.surface,
        d_txt = d.text,
        l_txt = l.text,
        d_muted = d.muted_text,
        l_muted = l.muted_text,
        d_bdr = d.border,
        l_bdr = l.border,
        d_suc = d.success,
        l_suc = l.success,
        d_warn = d.warning,
        l_warn = l.warning,
        d_err = d.danger,
        l_err = l.danger,
        h_font = t.heading_family,
        b_font = t.body_family,
        m_font = t.mono_family,
        fallbacks = t.fallback_stack,
        scale = t.size_scale,
        lh = t.line_heights,
        f_source = t.font_source,
        spacing = lt.spacing_scale,
        c_width = lt.content_width,
        grid = lt.grid_columns,
        radii = lt.radii,
        borders = lt.borders,
        shadows = lt.shadows,
        icons = lt.icon_style,
        buttons = lt.button_treatment,
        do_rules = do_rules,
        inclusions = inclusions,
        forbidden = forbidden,
    )
}

pub async fn sync_all_projects_organized_folders(
    pool: &SqlitePool,
    config: &AppConfig,
) -> AppResult<()> {
    let rows = sqlx::query("SELECT id FROM projects WHERE archive_status = 'active'")
        .fetch_all(pool)
        .await?;
    for r in rows {
        let pid: String = r.get("id");
        let _ = sync_project_organized_folders(pool, config, &pid).await;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{detect_product_archetype, BrandSpec, ProductArchetype};

    #[test]
    fn test_product_archetype_detection_and_pure_human_prose_handoff() {
        let game_arch = detect_product_archetype(
            "Aether Chronicles",
            "Fantasy action RPG game interface with health bar, minimap, and inventory slots",
            "RPG Game HUD & Inventory UI",
            "Gaming / Unreal Engine",
            "full_page",
            "page_view",
            "Design the main game HUD and inventory screen",
        );
        assert_eq!(game_arch, ProductArchetype::GameUiInterface);

        let multipage_arch = detect_product_archetype(
            "PulseCloud",
            "Web application with overview dashboard, analytics page, billing page, and settings route",
            "Multi-page SaaS Analytics Webapp",
            "B2B Software",
            "full_page",
            "page_view",
            "Generate the full multi-page webapp frontend",
        );
        assert_eq!(multipage_arch, ProductArchetype::MultiPageWebapp);

        let landing_arch = detect_product_archetype(
            "NovaLaunch",
            "High-converting single landing page with hero, features, and pricing",
            "Marketing Landing Page",
            "AI Productivity",
            "full_page",
            "page_view",
            "Create a landing page for our AI launch",
        );
        assert_eq!(landing_arch, ProductArchetype::LandingPage);

        let brand = BrandSpec::default();
        let mandate = build_placeholder_and_card_mandate(&brand, None, None, game_arch);
        assert!(
            !mandate.contains('`'),
            "Mandate must not contain backtick code snippets"
        );
        assert!(
            !mandate.contains("<Navbar"),
            "Mandate must not contain JSX tags"
        );
        assert!(
            mandate.contains("GAME UI") && mandate.contains("placeholder"),
            "Game UI mandate must explicitly forbid inventing game characters and mark them as placeholders"
        );
        assert!(
            mandate.contains("3D Objects") && mandate.contains("rotate"),
            "Mandate must specify 3D object construction and rotation/animation"
        );

        let chroma = build_chroma_and_asset_cropping_spec(&brand, None, None, None, None, &[]);
        assert!(
            !chroma.contains('`'),
            "Chroma/asset spec must not contain backtick code snippets"
        );
        assert!(
            !chroma.contains("<svg"),
            "Chroma/asset spec must not contain HTML/SVG tags"
        );
        assert!(
            chroma.contains("3D OBJECT") && chroma.contains("rotates"),
            "Chroma/asset spec must specify 3D object construction and rotation"
        );

        let motion = build_animations_and_motion_spec(&brand, None, None);
        assert!(
            !motion.contains('`'),
            "Motion spec must not contain backtick code snippets"
        );
        assert!(
            !motion.contains("<AnimatePresence"),
            "Motion spec must not contain JSX tags"
        );
        assert!(
            motion.contains("3D OBJECT") && motion.contains("rotation"),
            "Motion spec must describe 3D object rotation and component animation in pure human language"
        );

        let cleaned = clean_code_syntax_to_human_prose(
            "Use `<Navbar />` inside `<AppShell>` with `<AnimatePresence mode=\"wait\">` and `<svg>`.",
        );
        assert!(!cleaned.contains('`'));
        assert!(!cleaned.contains('<'));
        assert!(!cleaned.contains('>'));
    }
}

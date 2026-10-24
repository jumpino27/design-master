use crate::{
    context_builder::BuiltGenerationContext,
    models::{PlanItem, PlannerPlan},
};

pub const IMAGE_COMPILER_TEMPLATE_V1: &str = include_str!("../../../prompts/v1/image_compiler.txt");

/// Determines whether a planned output should be rendered with `background: "transparent"`
/// on `openai/gpt-image-2.5-sunburst`.
pub fn should_use_transparent_background(ctx: &BuiltGenerationContext, item: &PlanItem) -> bool {
    if item.transparent_background || item.extracted_from_item_id.is_some() {
        return true;
    }

    let is_asset = ctx.mode == "asset" || item.kind == "asset";
    if !is_asset {
        return false;
    }

    if let Some(ref opts) = ctx.model_settings.asset_options {
        if opts.seamless_tiling
            || opts.intended_usage == "texture"
            || ctx.model_settings.scope_preset == "seamless_pattern"
        {
            return false;
        }
        if opts.transparent_background {
            return true;
        }
    }

    let combined_text = format!(
        "{} {} {} {}",
        item.id, item.name, item.direction, item.image_prompt
    )
    .to_ascii_lowercase();

    let is_full_bleed_wallpaper = (combined_text.contains("hero background")
        || combined_text.contains("full-bleed background")
        || combined_text.contains("seamless texture")
        || combined_text.contains("wallpaper"))
        && !combined_text.contains("transparent")
        && !combined_text.contains("icon")
        && !combined_text.contains("cutout")
        && !combined_text.contains("illustration")
        && !combined_text.contains("object")
        && !combined_text.contains("badge");

    !is_full_bleed_wallpaper
}

pub fn compile_image_prompt(
    ctx: &BuiltGenerationContext,
    plan: &PlannerPlan,
    item: &PlanItem,
) -> String {
    let is_full_page_layout = ctx.mode == "full_page" && item.kind != "asset"
        || item.kind == "page"
        || item.name.to_ascii_lowercase().contains("landing page")
        || item.name.to_ascii_lowercase().contains("full website")
        || item.name.to_ascii_lowercase().contains("homepage");

    let scope_constraints = match ctx.mode.as_str() {
        "component" => match ctx.model_settings.scope_preset.as_str() {
            "isolated_component" => "Isolated UI component centered cleanly on a neutral surface. Do NOT generate an entire landing page, navbar, or footer around it.",
            "component_in_context" | "component_with_context" => "UI component shown in its immediate section context with realistic surrounding spacing.",
            "section" => "Full-width page section with complete internal layout and hierarchy.",
            "modal" => "Focused modal dialog overlay with clear header, body controls, and footer actions.",
            "navbar" => "Horizontal navigation bar showing brand mark, navigation links, and primary CTA.",
            "full_page" => "VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NOT A HORIZONTAL POWERPOINT SLIDE): Complete multi-section web page flowing vertically from the top navigation bar down through the hero, features, pricing/content sections, to the footer at the bottom.",
            _ => "Focused UI component reference.",
        },
        "full_page" => match item.kind.as_str() {
            "page" => "VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NOT A HORIZONTAL POWERPOINT SLIDE): Complete coordinated web page flowing vertically from top to bottom (Navbar at top -> Hero -> Features -> Pricing -> Footer at bottom) with internal column grids and dimensions strictly matching the target viewport breakpoint.",
            "section" => "Dedicated high-detail section view belonging to the coordinated page set.",
            "component" => "Canonical reusable UI component belonging to the coordinated design system.",
            "state" => "Interactive state/modal view belonging to the coordinated design system.",
            "asset" => "Standalone visual site asset (icon sheet, illustration, 3D cutout, or foreground graphic) rendered on a 100% native transparent background so it can be layered directly into the coordinated page/component set.",
            _ => "Coordinated design system output.",
        },
        "asset" => "Standalone website visual asset (icon, spot illustration, hero cutout, badge, or pattern — NOT a UI screen or webpage mockup).",
        _ => "Design reference output.",
    };

    let include_logo = if ctx.mode == "asset" {
        ctx.model_settings
            .asset_options
            .as_ref()
            .map(|o| o.should_include_logo())
            .unwrap_or(false)
    } else {
        ctx.brand_snapshot.logo.inclusion_policy != "never"
    };

    let logo_instruction = if !include_logo {
        "DO NOT include any brand logo, wordmark, or watermark in this image.".to_string()
    } else {
        match ctx.brand_snapshot.logo.mode.as_str() {
            "uploaded" => format!(
                "Include the provided brand logo faithfully ({}). Preserve its exact geometry and proportions.",
                ctx.brand_snapshot.logo.preferred_usage
            ),
            "temporary_monogram" => format!(
                "Use a clean temporary monogram '{}' integrated with the brand name '{}' ({}). Do not invent an unrelated complex emblem.",
                ctx.brand_snapshot
                    .logo
                    .monogram_text
                    .as_deref()
                    .unwrap_or("D"),
                ctx.project_meta.name,
                ctx.brand_snapshot.logo.preferred_usage
            ),
            _ => "No logo mark yet; use clean typographic wordmark only if appropriate.".to_string(),
        }
    };

    let req_map: std::collections::HashMap<&str, &str> = plan
        .requirements
        .iter()
        .map(|r| (r.id.as_str(), r.description.as_str()))
        .collect();

    let covered_reqs: Vec<String> = item
        .requirements_covered
        .iter()
        .map(|id| {
            if let Some(desc) = req_map.get(id.as_str()) {
                format!("- [{}] {}", id, desc)
            } else {
                format!("- [{}]", id)
            }
        })
        .collect();

    let mut ref_roles = Vec::new();
    let has_current_screenshot = ctx.current_screenshot.is_some();
    if let Some(ref cs) = ctx.current_screenshot {
        if item.reference_asset_ids.contains(&cs.asset_id)
            || ctx.mode == "component"
            || item.kind == "component"
        {
            ref_roles.push(format!(
                "- Current UI Screenshot ({}) [POLISH, ENHANCE & EXTEND MODE]: Inspect this screenshot to see all buttons, links, icons, inputs, and controls that currently exist, and make a BETTER version! Keep the existing controls, integrate all NEW buttons, new functionality, or layout changes requested by the user, and use text labels, icons ({}), or icon+text combinations matching the Chosen Web Design Style and user instructions (do NOT copy outdated/unbranded colors from the old screenshot).",
                cs.label, ctx.brand_snapshot.layout_tokens.icon_style
            ));
        }
    }
    if let Some(ref prev) = ctx.selected_previous_output {
        if item.reference_asset_ids.contains(&prev.asset_id) {
            ref_roles.push(format!(
                "- Selected Previous Output ({}) [ITERATIVE POLISH, ENHANCE & EXTEND]: See what buttons and controls this reference has, incorporate all newly requested buttons/features/layout changes from the user's prompt, and polish the component with text/icons ({}) strictly within the Project Brand Kit and Chosen Web Design Style.",
                prev.label, ctx.brand_snapshot.layout_tokens.icon_style
            ));
        }
    }
    if !has_current_screenshot
        && ctx.selected_previous_output.is_none()
        && (ctx.mode == "component" || item.kind == "component")
    {
        ref_roles.push(format!(
            "- Workflow Mode [NEW COMPONENT FROM SCRATCH]: No old/current component screenshot was provided. Generate a brand-new component from scratch based on the user's prompt, designing the appropriate buttons, controls, text labels, and icons ({}) strictly in the Project Brand Kit and Chosen Web Design Style.",
            ctx.brand_snapshot.layout_tokens.icon_style
        ));
    }
    for att in &ctx.explicit_attachments {
        if item.reference_asset_ids.contains(&att.asset_id) {
            ref_roles.push(format!(
                "- {} ({}): {} (preserve_as_is={})",
                att.role.as_str(),
                att.label,
                att.placement_notes,
                att.preserve_as_is
            ));
        }
    }

    let chosen_style_block = if let Some(ref ds) = ctx.brand_snapshot.design_style {
        if !ds.style_name.trim().is_empty() {
            format!(
                "- Primary Web Design Style: #{} {}\n\
- Exact Style Definition: {}\n\
- Mandatory Visual & CSS System Rules: {}\n\
- Combinable Layout / System Modifiers: {}\n\
- Combined Style Brief: {}",
                if ds.style_number > 0 {
                    ds.style_number
                } else {
                    1
                },
                ds.style_name,
                ds.definition,
                ds.visual_rules,
                if ds.modifiers.is_empty() {
                    "None".to_string()
                } else {
                    ds.modifiers.join(" + ")
                },
                ds.combined_brief
            )
        } else {
            format!(
                "- Primary Web Design Style: {} ({})",
                ctx.brand_snapshot.brand_personality, ctx.brand_snapshot.design_direction
            )
        }
    } else {
        format!(
            "- Primary Web Design Style: {} ({})",
            ctx.brand_snapshot.brand_personality, ctx.brand_snapshot.design_direction
        )
    };

    let use_transparent_bg = should_use_transparent_background(ctx, item);
    let is_extracted_foreground_asset = item.extracted_from_item_id.is_some()
        || (item.kind == "asset" && !item.depends_on.is_empty());
    let is_dependent_multi_page_view =
        !is_extracted_foreground_asset && item.kind != "asset" && !item.depends_on.is_empty();

    if is_extracted_foreground_asset {
        ref_roles.push(
            "- Prerequisite Hero/Component Reference Image [FOREGROUND ASSET EXTRACTION]: Extract and faithfully reproduce ONLY the specified foreground illustration, 3D object, icon set, or graphic element from this reference UI image onto a 100% transparent background (`background: transparent`). Strip away all background walls, gradients, UI buttons, and text!".to_string()
        );
    } else if is_dependent_multi_page_view {
        let anchor_desc = item
            .depends_on
            .iter()
            .filter_map(|dep_id| plan.items.iter().find(|p| &p.id == dep_id))
            .map(|p| {
                format!(
                    "`{}` ('{}', Route `{}`)",
                    p.id,
                    p.name,
                    p.page_route.as_deref().unwrap_or("/")
                )
            })
            .collect::<Vec<_>>()
            .join(", ");
        let current_route = item.page_route.as_deref().unwrap_or("/page");
        ref_roles.push(format!(
            "- Attached Visual Anchor Page Reference ({}) [IMAGE-TO-IMAGE MULTI-PAGE CONTINUITY]: This attached image is Page 1 (the Visual Anchor) of the same multi-page website/webapp. Use Image-to-Image conditioning to keep the EXACT SAME sticky top navigation bar (identical brand logo & wordmark on the left, identical horizontal nav link typography/spacing in the center with the active pill/underline indicator moved to route `{}`, identical CTA button on the right, identical header height, surface color, and border), the EXACT SAME footer chrome, and the EXACT SAME brand color palette, typography, border-radius, card borders, and shadow depth, while rendering the completely new page content and layout for '{}' (Route `{}`) below the navbar!",
            if anchor_desc.is_empty() { "`item_1`".to_string() } else { anchor_desc },
            current_route,
            item.name,
            current_route
        ));
    }
    if let Some(ref existing_out_id) = item
        .target_existing_output_id
        .as_ref()
        .or(ctx.model_settings.target_existing_output_id.as_ref())
    {
        ref_roles.push(format!(
            "- Existing Canvas Tab Reference (`{}`) [TAB EDIT / VIEWPORT ADAPTATION]: Use this existing tab output as the baseline reference. Keep its core identity and unaffected elements intact while applying the requested edit or viewport adaptation (`{}`).",
            existing_out_id, item.viewport.intent
        ));
    }

    let invariant_constraints = if ctx.mode == "asset" || item.kind == "asset" || use_transparent_bg
    {
        let mut lines = vec![
            "- Render a clean, production-ready standalone website asset matching the chosen Web Design Style above.".to_string(),
            "- Do NOT wrap the asset in browser chrome, UI buttons, or device frames.".to_string(),
        ];
        if use_transparent_bg {
            lines.push(
                "- NATIVE TRANSPARENT BACKGROUND (`background: \"transparent\"`): Isolate the subject (or organized icon sheet grid) on a 100% true transparent alpha background. Do NOT draw a fake checkerboard grid, do NOT add opaque background fills or vignettes, and keep clean alpha edges with a 10% safety margin around the silhouette.".to_string()
            );
        }
        if is_extracted_foreground_asset {
            lines.push(
                "- FOREGROUND EXTRACTION FROM COMPOSITE UI: Reproduce the exact foreground visual asset shown in the attached prerequisite Hero/Component reference image at full resolution on a transparent background, completely omitting the background environment, headlines, and UI buttons so this transparent PNG can be layered over the coded background.".to_string()
            );
        }
        if let Some(ref opts) = ctx.model_settings.asset_options {
            lines.push(format!("- Intended Usage: {}", opts.intended_usage));
            lines.push(format!("- Theme Context: {}", opts.theme_context));
            if !opts.composition.is_empty() {
                lines.push(format!("- Composition: {}", opts.composition));
            }
            if !opts.negative_space.is_empty() {
                lines.push(format!("- Negative Space: {}", opts.negative_space));
            }
            if !opts.forbidden_text.is_empty() {
                lines.push(format!("- Forbidden Text: {}", opts.forbidden_text));
            }
            if opts.seamless_tiling {
                lines.push("- Seamless Tiling REQUIRED: top/bottom and left/right edges must repeat continuously with zero visible seam.".to_string());
            }
        }
        lines.join("\n")
    } else {
        let mut lines = vec![
            "- Direct, full-bleed viewport UI design reference (do NOT place the UI inside an angled laptop/phone hardware mockup or external desk scene, and do NOT include a browser URL bar).".to_string(),
            "- Strictly execute the Chosen Web Design Style definition and visual rules above in every surface, border, shadow, color contrast, and typographic detail.".to_string(),
            "- Render crisp, legible typography, accurate button alignment, accessible contrast, and realistic spacing.".to_string(),
        ];
        if is_full_page_layout {
            lines.push(
                "- VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NEVER A HORIZONTAL POWERPOINT SLIDE): Render the entire page flowing vertically from top to bottom — starting with the top navigation bar at the very top edge, followed vertically by the Hero section, Feature sections, Pricing/Content sections, and ending with the Footer at the bottom edge.".to_string()
            );
        }
        match item.viewport.intent.as_str() {
            "mobile" => {
                lines.push(
                    "- INTERNAL BREAKPOINT DIMENSIONS = MOBILE (`390px` SMARTPHONE VIEWPORT): Inside this vertical image, strictly use `390px` Mobile breakpoint proportions — compact mobile top header with hamburger menu icon, single-column vertically stacked sections and cards, mobile typography scale, and full-width touch-friendly CTA buttons (`min-height: 48px`). Do NOT render wide multi-column desktop grids!".to_string()
                );
            }
            "tablet" => {
                lines.push(
                    "- INTERNAL BREAKPOINT DIMENSIONS = TABLET (`834px` TABLET VIEWPORT): Inside this image, strictly use `834px` Tablet breakpoint proportions — adapted tablet navigation bar, 2-column balanced card grids, and touch-friendly spacing.".to_string()
                );
            }
            _ => {
                if is_full_page_layout {
                    lines.push(
                        "- INTERNAL BREAKPOINT DIMENSIONS = DESKTOP (`1440px` WIDE DESKTOP GRID INSIDE VERTICAL FULL-PAGE SCROLL): Even though the image canvas is vertical (`2:3` / `9:16`) so the whole landing page is visible from top to bottom, the UI INSIDE the image MUST be a true `1440px`-wide Desktop layout — full horizontal desktop navbar (brand logo left, horizontal nav links center, CTA buttons right — NO mobile hamburger menu), 12-column desktop container with generous left/right desktop margins, 2-column desktop Hero, and 3-to-4 column side-by-side desktop Feature & Pricing cards in horizontal rows! Do NOT render a narrow single-column mobile phone layout!".to_string()
                    );
                } else {
                    lines.push(
                        "- INTERNAL BREAKPOINT DIMENSIONS = DESKTOP (`1440px` DESKTOP VIEWPORT): Render with authentic `1440px` desktop container proportions, horizontal desktop alignment, and desktop typography scale.".to_string()
                    );
                }
            }
        }
        if !ctx.model_settings.highlight_annotations.is_empty() {
            for h in &ctx.model_settings.highlight_annotations {
                lines.push(format!(
                    "- TARGETED REGION HIGHLIGHT EDIT (`quality: medium`): In the bounding box region [x={:.1}%, y={:.1}%, width={:.1}%, height={:.1}%], apply this exact change: \"{}\". Keep all other areas of the reference image consistent!",
                    h.x_pct, h.y_pct, h.w_pct, h.h_pct, h.comment
                ));
            }
        } else if has_current_screenshot
            || item.target_existing_output_id.is_some()
            || ctx.model_settings.target_existing_output_id.is_some()
        {
            lines.push("- POLISH, ENHANCE & EXTEND EXISTING COMPONENT: See what buttons, links, controls, and inputs the attached reference UI has and make a BETTER version! Keep the existing actions AND seamlessly add any NEW buttons, new functionality, or layout changes requested in the user's prompt — rendering controls with text labels, icons, or icon+text combinations matching the Chosen Web Design Style and user instructions.".to_string());
        } else if ctx.mode == "component" || item.kind == "component" {
            lines.push("- GENERATE COMPONENT FROM SCRATCH: Because no old screenshot is attached, design this component fresh from the user's prompt and requirements (using text labels, icons, or icon+text appropriate to the style and prompt) while strictly obeying the Project Brand Kit and Chosen Web Design Style above.".to_string());
        }
        if ctx.mode == "full_page" {
            if is_dependent_multi_page_view {
                let current_route = item.page_route.as_deref().unwrap_or("/page");
                lines.push(format!(
                    "- IMAGE-TO-IMAGE MULTI-PAGE CONSISTENCY WITH ATTACHED VISUAL ANCHOR PAGE (`item_1`): \
                    1) IDENTICAL STICKY TOP NAVBAR & FOOTER: Render the exact same top navigation bar at the top edge as shown in the attached Page 1 reference image (same logo mark & wordmark on the left, same nav link items in the center with the active state on `{}`, same right-side action button, same navbar height, background surface, and border) and the same footer at the bottom edge. \
                    2) DISTINCT PAGE BODY FOR '{}' (Route `{}`): Below the shared top navbar, do NOT repeat Page 1's hero section! Instead, render the complete top-to-bottom UI specifically for '{}' ({}) — showing every section, dashboard KPI tile, chart, filter bar, directory grid, form control, or table specified in the prompt above.",
                    current_route, item.name, current_route, item.name, item.direction
                ));
            } else {
                lines.push("- VISUAL ANCHOR PAGE (`item_1`): Establish the canonical sticky top navigation bar (brand logo left, clear route links for all pages of the webapp center, primary CTA right), brand surface palette, typography, border radius, shadow depth, and footer chrome that all subsequent pages in this multi-page set will inherit via Image-to-Image conditioning.".to_string());
            }
        }
        lines.join("\n")
    };

    IMAGE_COMPILER_TEMPLATE_V1
        .replace("{{mode}}", &ctx.mode)
        .replace("{{item_kind}}", &item.kind)
        .replace("{{item_name}}", &item.name)
        .replace("{{variant_index}}", &item.variant_index.to_string())
        .replace("{{viewport_intent}}", &item.viewport.intent)
        .replace("{{css_width}}", &item.viewport.css_width.to_string())
        .replace("{{aspect_ratio}}", &item.aspect_ratio)
        .replace("{{scope_constraints}}", scope_constraints)
        .replace("{{project_name}}", &ctx.project_meta.name)
        .replace("{{project_purpose}}", &ctx.project_meta.purpose)
        .replace("{{project_description}}", &ctx.project_meta.description)
        .replace("{{project_industry}}", &ctx.project_meta.industry)
        .replace("{{target_audience}}", &ctx.project_meta.target_audience)
        .replace("{{audience_needs}}", &ctx.project_meta.audience_needs)
        .replace(
            "{{desired_user_action}}",
            &ctx.project_meta.desired_user_action,
        )
        .replace("{{chosen_design_style_block}}", &chosen_style_block)
        .replace(
            "{{brand_direction}}",
            &format!(
                "{} — {}",
                ctx.brand_snapshot.brand_personality, ctx.brand_snapshot.design_direction
            ),
        )
        .replace("{{ui_density}}", &ctx.brand_snapshot.ui_density)
        .replace("{{tone_of_voice}}", &ctx.brand_snapshot.tone_of_voice)
        .replace("{{content_language}}", &ctx.brand_snapshot.content_language)
        .replace(
            "{{color_primary}}",
            &ctx.brand_snapshot.colors.light.primary,
        )
        .replace("{{color_accent}}", &ctx.brand_snapshot.colors.light.accent)
        .replace(
            "{{color_background}}",
            &ctx.brand_snapshot.colors.light.background,
        )
        .replace(
            "{{color_surface}}",
            &ctx.brand_snapshot.colors.light.surface,
        )
        .replace("{{color_text}}", &ctx.brand_snapshot.colors.light.text)
        .replace(
            "{{color_muted}}",
            &ctx.brand_snapshot.colors.light.muted_text,
        )
        .replace("{{color_border}}", &ctx.brand_snapshot.colors.light.border)
        .replace(
            "{{heading_font}}",
            &ctx.brand_snapshot.typography.heading_family,
        )
        .replace("{{body_font}}", &ctx.brand_snapshot.typography.body_family)
        .replace("{{mono_font}}", &ctx.brand_snapshot.typography.mono_family)
        .replace("{{border_radius}}", &ctx.brand_snapshot.layout_tokens.radii)
        .replace(
            "{{border_style}}",
            &ctx.brand_snapshot.layout_tokens.borders,
        )
        .replace(
            "{{shadow_style}}",
            &ctx.brand_snapshot.layout_tokens.shadows,
        )
        .replace("{{logo_instruction}}", &logo_instruction)
        .replace("{{direction_summary}}", &item.direction)
        .replace("{{planner_image_prompt}}", &item.image_prompt)
        .replace(
            "{{requirements_covered}}",
            &if covered_reqs.is_empty() {
                "- Satisfy user request".to_string()
            } else {
                covered_reqs.join("\n")
            },
        )
        .replace(
            "{{exact_copy}}",
            &if item.exact_copy.is_empty() {
                "Use concise, realistic copy matching brand language.".to_string()
            } else {
                item.exact_copy.join(" | ")
            },
        )
        .replace(
            "{{reference_roles_summary}}",
            &if ref_roles.is_empty() {
                "None attached.".to_string()
            } else {
                ref_roles.join("\n")
            },
        )
        .replace("{{invariant_constraints}}", &invariant_constraints)
}

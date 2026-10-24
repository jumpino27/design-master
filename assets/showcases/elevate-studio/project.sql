INSERT INTO projects (id, name, description, purpose, industry, target_audience, audience_needs, desired_user_action, archive_status, created_at, updated_at, is_example) VALUES ('example-elevate-studio', 'Elevate Studio', 'Tagline: Digital Experiences That Make an Impact.

A professional website for a creative agency offering web design, branding, and digital strategy. Includes dedicated Home, About, Services, Portfolio, and Contact pages, with cohesive branding and engaging layouts.', 'A professional multi-page website for Elevate Studio, a creative agency offering web design, branding, and digital strategy, designed to convert visitors into consultation bookings.', 'Creative Agency / Digital Design Services', 'Startup founders, marketing leads, and business owners seeking premium web design, branding, and digital strategy services', 'A credible, polished showcase of agency work, clear service explanations, proof of results, and a frictionless way to start a project conversation', 'Book a discovery call or submit a project inquiry via the Contact page', 'active', '2026-10-24T19:23:43.513169+00:00', '2026-10-24T19:23:43.513169+00:00', 1);
INSERT INTO brand_versions (id, project_id, version, brand_json, created_at) VALUES ('example-brand-elevate-studio-1', 'example-elevate-studio', 1, '{"brand_personality": "Confident, refined, and quietly bold — a studio that lets its work speak through restraint and precision", "design_direction": "A minimalist, editorial-driven site with generous whitespace, razor-thin hairline borders, and a single confident accent color. Large expressive headlines, spacious 48px–96px section padding, and a sticky top navigation guide visitors through Home, About, Services, Portfolio, and Contact. Portfolio work is showcased in a clean interactive grid; services are presented as focused benefit cards; the contact flow is a simple, elegant conversion form.", "tone_of_voice": "Clear, assured, and impact-focused — concise statements that promise measurable results without hype", "content_language": "English", "ui_density": "comfortable", "logo": {"mode": "temporary_monogram", "monogram_text": "E", "primary_asset_id": null, "mark_only_asset_id": null, "light_variant_asset_id": null, "dark_variant_asset_id": null, "preferred_usage": "Top-left header or card header mark with clear breathing room", "inclusion_policy": "allowed"}, "colors": {"light": {"primary": "#1A1A2E", "accent": "#E85D2A", "background": "#FAFAF8", "surface": "#FFFFFF", "text": "#16161F", "muted_text": "#6B6B76", "border": "#E4E4DE", "success": "#2B7A4B", "warning": "#B7791F", "danger": "#C53030"}, "dark": {"primary": "#F5F5F0", "accent": "#FF7A45", "background": "#101018", "surface": "#1A1A26", "text": "#F2F2EE", "muted_text": "#9A9AA6", "border": "#2C2C3A", "success": "#38A169", "warning": "#D69E2E", "danger": "#E53E3E"}}, "typography": {"heading_family": "Sora (or similar geometric sans) — 48px/56px hero, 24px section headings, tight tracking", "body_family": "Inter — 15px–16px body, 1.6 line height, generous paragraph spacing", "mono_family": "JetBrains Mono — for labels, indices, and technical metadata", "fallback_stack": "-apple-system, BlinkMacSystemFont, ''Segoe UI'', sans-serif", "weights": [400, 500, 600, 700], "size_scale": "12px / 14px / 16px / 20px / 28px / 40px", "line_heights": "Heading 1.15, Body 1.5, Compact UI 1.3", "font_source": "google_fonts", "uploaded_font_asset_ids": []}, "layout_tokens": {"spacing_scale": "4px / 8px / 16px / 24px / 32px / 48px / 64px / 96px", "content_width": "1200px max container with 24px padding", "grid_columns": "12-column responsive grid, 24px gap", "radii": "12px cards, 8px buttons, 999px pills", "borders": "1px solid subtle warm border (#E2DED5)", "shadows": "0 8px 24px rgba(0,0,0,0.06), hover: 0 12px 32px rgba(0,0,0,0.10)", "icon_style": "2px crisp geometric stroke icons", "button_treatment": "Solid high-contrast primary CTA with subtle arrow affordance"}, "brand_rules": ["Enforce 60%+ open whitespace with 48px–96px section padding", "Use razor-thin 1px hairline borders instead of heavy dividers", "Restrict the accent color to CTAs, key highlights, and hover states — never large fills", "Pair restrained neutral surfaces with one bold accent for focus", "Zero decorative clutter; every element must serve hierarchy or conversion", "Portfolio imagery framed in clean aspect-ratio grids with restrained overlay typography", "Sticky top navigation with a single high-contrast CTA button"], "mandatory_inclusions": [], "forbidden_treatments": ["No gradients on text or buttons", "No neon glows or heavy drop shadows", "No more than two typeface families", "No decorative illustrations competing with portfolio work", "No cluttered multi-column footers or dense card walls", "No autoplaying video or aggressive motion"], "approved_reference_asset_ids": [], "target_context": {"target_stack": "Next.js App Router + TypeScript + Tailwind CSS", "repository_root": "", "preferred_asset_dir": "public/assets", "component_library_notes": "Accessible Radix primitives with Lucide icons"}, "design_style": {"style_id": "01-minimalism", "style_number": 1, "style_name": "Minimalism", "definition": "Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content.", "category": "Minimal & Editorial", "visual_rules": "Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).", "keywords": ["minimalism", "minimal", "minimalist", "whitespace", "negative space", "clean", "restrained", "simple", "essential", "monochrome", "zen", "airy", "swiss"], "modifiers": [], "combined_brief": "Style brief: \"minimalism\". Primary style #1 Minimalism (Minimal & Editorial): Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm)."}}', '2026-10-24T19:23:43.513169+00:00');
INSERT INTO threads (id, project_id, title, archive_status, created_at, updated_at) VALUES ('example-thread-elevate-studio', 'example-elevate-studio', 'Main Design Workspace', 'active', '2026-10-24T19:23:43.513169+00:00', '2026-10-24T20:13:07.680267+00:00');
INSERT INTO runs (id, project_id, thread_id, idempotency_key, mode, scope_preset, user_prompt, brand_version, brand_snapshot_json, selected_references_json, model_settings_json, plan_json, consistency_report_json, status, estimated_cost_micros, actual_cost_micros, cost_is_unknown, is_test_adapter, error_message, created_at, updated_at, completed_at, target_tab_key, target_tab_label, target_output_id, highlight_annotations_json) VALUES ('example-run-elevate-studio', 'example-elevate-studio', 'example-thread-elevate-studio', NULL, 'full_page', 'full_page', 'Generate a complete 5-page desktop website for ''Elevate Studio'', a creative agency (web design, branding, digital strategy), tagline ''Digital Experiences That Make an Impact''. LIGHT THEME ONLY: Background #FAFAF8, Primary text #1A1A2E, Accent #E85D2A, hairline borders rgba(26,26,46,0.12). Typography: Sora geometric sans for headings (48-56px hero, 24px section headings, tight tracking), Inter 15-16px body at 1.6 line height, JetBrains Mono for labels, indices, and technical metadata. Icon style: 2px crisp geometric stroke icons. Buttons: solid high-contrast primary CTA in accent orange with subtle arrow affordance. Style: #1 Minimalism — generous whitespace, 48px-96px section padding, razor-thin 1px hairline borders, asymmetric editorial layouts, restrained single accent color. MANDATE Impeccable Anti-AI-Slop Craft: no card-in-card nesting, no 3-column icon-in-circle grids, no purple-blue SaaS gradients, no floating neon blobs, no fake filler copy; one unmistakable primary focal point per page, clear secondary action, authentic agency copy. Generate these as separate page items in one run: PAGE 1 ''/'' Home — visual anchor with large expressive Sora headline, tagline, primary CTA ''Book a discovery call'', editorial intro section, selected work teaser grid, services index list with mono indices, client proof strip, footer. PAGE 2 ''/about'' About — studio story, team/philosophy section with editorial typography, values presented as a numbered mono-indexed list, stats with hairline dividers, CTA to contact. PAGE 3 ''/services'' Services — three focused benefit sections (Web Design, Branding, Digital Strategy) as asymmetric editorial blocks with hairline separators, process steps, CTA. PAGE 4 ''/portfolio'' Portfolio — clean interactive-feeling project grid with varied image aspect ratios, hover-state hints, project names + categories in mono labels, case-study CTA. PAGE 5 ''/contact'' Contact — simple elegant conversion form (name, email, project type, budget, message) with hairline input underlines, primary CTA ''Book a discovery call'', contact details sidebar, studio info. ALL PAGES 2-5 must use Image-to-Image conditioning on Page 1 to share the EXACT same sticky top navbar (logo ''Elevate Studio'', links Home/About/Services/Portfolio/Contact, accent CTA button) and identical footer, brand chrome, palette, and typography. Desktop 1440px-wide multi-column layouts rendered top-to-bottom in vertical frames.', 1, '{"brand_personality": "Confident, refined, and quietly bold — a studio that lets its work speak through restraint and precision", "design_direction": "A minimalist, editorial-driven site with generous whitespace, razor-thin hairline borders, and a single confident accent color. Large expressive headlines, spacious 48px–96px section padding, and a sticky top navigation guide visitors through Home, About, Services, Portfolio, and Contact. Portfolio work is showcased in a clean interactive grid; services are presented as focused benefit cards; the contact flow is a simple, elegant conversion form.", "tone_of_voice": "Clear, assured, and impact-focused — concise statements that promise measurable results without hype", "content_language": "English", "ui_density": "comfortable", "logo": {"mode": "temporary_monogram", "monogram_text": "E", "primary_asset_id": null, "mark_only_asset_id": null, "light_variant_asset_id": null, "dark_variant_asset_id": null, "preferred_usage": "Top-left header or card header mark with clear breathing room", "inclusion_policy": "allowed"}, "colors": {"light": {"primary": "#1A1A2E", "accent": "#E85D2A", "background": "#FAFAF8", "surface": "#FFFFFF", "text": "#16161F", "muted_text": "#6B6B76", "border": "#E4E4DE", "success": "#2B7A4B", "warning": "#B7791F", "danger": "#C53030"}, "dark": {"primary": "#F5F5F0", "accent": "#FF7A45", "background": "#101018", "surface": "#1A1A26", "text": "#F2F2EE", "muted_text": "#9A9AA6", "border": "#2C2C3A", "success": "#38A169", "warning": "#D69E2E", "danger": "#E53E3E"}}, "typography": {"heading_family": "Sora (or similar geometric sans) — 48px/56px hero, 24px section headings, tight tracking", "body_family": "Inter — 15px–16px body, 1.6 line height, generous paragraph spacing", "mono_family": "JetBrains Mono — for labels, indices, and technical metadata", "fallback_stack": "-apple-system, BlinkMacSystemFont, ''Segoe UI'', sans-serif", "weights": [400, 500, 600, 700], "size_scale": "12px / 14px / 16px / 20px / 28px / 40px", "line_heights": "Heading 1.15, Body 1.5, Compact UI 1.3", "font_source": "google_fonts", "uploaded_font_asset_ids": []}, "layout_tokens": {"spacing_scale": "4px / 8px / 16px / 24px / 32px / 48px / 64px / 96px", "content_width": "1200px max container with 24px padding", "grid_columns": "12-column responsive grid, 24px gap", "radii": "12px cards, 8px buttons, 999px pills", "borders": "1px solid subtle warm border (#E2DED5)", "shadows": "0 8px 24px rgba(0,0,0,0.06), hover: 0 12px 32px rgba(0,0,0,0.10)", "icon_style": "2px crisp geometric stroke icons", "button_treatment": "Solid high-contrast primary CTA with subtle arrow affordance"}, "brand_rules": ["Enforce 60%+ open whitespace with 48px–96px section padding", "Use razor-thin 1px hairline borders instead of heavy dividers", "Restrict the accent color to CTAs, key highlights, and hover states — never large fills", "Pair restrained neutral surfaces with one bold accent for focus", "Zero decorative clutter; every element must serve hierarchy or conversion", "Portfolio imagery framed in clean aspect-ratio grids with restrained overlay typography", "Sticky top navigation with a single high-contrast CTA button"], "mandatory_inclusions": [], "forbidden_treatments": ["No gradients on text or buttons", "No neon glows or heavy drop shadows", "No more than two typeface families", "No decorative illustrations competing with portfolio work", "No cluttered multi-column footers or dense card walls", "No autoplaying video or aggressive motion"], "approved_reference_asset_ids": [], "target_context": {"target_stack": "Next.js App Router + TypeScript + Tailwind CSS", "repository_root": "", "preferred_asset_dir": "public/assets", "component_library_notes": "Accessible Radix primitives with Lucide icons"}, "design_style": {"style_id": "01-minimalism", "style_number": 1, "style_name": "Minimalism", "definition": "Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content.", "category": "Minimal & Editorial", "visual_rules": "Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).", "keywords": ["minimalism", "minimal", "minimalist", "whitespace", "negative space", "clean", "restrained", "simple", "essential", "monochrome", "zen", "airy", "swiss"], "modifiers": [], "combined_brief": "Style brief: \"minimalism\". Primary style #1 Minimalism (Minimal & Editorial): Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm)."}}', '[]', '{"planner_model":"z-ai/glm-5.3-flash","image_model":"openai/gpt-image-2.5-sunburst","quality":"medium","aspect_ratio":"2:3","output_size":"provider_default","scope_preset":"full_page","custom_dimensions":null,"review_plan_first":false,"asset_options":null,"target_tab_key":"full-website","target_tab_label":"Full Website","target_existing_output_id":null,"viewport_intent":"desktop","highlight_annotations":[],"single_focused_output":false}', '{
  "schema_version": 1,
  "mode": "full_page",
  "request_summary": "Generate a complete 5-page desktop website for Elevate Studio, a creative agency (web design, branding, digital strategy), in Style #1 Minimalism with the user''s light-theme brand kit (#FAFAF8 background, #1A1A2E ink, #E85D2A accent, Sora/Inter/JetBrains Mono typography), razor-thin hairline borders, 48–96px section rhythm, asymmetric editorial layouts, and a shared sticky navbar and footer across Home, About, Services, Portfolio, and Contact pages.",
  "requirements": [
    {
      "id": "req_1",
      "description": "Five distinct desktop pages: Home (/), About (/about), Services (/services), Portfolio (/portfolio), Contact (/contact), each as a separate vertical 2:3 page item rendered at 1440px desktop width",
      "source": "user"
    },
    {
      "id": "req_2",
      "description": "Pages 2–5 use Image-to-Image conditioning on Page 1 to share the exact same sticky top navbar (Elevate Studio logo, Home/About/Services/Portfolio/Contact links, accent CTA button) and identical footer, palette, and typography",
      "source": "user"
    },
    {
      "id": "req_3",
      "description": "Light theme only: background #FAFAF8, primary text #1A1A2E, accent #E85D2A, hairline borders rgba(26,26,46,0.12); Sora headings, Inter body, JetBrains Mono labels",
      "source": "user"
    },
    {
      "id": "req_4",
      "description": "Minimalism style enforcement: 60%+ whitespace, 48–96px section padding, 1px hairline borders, single restrained accent, zero decorative clutter, asymmetric editorial layouts, no card-in-card nesting, no 3-column icon-in-circle grids, no gradients or neon",
      "source": "user"
    },
    {
      "id": "req_5",
      "description": "Primary conversion: Book a discovery call CTA and a frictionless contact form (name, email, project type, budget, message) with hairline input underlines",
      "source": "user"
    },
    {
      "id": "req_6",
      "description": "Authentic agency copy only — no AI slop filler phrases; one unmistakable primary focal point per page with a clear secondary action",
      "source": "user"
    }
  ],
  "assumptions": [
    "Temporary monogram logo (a simple geometric ''E'' mark plus ''Elevate Studio'' wordmark) is used per the brand kit logo policy",
    "Portfolio project imagery is placeholder content; only the grid frames, typography, and hover treatments are coded 1:1",
    "All pages are light theme; dark palette tokens are reserved for a future theme toggle and are not rendered in these images"
  ],
  "shared_design_spec": {
    "colors": {
      "primary": "#1A1A2E",
      "surface": "#FFFFFF",
      "accent": "#E85D2A",
      "text": "#16161F",
      "background": "#FAFAF8",
      "muted": "#6B6B76"
    },
    "typography": {
      "mono": "JetBrains Mono — 12–14px uppercase labels, section indices, and technical metadata",
      "body": "Inter — 15–16px body at 1.6 line height, left-aligned, generous paragraph spacing",
      "heading": "Sora geometric sans — 48–56px hero headlines with tight tracking, 24px section headings"
    },
    "spacing": {
      "base": "4 / 8 / 16 / 24 / 32 / 48 / 64 / 96 px scale with 48–96px vertical section rhythm"
    },
    "radii": {
      "button": "8px (999px for pills)",
      "card": "12px"
    },
    "shadows": {
      "card": "0 8px 24px rgba(0,0,0,0.06); hover 0 12px 32px rgba(0,0,0,0.10) — used sparingly, hairlines preferred over shadows"
    },
    "layout_rules": [
      "STICKY VS NON-STICKY: The top navigation bar stays anchored to the top of the viewport on every page while the rest of the page scrolls naturally in normal flow; the footer and all content sections scroll normally",
      "Enforce 60%+ open whitespace; use razor-thin 1px hairline borders in rgba(26,26,46,0.12) instead of heavy dividers or boxed cards",
      "Restrict the accent #E85D2A to primary CTAs, key highlights, and hover states — never large fills or gradients",
      "Every page has exactly one unmistakable primary visual anchor (large expressive Sora headline or hero statement) and one clear secondary action",
      "Asymmetric editorial split compositions (e.g., 5/7 or 4/8 column splits) instead of uniform 3-column card rows; no icon-in-circle grids",
      "Left-align all body copy and data; use tabular numerals for stats and metrics; mono uppercase labels above headings as indices",
      "Zero decorative clutter: no gradients on text or buttons, no neon glows, no floating blobs, no more than two typeface families",
      "Color rule ''border'': rgba(26,26,46,0.12)"
    ],
    "asset_rules": [
      "NATIVE TRANSPARENT PNG ASSETS: The brand monogram logo and the custom icon sheet are standalone assets with transparent_background=true so Sunburst 2.5 renders them on a 100% native transparent alpha background with zero UI chrome",
      "Icons are generated together in one organized 1:1 icon sheet with clean spacing for conversion into scalable SVG graphics",
      "Do NOT generate separate asset images for portfolio project photos, team portraits, avatars, or UI banners — these are placeholder content documented in implementation notes",
      "CODED SEPARATOR / BACKGROUND / 3D SPEC FROM IMAGE (Home — Visual Anchor Landing Page): Section separators: The page uses thin, solid, light-gray horizontal rule lines (approximately one pixel, a warm light gray around #E5E2DC) that span the full content width to divide every major section — below the header nav, between the hero and About, between About and Selected Work, between Selected Work and Services, between Services and the Trusted By logo strip, and above the footer. In the Services list, each service row (Web Design, Branding, Digital Strategy) is separated by the same thin light rule line, with the final row closed by a rule beneath it. Recreate all of these separators 1:1 in code as full-width hairline borders with identical color and thickness — no gradients, no glow.",
      "CODED SEPARATOR / BACKGROUND / 3D SPEC FROM IMAGE (Home — Visual Anchor Landing Page): Background layers: The entire page sits on a single warm off-white/cream canvas (approximately #F5F3EF) with no visible texture, grain, or grid. The header bar is the same cream tone, and the hero, About, Selected Work, Services, Trusted By, and CTA sections all share this continuous background with no color transitions between sections. The two hero CTAs and the CTA-section button use a solid burnt-orange fill (approximately #E05A1E) with white text and slightly rounded corners. Recreate this background 1:1 in code as a flat warm cream color with the orange accent buttons built in code — do not use a background screenshot.",
      "CODED SEPARATOR / BACKGROUND / 3D SPEC FROM IMAGE (Home — Visual Anchor Landing Page): Typography and layout rhythm as coded structure: The design relies on a monospace uppercase micro-label style (letter-spaced, small, e.g. ''CREATIVE STUDIO — EST. 2018'', ''ABOUT'', ''SELECTED WORK'', ''SERVICES'', ''TRUSTED BY TEAMS AT'') paired with a large dark charcoal serif-adjacent sans headline face. The hero right side features a large photo block with a vertical ''01 / 03'' slide counter and a small right-aligned monospace list (''STRATEGY / DESIGN / REAL RESULTS''). Recreate these labels, counters, and spacing rhythm exactly in code as text and layout, not images.",
      "CODED SEPARATOR / BACKGROUND / 3D SPEC FROM IMAGE (Home — Visual Anchor Landing Page): Animation note: No 3D objects or animated centerpiece props are visible in this image; the only motion-relevant elements are the hero image carousel counter (01/03) and arrow-link hover states, which should be implemented as simple code-driven transitions (arrow slide on hover, carousel counter update) rather than 3D reconstruction."
    ]
  },
  "items": [
    {
      "id": "item_1",
      "kind": "page",
      "name": "Home — Visual Anchor Landing Page",
      "tab_key": "full-website",
      "tab_label": "Full Website",
      "page_route": "/",
      "variant_index": 1,
      "viewport": {
        "intent": "desktop",
        "css_width": 1440
      },
      "aspect_ratio": "2:3",
      "transparent_background": false,
      "extracted_from_item_id": null,
      "target_existing_output_id": null,
      "quality_override": null,
      "requirements_covered": [
        "req_1",
        "req_2",
        "req_3",
        "req_4",
        "req_5",
        "req_6"
      ],
      "direction": "Minimalist editorial agency homepage on warm off-white #FAFAF8. One dominant oversized Sora headline as the primary anchor, a single accent-orange CTA, hairline-separated sections with 96px rhythm, asymmetric 5/7 splits, mono uppercase indices. Zero clutter, zero gradients.",
      "exact_copy": [
        "Elevate Studio",
        "Home",
        "About",
        "Services",
        "Portfolio",
        "Contact",
        "Book a discovery call",
        "Digital Experiences That Make an Impact",
        "We are a creative studio partnering with ambitious teams to design websites, brands, and digital strategies that perform — measured in outcomes, not adjectives.",
        "Selected Work",
        "View all projects",
        "Services",
        "01 — Web Design",
        "02 — Branding",
        "03 — Digital Strategy",
        "Trusted by teams at",
        "Northwind",
        "Fielder & Co.",
        "Meridian Labs",
        "Atlas Health",
        "Corvus",
        "Let''s build something that lasts.",
        "Book a discovery call",
        "hello@elevate.studio",
        "© 2025 Elevate Studio"
      ],
      "reference_asset_ids": [],
      "depends_on": [],
      "image_prompt": "A tall vertical full-page scroll capture of a 1440px-wide desktop website homepage for a minimalist creative agency, rendered top-to-bottom on a warm off-white background (#FAFAF8) with ink text (#1A1A2E) and a single burnt-orange accent (#E85D2A). Sticky top navigation bar: small geometric monogram logo and ''Elevate Studio'' wordmark on the left, five horizontal nav links (Home active with accent underline, About, Services, Portfolio, Contact) centered, and a solid burnt-orange rounded button ''Book a discovery call'' with a small right-arrow on the right; a 1px hairline border under the header. Hero section with 96px padding: a mono uppercase eyebrow label ''CREATIVE STUDIO — EST. 2018'', then an enormous 56px+ tight-tracked Sora headline ''Digital Experiences That Make an Impact'' spanning two lines as the page''s single dominant focal point, a short left-aligned Inter paragraph beneath, and the orange CTA button plus a quiet secondary text link ''View our work'' with an arrow. Editorial intro section split asymmetrically 5/7 columns: left a mono label ''ABOUT'', right a large 28px Sora statement paragraph. Selected Work section: a 2-column asymmetric portfolio teaser grid with two large placeholder image frames of different aspect ratios (one tall, one wide) framed by hairline borders, each with a mono category label and project name in Sora below; a ''View all projects'' text link. Services index: a vertical list of three rows separated by 1px hairlines, each row with a JetBrains Mono index (''01'', ''02'', ''03''), service name in large Sora, and a right-aligned arrow; no cards, no icons in circles. Client proof strip: a single hairline-bordered row with five muted wordmark texts spaced evenly. Footer: large Sora closing line ''Let''s build something that lasts.'', orange CTA button, email in mono, hairline top border, small copyright row. Generous 60%+ whitespace throughout, razor-thin hairlines instead of shadows, no gradients, no decorative blobs, no card-in-card nesting, authentic restrained agency aesthetic.",
      "implementation_notes": [
        "POSITIONING (STICKY VS NON-STICKY): The top navigation bar stays fixed to the top of the viewport on scroll with a solid #FAFAF8 surface and a 1px hairline bottom border; the hero, work grid, services index, proof strip, and footer all scroll naturally in normal document flow",
        "PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: The two portfolio teaser images and the five client wordmarks are placeholder sample content for the user''s future CMS/database; only the layout frames, hairline borders, typography, buttons, and grid structure are to be implemented 1:1 in code right now, with image slots wired to a data source later",
        "CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: All section separators, hairline rules, hover underline indicators, and the flat off-white background are built purely with code; the monogram logo and nav/CTA/UI icons come from the extracted transparent assets and must be converted into scalable vectorial SVG graphics; this page contains no 3D objects, so no Image-to-3D reconstruction is needed",
        "FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Page route transitions use a quick fade-and-slight-upward drift with exponential ease-out; the active nav link indicator slides smoothly between routes; sections reveal with a staggered fade-up on scroll; the CTA button uses a crisp spring press (stiffness 320, damping 28) with the arrow nudging right on hover; service index rows expand a hairline accent underline on hover; all motion respects the user''s reduced-motion preference by disabling transforms and using simple opacity fades",
        "REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No existing screenshot was attached, so build all components fresh; if a prior header or hero exists in the codebase, replace it to match this design 1:1 while preserving routes and data bindings",
        "CODED SEPARATOR / BACKGROUND SPEC: Section separators: The page uses thin, solid, light-gray horizontal rule lines (approximately one pixel, a warm light gray around #E5E2DC) that span the full content width to divide every major section — below the header nav, between the hero and About, between About and Selected Work, between Selected Work and Services, between Services and the Trusted By logo strip, and above the footer. In the Services list, each service row (Web Design, Branding, Digital Strategy) is separated by the same thin light rule line, with the final row closed by a rule beneath it. Recreate all of these separators 1:1 in code as full-width hairline borders with identical color and thickness — no gradients, no glow.",
        "CODED SEPARATOR / BACKGROUND SPEC: Background layers: The entire page sits on a single warm off-white/cream canvas (approximately #F5F3EF) with no visible texture, grain, or grid. The header bar is the same cream tone, and the hero, About, Selected Work, Services, Trusted By, and CTA sections all share this continuous background with no color transitions between sections. The two hero CTAs and the CTA-section button use a solid burnt-orange fill (approximately #E05A1E) with white text and slightly rounded corners. Recreate this background 1:1 in code as a flat warm cream color with the orange accent buttons built in code — do not use a background screenshot.",
        "CODED SEPARATOR / BACKGROUND SPEC: Typography and layout rhythm as coded structure: The design relies on a monospace uppercase micro-label style (letter-spaced, small, e.g. ''CREATIVE STUDIO — EST. 2018'', ''ABOUT'', ''SELECTED WORK'', ''SERVICES'', ''TRUSTED BY TEAMS AT'') paired with a large dark charcoal serif-adjacent sans headline face. The hero right side features a large photo block with a vertical ''01 / 03'' slide counter and a small right-aligned monospace list (''STRATEGY / DESIGN / REAL RESULTS''). Recreate these labels, counters, and spacing rhythm exactly in code as text and layout, not images.",
        "CODED SEPARATOR / BACKGROUND SPEC: Animation note: No 3D objects or animated centerpiece props are visible in this image; the only motion-relevant elements are the hero image carousel counter (01/03) and arrow-link hover states, which should be implemented as simple code-driven transitions (arrow slide on hover, carousel counter update) rather than 3D reconstruction."
      ],
      "behaviors": [
        "HOVER STATE: Primary CTA hover darkens the button to a deeper burnt orange (#D14E1F) with a subtle upward lift of 2px and the arrow translating right; nav links hover to accent #E85D2A with a hairline underline; portfolio frames hover to show a soft 0 12px 32px shadow and a ''View case study'' mono hint",
        "CLICK / ACTIVE STATE: CTA press compresses scale to 0.98 with background #C2461B; active nav link shows the accent underline indicator and full ink color; service rows press to a faint warm surface tint #F4F2EE",
        "FOCUS & DISABLED STATES: Keyboard focus shows a 2px accent #E85D2A outline with 2px offset on all interactive elements; disabled buttons render at 40% opacity with no hover lift"
      ],
      "responsive_rules": [
        "Desktop 1440px: full multi-column asymmetric layouts as shown, 96px section padding",
        "Tablet 834px: collapse to a 2-column grid, hero headline scales to 40px, services list keeps hairline rows, nav condenses with links intact where possible",
        "Mobile 390px: single column stacking, hamburger menu replaces nav links, hero at 32–36px, portfolio grid becomes one column, 48px section padding"
      ],
      "asset_instructions": [
        "CODED SEPARATOR / BACKGROUND SPEC: Section separators: The page uses thin, solid, light-gray horizontal rule lines (approximately one pixel, a warm light gray around #E5E2DC) that span the full content width to divide every major section — below the header nav, between the hero and About, between About and Selected Work, between Selected Work and Services, between Services and the Trusted By logo strip, and above the footer. In the Services list, each service row (Web Design, Branding, Digital Strategy) is separated by the same thin light rule line, with the final row closed by a rule beneath it. Recreate all of these separators 1:1 in code as full-width hairline borders with identical color and thickness — no gradients, no glow.",
        "CODED SEPARATOR / BACKGROUND SPEC: Background layers: The entire page sits on a single warm off-white/cream canvas (approximately #F5F3EF) with no visible texture, grain, or grid. The header bar is the same cream tone, and the hero, About, Selected Work, Services, Trusted By, and CTA sections all share this continuous background with no color transitions between sections. The two hero CTAs and the CTA-section button use a solid burnt-orange fill (approximately #E05A1E) with white text and slightly rounded corners. Recreate this background 1:1 in code as a flat warm cream color with the orange accent buttons built in code — do not use a background screenshot.",
        "CODED SEPARATOR / BACKGROUND SPEC: Typography and layout rhythm as coded structure: The design relies on a monospace uppercase micro-label style (letter-spaced, small, e.g. ''CREATIVE STUDIO — EST. 2018'', ''ABOUT'', ''SELECTED WORK'', ''SERVICES'', ''TRUSTED BY TEAMS AT'') paired with a large dark charcoal serif-adjacent sans headline face. The hero right side features a large photo block with a vertical ''01 / 03'' slide counter and a small right-aligned monospace list (''STRATEGY / DESIGN / REAL RESULTS''). Recreate these labels, counters, and spacing rhythm exactly in code as text and layout, not images.",
        "CODED SEPARATOR / BACKGROUND SPEC: Animation note: No 3D objects or animated centerpiece props are visible in this image; the only motion-relevant elements are the hero image carousel counter (01/03) and arrow-link hover states, which should be implemented as simple code-driven transitions (arrow slide on hover, carousel counter update) rather than 3D reconstruction."
      ],
      "is_visual_anchor": true
    },
    {
      "id": "item_2",
      "kind": "page",
      "name": "About — Studio Story & Philosophy Page",
      "tab_key": "full-website",
      "tab_label": "Full Website",
      "page_route": "/about",
      "variant_index": 1,
      "viewport": {
        "intent": "desktop",
        "css_width": 1440
      },
      "aspect_ratio": "2:3",
      "transparent_background": false,
      "extracted_from_item_id": null,
      "target_existing_output_id": null,
      "quality_override": null,
      "requirements_covered": [
        "req_1",
        "req_2",
        "req_3",
        "req_4",
        "req_6"
      ],
      "direction": "Minimalist editorial About page sharing item_1''s exact navbar and footer via Image-to-Image. Studio story as a large editorial statement, numbered mono-indexed values list, hairline-divided stats with tabular numerals, closing CTA.",
      "exact_copy": [
        "Elevate Studio",
        "Home",
        "About",
        "Services",
        "Portfolio",
        "Contact",
        "Book a discovery call",
        "A small studio with a deliberate approach.",
        "We believe good design is quiet, useful, and honest. Since 2018 we''ve helped founders and marketing teams turn complex ideas into clear digital experiences.",
        "Our Values",
        "01 — Clarity over cleverness",
        "02 — Evidence over opinion",
        "03 — Craft over volume",
        "04 — Partnership over handoff",
        "By the numbers",
        "48",
        "Projects shipped",
        "12",
        "Industry awards",
        "96%",
        "Client retention",
        "7",
        "Years in practice",
        "Sound like your kind of studio?",
        "Book a discovery call",
        "© 2025 Elevate Studio"
      ],
      "reference_asset_ids": [],
      "depends_on": [
        "item_1"
      ],
      "image_prompt": "A tall vertical full-page scroll capture of a 1440px-wide desktop About page for the same minimalist creative agency, using the EXACT same sticky top navigation bar as the attached item_1 reference image (identical monogram logo and ''Elevate Studio'' wordmark left, identical nav links with the active underline indicator now on ''About'', identical orange ''Book a discovery call'' button right, identical header height, surface #FAFAF8, and hairline bottom border) and the identical footer. Page content top-to-bottom on warm off-white: a page header with mono eyebrow ''ABOUT — 01'' and a large 48px Sora headline ''A small studio with a deliberate approach.'' as the single dominant focal point, followed by a 5/7 asymmetric split where the left column holds a muted Inter paragraph and the right holds a large 28px Sora editorial statement. Studio philosophy section: a wide editorial block of two left-aligned Inter paragraphs with generous whitespace. ''Our Values'' section: a vertical numbered list of four rows separated by 1px hairlines, each with a JetBrains Mono index (''01''–''04''), value title in 24px Sora, and a short muted description — no cards, no icons. ''By the numbers'' stats section: a single horizontal row of four large tabular-numeral figures (48, 12, 96%, 7) in 40px Sora with mono uppercase captions beneath, separated by vertical 1px hairlines instead of cards. Closing conversion band: large Sora line ''Sound like your kind of studio?'' with the orange CTA button, then the identical hairline-bordered footer with copyright. 60%+ whitespace, 96px section rhythm, no gradients, no clutter, restrained single accent.",
      "implementation_notes": [
        "POSITIONING (STICKY VS NON-STICKY): The shared top navbar remains anchored at the viewport top on scroll; the story, values, stats, and CTA sections scroll in normal flow",
        "PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: The stats figures and value descriptions are editable sample content for the user''s future data; the layout, hairline dividers, typography, and CTA must be coded 1:1 in code",
        "CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Hairline dividers, vertical stat separators, and background surfaces are pure code; the logo and any small UI icons come from the extracted transparent assets converted to SVG; no 3D objects on this page",
        "FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Values rows reveal with a staggered fade-up on scroll; stat figures count up once when scrolled into view using exponential ease-out; the shared nav indicator slides to the About route; CTA uses the same crisp spring press (stiffness 320, damping 28); reduced-motion swaps all transforms for opacity fades",
        "REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No reference attached; build fresh and reuse the shared navbar and footer components from item_1 exactly"
      ],
      "behaviors": [
        "HOVER STATE: Value rows hover to a faint warm surface tint with the mono index turning accent #E85D2A; CTA hover darkens to #D14E1F with 2px lift; nav links hover to accent",
        "CLICK / ACTIVE STATE: CTA press compresses to 0.98 scale on #C2461B; active nav link ''About'' carries the accent underline indicator",
        "FOCUS & DISABLED STATES: 2px accent outline with offset on keyboard focus; disabled states at 40% opacity"
      ],
      "responsive_rules": [
        "Desktop 1440px: 5/7 editorial split, four-across stats row with vertical hairlines",
        "Tablet 834px: stats become a 2x2 grid with horizontal hairlines, split collapses to single column",
        "Mobile 390px: fully stacked single column, stats as a vertical list, 48px padding"
      ],
      "asset_instructions": [],
      "is_visual_anchor": false
    },
    {
      "id": "item_3",
      "kind": "page",
      "name": "Services — Editorial Service Blocks & Process Page",
      "tab_key": "full-website",
      "tab_label": "Full Website",
      "page_route": "/services",
      "variant_index": 1,
      "viewport": {
        "intent": "desktop",
        "css_width": 1440
      },
      "aspect_ratio": "2:3",
      "transparent_background": false,
      "extracted_from_item_id": null,
      "target_existing_output_id": null,
      "quality_override": null,
      "requirements_covered": [
        "req_1",
        "req_2",
        "req_3",
        "req_4",
        "req_6"
      ],
      "direction": "Minimalist Services page sharing item_1''s chrome via Image-to-Image. Three asymmetric editorial service blocks separated by hairlines, a numbered process rail, closing CTA. No card grids, no icon circles.",
      "exact_copy": [
        "Elevate Studio",
        "Home",
        "About",
        "Services",
        "Portfolio",
        "Contact",
        "Book a discovery call",
        "Three disciplines. One standard.",
        "Web Design",
        "Websites engineered to convert — fast, accessible, and unmistakably yours. Design systems, marketing sites, and product interfaces built on modern foundations.",
        "Branding",
        "Identity systems that hold up in the wild. Strategy, naming, visual identity, and guidelines your team can actually use.",
        "Digital Strategy",
        "Research-backed roadmaps that connect design decisions to business outcomes. Positioning, analytics, and growth experiments.",
        "Process",
        "01 — Discover",
        "02 — Define",
        "03 — Design",
        "04 — Deliver",
        "Ready to scope your project?",
        "Book a discovery call",
        "© 2025 Elevate Studio"
      ],
      "reference_asset_ids": [],
      "depends_on": [
        "item_1"
      ],
      "image_prompt": "A tall vertical full-page scroll capture of a 1440px-wide desktop Services page for the same minimalist creative agency, using the EXACT same sticky top navigation bar as the attached item_1 reference image (identical logo, links with the active underline now on ''Services'', identical orange CTA button, identical header surface and hairline border) and identical footer. Content top-to-bottom on #FAFAF8: page header with mono eyebrow ''SERVICES — 02'' and a large 48px Sora headline ''Three disciplines. One standard.'' as the dominant focal point. Three service sections stacked vertically, each an asymmetric editorial block separated by a full-width 1px hairline: alternating 7/5 and 5/7 column splits where one column holds a mono index (''01'', ''02'', ''03'') plus the service name in 32px Sora, and the other holds a left-aligned Inter description and a quiet ''Learn more'' text link with arrow; no cards, no boxes, no icons in circles. ''Process'' section: a horizontal four-step rail with JetBrains Mono step numbers (''01 — Discover'' through ''04 — Deliver''), short captions, connected by a thin hairline running behind the steps, each step separated by whitespace rather than boxes. Closing band: Sora line ''Ready to scope your project?'' with the orange CTA button, then the identical footer. 60%+ whitespace, 96px section rhythm, single restrained accent, zero gradients and zero decorative clutter.",
      "implementation_notes": [
        "POSITIONING (STICKY VS NON-STICKY): The shared navbar stays fixed at the top; the three service blocks, process rail, and CTA band scroll in normal flow",
        "PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: Service descriptions and process captions are sample copy for the user''s real offering data; the editorial blocks, hairline separators, and step rail must be coded 1:1",
        "CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: All hairlines, the process connector line, and surfaces are pure code; logo and small UI icons come from extracted transparent assets converted to SVG; no 3D objects on this page",
        "FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Service blocks reveal with alternating slide-in-from-side drift on scroll; process steps light up sequentially with a staggered fade; the shared nav indicator slides to Services; CTA spring press (stiffness 320, damping 28); reduced-motion uses opacity-only fades",
        "REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No reference attached; reuse the shared navbar and footer components from item_1 exactly"
      ],
      "behaviors": [
        "HOVER STATE: ''Learn more'' links hover with the arrow translating right and text turning accent #E85D2A; process steps hover to lift their mono index into accent; CTA hover darkens to #D14E1F with 2px lift",
        "CLICK / ACTIVE STATE: CTA press compresses to 0.98 on #C2461B; active nav link ''Services'' carries the accent underline",
        "FOCUS & DISABLED STATES: 2px accent focus outline with offset; disabled at 40% opacity"
      ],
      "responsive_rules": [
        "Desktop 1440px: alternating 7/5 and 5/7 editorial splits, four-across process rail",
        "Tablet 834px: service blocks stack to single column with the index above the title, process becomes a 2x2 grid",
        "Mobile 390px: everything single-column stacked, process becomes a vertical list with a left hairline spine, 48px padding"
      ],
      "asset_instructions": [],
      "is_visual_anchor": false
    },
    {
      "id": "item_4",
      "kind": "page",
      "name": "Portfolio — Project Grid Page",
      "tab_key": "full-website",
      "tab_label": "Full Website",
      "page_route": "/portfolio",
      "variant_index": 1,
      "viewport": {
        "intent": "desktop",
        "css_width": 1440
      },
      "aspect_ratio": "2:3",
      "transparent_background": false,
      "extracted_from_item_id": null,
      "target_existing_output_id": null,
      "quality_override": null,
      "requirements_covered": [
        "req_1",
        "req_2",
        "req_3",
        "req_4",
        "req_6"
      ],
      "direction": "Minimalist Portfolio page sharing item_1''s chrome via Image-to-Image. Varied-aspect-ratio project grid with hairline frames, mono category labels, hover hints, case-study CTA.",
      "exact_copy": [
        "Elevate Studio",
        "Home",
        "About",
        "Services",
        "Portfolio",
        "Contact",
        "Book a discovery call",
        "Work that earns its keep.",
        "All",
        "Web Design",
        "Branding",
        "Strategy",
        "Northwind Commerce",
        "Web Design — 2025",
        "Meridian Labs",
        "Branding — 2024",
        "Atlas Health",
        "Web Design — 2024",
        "Corvus Analytics",
        "Strategy — 2025",
        "Fielder & Co.",
        "Branding — 2023",
        "Start your project",
        "Book a discovery call",
        "© 2025 Elevate Studio"
      ],
      "reference_asset_ids": [],
      "depends_on": [
        "item_1"
      ],
      "image_prompt": "A tall vertical full-page scroll capture of a 1440px-wide desktop Portfolio page for the same minimalist creative agency, using the EXACT same sticky top navigation bar as the attached item_1 reference image (identical logo, links with the active underline now on ''Portfolio'', identical orange CTA button, identical header surface and hairline border) and identical footer. Content top-to-bottom on #FAFAF8: page header with mono eyebrow ''PORTFOLIO — 03'' and a large 48px Sora headline ''Work that earns its keep.'' as the dominant focal point, plus a horizontal filter row of five quiet text pills (All active in ink with an accent underline, Web Design, Branding, Strategy) separated by whitespace. Project grid: a clean two-column masonry-style grid of five placeholder project frames with deliberately varied aspect ratios (one tall 3:4, one wide 4:3, one near-square) separated by generous gutters, each frame a flat neutral placeholder surface framed by a 1px hairline with restrained overlay typography — project name in 20px Sora and a JetBrains Mono category-year label (''Web Design — 2025'') beneath the frame, never on top of busy imagery; one frame shows a subtle hover-state hint with a soft shadow and a mono ''View case study'' tag in the corner. Closing conversion band: Sora line ''Start your project'' with the orange CTA button, then the identical footer. 60%+ whitespace, no card-in-card nesting, no gradients, restrained single accent.",
      "implementation_notes": [
        "POSITIONING (STICKY VS NON-STICKY): The shared navbar stays fixed at the top; the filter row, project grid, and CTA band scroll in normal flow",
        "PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: All five project images, names, and category-year labels are placeholder sample content for the user''s future portfolio database; only the grid frames, hairline borders, filter pills, typography, and hover treatments are coded 1:1 in code right now, with image slots bound to a data source later",
        "CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Grid frames, hairlines, filter pill indicators, and hover overlays are pure code; logo and small UI icons (filter arrows, external-link marks) come from extracted transparent assets converted to SVG; no 3D objects on this page",
        "FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Filter pills reflow the grid with a smooth layout animation and staggered fade-in of matching frames; project frames lift with a soft shadow and reveal the ''View case study'' mono tag on hover using exponential ease-out; the shared nav indicator slides to Portfolio; CTA spring press (stiffness 320, damping 28); reduced-motion disables transforms and layout shifts, using opacity fades only",
        "REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No reference attached; reuse the shared navbar and footer components from item_1 exactly"
      ],
      "behaviors": [
        "HOVER STATE: Project frames hover to shadow elevation 0 12px 32px rgba(0,0,0,0.10) with a 2px lift and the mono ''View case study'' tag fading in; filter pills hover to ink text with an accent underline; CTA hover darkens to #D14E1F",
        "CLICK / ACTIVE STATE: Active filter pill shows ink text with a persistent accent underline; CTA press compresses to 0.98 on #C2461B; active nav link ''Portfolio'' carries the accent underline",
        "FOCUS & DISABLED STATES: 2px accent focus outline with offset on frames and pills; disabled filters at 40% opacity"
      ],
      "responsive_rules": [
        "Desktop 1440px: two-column varied-aspect masonry grid with generous gutters",
        "Tablet 834px: two-column grid with uniform gutters, filter row wraps",
        "Mobile 390px: single-column stacked project frames, horizontally scrollable filter row, 48px padding"
      ],
      "asset_instructions": [],
      "is_visual_anchor": false
    },
    {
      "id": "item_5",
      "kind": "page",
      "name": "Contact — Conversion Form Page",
      "tab_key": "full-website",
      "tab_label": "Full Website",
      "page_route": "/contact",
      "variant_index": 1,
      "viewport": {
        "intent": "desktop",
        "css_width": 1440
      },
      "aspect_ratio": "2:3",
      "transparent_background": false,
      "extracted_from_item_id": null,
      "target_existing_output_id": null,
      "quality_override": null,
      "requirements_covered": [
        "req_1",
        "req_2",
        "req_3",
        "req_4",
        "req_5",
        "req_6"
      ],
      "direction": "Minimalist Contact page sharing item_1''s chrome via Image-to-Image. Elegant conversion form with hairline input underlines in a 7/5 asymmetric split with a contact-details sidebar, single dominant CTA.",
      "exact_copy": [
        "Elevate Studio",
        "Home",
        "About",
        "Services",
        "Portfolio",
        "Contact",
        "Book a discovery call",
        "Tell us about your project.",
        "Name",
        "Email",
        "Project type",
        "Budget",
        "Message",
        "Book a discovery call",
        "We reply within one business day.",
        "Contact",
        "hello@elevate.studio",
        "+1 (415) 555-0134",
        "Studio",
        "214 Kearny Street, Floor 3",
        "San Francisco, CA 94108",
        "Hours",
        "Mon–Fri, 9:00–18:00 PT",
        "© 2025 Elevate Studio"
      ],
      "reference_asset_ids": [],
      "depends_on": [
        "item_1"
      ],
      "image_prompt": "A tall vertical full-page scroll capture of a 1440px-wide desktop Contact page for the same minimalist creative agency, using the EXACT same sticky top navigation bar as the attached item_1 reference image (identical logo, links with the active underline now on ''Contact'', identical orange CTA button, identical header surface and hairline border) and identical footer. Content top-to-bottom on #FAFAF8: page header with mono eyebrow ''CONTACT — 04'' and a large 48px Sora headline ''Tell us about your project.'' as the dominant focal point. Main content in a 7/5 asymmetric split: the left column holds the conversion form — five fields (Name, Email, Project type, Budget, Message) each with a JetBrains Mono uppercase label above and a minimal input rendered as only a 1px hairline underline on the warm background (no boxes, no filled inputs), the Message field taller, and beneath them the large solid burnt-orange ''Book a discovery call'' CTA button with arrow plus a quiet mono reassurance line ''We reply within one business day.'' The right sidebar column, separated by a vertical 1px hairline, holds three stacked detail groups with mono uppercase headings (''CONTACT'', ''STUDIO'', ''HOURS'') and left-aligned Inter details including email, phone, street address, and hours. Identical hairline-bordered footer with copyright below. 60%+ whitespace, no boxes or card nesting, single restrained accent, zero gradients.",
      "implementation_notes": [
        "POSITIONING (STICKY VS NON-STICKY): The shared navbar stays fixed at the top; the form, sidebar, and footer scroll in normal flow",
        "PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: The contact details, address, and hours are sample content for the user''s real business data; the form fields, hairline underlines, sidebar layout, and CTA must be coded 1:1 in code with proper form state handling",
        "CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Input underlines, the vertical sidebar hairline, and focus indicators are pure code; logo and small UI icons (mail, phone, map pin, clock as 2px crisp geometric strokes) come from the extracted transparent icon sheet converted to SVG; no 3D objects on this page",
        "FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Form fields reveal with a quick staggered fade-up; focusing a field animates its hairline underline from border gray to accent #E85D2A with exponential ease-out; the CTA uses the crisp spring press (stiffness 320, damping 28) with arrow nudge on hover; a submit success state fades in a confirmation line; the shared nav indicator slides to Contact; reduced-motion uses opacity-only transitions",
        "REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No reference attached; reuse the shared navbar and footer components from item_1 exactly"
      ],
      "behaviors": [
        "HOVER STATE: CTA hover darkens to #D14E1F with a 2px lift and arrow nudge; sidebar detail rows hover to a faint warm tint; idle input underlines sit at rgba(26,26,46,0.12) and darken to ink on hover",
        "CLICK / ACTIVE STATE: Focused input underline animates to accent #E85D2A at 2px visual weight; CTA press compresses to 0.98 on #C2461B; active nav link ''Contact'' carries the accent underline",
        "FOCUS & DISABLED STATES: Inputs show the accent underline plus a subtle mono helper hint on keyboard focus; the CTA shows a 2px accent outline with offset when tabbed to; the submit button disables at 40% opacity while the form is invalid"
      ],
      "responsive_rules": [
        "Desktop 1440px: 7/5 form-to-sidebar split with vertical hairline divider",
        "Tablet 834px: sidebar moves below the form as a horizontal three-column detail strip",
        "Mobile 390px: single column, form fields stacked full-width, details stacked vertically, 48px padding"
      ],
      "asset_instructions": [],
      "is_visual_anchor": false
    },
    {
      "id": "item_6",
      "kind": "asset",
      "name": "Elevate Studio Monogram Logo & Wordmark (Transparent)",
      "tab_key": "full-website",
      "tab_label": "Full Website",
      "page_route": null,
      "variant_index": 1,
      "viewport": {
        "intent": "isolated",
        "css_width": 1440
      },
      "aspect_ratio": "1:1",
      "transparent_background": true,
      "extracted_from_item_id": "item_1",
      "target_existing_output_id": null,
      "quality_override": null,
      "requirements_covered": [
        "req_2",
        "req_3"
      ],
      "direction": "Temporary monogram brand mark: a minimal geometric ''E'' constructed from three precise horizontal bars with one accent-orange bar, paired with the ''Elevate Studio'' wordmark in Sora.",
      "exact_copy": [
        "Elevate Studio"
      ],
      "reference_asset_ids": [],
      "depends_on": [
        "item_1"
      ],
      "image_prompt": "A minimalist brand logo lockup isolated on a clean 100% transparent background with zero UI chrome and zero background clutter: on the left, a geometric monogram letter ''E'' built from three crisp horizontal rectangular bars in deep ink #1A1A2E where the middle bar is burnt orange #E85D2A, forming a confident abstract mark; to its right, the wordmark ''Elevate Studio'' in a clean geometric sans-serif, ''Elevate'' in bold ink and ''Studio'' in regular weight muted gray. Flat vector style, razor-sharp edges, generous even padding around the lockup, no shadows, no gradients, no textures, no extra text.",
      "implementation_notes": [
        "POSITIONING (STICKY VS NON-STICKY): This asset is used inside the sticky top navigation bar on every page and optionally in the footer; it is not a standalone layout element",
        "PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: This is a real brand asset, not placeholder content; it must be reproduced exactly as generated",
        "CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: The monogram is a flat geometric mark and must be traced and converted into a scalable vectorial SVG graphic for razor-sharp rendering at all sizes; it is not a 3D object and requires no Image-to-3D reconstruction",
        "FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: On the homepage the logo may perform a subtle opacity fade-in on load; clicking it navigates to the home route; no rotation or 3D animation is appropriate for this flat mark",
        "REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No reference attached; use this mark as the canonical logo everywhere the brand appears"
      ],
      "behaviors": [
        "HOVER STATE: The logo hovers with a subtle opacity shift from 100% to 85% and the cursor becomes a pointer",
        "CLICK / ACTIVE STATE: Clicking navigates to the home route with a quick 0.98 scale press feedback",
        "FOCUS & DISABLED STATES: Keyboard focus shows a 2px accent outline with 2px offset around the logo link"
      ],
      "responsive_rules": [
        "Desktop and tablet: monogram at 28px height with wordmark visible",
        "Mobile: monogram alone at 24px height with the wordmark hidden to save space"
      ],
      "asset_instructions": [
        "Convert this transparent logo lockup into a scalable vectorial SVG graphic (trace the geometric bars and set the wordmark in the brand''s Sora typeface) so it renders razor-sharp in the navbar, footer, and favicon at any size; layer nothing as a raster cutout"
      ],
      "is_visual_anchor": false
    },
    {
      "id": "item_7",
      "kind": "asset",
      "name": "Site Icon Sheet — 2px Geometric Stroke Icons (Transparent)",
      "tab_key": "full-website",
      "tab_label": "Full Website",
      "page_route": null,
      "variant_index": 1,
      "viewport": {
        "intent": "isolated",
        "css_width": 1440
      },
      "aspect_ratio": "1:1",
      "transparent_background": true,
      "extracted_from_item_id": "item_1",
      "target_existing_output_id": null,
      "quality_override": null,
      "requirements_covered": [
        "req_3",
        "req_4"
      ],
      "direction": "One organized icon sheet of crisp 2px geometric stroke icons in ink #1A1A2E on a transparent background, evenly spaced in a grid for SVG conversion.",
      "exact_copy": [],
      "reference_asset_ids": [],
      "depends_on": [
        "item_1"
      ],
      "image_prompt": "An organized icon sheet isolated on a clean 100% transparent background with zero UI chrome, zero text labels, and zero background clutter: a neat evenly-spaced grid of eight minimalist line icons drawn with crisp uniform 2px geometric strokes in deep ink #1A1A2E — an arrow pointing right, a mail envelope, a telephone handset, a map pin, a clock face, a plus sign, an external-link arrow leaving a bracket, and a simple menu line pair. Each icon sits in its own invisible square cell with generous consistent spacing, perfectly aligned, flat vector style, no fills, no shadows, no gradients, no rounded-square containers.",
      "implementation_notes": [
        "POSITIONING (STICKY VS NON-STICKY): These icons are used inline within buttons, form labels, the contact sidebar, and navigation; they are not standalone layout blocks",
        "PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: These are real reusable UI icons, not placeholder content; each maps to a specific interface use (arrow for CTAs, mail/phone/pin/clock for the contact sidebar, plus and external-link for portfolio actions, menu lines for the mobile hamburger)",
        "CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Every icon must be traced and converted into scalable vectorial SVG graphics with 2px strokes so they render crisply at 16–24px; none are raster cutouts and none are 3D objects",
        "FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: The arrow icon nudges right on CTA hover; the menu icon animates between two lines and a close cross when the mobile menu opens; all icon transitions use exponential ease-out and respect reduced-motion",
        "REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No reference attached; use these as the canonical icon set, replacing any default icon library usage where the design calls for custom marks"
      ],
      "behaviors": [
        "HOVER STATE: Icons inside interactive elements inherit the parent hover color, shifting from ink #1A1A2E to accent #E85D2A",
        "CLICK / ACTIVE STATE: Icons inside pressed buttons compress with the parent''s 0.98 scale spring",
        "FOCUS & DISABLED STATES: Icons inherit the parent''s 2px accent focus outline; disabled icons render at 40% opacity"
      ],
      "responsive_rules": [
        "Desktop and tablet: icons render at 20–24px inline with text",
        "Mobile: icons render at 18–20px with touch targets kept at a minimum 44px interactive area"
      ],
      "asset_instructions": [
        "Slice each icon from the transparent sheet and convert it into a scalable vectorial SVG graphic with a 2px stroke width, currentColor stroke, and no fill, so icons recolor instantly with the brand tokens and stay razor-sharp at every size"
      ],
      "is_visual_anchor": false
    },
    {
      "id": "item_1_auto_asset_1",
      "kind": "asset",
      "name": "Elevate Studio Brand Logo & Wordmark (Transparent)",
      "tab_key": "full-website",
      "tab_label": "Full Website",
      "page_route": null,
      "variant_index": 8,
      "viewport": {
        "intent": "isolated",
        "css_width": 1024
      },
      "aspect_ratio": "1:1",
      "transparent_background": true,
      "extracted_from_item_id": "item_1",
      "target_existing_output_id": null,
      "quality_override": null,
      "requirements_covered": [
        "req_1"
      ],
      "direction": "Minimal geometric ''E'' monogram in dark charcoal paired with the ''Elevate Studio'' wordmark, matching the top-left header lockup.",
      "exact_copy": [],
      "reference_asset_ids": [],
      "depends_on": [
        "item_1"
      ],
      "image_prompt": "Extract ONLY the Elevate Studio brand logo lockup from the top-left corner of the attached reference UI image: the bold geometric ''E'' monogram mark plus the ''Elevate Studio'' wordmark text beside it, in dark charcoal. Isolate it on a clean 100% transparent background with all header background, nav links, and unrelated elements removed. Reproduce the exact letterform geometry, weight, and spacing faithfully.",
      "implementation_notes": [
        "Render on a 100% transparent PNG background and layer in the component/page layout"
      ],
      "behaviors": [],
      "responsive_rules": [
        "Scale proportionally across breakpoints"
      ],
      "asset_instructions": [
        "Place this logo lockup in the top-left of the site header and reuse it in the footer copyright area. Convert it into a vectorial SVG graphic so it scales crisply; do not rasterize it into a card or banner."
      ],
      "is_visual_anchor": false
    },
    {
      "id": "item_1_auto_asset_2",
      "kind": "asset",
      "name": "Social Media Icon Set (Transparent)",
      "tab_key": "full-website",
      "tab_label": "Full Website",
      "page_route": null,
      "variant_index": 9,
      "viewport": {
        "intent": "isolated",
        "css_width": 1024
      },
      "aspect_ratio": "1:1",
      "transparent_background": true,
      "extracted_from_item_id": "item_1",
      "target_existing_output_id": null,
      "quality_override": null,
      "requirements_covered": [
        "req_1"
      ],
      "direction": "Three thin-line social icons — LinkedIn, Instagram, and Twitter/X — in dark charcoal, matching the footer''s minimal icon row.",
      "exact_copy": [],
      "reference_asset_ids": [],
      "depends_on": [
        "item_1"
      ],
      "image_prompt": "Extract ONLY the three small social media icons from the bottom-right footer of the attached reference UI image: the LinkedIn, Instagram, and Twitter/X glyphs in thin dark charcoal line style. Isolate each glyph on a clean 100% transparent background with the footer background and all text removed, faithfully reproducing stroke weight and proportions.",
      "implementation_notes": [
        "Render on a 100% transparent PNG background and layer in the component/page layout"
      ],
      "behaviors": [],
      "responsive_rules": [
        "Scale proportionally across breakpoints"
      ],
      "asset_instructions": [
        "Place these icons in the footer''s bottom-right social row as inline vectorial SVG graphics with hover color shift to the brand orange. Keep them as icons, not embedded raster images."
      ],
      "is_visual_anchor": false
    }
  ],
  "shared_components": [
    {
      "id": "shared_1",
      "name": "Sticky Top Navbar",
      "canonical_item_id": "item_1",
      "used_on_routes": [
        "/",
        "/about",
        "/services",
        "/portfolio",
        "/contact"
      ],
      "description": "Anchored header with monogram logo, five nav links with sliding accent underline indicator, and solid accent CTA button"
    },
    {
      "id": "shared_2",
      "name": "Global Footer",
      "canonical_item_id": "item_1",
      "used_on_routes": [
        "/",
        "/about",
        "/services",
        "/portfolio",
        "/contact"
      ],
      "description": "Hairline-topped footer with closing Sora line, accent CTA, mono email, and copyright row"
    }
  ],
  "page_composition": [
    {
      "page_name": "Home",
      "route": "/",
      "viewport_targets": [
        "desktop"
      ],
      "ordered_sections": [
        "item_1"
      ],
      "shared_component_ids": [
        "shared_1",
        "shared_2"
      ],
      "required_ctas": [
        "Book a discovery call"
      ],
      "states": [
        "default",
        "hover",
        "focus"
      ]
    },
    {
      "page_name": "About",
      "route": "/about",
      "viewport_targets": [
        "desktop"
      ],
      "ordered_sections": [
        "item_2"
      ],
      "shared_component_ids": [
        "shared_1",
        "shared_2"
      ],
      "required_ctas": [
        "Book a discovery call"
      ],
      "states": [
        "default",
        "hover",
        "focus"
      ]
    },
    {
      "page_name": "Services",
      "route": "/services",
      "viewport_targets": [
        "desktop"
      ],
      "ordered_sections": [
        "item_3"
      ],
      "shared_component_ids": [
        "shared_1",
        "shared_2"
      ],
      "required_ctas": [
        "Book a discovery call"
      ],
      "states": [
        "default",
        "hover",
        "focus"
      ]
    },
    {
      "page_name": "Portfolio",
      "route": "/portfolio",
      "viewport_targets": [
        "desktop"
      ],
      "ordered_sections": [
        "item_4"
      ],
      "shared_component_ids": [
        "shared_1",
        "shared_2"
      ],
      "required_ctas": [
        "Book a discovery call"
      ],
      "states": [
        "default",
        "hover",
        "focus"
      ]
    },
    {
      "page_name": "Contact",
      "route": "/contact",
      "viewport_targets": [
        "desktop"
      ],
      "ordered_sections": [
        "item_5"
      ],
      "shared_component_ids": [
        "shared_1",
        "shared_2"
      ],
      "required_ctas": [
        "Book a discovery call"
      ],
      "states": [
        "default",
        "hover",
        "focus",
        "submit-success"
      ]
    }
  ]
}', NULL, 'completed', 0, 0, 0, 0, NULL, '2026-10-24T20:13:07.680267+00:00', '2026-10-24T20:16:15.257209+00:00', '2026-10-24T20:16:15.257209+00:00', 'full-website', 'Full Website', NULL, '[]');
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-elevate-studio-1', 'example-run-elevate-studio', 'example-elevate-studio', 'item_5', 5, 'page', 'Contact — Conversion Form Page', 'Minimalist Contact page sharing item_1''s chrome via Image-to-Image. Elegant conversion form with hairline input underlines in a 7/5 asymmetric split with a contact-details sidebar, single dominant CTA.', 'page', '/contact', '2:3', 'completed', 'showcases/elevate-studio/01.png', 'showcases/elevate-studio/01.webp', 'image/png', 1024, 1536, 1311095, 0, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (page)
- Direction / Item Title: Contact — Conversion Form Page (Variant #1)
- Viewport & Aspect Ratio: desktop (1440px target width), Aspect Ratio 2:3
- Scope Rules: VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NOT A HORIZONTAL POWERPOINT SLIDE): Complete coordinated web page flowing vertically from top to bottom (Navbar at top -> Hero -> Features -> Pricing -> Footer at bottom) with internal column grids and dimensions strictly matching the target viewport breakpoint.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Elevate Studio — A professional multi-page website for Elevate Studio, a creative agency offering web design, branding, and digital strategy, designed to convert visitors into consultation bookings.
- Product / Application Description: Tagline: Digital Experiences That Make an Impact.

A professional website for a creative agency offering web design, branding, and digital strategy. Includes dedicated Home, About, Services, Portfolio, and Contact pages, with cohesive branding and engaging layouts.
- Industry & Target Audience: Creative Agency / Digital Design Services | Audience: Startup founders, marketing leads, and business owners seeking premium web design, branding, and digital strategy services (A credible, polished showcase of agency work, clear service explanations, proof of results, and a frictionless way to start a project conversation)
- Primary User Goal / Conversion Action: Book a discovery call or submit a project inquiry via the Contact page

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #1 Minimalism
- Exact Style Definition: Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content.
- Mandatory Visual & CSS System Rules: Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "minimalism". Primary style #1 Minimalism (Minimal & Editorial): Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, refined, and quietly bold — a studio that lets its work speak through restraint and precision — A minimalist, editorial-driven site with generous whitespace, razor-thin hairline borders, and a single confident accent color. Large expressive headlines, spacious 48px–96px section padding, and a sticky top navigation guide visitors through Home, About, Services, Portfolio, and Contact. Portfolio work is showcased in a clean interactive grid; services are presented as focused benefit cards; the contact flow is a simple, elegant conversion form.
- UI Density & Tone: comfortable / Clear, assured, and impact-focused — concise statements that promise measurable results without hype (English)
- Color Palette: Primary #1A1A2E, Accent #E85D2A, Background #FAFAF8, Surface #FFFFFF, Text #16161F, Muted #6B6B76, Border #E4E4DE
- Typography Intent: Heading Sora (or similar geometric sans) — 48px/56px hero, 24px section headings, tight tracking, Body Inter — 15px–16px body, 1.6 line height, generous paragraph spacing, Mono JetBrains Mono — for labels, indices, and technical metadata
- Radii, Borders & Shadows: Radius 12px cards, 8px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(0,0,0,0.06), hover: 0 12px 32px rgba(0,0,0,0.10)
- Logo Instruction: Use a clean temporary monogram ''E'' integrated with the brand name ''Elevate Studio'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E4E4DE), and subtle surface luminance shifts (#FAFAF8 vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#16161F on #FAFAF8/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Minimalist Contact page sharing item_1''s chrome via Image-to-Image. Elegant conversion form with hairline input underlines in a 7/5 asymmetric split with a contact-details sidebar, single dominant CTA.

PLANNER IMAGE PROMPT:
A tall vertical full-page scroll capture of a 1440px-wide desktop Contact page for the same minimalist creative agency, using the EXACT same sticky top navigation bar as the attached item_1 reference image (identical logo, links with the active underline now on ''Contact'', identical orange CTA button, identical header surface and hairline border) and identical footer. Content top-to-bottom on #FAFAF8: page header with mono eyebrow ''CONTACT — 04'' and a large 48px Sora headline ''Tell us about your project.'' as the dominant focal point. Main content in a 7/5 asymmetric split: the left column holds the conversion form — five fields (Name, Email, Project type, Budget, Message) each with a JetBrains Mono uppercase label above and a minimal input rendered as only a 1px hairline underline on the warm background (no boxes, no filled inputs), the Message field taller, and beneath them the large solid burnt-orange ''Book a discovery call'' CTA button with arrow plus a quiet mono reassurance line ''We reply within one business day.'' The right sidebar column, separated by a vertical 1px hairline, holds three stacked detail groups with mono uppercase headings (''CONTACT'', ''STUDIO'', ''HOURS'') and left-aligned Inter details including email, phone, street address, and hours. Identical hairline-bordered footer with copyright below. 60%+ whitespace, no boxes or card nesting, single restrained accent, zero gradients.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_1] Five distinct desktop pages: Home (/), About (/about), Services (/services), Portfolio (/portfolio), Contact (/contact), each as a separate vertical 2:3 page item rendered at 1440px desktop width
- [req_2] Pages 2–5 use Image-to-Image conditioning on Page 1 to share the exact same sticky top navbar (Elevate Studio logo, Home/About/Services/Portfolio/Contact links, accent CTA button) and identical footer, palette, and typography
- [req_3] Light theme only: background #FAFAF8, primary text #1A1A2E, accent #E85D2A, hairline borders rgba(26,26,46,0.12); Sora headings, Inter body, JetBrains Mono labels
- [req_4] Minimalism style enforcement: 60%+ whitespace, 48–96px section padding, 1px hairline borders, single restrained accent, zero decorative clutter, asymmetric editorial layouts, no card-in-card nesting, no 3-column icon-in-circle grids, no gradients or neon
- [req_5] Primary conversion: Book a discovery call CTA and a frictionless contact form (name, email, project type, budget, message) with hairline input underlines
- [req_6] Authentic agency copy only — no AI slop filler phrases; one unmistakable primary focal point per page with a clear secondary action

EXACT COPY & HIERARCHY:
Elevate Studio | Home | About | Services | Portfolio | Contact | Book a discovery call | Tell us about your project. | Name | Email | Project type | Budget | Message | Book a discovery call | We reply within one business day. | Contact | hello@elevate.studio | +1 (415) 555-0134 | Studio | 214 Kearny Street, Floor 3 | San Francisco, CA 94108 | Hours | Mon–Fri, 9:00–18:00 PT | © 2025 Elevate Studio

ATTACHED REFERENCE ROLES:
- Attached Visual Anchor Page Reference (`item_1` (''Home — Visual Anchor Landing Page'', Route `/`)) [IMAGE-TO-IMAGE MULTI-PAGE CONTINUITY]: This attached image is Page 1 (the Visual Anchor) of the same multi-page website/webapp. Use Image-to-Image conditioning to keep the EXACT SAME sticky top navigation bar (identical brand logo & wordmark on the left, identical horizontal nav link typography/spacing in the center with the active pill/underline indicator moved to route `/contact`, identical CTA button on the right, identical header height, surface color, and border), the EXACT SAME footer chrome, and the EXACT SAME brand color palette, typography, border-radius, card borders, and shadow depth, while rendering the completely new page content and layout for ''Contact — Conversion Form Page'' (Route `/contact`) below the navbar!

INVARIANT RENDERING CONSTRAINTS:
- Direct, full-bleed viewport UI design reference (do NOT place the UI inside an angled laptop/phone hardware mockup or external desk scene, and do NOT include a browser URL bar).
- Strictly execute the Chosen Web Design Style definition and visual rules above in every surface, border, shadow, color contrast, and typographic detail.
- Render crisp, legible typography, accurate button alignment, accessible contrast, and realistic spacing.
- VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NEVER A HORIZONTAL POWERPOINT SLIDE): Render the entire page flowing vertically from top to bottom — starting with the top navigation bar at the very top edge, followed vertically by the Hero section, Feature sections, Pricing/Content sections, and ending with the Footer at the bottom edge.
- INTERNAL BREAKPOINT DIMENSIONS = DESKTOP (`1440px` WIDE DESKTOP GRID INSIDE VERTICAL FULL-PAGE SCROLL): Even though the image canvas is vertical (`2:3` / `9:16`) so the whole landing page is visible from top to bottom, the UI INSIDE the image MUST be a true `1440px`-wide Desktop layout — full horizontal desktop navbar (brand logo left, horizontal nav links center, CTA buttons right — NO mobile hamburger menu), 12-column desktop container with generous left/right desktop margins, 2-column desktop Hero, and 3-to-4 column side-by-side desktop Feature & Pricing cards in horizontal rows! Do NOT render a narrow single-column mobile phone layout!
- IMAGE-TO-IMAGE MULTI-PAGE CONSISTENCY WITH ATTACHED VISUAL ANCHOR PAGE (`item_1`): 1) IDENTICAL STICKY TOP NAVBAR & FOOTER: Render the exact same top navigation bar at the top edge as shown in the attached Page 1 reference image (same logo mark & wordmark on the left, same nav link items in the center with the active state on `/contact`, same right-side action button, same navbar height, background surface, and border) and the same footer at the bottom edge. 2) DISTINCT PAGE BODY FOR ''Contact — Conversion Form Page'' (Route `/contact`): Below the shared top navbar, do NOT repeat Page 1''s hero section! Instead, render the complete top-to-bottom UI specifically for ''Contact — Conversion Form Page'' (Minimalist Contact page sharing item_1''s chrome via Image-to-Image. Elegant conversion form with hairline input underlines in a 7/5 asymmetric split with a contact-details sidebar, single dominant CTA.) — showing every section, dashboard KPI tile, chart, filter bar, directory grid, form control, or table specified in the prompt above.
', '[]', '["item_1"]', '{"id":"item_5","kind":"page","name":"Contact — Conversion Form Page","tab_key":"full-website","tab_label":"Full Website","page_route":"/contact","variant_index":1,"viewport":{"intent":"desktop","css_width":1440},"aspect_ratio":"2:3","transparent_background":false,"extracted_from_item_id":null,"target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_1","req_2","req_3","req_4","req_5","req_6"],"direction":"Minimalist Contact page sharing item_1''s chrome via Image-to-Image. Elegant conversion form with hairline input underlines in a 7/5 asymmetric split with a contact-details sidebar, single dominant CTA.","exact_copy":["Elevate Studio","Home","About","Services","Portfolio","Contact","Book a discovery call","Tell us about your project.","Name","Email","Project type","Budget","Message","Book a discovery call","We reply within one business day.","Contact","hello@elevate.studio","+1 (415) 555-0134","Studio","214 Kearny Street, Floor 3","San Francisco, CA 94108","Hours","Mon–Fri, 9:00–18:00 PT","© 2025 Elevate Studio"],"reference_asset_ids":[],"depends_on":["item_1"],"image_prompt":"A tall vertical full-page scroll capture of a 1440px-wide desktop Contact page for the same minimalist creative agency, using the EXACT same sticky top navigation bar as the attached item_1 reference image (identical logo, links with the active underline now on ''Contact'', identical orange CTA button, identical header surface and hairline border) and identical footer. Content top-to-bottom on #FAFAF8: page header with mono eyebrow ''CONTACT — 04'' and a large 48px Sora headline ''Tell us about your project.'' as the dominant focal point. Main content in a 7/5 asymmetric split: the left column holds the conversion form — five fields (Name, Email, Project type, Budget, Message) each with a JetBrains Mono uppercase label above and a minimal input rendered as only a 1px hairline underline on the warm background (no boxes, no filled inputs), the Message field taller, and beneath them the large solid burnt-orange ''Book a discovery call'' CTA button with arrow plus a quiet mono reassurance line ''We reply within one business day.'' The right sidebar column, separated by a vertical 1px hairline, holds three stacked detail groups with mono uppercase headings (''CONTACT'', ''STUDIO'', ''HOURS'') and left-aligned Inter details including email, phone, street address, and hours. Identical hairline-bordered footer with copyright below. 60%+ whitespace, no boxes or card nesting, single restrained accent, zero gradients.","implementation_notes":["POSITIONING (STICKY VS NON-STICKY): The shared navbar stays fixed at the top; the form, sidebar, and footer scroll in normal flow","PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: The contact details, address, and hours are sample content for the user''s real business data; the form fields, hairline underlines, sidebar layout, and CTA must be coded 1:1 in code with proper form state handling","CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Input underlines, the vertical sidebar hairline, and focus indicators are pure code; logo and small UI icons (mail, phone, map pin, clock as 2px crisp geometric strokes) come from the extracted transparent icon sheet converted to SVG; no 3D objects on this page","FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Form fields reveal with a quick staggered fade-up; focusing a field animates its hairline underline from border gray to accent #E85D2A with exponential ease-out; the CTA uses the crisp spring press (stiffness 320, damping 28) with arrow nudge on hover; a submit success state fades in a confirmation line; the shared nav indicator slides to Contact; reduced-motion uses opacity-only transitions","REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No reference attached; reuse the shared navbar and footer components from item_1 exactly"],"behaviors":["HOVER STATE: CTA hover darkens to #D14E1F with a 2px lift and arrow nudge; sidebar detail rows hover to a faint warm tint; idle input underlines sit at rgba(26,26,46,0.12) and darken to ink on hover","CLICK / ACTIVE STATE: Focused input underline animates to accent #E85D2A at 2px visual weight; CTA press compresses to 0.98 on #C2461B; active nav link ''Contact'' carries the accent underline","FOCUS & DISABLED STATES: Inputs show the accent underline plus a subtle mono helper hint on keyboard focus; the CTA shows a 2px accent outline with offset when tabbed to; the submit button disables at 40% opacity while the form is invalid"],"responsive_rules":["Desktop 1440px: 7/5 form-to-sidebar split with vertical hairline divider","Tablet 834px: sidebar moves below the form as a horizontal three-column detail strip","Mobile 390px: single column, form fields stacked full-width, details stacked vertically, 48px padding"],"asset_instructions":[],"is_visual_anchor":false}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T20:14:17.936486+00:00', '2026-10-24T20:15:35.188864+00:00', 'full-website', 'Full Website', NULL, 0, NULL);
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-elevate-studio-2', 'example-run-elevate-studio', 'example-elevate-studio', 'item_6', 6, 'asset', 'Elevate Studio Monogram Logo & Wordmark (Transparent)', 'Temporary monogram brand mark: a minimal geometric ''E'' constructed from three precise horizontal bars with one accent-orange bar, paired with the ''Elevate Studio'' wordmark in Sora.', 'asset', NULL, '1:1', 'completed', 'showcases/elevate-studio/02.png', 'showcases/elevate-studio/02.webp', 'image/png', 1024, 1024, 522375, 1, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (asset)
- Direction / Item Title: Elevate Studio Monogram Logo & Wordmark (Transparent) (Variant #1)
- Viewport & Aspect Ratio: isolated (1440px target width), Aspect Ratio 1:1
- Scope Rules: Standalone visual site asset (icon sheet, illustration, 3D cutout, or foreground graphic) rendered on a 100% native transparent background so it can be layered directly into the coordinated page/component set.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Elevate Studio — A professional multi-page website for Elevate Studio, a creative agency offering web design, branding, and digital strategy, designed to convert visitors into consultation bookings.
- Product / Application Description: Tagline: Digital Experiences That Make an Impact.

A professional website for a creative agency offering web design, branding, and digital strategy. Includes dedicated Home, About, Services, Portfolio, and Contact pages, with cohesive branding and engaging layouts.
- Industry & Target Audience: Creative Agency / Digital Design Services | Audience: Startup founders, marketing leads, and business owners seeking premium web design, branding, and digital strategy services (A credible, polished showcase of agency work, clear service explanations, proof of results, and a frictionless way to start a project conversation)
- Primary User Goal / Conversion Action: Book a discovery call or submit a project inquiry via the Contact page

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #1 Minimalism
- Exact Style Definition: Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content.
- Mandatory Visual & CSS System Rules: Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "minimalism". Primary style #1 Minimalism (Minimal & Editorial): Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, refined, and quietly bold — a studio that lets its work speak through restraint and precision — A minimalist, editorial-driven site with generous whitespace, razor-thin hairline borders, and a single confident accent color. Large expressive headlines, spacious 48px–96px section padding, and a sticky top navigation guide visitors through Home, About, Services, Portfolio, and Contact. Portfolio work is showcased in a clean interactive grid; services are presented as focused benefit cards; the contact flow is a simple, elegant conversion form.
- UI Density & Tone: comfortable / Clear, assured, and impact-focused — concise statements that promise measurable results without hype (English)
- Color Palette: Primary #1A1A2E, Accent #E85D2A, Background #FAFAF8, Surface #FFFFFF, Text #16161F, Muted #6B6B76, Border #E4E4DE
- Typography Intent: Heading Sora (or similar geometric sans) — 48px/56px hero, 24px section headings, tight tracking, Body Inter — 15px–16px body, 1.6 line height, generous paragraph spacing, Mono JetBrains Mono — for labels, indices, and technical metadata
- Radii, Borders & Shadows: Radius 12px cards, 8px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(0,0,0,0.06), hover: 0 12px 32px rgba(0,0,0,0.10)
- Logo Instruction: Use a clean temporary monogram ''E'' integrated with the brand name ''Elevate Studio'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E4E4DE), and subtle surface luminance shifts (#FAFAF8 vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#16161F on #FAFAF8/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Temporary monogram brand mark: a minimal geometric ''E'' constructed from three precise horizontal bars with one accent-orange bar, paired with the ''Elevate Studio'' wordmark in Sora.

PLANNER IMAGE PROMPT:
A minimalist brand logo lockup isolated on a clean 100% transparent background with zero UI chrome and zero background clutter: on the left, a geometric monogram letter ''E'' built from three crisp horizontal rectangular bars in deep ink #1A1A2E where the middle bar is burnt orange #E85D2A, forming a confident abstract mark; to its right, the wordmark ''Elevate Studio'' in a clean geometric sans-serif, ''Elevate'' in bold ink and ''Studio'' in regular weight muted gray. Flat vector style, razor-sharp edges, generous even padding around the lockup, no shadows, no gradients, no textures, no extra text.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_2] Pages 2–5 use Image-to-Image conditioning on Page 1 to share the exact same sticky top navbar (Elevate Studio logo, Home/About/Services/Portfolio/Contact links, accent CTA button) and identical footer, palette, and typography
- [req_3] Light theme only: background #FAFAF8, primary text #1A1A2E, accent #E85D2A, hairline borders rgba(26,26,46,0.12); Sora headings, Inter body, JetBrains Mono labels

EXACT COPY & HIERARCHY:
Elevate Studio

ATTACHED REFERENCE ROLES:
- Prerequisite Hero/Component Reference Image [FOREGROUND ASSET EXTRACTION]: Extract and faithfully reproduce ONLY the specified foreground illustration, 3D object, icon set, or graphic element from this reference UI image onto a 100% transparent background (`background: transparent`). Strip away all background walls, gradients, UI buttons, and text!

INVARIANT RENDERING CONSTRAINTS:
- Render a clean, production-ready standalone website asset matching the chosen Web Design Style above.
- Do NOT wrap the asset in browser chrome, UI buttons, or device frames.
- NATIVE TRANSPARENT BACKGROUND (`background: "transparent"`): Isolate the subject (or organized icon sheet grid) on a 100% true transparent alpha background. Do NOT draw a fake checkerboard grid, do NOT add opaque background fills or vignettes, and keep clean alpha edges with a 10% safety margin around the silhouette.
- FOREGROUND EXTRACTION FROM COMPOSITE UI: Reproduce the exact foreground visual asset shown in the attached prerequisite Hero/Component reference image at full resolution on a transparent background, completely omitting the background environment, headlines, and UI buttons so this transparent PNG can be layered over the coded background.
', '[]', '["item_1"]', '{"id":"item_6","kind":"asset","name":"Elevate Studio Monogram Logo & Wordmark (Transparent)","tab_key":"full-website","tab_label":"Full Website","page_route":null,"variant_index":1,"viewport":{"intent":"isolated","css_width":1440},"aspect_ratio":"1:1","transparent_background":true,"extracted_from_item_id":"item_1","target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_2","req_3"],"direction":"Temporary monogram brand mark: a minimal geometric ''E'' constructed from three precise horizontal bars with one accent-orange bar, paired with the ''Elevate Studio'' wordmark in Sora.","exact_copy":["Elevate Studio"],"reference_asset_ids":[],"depends_on":["item_1"],"image_prompt":"A minimalist brand logo lockup isolated on a clean 100% transparent background with zero UI chrome and zero background clutter: on the left, a geometric monogram letter ''E'' built from three crisp horizontal rectangular bars in deep ink #1A1A2E where the middle bar is burnt orange #E85D2A, forming a confident abstract mark; to its right, the wordmark ''Elevate Studio'' in a clean geometric sans-serif, ''Elevate'' in bold ink and ''Studio'' in regular weight muted gray. Flat vector style, razor-sharp edges, generous even padding around the lockup, no shadows, no gradients, no textures, no extra text.","implementation_notes":["POSITIONING (STICKY VS NON-STICKY): This asset is used inside the sticky top navigation bar on every page and optionally in the footer; it is not a standalone layout element","PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: This is a real brand asset, not placeholder content; it must be reproduced exactly as generated","CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: The monogram is a flat geometric mark and must be traced and converted into a scalable vectorial SVG graphic for razor-sharp rendering at all sizes; it is not a 3D object and requires no Image-to-3D reconstruction","FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: On the homepage the logo may perform a subtle opacity fade-in on load; clicking it navigates to the home route; no rotation or 3D animation is appropriate for this flat mark","REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No reference attached; use this mark as the canonical logo everywhere the brand appears"],"behaviors":["HOVER STATE: The logo hovers with a subtle opacity shift from 100% to 85% and the cursor becomes a pointer","CLICK / ACTIVE STATE: Clicking navigates to the home route with a quick 0.98 scale press feedback","FOCUS & DISABLED STATES: Keyboard focus shows a 2px accent outline with 2px offset around the logo link"],"responsive_rules":["Desktop and tablet: monogram at 28px height with wordmark visible","Mobile: monogram alone at 24px height with the wordmark hidden to save space"],"asset_instructions":["Convert this transparent logo lockup into a scalable vectorial SVG graphic (trace the geometric bars and set the wordmark in the brand''s Sora typeface) so it renders razor-sharp in the navbar, footer, and favicon at any size; layer nothing as a raster cutout"],"is_visual_anchor":false}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T20:14:17.936486+00:00', '2026-10-24T20:15:48.114200+00:00', 'full-website', 'Full Website', 'example-out-elevate-studio-5', 1, 'example-out-elevate-studio-5');
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-elevate-studio-3', 'example-run-elevate-studio', 'example-elevate-studio', 'item_3', 3, 'page', 'Services — Editorial Service Blocks & Process Page', 'Minimalist Services page sharing item_1''s chrome via Image-to-Image. Three asymmetric editorial service blocks separated by hairlines, a numbered process rail, closing CTA. No card grids, no icon circles.', 'page', '/services', '2:3', 'completed', 'showcases/elevate-studio/03.png', 'showcases/elevate-studio/03.webp', 'image/png', 1024, 1536, 1540477, 0, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (page)
- Direction / Item Title: Services — Editorial Service Blocks & Process Page (Variant #1)
- Viewport & Aspect Ratio: desktop (1440px target width), Aspect Ratio 2:3
- Scope Rules: VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NOT A HORIZONTAL POWERPOINT SLIDE): Complete coordinated web page flowing vertically from top to bottom (Navbar at top -> Hero -> Features -> Pricing -> Footer at bottom) with internal column grids and dimensions strictly matching the target viewport breakpoint.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Elevate Studio — A professional multi-page website for Elevate Studio, a creative agency offering web design, branding, and digital strategy, designed to convert visitors into consultation bookings.
- Product / Application Description: Tagline: Digital Experiences That Make an Impact.

A professional website for a creative agency offering web design, branding, and digital strategy. Includes dedicated Home, About, Services, Portfolio, and Contact pages, with cohesive branding and engaging layouts.
- Industry & Target Audience: Creative Agency / Digital Design Services | Audience: Startup founders, marketing leads, and business owners seeking premium web design, branding, and digital strategy services (A credible, polished showcase of agency work, clear service explanations, proof of results, and a frictionless way to start a project conversation)
- Primary User Goal / Conversion Action: Book a discovery call or submit a project inquiry via the Contact page

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #1 Minimalism
- Exact Style Definition: Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content.
- Mandatory Visual & CSS System Rules: Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "minimalism". Primary style #1 Minimalism (Minimal & Editorial): Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, refined, and quietly bold — a studio that lets its work speak through restraint and precision — A minimalist, editorial-driven site with generous whitespace, razor-thin hairline borders, and a single confident accent color. Large expressive headlines, spacious 48px–96px section padding, and a sticky top navigation guide visitors through Home, About, Services, Portfolio, and Contact. Portfolio work is showcased in a clean interactive grid; services are presented as focused benefit cards; the contact flow is a simple, elegant conversion form.
- UI Density & Tone: comfortable / Clear, assured, and impact-focused — concise statements that promise measurable results without hype (English)
- Color Palette: Primary #1A1A2E, Accent #E85D2A, Background #FAFAF8, Surface #FFFFFF, Text #16161F, Muted #6B6B76, Border #E4E4DE
- Typography Intent: Heading Sora (or similar geometric sans) — 48px/56px hero, 24px section headings, tight tracking, Body Inter — 15px–16px body, 1.6 line height, generous paragraph spacing, Mono JetBrains Mono — for labels, indices, and technical metadata
- Radii, Borders & Shadows: Radius 12px cards, 8px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(0,0,0,0.06), hover: 0 12px 32px rgba(0,0,0,0.10)
- Logo Instruction: Use a clean temporary monogram ''E'' integrated with the brand name ''Elevate Studio'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E4E4DE), and subtle surface luminance shifts (#FAFAF8 vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#16161F on #FAFAF8/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Minimalist Services page sharing item_1''s chrome via Image-to-Image. Three asymmetric editorial service blocks separated by hairlines, a numbered process rail, closing CTA. No card grids, no icon circles.

PLANNER IMAGE PROMPT:
A tall vertical full-page scroll capture of a 1440px-wide desktop Services page for the same minimalist creative agency, using the EXACT same sticky top navigation bar as the attached item_1 reference image (identical logo, links with the active underline now on ''Services'', identical orange CTA button, identical header surface and hairline border) and identical footer. Content top-to-bottom on #FAFAF8: page header with mono eyebrow ''SERVICES — 02'' and a large 48px Sora headline ''Three disciplines. One standard.'' as the dominant focal point. Three service sections stacked vertically, each an asymmetric editorial block separated by a full-width 1px hairline: alternating 7/5 and 5/7 column splits where one column holds a mono index (''01'', ''02'', ''03'') plus the service name in 32px Sora, and the other holds a left-aligned Inter description and a quiet ''Learn more'' text link with arrow; no cards, no boxes, no icons in circles. ''Process'' section: a horizontal four-step rail with JetBrains Mono step numbers (''01 — Discover'' through ''04 — Deliver''), short captions, connected by a thin hairline running behind the steps, each step separated by whitespace rather than boxes. Closing band: Sora line ''Ready to scope your project?'' with the orange CTA button, then the identical footer. 60%+ whitespace, 96px section rhythm, single restrained accent, zero gradients and zero decorative clutter.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_1] Five distinct desktop pages: Home (/), About (/about), Services (/services), Portfolio (/portfolio), Contact (/contact), each as a separate vertical 2:3 page item rendered at 1440px desktop width
- [req_2] Pages 2–5 use Image-to-Image conditioning on Page 1 to share the exact same sticky top navbar (Elevate Studio logo, Home/About/Services/Portfolio/Contact links, accent CTA button) and identical footer, palette, and typography
- [req_3] Light theme only: background #FAFAF8, primary text #1A1A2E, accent #E85D2A, hairline borders rgba(26,26,46,0.12); Sora headings, Inter body, JetBrains Mono labels
- [req_4] Minimalism style enforcement: 60%+ whitespace, 48–96px section padding, 1px hairline borders, single restrained accent, zero decorative clutter, asymmetric editorial layouts, no card-in-card nesting, no 3-column icon-in-circle grids, no gradients or neon
- [req_6] Authentic agency copy only — no AI slop filler phrases; one unmistakable primary focal point per page with a clear secondary action

EXACT COPY & HIERARCHY:
Elevate Studio | Home | About | Services | Portfolio | Contact | Book a discovery call | Three disciplines. One standard. | Web Design | Websites engineered to convert — fast, accessible, and unmistakably yours. Design systems, marketing sites, and product interfaces built on modern foundations. | Branding | Identity systems that hold up in the wild. Strategy, naming, visual identity, and guidelines your team can actually use. | Digital Strategy | Research-backed roadmaps that connect design decisions to business outcomes. Positioning, analytics, and growth experiments. | Process | 01 — Discover | 02 — Define | 03 — Design | 04 — Deliver | Ready to scope your project? | Book a discovery call | © 2025 Elevate Studio

ATTACHED REFERENCE ROLES:
- Attached Visual Anchor Page Reference (`item_1` (''Home — Visual Anchor Landing Page'', Route `/`)) [IMAGE-TO-IMAGE MULTI-PAGE CONTINUITY]: This attached image is Page 1 (the Visual Anchor) of the same multi-page website/webapp. Use Image-to-Image conditioning to keep the EXACT SAME sticky top navigation bar (identical brand logo & wordmark on the left, identical horizontal nav link typography/spacing in the center with the active pill/underline indicator moved to route `/services`, identical CTA button on the right, identical header height, surface color, and border), the EXACT SAME footer chrome, and the EXACT SAME brand color palette, typography, border-radius, card borders, and shadow depth, while rendering the completely new page content and layout for ''Services — Editorial Service Blocks & Process Page'' (Route `/services`) below the navbar!

INVARIANT RENDERING CONSTRAINTS:
- Direct, full-bleed viewport UI design reference (do NOT place the UI inside an angled laptop/phone hardware mockup or external desk scene, and do NOT include a browser URL bar).
- Strictly execute the Chosen Web Design Style definition and visual rules above in every surface, border, shadow, color contrast, and typographic detail.
- Render crisp, legible typography, accurate button alignment, accessible contrast, and realistic spacing.
- VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NEVER A HORIZONTAL POWERPOINT SLIDE): Render the entire page flowing vertically from top to bottom — starting with the top navigation bar at the very top edge, followed vertically by the Hero section, Feature sections, Pricing/Content sections, and ending with the Footer at the bottom edge.
- INTERNAL BREAKPOINT DIMENSIONS = DESKTOP (`1440px` WIDE DESKTOP GRID INSIDE VERTICAL FULL-PAGE SCROLL): Even though the image canvas is vertical (`2:3` / `9:16`) so the whole landing page is visible from top to bottom, the UI INSIDE the image MUST be a true `1440px`-wide Desktop layout — full horizontal desktop navbar (brand logo left, horizontal nav links center, CTA buttons right — NO mobile hamburger menu), 12-column desktop container with generous left/right desktop margins, 2-column desktop Hero, and 3-to-4 column side-by-side desktop Feature & Pricing cards in horizontal rows! Do NOT render a narrow single-column mobile phone layout!
- IMAGE-TO-IMAGE MULTI-PAGE CONSISTENCY WITH ATTACHED VISUAL ANCHOR PAGE (`item_1`): 1) IDENTICAL STICKY TOP NAVBAR & FOOTER: Render the exact same top navigation bar at the top edge as shown in the attached Page 1 reference image (same logo mark & wordmark on the left, same nav link items in the center with the active state on `/services`, same right-side action button, same navbar height, background surface, and border) and the same footer at the bottom edge. 2) DISTINCT PAGE BODY FOR ''Services — Editorial Service Blocks & Process Page'' (Route `/services`): Below the shared top navbar, do NOT repeat Page 1''s hero section! Instead, render the complete top-to-bottom UI specifically for ''Services — Editorial Service Blocks & Process Page'' (Minimalist Services page sharing item_1''s chrome via Image-to-Image. Three asymmetric editorial service blocks separated by hairlines, a numbered process rail, closing CTA. No card grids, no icon circles.) — showing every section, dashboard KPI tile, chart, filter bar, directory grid, form control, or table specified in the prompt above.
', '[]', '["item_1"]', '{"id":"item_3","kind":"page","name":"Services — Editorial Service Blocks & Process Page","tab_key":"full-website","tab_label":"Full Website","page_route":"/services","variant_index":1,"viewport":{"intent":"desktop","css_width":1440},"aspect_ratio":"2:3","transparent_background":false,"extracted_from_item_id":null,"target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_1","req_2","req_3","req_4","req_6"],"direction":"Minimalist Services page sharing item_1''s chrome via Image-to-Image. Three asymmetric editorial service blocks separated by hairlines, a numbered process rail, closing CTA. No card grids, no icon circles.","exact_copy":["Elevate Studio","Home","About","Services","Portfolio","Contact","Book a discovery call","Three disciplines. One standard.","Web Design","Websites engineered to convert — fast, accessible, and unmistakably yours. Design systems, marketing sites, and product interfaces built on modern foundations.","Branding","Identity systems that hold up in the wild. Strategy, naming, visual identity, and guidelines your team can actually use.","Digital Strategy","Research-backed roadmaps that connect design decisions to business outcomes. Positioning, analytics, and growth experiments.","Process","01 — Discover","02 — Define","03 — Design","04 — Deliver","Ready to scope your project?","Book a discovery call","© 2025 Elevate Studio"],"reference_asset_ids":[],"depends_on":["item_1"],"image_prompt":"A tall vertical full-page scroll capture of a 1440px-wide desktop Services page for the same minimalist creative agency, using the EXACT same sticky top navigation bar as the attached item_1 reference image (identical logo, links with the active underline now on ''Services'', identical orange CTA button, identical header surface and hairline border) and identical footer. Content top-to-bottom on #FAFAF8: page header with mono eyebrow ''SERVICES — 02'' and a large 48px Sora headline ''Three disciplines. One standard.'' as the dominant focal point. Three service sections stacked vertically, each an asymmetric editorial block separated by a full-width 1px hairline: alternating 7/5 and 5/7 column splits where one column holds a mono index (''01'', ''02'', ''03'') plus the service name in 32px Sora, and the other holds a left-aligned Inter description and a quiet ''Learn more'' text link with arrow; no cards, no boxes, no icons in circles. ''Process'' section: a horizontal four-step rail with JetBrains Mono step numbers (''01 — Discover'' through ''04 — Deliver''), short captions, connected by a thin hairline running behind the steps, each step separated by whitespace rather than boxes. Closing band: Sora line ''Ready to scope your project?'' with the orange CTA button, then the identical footer. 60%+ whitespace, 96px section rhythm, single restrained accent, zero gradients and zero decorative clutter.","implementation_notes":["POSITIONING (STICKY VS NON-STICKY): The shared navbar stays fixed at the top; the three service blocks, process rail, and CTA band scroll in normal flow","PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: Service descriptions and process captions are sample copy for the user''s real offering data; the editorial blocks, hairline separators, and step rail must be coded 1:1","CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: All hairlines, the process connector line, and surfaces are pure code; logo and small UI icons come from extracted transparent assets converted to SVG; no 3D objects on this page","FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Service blocks reveal with alternating slide-in-from-side drift on scroll; process steps light up sequentially with a staggered fade; the shared nav indicator slides to Services; CTA spring press (stiffness 320, damping 28); reduced-motion uses opacity-only fades","REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No reference attached; reuse the shared navbar and footer components from item_1 exactly"],"behaviors":["HOVER STATE: ''Learn more'' links hover with the arrow translating right and text turning accent #E85D2A; process steps hover to lift their mono index into accent; CTA hover darkens to #D14E1F with 2px lift","CLICK / ACTIVE STATE: CTA press compresses to 0.98 on #C2461B; active nav link ''Services'' carries the accent underline","FOCUS & DISABLED STATES: 2px accent focus outline with offset; disabled at 40% opacity"],"responsive_rules":["Desktop 1440px: alternating 7/5 and 5/7 editorial splits, four-across process rail","Tablet 834px: service blocks stack to single column with the index above the title, process becomes a 2x2 grid","Mobile 390px: everything single-column stacked, process becomes a vertical list with a left hairline spine, 48px padding"],"asset_instructions":[],"is_visual_anchor":false}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T20:14:17.936486+00:00', '2026-10-24T20:15:12.097175+00:00', 'full-website', 'Full Website', NULL, 0, NULL);
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-elevate-studio-4', 'example-run-elevate-studio', 'example-elevate-studio', 'item_2', 2, 'page', 'About — Studio Story & Philosophy Page', 'Minimalist editorial About page sharing item_1''s exact navbar and footer via Image-to-Image. Studio story as a large editorial statement, numbered mono-indexed values list, hairline-divided stats with tabular numerals, closing CTA.', 'page', '/about', '2:3', 'completed', 'showcases/elevate-studio/04.png', 'showcases/elevate-studio/04.webp', 'image/png', 1024, 1536, 1472085, 0, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (page)
- Direction / Item Title: About — Studio Story & Philosophy Page (Variant #1)
- Viewport & Aspect Ratio: desktop (1440px target width), Aspect Ratio 2:3
- Scope Rules: VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NOT A HORIZONTAL POWERPOINT SLIDE): Complete coordinated web page flowing vertically from top to bottom (Navbar at top -> Hero -> Features -> Pricing -> Footer at bottom) with internal column grids and dimensions strictly matching the target viewport breakpoint.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Elevate Studio — A professional multi-page website for Elevate Studio, a creative agency offering web design, branding, and digital strategy, designed to convert visitors into consultation bookings.
- Product / Application Description: Tagline: Digital Experiences That Make an Impact.

A professional website for a creative agency offering web design, branding, and digital strategy. Includes dedicated Home, About, Services, Portfolio, and Contact pages, with cohesive branding and engaging layouts.
- Industry & Target Audience: Creative Agency / Digital Design Services | Audience: Startup founders, marketing leads, and business owners seeking premium web design, branding, and digital strategy services (A credible, polished showcase of agency work, clear service explanations, proof of results, and a frictionless way to start a project conversation)
- Primary User Goal / Conversion Action: Book a discovery call or submit a project inquiry via the Contact page

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #1 Minimalism
- Exact Style Definition: Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content.
- Mandatory Visual & CSS System Rules: Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "minimalism". Primary style #1 Minimalism (Minimal & Editorial): Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, refined, and quietly bold — a studio that lets its work speak through restraint and precision — A minimalist, editorial-driven site with generous whitespace, razor-thin hairline borders, and a single confident accent color. Large expressive headlines, spacious 48px–96px section padding, and a sticky top navigation guide visitors through Home, About, Services, Portfolio, and Contact. Portfolio work is showcased in a clean interactive grid; services are presented as focused benefit cards; the contact flow is a simple, elegant conversion form.
- UI Density & Tone: comfortable / Clear, assured, and impact-focused — concise statements that promise measurable results without hype (English)
- Color Palette: Primary #1A1A2E, Accent #E85D2A, Background #FAFAF8, Surface #FFFFFF, Text #16161F, Muted #6B6B76, Border #E4E4DE
- Typography Intent: Heading Sora (or similar geometric sans) — 48px/56px hero, 24px section headings, tight tracking, Body Inter — 15px–16px body, 1.6 line height, generous paragraph spacing, Mono JetBrains Mono — for labels, indices, and technical metadata
- Radii, Borders & Shadows: Radius 12px cards, 8px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(0,0,0,0.06), hover: 0 12px 32px rgba(0,0,0,0.10)
- Logo Instruction: Use a clean temporary monogram ''E'' integrated with the brand name ''Elevate Studio'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E4E4DE), and subtle surface luminance shifts (#FAFAF8 vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#16161F on #FAFAF8/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Minimalist editorial About page sharing item_1''s exact navbar and footer via Image-to-Image. Studio story as a large editorial statement, numbered mono-indexed values list, hairline-divided stats with tabular numerals, closing CTA.

PLANNER IMAGE PROMPT:
A tall vertical full-page scroll capture of a 1440px-wide desktop About page for the same minimalist creative agency, using the EXACT same sticky top navigation bar as the attached item_1 reference image (identical monogram logo and ''Elevate Studio'' wordmark left, identical nav links with the active underline indicator now on ''About'', identical orange ''Book a discovery call'' button right, identical header height, surface #FAFAF8, and hairline bottom border) and the identical footer. Page content top-to-bottom on warm off-white: a page header with mono eyebrow ''ABOUT — 01'' and a large 48px Sora headline ''A small studio with a deliberate approach.'' as the single dominant focal point, followed by a 5/7 asymmetric split where the left column holds a muted Inter paragraph and the right holds a large 28px Sora editorial statement. Studio philosophy section: a wide editorial block of two left-aligned Inter paragraphs with generous whitespace. ''Our Values'' section: a vertical numbered list of four rows separated by 1px hairlines, each with a JetBrains Mono index (''01''–''04''), value title in 24px Sora, and a short muted description — no cards, no icons. ''By the numbers'' stats section: a single horizontal row of four large tabular-numeral figures (48, 12, 96%, 7) in 40px Sora with mono uppercase captions beneath, separated by vertical 1px hairlines instead of cards. Closing conversion band: large Sora line ''Sound like your kind of studio?'' with the orange CTA button, then the identical hairline-bordered footer with copyright. 60%+ whitespace, 96px section rhythm, no gradients, no clutter, restrained single accent.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_1] Five distinct desktop pages: Home (/), About (/about), Services (/services), Portfolio (/portfolio), Contact (/contact), each as a separate vertical 2:3 page item rendered at 1440px desktop width
- [req_2] Pages 2–5 use Image-to-Image conditioning on Page 1 to share the exact same sticky top navbar (Elevate Studio logo, Home/About/Services/Portfolio/Contact links, accent CTA button) and identical footer, palette, and typography
- [req_3] Light theme only: background #FAFAF8, primary text #1A1A2E, accent #E85D2A, hairline borders rgba(26,26,46,0.12); Sora headings, Inter body, JetBrains Mono labels
- [req_4] Minimalism style enforcement: 60%+ whitespace, 48–96px section padding, 1px hairline borders, single restrained accent, zero decorative clutter, asymmetric editorial layouts, no card-in-card nesting, no 3-column icon-in-circle grids, no gradients or neon
- [req_6] Authentic agency copy only — no AI slop filler phrases; one unmistakable primary focal point per page with a clear secondary action

EXACT COPY & HIERARCHY:
Elevate Studio | Home | About | Services | Portfolio | Contact | Book a discovery call | A small studio with a deliberate approach. | We believe good design is quiet, useful, and honest. Since 2018 we''ve helped founders and marketing teams turn complex ideas into clear digital experiences. | Our Values | 01 — Clarity over cleverness | 02 — Evidence over opinion | 03 — Craft over volume | 04 — Partnership over handoff | By the numbers | 48 | Projects shipped | 12 | Industry awards | 96% | Client retention | 7 | Years in practice | Sound like your kind of studio? | Book a discovery call | © 2025 Elevate Studio

ATTACHED REFERENCE ROLES:
- Attached Visual Anchor Page Reference (`item_1` (''Home — Visual Anchor Landing Page'', Route `/`)) [IMAGE-TO-IMAGE MULTI-PAGE CONTINUITY]: This attached image is Page 1 (the Visual Anchor) of the same multi-page website/webapp. Use Image-to-Image conditioning to keep the EXACT SAME sticky top navigation bar (identical brand logo & wordmark on the left, identical horizontal nav link typography/spacing in the center with the active pill/underline indicator moved to route `/about`, identical CTA button on the right, identical header height, surface color, and border), the EXACT SAME footer chrome, and the EXACT SAME brand color palette, typography, border-radius, card borders, and shadow depth, while rendering the completely new page content and layout for ''About — Studio Story & Philosophy Page'' (Route `/about`) below the navbar!

INVARIANT RENDERING CONSTRAINTS:
- Direct, full-bleed viewport UI design reference (do NOT place the UI inside an angled laptop/phone hardware mockup or external desk scene, and do NOT include a browser URL bar).
- Strictly execute the Chosen Web Design Style definition and visual rules above in every surface, border, shadow, color contrast, and typographic detail.
- Render crisp, legible typography, accurate button alignment, accessible contrast, and realistic spacing.
- VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NEVER A HORIZONTAL POWERPOINT SLIDE): Render the entire page flowing vertically from top to bottom — starting with the top navigation bar at the very top edge, followed vertically by the Hero section, Feature sections, Pricing/Content sections, and ending with the Footer at the bottom edge.
- INTERNAL BREAKPOINT DIMENSIONS = DESKTOP (`1440px` WIDE DESKTOP GRID INSIDE VERTICAL FULL-PAGE SCROLL): Even though the image canvas is vertical (`2:3` / `9:16`) so the whole landing page is visible from top to bottom, the UI INSIDE the image MUST be a true `1440px`-wide Desktop layout — full horizontal desktop navbar (brand logo left, horizontal nav links center, CTA buttons right — NO mobile hamburger menu), 12-column desktop container with generous left/right desktop margins, 2-column desktop Hero, and 3-to-4 column side-by-side desktop Feature & Pricing cards in horizontal rows! Do NOT render a narrow single-column mobile phone layout!
- IMAGE-TO-IMAGE MULTI-PAGE CONSISTENCY WITH ATTACHED VISUAL ANCHOR PAGE (`item_1`): 1) IDENTICAL STICKY TOP NAVBAR & FOOTER: Render the exact same top navigation bar at the top edge as shown in the attached Page 1 reference image (same logo mark & wordmark on the left, same nav link items in the center with the active state on `/about`, same right-side action button, same navbar height, background surface, and border) and the same footer at the bottom edge. 2) DISTINCT PAGE BODY FOR ''About — Studio Story & Philosophy Page'' (Route `/about`): Below the shared top navbar, do NOT repeat Page 1''s hero section! Instead, render the complete top-to-bottom UI specifically for ''About — Studio Story & Philosophy Page'' (Minimalist editorial About page sharing item_1''s exact navbar and footer via Image-to-Image. Studio story as a large editorial statement, numbered mono-indexed values list, hairline-divided stats with tabular numerals, closing CTA.) — showing every section, dashboard KPI tile, chart, filter bar, directory grid, form control, or table specified in the prompt above.
', '[]', '["item_1"]', '{"id":"item_2","kind":"page","name":"About — Studio Story & Philosophy Page","tab_key":"full-website","tab_label":"Full Website","page_route":"/about","variant_index":1,"viewport":{"intent":"desktop","css_width":1440},"aspect_ratio":"2:3","transparent_background":false,"extracted_from_item_id":null,"target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_1","req_2","req_3","req_4","req_6"],"direction":"Minimalist editorial About page sharing item_1''s exact navbar and footer via Image-to-Image. Studio story as a large editorial statement, numbered mono-indexed values list, hairline-divided stats with tabular numerals, closing CTA.","exact_copy":["Elevate Studio","Home","About","Services","Portfolio","Contact","Book a discovery call","A small studio with a deliberate approach.","We believe good design is quiet, useful, and honest. Since 2018 we''ve helped founders and marketing teams turn complex ideas into clear digital experiences.","Our Values","01 — Clarity over cleverness","02 — Evidence over opinion","03 — Craft over volume","04 — Partnership over handoff","By the numbers","48","Projects shipped","12","Industry awards","96%","Client retention","7","Years in practice","Sound like your kind of studio?","Book a discovery call","© 2025 Elevate Studio"],"reference_asset_ids":[],"depends_on":["item_1"],"image_prompt":"A tall vertical full-page scroll capture of a 1440px-wide desktop About page for the same minimalist creative agency, using the EXACT same sticky top navigation bar as the attached item_1 reference image (identical monogram logo and ''Elevate Studio'' wordmark left, identical nav links with the active underline indicator now on ''About'', identical orange ''Book a discovery call'' button right, identical header height, surface #FAFAF8, and hairline bottom border) and the identical footer. Page content top-to-bottom on warm off-white: a page header with mono eyebrow ''ABOUT — 01'' and a large 48px Sora headline ''A small studio with a deliberate approach.'' as the single dominant focal point, followed by a 5/7 asymmetric split where the left column holds a muted Inter paragraph and the right holds a large 28px Sora editorial statement. Studio philosophy section: a wide editorial block of two left-aligned Inter paragraphs with generous whitespace. ''Our Values'' section: a vertical numbered list of four rows separated by 1px hairlines, each with a JetBrains Mono index (''01''–''04''), value title in 24px Sora, and a short muted description — no cards, no icons. ''By the numbers'' stats section: a single horizontal row of four large tabular-numeral figures (48, 12, 96%, 7) in 40px Sora with mono uppercase captions beneath, separated by vertical 1px hairlines instead of cards. Closing conversion band: large Sora line ''Sound like your kind of studio?'' with the orange CTA button, then the identical hairline-bordered footer with copyright. 60%+ whitespace, 96px section rhythm, no gradients, no clutter, restrained single accent.","implementation_notes":["POSITIONING (STICKY VS NON-STICKY): The shared top navbar remains anchored at the viewport top on scroll; the story, values, stats, and CTA sections scroll in normal flow","PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: The stats figures and value descriptions are editable sample content for the user''s future data; the layout, hairline dividers, typography, and CTA must be coded 1:1 in code","CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Hairline dividers, vertical stat separators, and background surfaces are pure code; the logo and any small UI icons come from the extracted transparent assets converted to SVG; no 3D objects on this page","FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Values rows reveal with a staggered fade-up on scroll; stat figures count up once when scrolled into view using exponential ease-out; the shared nav indicator slides to the About route; CTA uses the same crisp spring press (stiffness 320, damping 28); reduced-motion swaps all transforms for opacity fades","REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No reference attached; build fresh and reuse the shared navbar and footer components from item_1 exactly"],"behaviors":["HOVER STATE: Value rows hover to a faint warm surface tint with the mono index turning accent #E85D2A; CTA hover darkens to #D14E1F with 2px lift; nav links hover to accent","CLICK / ACTIVE STATE: CTA press compresses to 0.98 scale on #C2461B; active nav link ''About'' carries the accent underline indicator","FOCUS & DISABLED STATES: 2px accent outline with offset on keyboard focus; disabled states at 40% opacity"],"responsive_rules":["Desktop 1440px: 5/7 editorial split, four-across stats row with vertical hairlines","Tablet 834px: stats become a 2x2 grid with horizontal hairlines, split collapses to single column","Mobile 390px: fully stacked single column, stats as a vertical list, 48px padding"],"asset_instructions":[],"is_visual_anchor":false}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T20:14:17.936486+00:00', '2026-10-24T20:15:03.830966+00:00', 'full-website', 'Full Website', NULL, 0, NULL);
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-elevate-studio-5', 'example-run-elevate-studio', 'example-elevate-studio', 'item_1', 1, 'page', 'Home — Visual Anchor Landing Page', 'Minimalist editorial agency homepage on warm off-white #FAFAF8. One dominant oversized Sora headline as the primary anchor, a single accent-orange CTA, hairline-separated sections with 96px rhythm, asymmetric 5/7 splits, mono uppercase indices. Zero clutter, zero gradients.', 'visual_anchor', '/', '2:3', 'completed', 'showcases/elevate-studio/05.png', 'showcases/elevate-studio/05.webp', 'image/png', 1024, 1536, 1510164, 0, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (page)
- Direction / Item Title: Home — Visual Anchor Landing Page (Variant #1)
- Viewport & Aspect Ratio: desktop (1440px target width), Aspect Ratio 2:3
- Scope Rules: VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NOT A HORIZONTAL POWERPOINT SLIDE): Complete coordinated web page flowing vertically from top to bottom (Navbar at top -> Hero -> Features -> Pricing -> Footer at bottom) with internal column grids and dimensions strictly matching the target viewport breakpoint.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Elevate Studio — A professional multi-page website for Elevate Studio, a creative agency offering web design, branding, and digital strategy, designed to convert visitors into consultation bookings.
- Product / Application Description: Tagline: Digital Experiences That Make an Impact.

A professional website for a creative agency offering web design, branding, and digital strategy. Includes dedicated Home, About, Services, Portfolio, and Contact pages, with cohesive branding and engaging layouts.
- Industry & Target Audience: Creative Agency / Digital Design Services | Audience: Startup founders, marketing leads, and business owners seeking premium web design, branding, and digital strategy services (A credible, polished showcase of agency work, clear service explanations, proof of results, and a frictionless way to start a project conversation)
- Primary User Goal / Conversion Action: Book a discovery call or submit a project inquiry via the Contact page

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #1 Minimalism
- Exact Style Definition: Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content.
- Mandatory Visual & CSS System Rules: Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "minimalism". Primary style #1 Minimalism (Minimal & Editorial): Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, refined, and quietly bold — a studio that lets its work speak through restraint and precision — A minimalist, editorial-driven site with generous whitespace, razor-thin hairline borders, and a single confident accent color. Large expressive headlines, spacious 48px–96px section padding, and a sticky top navigation guide visitors through Home, About, Services, Portfolio, and Contact. Portfolio work is showcased in a clean interactive grid; services are presented as focused benefit cards; the contact flow is a simple, elegant conversion form.
- UI Density & Tone: comfortable / Clear, assured, and impact-focused — concise statements that promise measurable results without hype (English)
- Color Palette: Primary #1A1A2E, Accent #E85D2A, Background #FAFAF8, Surface #FFFFFF, Text #16161F, Muted #6B6B76, Border #E4E4DE
- Typography Intent: Heading Sora (or similar geometric sans) — 48px/56px hero, 24px section headings, tight tracking, Body Inter — 15px–16px body, 1.6 line height, generous paragraph spacing, Mono JetBrains Mono — for labels, indices, and technical metadata
- Radii, Borders & Shadows: Radius 12px cards, 8px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(0,0,0,0.06), hover: 0 12px 32px rgba(0,0,0,0.10)
- Logo Instruction: Use a clean temporary monogram ''E'' integrated with the brand name ''Elevate Studio'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E4E4DE), and subtle surface luminance shifts (#FAFAF8 vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#16161F on #FAFAF8/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Minimalist editorial agency homepage on warm off-white #FAFAF8. One dominant oversized Sora headline as the primary anchor, a single accent-orange CTA, hairline-separated sections with 96px rhythm, asymmetric 5/7 splits, mono uppercase indices. Zero clutter, zero gradients.

PLANNER IMAGE PROMPT:
A tall vertical full-page scroll capture of a 1440px-wide desktop website homepage for a minimalist creative agency, rendered top-to-bottom on a warm off-white background (#FAFAF8) with ink text (#1A1A2E) and a single burnt-orange accent (#E85D2A). Sticky top navigation bar: small geometric monogram logo and ''Elevate Studio'' wordmark on the left, five horizontal nav links (Home active with accent underline, About, Services, Portfolio, Contact) centered, and a solid burnt-orange rounded button ''Book a discovery call'' with a small right-arrow on the right; a 1px hairline border under the header. Hero section with 96px padding: a mono uppercase eyebrow label ''CREATIVE STUDIO — EST. 2018'', then an enormous 56px+ tight-tracked Sora headline ''Digital Experiences That Make an Impact'' spanning two lines as the page''s single dominant focal point, a short left-aligned Inter paragraph beneath, and the orange CTA button plus a quiet secondary text link ''View our work'' with an arrow. Editorial intro section split asymmetrically 5/7 columns: left a mono label ''ABOUT'', right a large 28px Sora statement paragraph. Selected Work section: a 2-column asymmetric portfolio teaser grid with two large placeholder image frames of different aspect ratios (one tall, one wide) framed by hairline borders, each with a mono category label and project name in Sora below; a ''View all projects'' text link. Services index: a vertical list of three rows separated by 1px hairlines, each row with a JetBrains Mono index (''01'', ''02'', ''03''), service name in large Sora, and a right-aligned arrow; no cards, no icons in circles. Client proof strip: a single hairline-bordered row with five muted wordmark texts spaced evenly. Footer: large Sora closing line ''Let''s build something that lasts.'', orange CTA button, email in mono, hairline top border, small copyright row. Generous 60%+ whitespace throughout, razor-thin hairlines instead of shadows, no gradients, no decorative blobs, no card-in-card nesting, authentic restrained agency aesthetic.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_1] Five distinct desktop pages: Home (/), About (/about), Services (/services), Portfolio (/portfolio), Contact (/contact), each as a separate vertical 2:3 page item rendered at 1440px desktop width
- [req_2] Pages 2–5 use Image-to-Image conditioning on Page 1 to share the exact same sticky top navbar (Elevate Studio logo, Home/About/Services/Portfolio/Contact links, accent CTA button) and identical footer, palette, and typography
- [req_3] Light theme only: background #FAFAF8, primary text #1A1A2E, accent #E85D2A, hairline borders rgba(26,26,46,0.12); Sora headings, Inter body, JetBrains Mono labels
- [req_4] Minimalism style enforcement: 60%+ whitespace, 48–96px section padding, 1px hairline borders, single restrained accent, zero decorative clutter, asymmetric editorial layouts, no card-in-card nesting, no 3-column icon-in-circle grids, no gradients or neon
- [req_5] Primary conversion: Book a discovery call CTA and a frictionless contact form (name, email, project type, budget, message) with hairline input underlines
- [req_6] Authentic agency copy only — no AI slop filler phrases; one unmistakable primary focal point per page with a clear secondary action

EXACT COPY & HIERARCHY:
Elevate Studio | Home | About | Services | Portfolio | Contact | Book a discovery call | Digital Experiences That Make an Impact | We are a creative studio partnering with ambitious teams to design websites, brands, and digital strategies that perform — measured in outcomes, not adjectives. | Selected Work | View all projects | Services | 01 — Web Design | 02 — Branding | 03 — Digital Strategy | Trusted by teams at | Northwind | Fielder & Co. | Meridian Labs | Atlas Health | Corvus | Let''s build something that lasts. | Book a discovery call | hello@elevate.studio | © 2025 Elevate Studio

ATTACHED REFERENCE ROLES:
None attached.

INVARIANT RENDERING CONSTRAINTS:
- Direct, full-bleed viewport UI design reference (do NOT place the UI inside an angled laptop/phone hardware mockup or external desk scene, and do NOT include a browser URL bar).
- Strictly execute the Chosen Web Design Style definition and visual rules above in every surface, border, shadow, color contrast, and typographic detail.
- Render crisp, legible typography, accurate button alignment, accessible contrast, and realistic spacing.
- VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NEVER A HORIZONTAL POWERPOINT SLIDE): Render the entire page flowing vertically from top to bottom — starting with the top navigation bar at the very top edge, followed vertically by the Hero section, Feature sections, Pricing/Content sections, and ending with the Footer at the bottom edge.
- INTERNAL BREAKPOINT DIMENSIONS = DESKTOP (`1440px` WIDE DESKTOP GRID INSIDE VERTICAL FULL-PAGE SCROLL): Even though the image canvas is vertical (`2:3` / `9:16`) so the whole landing page is visible from top to bottom, the UI INSIDE the image MUST be a true `1440px`-wide Desktop layout — full horizontal desktop navbar (brand logo left, horizontal nav links center, CTA buttons right — NO mobile hamburger menu), 12-column desktop container with generous left/right desktop margins, 2-column desktop Hero, and 3-to-4 column side-by-side desktop Feature & Pricing cards in horizontal rows! Do NOT render a narrow single-column mobile phone layout!
- VISUAL ANCHOR PAGE (`item_1`): Establish the canonical sticky top navigation bar (brand logo left, clear route links for all pages of the webapp center, primary CTA right), brand surface palette, typography, border radius, shadow depth, and footer chrome that all subsequent pages in this multi-page set will inherit via Image-to-Image conditioning.
', '[]', '[]', '{"id":"item_1","kind":"page","name":"Home — Visual Anchor Landing Page","tab_key":"full-website","tab_label":"Full Website","page_route":"/","variant_index":1,"viewport":{"intent":"desktop","css_width":1440},"aspect_ratio":"2:3","transparent_background":false,"extracted_from_item_id":null,"target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_1","req_2","req_3","req_4","req_5","req_6"],"direction":"Minimalist editorial agency homepage on warm off-white #FAFAF8. One dominant oversized Sora headline as the primary anchor, a single accent-orange CTA, hairline-separated sections with 96px rhythm, asymmetric 5/7 splits, mono uppercase indices. Zero clutter, zero gradients.","exact_copy":["Elevate Studio","Home","About","Services","Portfolio","Contact","Book a discovery call","Digital Experiences That Make an Impact","We are a creative studio partnering with ambitious teams to design websites, brands, and digital strategies that perform — measured in outcomes, not adjectives.","Selected Work","View all projects","Services","01 — Web Design","02 — Branding","03 — Digital Strategy","Trusted by teams at","Northwind","Fielder & Co.","Meridian Labs","Atlas Health","Corvus","Let''s build something that lasts.","Book a discovery call","hello@elevate.studio","© 2025 Elevate Studio"],"reference_asset_ids":[],"depends_on":[],"image_prompt":"A tall vertical full-page scroll capture of a 1440px-wide desktop website homepage for a minimalist creative agency, rendered top-to-bottom on a warm off-white background (#FAFAF8) with ink text (#1A1A2E) and a single burnt-orange accent (#E85D2A). Sticky top navigation bar: small geometric monogram logo and ''Elevate Studio'' wordmark on the left, five horizontal nav links (Home active with accent underline, About, Services, Portfolio, Contact) centered, and a solid burnt-orange rounded button ''Book a discovery call'' with a small right-arrow on the right; a 1px hairline border under the header. Hero section with 96px padding: a mono uppercase eyebrow label ''CREATIVE STUDIO — EST. 2018'', then an enormous 56px+ tight-tracked Sora headline ''Digital Experiences That Make an Impact'' spanning two lines as the page''s single dominant focal point, a short left-aligned Inter paragraph beneath, and the orange CTA button plus a quiet secondary text link ''View our work'' with an arrow. Editorial intro section split asymmetrically 5/7 columns: left a mono label ''ABOUT'', right a large 28px Sora statement paragraph. Selected Work section: a 2-column asymmetric portfolio teaser grid with two large placeholder image frames of different aspect ratios (one tall, one wide) framed by hairline borders, each with a mono category label and project name in Sora below; a ''View all projects'' text link. Services index: a vertical list of three rows separated by 1px hairlines, each row with a JetBrains Mono index (''01'', ''02'', ''03''), service name in large Sora, and a right-aligned arrow; no cards, no icons in circles. Client proof strip: a single hairline-bordered row with five muted wordmark texts spaced evenly. Footer: large Sora closing line ''Let''s build something that lasts.'', orange CTA button, email in mono, hairline top border, small copyright row. Generous 60%+ whitespace throughout, razor-thin hairlines instead of shadows, no gradients, no decorative blobs, no card-in-card nesting, authentic restrained agency aesthetic.","implementation_notes":["POSITIONING (STICKY VS NON-STICKY): The top navigation bar stays fixed to the top of the viewport on scroll with a solid #FAFAF8 surface and a 1px hairline bottom border; the hero, work grid, services index, proof strip, and footer all scroll naturally in normal document flow","PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: The two portfolio teaser images and the five client wordmarks are placeholder sample content for the user''s future CMS/database; only the layout frames, hairline borders, typography, buttons, and grid structure are to be implemented 1:1 in code right now, with image slots wired to a data source later","CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: All section separators, hairline rules, hover underline indicators, and the flat off-white background are built purely with code; the monogram logo and nav/CTA/UI icons come from the extracted transparent assets and must be converted into scalable vectorial SVG graphics; this page contains no 3D objects, so no Image-to-3D reconstruction is needed","FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Page route transitions use a quick fade-and-slight-upward drift with exponential ease-out; the active nav link indicator slides smoothly between routes; sections reveal with a staggered fade-up on scroll; the CTA button uses a crisp spring press (stiffness 320, damping 28) with the arrow nudging right on hover; service index rows expand a hairline accent underline on hover; all motion respects the user''s reduced-motion preference by disabling transforms and using simple opacity fades","REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No existing screenshot was attached, so build all components fresh; if a prior header or hero exists in the codebase, replace it to match this design 1:1 while preserving routes and data bindings","CODED SEPARATOR / BACKGROUND SPEC: Section separators: The page uses thin, solid, light-gray horizontal rule lines (approximately one pixel, a warm light gray around #E5E2DC) that span the full content width to divide every major section — below the header nav, between the hero and About, between About and Selected Work, between Selected Work and Services, between Services and the Trusted By logo strip, and above the footer. In the Services list, each service row (Web Design, Branding, Digital Strategy) is separated by the same thin light rule line, with the final row closed by a rule beneath it. Recreate all of these separators 1:1 in code as full-width hairline borders with identical color and thickness — no gradients, no glow.","CODED SEPARATOR / BACKGROUND SPEC: Background layers: The entire page sits on a single warm off-white/cream canvas (approximately #F5F3EF) with no visible texture, grain, or grid. The header bar is the same cream tone, and the hero, About, Selected Work, Services, Trusted By, and CTA sections all share this continuous background with no color transitions between sections. The two hero CTAs and the CTA-section button use a solid burnt-orange fill (approximately #E05A1E) with white text and slightly rounded corners. Recreate this background 1:1 in code as a flat warm cream color with the orange accent buttons built in code — do not use a background screenshot.","CODED SEPARATOR / BACKGROUND SPEC: Typography and layout rhythm as coded structure: The design relies on a monospace uppercase micro-label style (letter-spaced, small, e.g. ''CREATIVE STUDIO — EST. 2018'', ''ABOUT'', ''SELECTED WORK'', ''SERVICES'', ''TRUSTED BY TEAMS AT'') paired with a large dark charcoal serif-adjacent sans headline face. The hero right side features a large photo block with a vertical ''01 / 03'' slide counter and a small right-aligned monospace list (''STRATEGY / DESIGN / REAL RESULTS''). Recreate these labels, counters, and spacing rhythm exactly in code as text and layout, not images.","CODED SEPARATOR / BACKGROUND SPEC: Animation note: No 3D objects or animated centerpiece props are visible in this image; the only motion-relevant elements are the hero image carousel counter (01/03) and arrow-link hover states, which should be implemented as simple code-driven transitions (arrow slide on hover, carousel counter update) rather than 3D reconstruction."],"behaviors":["HOVER STATE: Primary CTA hover darkens the button to a deeper burnt orange (#D14E1F) with a subtle upward lift of 2px and the arrow translating right; nav links hover to accent #E85D2A with a hairline underline; portfolio frames hover to show a soft 0 12px 32px shadow and a ''View case study'' mono hint","CLICK / ACTIVE STATE: CTA press compresses scale to 0.98 with background #C2461B; active nav link shows the accent underline indicator and full ink color; service rows press to a faint warm surface tint #F4F2EE","FOCUS & DISABLED STATES: Keyboard focus shows a 2px accent #E85D2A outline with 2px offset on all interactive elements; disabled buttons render at 40% opacity with no hover lift"],"responsive_rules":["Desktop 1440px: full multi-column asymmetric layouts as shown, 96px section padding","Tablet 834px: collapse to a 2-column grid, hero headline scales to 40px, services list keeps hairline rows, nav condenses with links intact where possible","Mobile 390px: single column stacking, hamburger menu replaces nav links, hero at 32–36px, portfolio grid becomes one column, 48px section padding"],"asset_instructions":["CODED SEPARATOR / BACKGROUND SPEC: Section separators: The page uses thin, solid, light-gray horizontal rule lines (approximately one pixel, a warm light gray around #E5E2DC) that span the full content width to divide every major section — below the header nav, between the hero and About, between About and Selected Work, between Selected Work and Services, between Services and the Trusted By logo strip, and above the footer. In the Services list, each service row (Web Design, Branding, Digital Strategy) is separated by the same thin light rule line, with the final row closed by a rule beneath it. Recreate all of these separators 1:1 in code as full-width hairline borders with identical color and thickness — no gradients, no glow.","CODED SEPARATOR / BACKGROUND SPEC: Background layers: The entire page sits on a single warm off-white/cream canvas (approximately #F5F3EF) with no visible texture, grain, or grid. The header bar is the same cream tone, and the hero, About, Selected Work, Services, Trusted By, and CTA sections all share this continuous background with no color transitions between sections. The two hero CTAs and the CTA-section button use a solid burnt-orange fill (approximately #E05A1E) with white text and slightly rounded corners. Recreate this background 1:1 in code as a flat warm cream color with the orange accent buttons built in code — do not use a background screenshot.","CODED SEPARATOR / BACKGROUND SPEC: Typography and layout rhythm as coded structure: The design relies on a monospace uppercase micro-label style (letter-spaced, small, e.g. ''CREATIVE STUDIO — EST. 2018'', ''ABOUT'', ''SELECTED WORK'', ''SERVICES'', ''TRUSTED BY TEAMS AT'') paired with a large dark charcoal serif-adjacent sans headline face. The hero right side features a large photo block with a vertical ''01 / 03'' slide counter and a small right-aligned monospace list (''STRATEGY / DESIGN / REAL RESULTS''). Recreate these labels, counters, and spacing rhythm exactly in code as text and layout, not images.","CODED SEPARATOR / BACKGROUND SPEC: Animation note: No 3D objects or animated centerpiece props are visible in this image; the only motion-relevant elements are the hero image carousel counter (01/03) and arrow-link hover states, which should be implemented as simple code-driven transitions (arrow slide on hover, carousel counter update) rather than 3D reconstruction."],"is_visual_anchor":true}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T20:14:17.936486+00:00', '2026-10-24T20:14:47.619270+00:00', 'full-website', 'Full Website', NULL, 0, NULL);
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-elevate-studio-6', 'example-run-elevate-studio', 'example-elevate-studio', 'item_7', 7, 'asset', 'Site Icon Sheet — 2px Geometric Stroke Icons (Transparent)', 'One organized icon sheet of crisp 2px geometric stroke icons in ink #1A1A2E on a transparent background, evenly spaced in a grid for SVG conversion.', 'asset', NULL, '1:1', 'completed', 'showcases/elevate-studio/06.png', 'showcases/elevate-studio/06.webp', 'image/png', 1024, 1024, 631334, 1, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (asset)
- Direction / Item Title: Site Icon Sheet — 2px Geometric Stroke Icons (Transparent) (Variant #1)
- Viewport & Aspect Ratio: isolated (1440px target width), Aspect Ratio 1:1
- Scope Rules: Standalone visual site asset (icon sheet, illustration, 3D cutout, or foreground graphic) rendered on a 100% native transparent background so it can be layered directly into the coordinated page/component set.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Elevate Studio — A professional multi-page website for Elevate Studio, a creative agency offering web design, branding, and digital strategy, designed to convert visitors into consultation bookings.
- Product / Application Description: Tagline: Digital Experiences That Make an Impact.

A professional website for a creative agency offering web design, branding, and digital strategy. Includes dedicated Home, About, Services, Portfolio, and Contact pages, with cohesive branding and engaging layouts.
- Industry & Target Audience: Creative Agency / Digital Design Services | Audience: Startup founders, marketing leads, and business owners seeking premium web design, branding, and digital strategy services (A credible, polished showcase of agency work, clear service explanations, proof of results, and a frictionless way to start a project conversation)
- Primary User Goal / Conversion Action: Book a discovery call or submit a project inquiry via the Contact page

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #1 Minimalism
- Exact Style Definition: Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content.
- Mandatory Visual & CSS System Rules: Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "minimalism". Primary style #1 Minimalism (Minimal & Editorial): Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, refined, and quietly bold — a studio that lets its work speak through restraint and precision — A minimalist, editorial-driven site with generous whitespace, razor-thin hairline borders, and a single confident accent color. Large expressive headlines, spacious 48px–96px section padding, and a sticky top navigation guide visitors through Home, About, Services, Portfolio, and Contact. Portfolio work is showcased in a clean interactive grid; services are presented as focused benefit cards; the contact flow is a simple, elegant conversion form.
- UI Density & Tone: comfortable / Clear, assured, and impact-focused — concise statements that promise measurable results without hype (English)
- Color Palette: Primary #1A1A2E, Accent #E85D2A, Background #FAFAF8, Surface #FFFFFF, Text #16161F, Muted #6B6B76, Border #E4E4DE
- Typography Intent: Heading Sora (or similar geometric sans) — 48px/56px hero, 24px section headings, tight tracking, Body Inter — 15px–16px body, 1.6 line height, generous paragraph spacing, Mono JetBrains Mono — for labels, indices, and technical metadata
- Radii, Borders & Shadows: Radius 12px cards, 8px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(0,0,0,0.06), hover: 0 12px 32px rgba(0,0,0,0.10)
- Logo Instruction: Use a clean temporary monogram ''E'' integrated with the brand name ''Elevate Studio'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E4E4DE), and subtle surface luminance shifts (#FAFAF8 vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#16161F on #FAFAF8/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
One organized icon sheet of crisp 2px geometric stroke icons in ink #1A1A2E on a transparent background, evenly spaced in a grid for SVG conversion.

PLANNER IMAGE PROMPT:
An organized icon sheet isolated on a clean 100% transparent background with zero UI chrome, zero text labels, and zero background clutter: a neat evenly-spaced grid of eight minimalist line icons drawn with crisp uniform 2px geometric strokes in deep ink #1A1A2E — an arrow pointing right, a mail envelope, a telephone handset, a map pin, a clock face, a plus sign, an external-link arrow leaving a bracket, and a simple menu line pair. Each icon sits in its own invisible square cell with generous consistent spacing, perfectly aligned, flat vector style, no fills, no shadows, no gradients, no rounded-square containers.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_3] Light theme only: background #FAFAF8, primary text #1A1A2E, accent #E85D2A, hairline borders rgba(26,26,46,0.12); Sora headings, Inter body, JetBrains Mono labels
- [req_4] Minimalism style enforcement: 60%+ whitespace, 48–96px section padding, 1px hairline borders, single restrained accent, zero decorative clutter, asymmetric editorial layouts, no card-in-card nesting, no 3-column icon-in-circle grids, no gradients or neon

EXACT COPY & HIERARCHY:
Use concise, realistic copy matching brand language.

ATTACHED REFERENCE ROLES:
- Prerequisite Hero/Component Reference Image [FOREGROUND ASSET EXTRACTION]: Extract and faithfully reproduce ONLY the specified foreground illustration, 3D object, icon set, or graphic element from this reference UI image onto a 100% transparent background (`background: transparent`). Strip away all background walls, gradients, UI buttons, and text!

INVARIANT RENDERING CONSTRAINTS:
- Render a clean, production-ready standalone website asset matching the chosen Web Design Style above.
- Do NOT wrap the asset in browser chrome, UI buttons, or device frames.
- NATIVE TRANSPARENT BACKGROUND (`background: "transparent"`): Isolate the subject (or organized icon sheet grid) on a 100% true transparent alpha background. Do NOT draw a fake checkerboard grid, do NOT add opaque background fills or vignettes, and keep clean alpha edges with a 10% safety margin around the silhouette.
- FOREGROUND EXTRACTION FROM COMPOSITE UI: Reproduce the exact foreground visual asset shown in the attached prerequisite Hero/Component reference image at full resolution on a transparent background, completely omitting the background environment, headlines, and UI buttons so this transparent PNG can be layered over the coded background.
', '[]', '["item_1"]', '{"id":"item_7","kind":"asset","name":"Site Icon Sheet — 2px Geometric Stroke Icons (Transparent)","tab_key":"full-website","tab_label":"Full Website","page_route":null,"variant_index":1,"viewport":{"intent":"isolated","css_width":1440},"aspect_ratio":"1:1","transparent_background":true,"extracted_from_item_id":"item_1","target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_3","req_4"],"direction":"One organized icon sheet of crisp 2px geometric stroke icons in ink #1A1A2E on a transparent background, evenly spaced in a grid for SVG conversion.","exact_copy":[],"reference_asset_ids":[],"depends_on":["item_1"],"image_prompt":"An organized icon sheet isolated on a clean 100% transparent background with zero UI chrome, zero text labels, and zero background clutter: a neat evenly-spaced grid of eight minimalist line icons drawn with crisp uniform 2px geometric strokes in deep ink #1A1A2E — an arrow pointing right, a mail envelope, a telephone handset, a map pin, a clock face, a plus sign, an external-link arrow leaving a bracket, and a simple menu line pair. Each icon sits in its own invisible square cell with generous consistent spacing, perfectly aligned, flat vector style, no fills, no shadows, no gradients, no rounded-square containers.","implementation_notes":["POSITIONING (STICKY VS NON-STICKY): These icons are used inline within buttons, form labels, the contact sidebar, and navigation; they are not standalone layout blocks","PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: These are real reusable UI icons, not placeholder content; each maps to a specific interface use (arrow for CTAs, mail/phone/pin/clock for the contact sidebar, plus and external-link for portfolio actions, menu lines for the mobile hamburger)","CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Every icon must be traced and converted into scalable vectorial SVG graphics with 2px strokes so they render crisply at 16–24px; none are raster cutouts and none are 3D objects","FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: The arrow icon nudges right on CTA hover; the menu icon animates between two lines and a close cross when the mobile menu opens; all icon transitions use exponential ease-out and respect reduced-motion","REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No reference attached; use these as the canonical icon set, replacing any default icon library usage where the design calls for custom marks"],"behaviors":["HOVER STATE: Icons inside interactive elements inherit the parent hover color, shifting from ink #1A1A2E to accent #E85D2A","CLICK / ACTIVE STATE: Icons inside pressed buttons compress with the parent''s 0.98 scale spring","FOCUS & DISABLED STATES: Icons inherit the parent''s 2px accent focus outline; disabled icons render at 40% opacity"],"responsive_rules":["Desktop and tablet: icons render at 20–24px inline with text","Mobile: icons render at 18–20px with touch targets kept at a minimum 44px interactive area"],"asset_instructions":["Slice each icon from the transparent sheet and convert it into a scalable vectorial SVG graphic with a 2px stroke width, currentColor stroke, and no fill, so icons recolor instantly with the brand tokens and stay razor-sharp at every size"],"is_visual_anchor":false}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T20:14:17.936486+00:00', '2026-10-24T20:15:53.723226+00:00', 'full-website', 'Full Website', 'example-out-elevate-studio-5', 1, 'example-out-elevate-studio-5');
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-elevate-studio-7', 'example-run-elevate-studio', 'example-elevate-studio', 'item_4', 4, 'page', 'Portfolio — Project Grid Page', 'Minimalist Portfolio page sharing item_1''s chrome via Image-to-Image. Varied-aspect-ratio project grid with hairline frames, mono category labels, hover hints, case-study CTA.', 'page', '/portfolio', '2:3', 'completed', 'showcases/elevate-studio/07.png', 'showcases/elevate-studio/07.webp', 'image/png', 1024, 1536, 1706217, 0, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (page)
- Direction / Item Title: Portfolio — Project Grid Page (Variant #1)
- Viewport & Aspect Ratio: desktop (1440px target width), Aspect Ratio 2:3
- Scope Rules: VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NOT A HORIZONTAL POWERPOINT SLIDE): Complete coordinated web page flowing vertically from top to bottom (Navbar at top -> Hero -> Features -> Pricing -> Footer at bottom) with internal column grids and dimensions strictly matching the target viewport breakpoint.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Elevate Studio — A professional multi-page website for Elevate Studio, a creative agency offering web design, branding, and digital strategy, designed to convert visitors into consultation bookings.
- Product / Application Description: Tagline: Digital Experiences That Make an Impact.

A professional website for a creative agency offering web design, branding, and digital strategy. Includes dedicated Home, About, Services, Portfolio, and Contact pages, with cohesive branding and engaging layouts.
- Industry & Target Audience: Creative Agency / Digital Design Services | Audience: Startup founders, marketing leads, and business owners seeking premium web design, branding, and digital strategy services (A credible, polished showcase of agency work, clear service explanations, proof of results, and a frictionless way to start a project conversation)
- Primary User Goal / Conversion Action: Book a discovery call or submit a project inquiry via the Contact page

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #1 Minimalism
- Exact Style Definition: Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content.
- Mandatory Visual & CSS System Rules: Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "minimalism". Primary style #1 Minimalism (Minimal & Editorial): Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, refined, and quietly bold — a studio that lets its work speak through restraint and precision — A minimalist, editorial-driven site with generous whitespace, razor-thin hairline borders, and a single confident accent color. Large expressive headlines, spacious 48px–96px section padding, and a sticky top navigation guide visitors through Home, About, Services, Portfolio, and Contact. Portfolio work is showcased in a clean interactive grid; services are presented as focused benefit cards; the contact flow is a simple, elegant conversion form.
- UI Density & Tone: comfortable / Clear, assured, and impact-focused — concise statements that promise measurable results without hype (English)
- Color Palette: Primary #1A1A2E, Accent #E85D2A, Background #FAFAF8, Surface #FFFFFF, Text #16161F, Muted #6B6B76, Border #E4E4DE
- Typography Intent: Heading Sora (or similar geometric sans) — 48px/56px hero, 24px section headings, tight tracking, Body Inter — 15px–16px body, 1.6 line height, generous paragraph spacing, Mono JetBrains Mono — for labels, indices, and technical metadata
- Radii, Borders & Shadows: Radius 12px cards, 8px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(0,0,0,0.06), hover: 0 12px 32px rgba(0,0,0,0.10)
- Logo Instruction: Use a clean temporary monogram ''E'' integrated with the brand name ''Elevate Studio'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E4E4DE), and subtle surface luminance shifts (#FAFAF8 vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#16161F on #FAFAF8/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Minimalist Portfolio page sharing item_1''s chrome via Image-to-Image. Varied-aspect-ratio project grid with hairline frames, mono category labels, hover hints, case-study CTA.

PLANNER IMAGE PROMPT:
A tall vertical full-page scroll capture of a 1440px-wide desktop Portfolio page for the same minimalist creative agency, using the EXACT same sticky top navigation bar as the attached item_1 reference image (identical logo, links with the active underline now on ''Portfolio'', identical orange CTA button, identical header surface and hairline border) and identical footer. Content top-to-bottom on #FAFAF8: page header with mono eyebrow ''PORTFOLIO — 03'' and a large 48px Sora headline ''Work that earns its keep.'' as the dominant focal point, plus a horizontal filter row of five quiet text pills (All active in ink with an accent underline, Web Design, Branding, Strategy) separated by whitespace. Project grid: a clean two-column masonry-style grid of five placeholder project frames with deliberately varied aspect ratios (one tall 3:4, one wide 4:3, one near-square) separated by generous gutters, each frame a flat neutral placeholder surface framed by a 1px hairline with restrained overlay typography — project name in 20px Sora and a JetBrains Mono category-year label (''Web Design — 2025'') beneath the frame, never on top of busy imagery; one frame shows a subtle hover-state hint with a soft shadow and a mono ''View case study'' tag in the corner. Closing conversion band: Sora line ''Start your project'' with the orange CTA button, then the identical footer. 60%+ whitespace, no card-in-card nesting, no gradients, restrained single accent.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_1] Five distinct desktop pages: Home (/), About (/about), Services (/services), Portfolio (/portfolio), Contact (/contact), each as a separate vertical 2:3 page item rendered at 1440px desktop width
- [req_2] Pages 2–5 use Image-to-Image conditioning on Page 1 to share the exact same sticky top navbar (Elevate Studio logo, Home/About/Services/Portfolio/Contact links, accent CTA button) and identical footer, palette, and typography
- [req_3] Light theme only: background #FAFAF8, primary text #1A1A2E, accent #E85D2A, hairline borders rgba(26,26,46,0.12); Sora headings, Inter body, JetBrains Mono labels
- [req_4] Minimalism style enforcement: 60%+ whitespace, 48–96px section padding, 1px hairline borders, single restrained accent, zero decorative clutter, asymmetric editorial layouts, no card-in-card nesting, no 3-column icon-in-circle grids, no gradients or neon
- [req_6] Authentic agency copy only — no AI slop filler phrases; one unmistakable primary focal point per page with a clear secondary action

EXACT COPY & HIERARCHY:
Elevate Studio | Home | About | Services | Portfolio | Contact | Book a discovery call | Work that earns its keep. | All | Web Design | Branding | Strategy | Northwind Commerce | Web Design — 2025 | Meridian Labs | Branding — 2024 | Atlas Health | Web Design — 2024 | Corvus Analytics | Strategy — 2025 | Fielder & Co. | Branding — 2023 | Start your project | Book a discovery call | © 2025 Elevate Studio

ATTACHED REFERENCE ROLES:
- Attached Visual Anchor Page Reference (`item_1` (''Home — Visual Anchor Landing Page'', Route `/`)) [IMAGE-TO-IMAGE MULTI-PAGE CONTINUITY]: This attached image is Page 1 (the Visual Anchor) of the same multi-page website/webapp. Use Image-to-Image conditioning to keep the EXACT SAME sticky top navigation bar (identical brand logo & wordmark on the left, identical horizontal nav link typography/spacing in the center with the active pill/underline indicator moved to route `/portfolio`, identical CTA button on the right, identical header height, surface color, and border), the EXACT SAME footer chrome, and the EXACT SAME brand color palette, typography, border-radius, card borders, and shadow depth, while rendering the completely new page content and layout for ''Portfolio — Project Grid Page'' (Route `/portfolio`) below the navbar!

INVARIANT RENDERING CONSTRAINTS:
- Direct, full-bleed viewport UI design reference (do NOT place the UI inside an angled laptop/phone hardware mockup or external desk scene, and do NOT include a browser URL bar).
- Strictly execute the Chosen Web Design Style definition and visual rules above in every surface, border, shadow, color contrast, and typographic detail.
- Render crisp, legible typography, accurate button alignment, accessible contrast, and realistic spacing.
- VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NEVER A HORIZONTAL POWERPOINT SLIDE): Render the entire page flowing vertically from top to bottom — starting with the top navigation bar at the very top edge, followed vertically by the Hero section, Feature sections, Pricing/Content sections, and ending with the Footer at the bottom edge.
- INTERNAL BREAKPOINT DIMENSIONS = DESKTOP (`1440px` WIDE DESKTOP GRID INSIDE VERTICAL FULL-PAGE SCROLL): Even though the image canvas is vertical (`2:3` / `9:16`) so the whole landing page is visible from top to bottom, the UI INSIDE the image MUST be a true `1440px`-wide Desktop layout — full horizontal desktop navbar (brand logo left, horizontal nav links center, CTA buttons right — NO mobile hamburger menu), 12-column desktop container with generous left/right desktop margins, 2-column desktop Hero, and 3-to-4 column side-by-side desktop Feature & Pricing cards in horizontal rows! Do NOT render a narrow single-column mobile phone layout!
- IMAGE-TO-IMAGE MULTI-PAGE CONSISTENCY WITH ATTACHED VISUAL ANCHOR PAGE (`item_1`): 1) IDENTICAL STICKY TOP NAVBAR & FOOTER: Render the exact same top navigation bar at the top edge as shown in the attached Page 1 reference image (same logo mark & wordmark on the left, same nav link items in the center with the active state on `/portfolio`, same right-side action button, same navbar height, background surface, and border) and the same footer at the bottom edge. 2) DISTINCT PAGE BODY FOR ''Portfolio — Project Grid Page'' (Route `/portfolio`): Below the shared top navbar, do NOT repeat Page 1''s hero section! Instead, render the complete top-to-bottom UI specifically for ''Portfolio — Project Grid Page'' (Minimalist Portfolio page sharing item_1''s chrome via Image-to-Image. Varied-aspect-ratio project grid with hairline frames, mono category labels, hover hints, case-study CTA.) — showing every section, dashboard KPI tile, chart, filter bar, directory grid, form control, or table specified in the prompt above.
', '[]', '["item_1"]', '{"id":"item_4","kind":"page","name":"Portfolio — Project Grid Page","tab_key":"full-website","tab_label":"Full Website","page_route":"/portfolio","variant_index":1,"viewport":{"intent":"desktop","css_width":1440},"aspect_ratio":"2:3","transparent_background":false,"extracted_from_item_id":null,"target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_1","req_2","req_3","req_4","req_6"],"direction":"Minimalist Portfolio page sharing item_1''s chrome via Image-to-Image. Varied-aspect-ratio project grid with hairline frames, mono category labels, hover hints, case-study CTA.","exact_copy":["Elevate Studio","Home","About","Services","Portfolio","Contact","Book a discovery call","Work that earns its keep.","All","Web Design","Branding","Strategy","Northwind Commerce","Web Design — 2025","Meridian Labs","Branding — 2024","Atlas Health","Web Design — 2024","Corvus Analytics","Strategy — 2025","Fielder & Co.","Branding — 2023","Start your project","Book a discovery call","© 2025 Elevate Studio"],"reference_asset_ids":[],"depends_on":["item_1"],"image_prompt":"A tall vertical full-page scroll capture of a 1440px-wide desktop Portfolio page for the same minimalist creative agency, using the EXACT same sticky top navigation bar as the attached item_1 reference image (identical logo, links with the active underline now on ''Portfolio'', identical orange CTA button, identical header surface and hairline border) and identical footer. Content top-to-bottom on #FAFAF8: page header with mono eyebrow ''PORTFOLIO — 03'' and a large 48px Sora headline ''Work that earns its keep.'' as the dominant focal point, plus a horizontal filter row of five quiet text pills (All active in ink with an accent underline, Web Design, Branding, Strategy) separated by whitespace. Project grid: a clean two-column masonry-style grid of five placeholder project frames with deliberately varied aspect ratios (one tall 3:4, one wide 4:3, one near-square) separated by generous gutters, each frame a flat neutral placeholder surface framed by a 1px hairline with restrained overlay typography — project name in 20px Sora and a JetBrains Mono category-year label (''Web Design — 2025'') beneath the frame, never on top of busy imagery; one frame shows a subtle hover-state hint with a soft shadow and a mono ''View case study'' tag in the corner. Closing conversion band: Sora line ''Start your project'' with the orange CTA button, then the identical footer. 60%+ whitespace, no card-in-card nesting, no gradients, restrained single accent.","implementation_notes":["POSITIONING (STICKY VS NON-STICKY): The shared navbar stays fixed at the top; the filter row, project grid, and CTA band scroll in normal flow","PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: All five project images, names, and category-year labels are placeholder sample content for the user''s future portfolio database; only the grid frames, hairline borders, filter pills, typography, and hover treatments are coded 1:1 in code right now, with image slots bound to a data source later","CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Grid frames, hairlines, filter pill indicators, and hover overlays are pure code; logo and small UI icons (filter arrows, external-link marks) come from extracted transparent assets converted to SVG; no 3D objects on this page","FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Filter pills reflow the grid with a smooth layout animation and staggered fade-in of matching frames; project frames lift with a soft shadow and reveal the ''View case study'' mono tag on hover using exponential ease-out; the shared nav indicator slides to Portfolio; CTA spring press (stiffness 320, damping 28); reduced-motion disables transforms and layout shifts, using opacity fades only","REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No reference attached; reuse the shared navbar and footer components from item_1 exactly"],"behaviors":["HOVER STATE: Project frames hover to shadow elevation 0 12px 32px rgba(0,0,0,0.10) with a 2px lift and the mono ''View case study'' tag fading in; filter pills hover to ink text with an accent underline; CTA hover darkens to #D14E1F","CLICK / ACTIVE STATE: Active filter pill shows ink text with a persistent accent underline; CTA press compresses to 0.98 on #C2461B; active nav link ''Portfolio'' carries the accent underline","FOCUS & DISABLED STATES: 2px accent focus outline with offset on frames and pills; disabled filters at 40% opacity"],"responsive_rules":["Desktop 1440px: two-column varied-aspect masonry grid with generous gutters","Tablet 834px: two-column grid with uniform gutters, filter row wraps","Mobile 390px: single-column stacked project frames, horizontally scrollable filter row, 48px padding"],"asset_instructions":[],"is_visual_anchor":false}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T20:14:17.936486+00:00', '2026-10-24T20:15:28.542285+00:00', 'full-website', 'Full Website', NULL, 0, NULL);
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-elevate-studio-8', 'example-run-elevate-studio', 'example-elevate-studio', 'item_1_auto_asset_1', 8, 'asset', 'Elevate Studio Brand Logo & Wordmark (Transparent)', 'Minimal geometric ''E'' monogram in dark charcoal paired with the ''Elevate Studio'' wordmark, matching the top-left header lockup.', 'asset', NULL, '1:1', 'completed', 'showcases/elevate-studio/08.png', 'showcases/elevate-studio/08.webp', 'image/png', 1024, 1024, 520882, 1, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (asset)
- Direction / Item Title: Elevate Studio Brand Logo & Wordmark (Transparent) (Variant #8)
- Viewport & Aspect Ratio: isolated (1024px target width), Aspect Ratio 1:1
- Scope Rules: Standalone visual site asset (icon sheet, illustration, 3D cutout, or foreground graphic) rendered on a 100% native transparent background so it can be layered directly into the coordinated page/component set.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Elevate Studio — A professional multi-page website for Elevate Studio, a creative agency offering web design, branding, and digital strategy, designed to convert visitors into consultation bookings.
- Product / Application Description: Tagline: Digital Experiences That Make an Impact.

A professional website for a creative agency offering web design, branding, and digital strategy. Includes dedicated Home, About, Services, Portfolio, and Contact pages, with cohesive branding and engaging layouts.
- Industry & Target Audience: Creative Agency / Digital Design Services | Audience: Startup founders, marketing leads, and business owners seeking premium web design, branding, and digital strategy services (A credible, polished showcase of agency work, clear service explanations, proof of results, and a frictionless way to start a project conversation)
- Primary User Goal / Conversion Action: Book a discovery call or submit a project inquiry via the Contact page

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #1 Minimalism
- Exact Style Definition: Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content.
- Mandatory Visual & CSS System Rules: Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "minimalism". Primary style #1 Minimalism (Minimal & Editorial): Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, refined, and quietly bold — a studio that lets its work speak through restraint and precision — A minimalist, editorial-driven site with generous whitespace, razor-thin hairline borders, and a single confident accent color. Large expressive headlines, spacious 48px–96px section padding, and a sticky top navigation guide visitors through Home, About, Services, Portfolio, and Contact. Portfolio work is showcased in a clean interactive grid; services are presented as focused benefit cards; the contact flow is a simple, elegant conversion form.
- UI Density & Tone: comfortable / Clear, assured, and impact-focused — concise statements that promise measurable results without hype (English)
- Color Palette: Primary #1A1A2E, Accent #E85D2A, Background #FAFAF8, Surface #FFFFFF, Text #16161F, Muted #6B6B76, Border #E4E4DE
- Typography Intent: Heading Sora (or similar geometric sans) — 48px/56px hero, 24px section headings, tight tracking, Body Inter — 15px–16px body, 1.6 line height, generous paragraph spacing, Mono JetBrains Mono — for labels, indices, and technical metadata
- Radii, Borders & Shadows: Radius 12px cards, 8px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(0,0,0,0.06), hover: 0 12px 32px rgba(0,0,0,0.10)
- Logo Instruction: Use a clean temporary monogram ''E'' integrated with the brand name ''Elevate Studio'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E4E4DE), and subtle surface luminance shifts (#FAFAF8 vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#16161F on #FAFAF8/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Minimal geometric ''E'' monogram in dark charcoal paired with the ''Elevate Studio'' wordmark, matching the top-left header lockup.

PLANNER IMAGE PROMPT:
Extract ONLY the Elevate Studio brand logo lockup from the top-left corner of the attached reference UI image: the bold geometric ''E'' monogram mark plus the ''Elevate Studio'' wordmark text beside it, in dark charcoal. Isolate it on a clean 100% transparent background with all header background, nav links, and unrelated elements removed. Reproduce the exact letterform geometry, weight, and spacing faithfully.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_1] Five distinct desktop pages: Home (/), About (/about), Services (/services), Portfolio (/portfolio), Contact (/contact), each as a separate vertical 2:3 page item rendered at 1440px desktop width

EXACT COPY & HIERARCHY:
Use concise, realistic copy matching brand language.

ATTACHED REFERENCE ROLES:
- Prerequisite Hero/Component Reference Image [FOREGROUND ASSET EXTRACTION]: Extract and faithfully reproduce ONLY the specified foreground illustration, 3D object, icon set, or graphic element from this reference UI image onto a 100% transparent background (`background: transparent`). Strip away all background walls, gradients, UI buttons, and text!

INVARIANT RENDERING CONSTRAINTS:
- Render a clean, production-ready standalone website asset matching the chosen Web Design Style above.
- Do NOT wrap the asset in browser chrome, UI buttons, or device frames.
- NATIVE TRANSPARENT BACKGROUND (`background: "transparent"`): Isolate the subject (or organized icon sheet grid) on a 100% true transparent alpha background. Do NOT draw a fake checkerboard grid, do NOT add opaque background fills or vignettes, and keep clean alpha edges with a 10% safety margin around the silhouette.
- FOREGROUND EXTRACTION FROM COMPOSITE UI: Reproduce the exact foreground visual asset shown in the attached prerequisite Hero/Component reference image at full resolution on a transparent background, completely omitting the background environment, headlines, and UI buttons so this transparent PNG can be layered over the coded background.
', '[]', '["item_1"]', '{"id":"item_1_auto_asset_1","kind":"asset","name":"Elevate Studio Brand Logo & Wordmark (Transparent)","tab_key":"full-website","tab_label":"Full Website","page_route":null,"variant_index":8,"viewport":{"intent":"isolated","css_width":1024},"aspect_ratio":"1:1","transparent_background":true,"extracted_from_item_id":"item_1","target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_1"],"direction":"Minimal geometric ''E'' monogram in dark charcoal paired with the ''Elevate Studio'' wordmark, matching the top-left header lockup.","exact_copy":[],"reference_asset_ids":[],"depends_on":["item_1"],"image_prompt":"Extract ONLY the Elevate Studio brand logo lockup from the top-left corner of the attached reference UI image: the bold geometric ''E'' monogram mark plus the ''Elevate Studio'' wordmark text beside it, in dark charcoal. Isolate it on a clean 100% transparent background with all header background, nav links, and unrelated elements removed. Reproduce the exact letterform geometry, weight, and spacing faithfully.","implementation_notes":["Render on a 100% transparent PNG background and layer in the component/page layout"],"behaviors":[],"responsive_rules":["Scale proportionally across breakpoints"],"asset_instructions":["Place this logo lockup in the top-left of the site header and reuse it in the footer copyright area. Convert it into a vectorial SVG graphic so it scales crisply; do not rasterize it into a card or banner."],"is_visual_anchor":false}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T20:14:47.619270+00:00', '2026-10-24T20:16:06.745294+00:00', 'full-website', 'Full Website', 'example-out-elevate-studio-5', 1, 'example-out-elevate-studio-5');
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-elevate-studio-9', 'example-run-elevate-studio', 'example-elevate-studio', 'item_1_auto_asset_2', 9, 'asset', 'Social Media Icon Set (Transparent)', 'Three thin-line social icons — LinkedIn, Instagram, and Twitter/X — in dark charcoal, matching the footer''s minimal icon row.', 'asset', NULL, '1:1', 'completed', 'showcases/elevate-studio/09.png', 'showcases/elevate-studio/09.webp', 'image/png', 1024, 1024, 508033, 1, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (asset)
- Direction / Item Title: Social Media Icon Set (Transparent) (Variant #9)
- Viewport & Aspect Ratio: isolated (1024px target width), Aspect Ratio 1:1
- Scope Rules: Standalone visual site asset (icon sheet, illustration, 3D cutout, or foreground graphic) rendered on a 100% native transparent background so it can be layered directly into the coordinated page/component set.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Elevate Studio — A professional multi-page website for Elevate Studio, a creative agency offering web design, branding, and digital strategy, designed to convert visitors into consultation bookings.
- Product / Application Description: Tagline: Digital Experiences That Make an Impact.

A professional website for a creative agency offering web design, branding, and digital strategy. Includes dedicated Home, About, Services, Portfolio, and Contact pages, with cohesive branding and engaging layouts.
- Industry & Target Audience: Creative Agency / Digital Design Services | Audience: Startup founders, marketing leads, and business owners seeking premium web design, branding, and digital strategy services (A credible, polished showcase of agency work, clear service explanations, proof of results, and a frictionless way to start a project conversation)
- Primary User Goal / Conversion Action: Book a discovery call or submit a project inquiry via the Contact page

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #1 Minimalism
- Exact Style Definition: Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content.
- Mandatory Visual & CSS System Rules: Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "minimalism". Primary style #1 Minimalism (Minimal & Editorial): Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Enforce 60%+ open whitespace, razor-thin 1px borders (#E6E4E0), restrained neutral palette with a single high-contrast ink accent, zero decorative clutter, and generous padding (48px–96px section rhythm).

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, refined, and quietly bold — a studio that lets its work speak through restraint and precision — A minimalist, editorial-driven site with generous whitespace, razor-thin hairline borders, and a single confident accent color. Large expressive headlines, spacious 48px–96px section padding, and a sticky top navigation guide visitors through Home, About, Services, Portfolio, and Contact. Portfolio work is showcased in a clean interactive grid; services are presented as focused benefit cards; the contact flow is a simple, elegant conversion form.
- UI Density & Tone: comfortable / Clear, assured, and impact-focused — concise statements that promise measurable results without hype (English)
- Color Palette: Primary #1A1A2E, Accent #E85D2A, Background #FAFAF8, Surface #FFFFFF, Text #16161F, Muted #6B6B76, Border #E4E4DE
- Typography Intent: Heading Sora (or similar geometric sans) — 48px/56px hero, 24px section headings, tight tracking, Body Inter — 15px–16px body, 1.6 line height, generous paragraph spacing, Mono JetBrains Mono — for labels, indices, and technical metadata
- Radii, Borders & Shadows: Radius 12px cards, 8px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(0,0,0,0.06), hover: 0 12px 32px rgba(0,0,0,0.10)
- Logo Instruction: Use a clean temporary monogram ''E'' integrated with the brand name ''Elevate Studio'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E4E4DE), and subtle surface luminance shifts (#FAFAF8 vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#16161F on #FAFAF8/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Three thin-line social icons — LinkedIn, Instagram, and Twitter/X — in dark charcoal, matching the footer''s minimal icon row.

PLANNER IMAGE PROMPT:
Extract ONLY the three small social media icons from the bottom-right footer of the attached reference UI image: the LinkedIn, Instagram, and Twitter/X glyphs in thin dark charcoal line style. Isolate each glyph on a clean 100% transparent background with the footer background and all text removed, faithfully reproducing stroke weight and proportions.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_1] Five distinct desktop pages: Home (/), About (/about), Services (/services), Portfolio (/portfolio), Contact (/contact), each as a separate vertical 2:3 page item rendered at 1440px desktop width

EXACT COPY & HIERARCHY:
Use concise, realistic copy matching brand language.

ATTACHED REFERENCE ROLES:
- Prerequisite Hero/Component Reference Image [FOREGROUND ASSET EXTRACTION]: Extract and faithfully reproduce ONLY the specified foreground illustration, 3D object, icon set, or graphic element from this reference UI image onto a 100% transparent background (`background: transparent`). Strip away all background walls, gradients, UI buttons, and text!

INVARIANT RENDERING CONSTRAINTS:
- Render a clean, production-ready standalone website asset matching the chosen Web Design Style above.
- Do NOT wrap the asset in browser chrome, UI buttons, or device frames.
- NATIVE TRANSPARENT BACKGROUND (`background: "transparent"`): Isolate the subject (or organized icon sheet grid) on a 100% true transparent alpha background. Do NOT draw a fake checkerboard grid, do NOT add opaque background fills or vignettes, and keep clean alpha edges with a 10% safety margin around the silhouette.
- FOREGROUND EXTRACTION FROM COMPOSITE UI: Reproduce the exact foreground visual asset shown in the attached prerequisite Hero/Component reference image at full resolution on a transparent background, completely omitting the background environment, headlines, and UI buttons so this transparent PNG can be layered over the coded background.
', '[]', '["item_1"]', '{"id":"item_1_auto_asset_2","kind":"asset","name":"Social Media Icon Set (Transparent)","tab_key":"full-website","tab_label":"Full Website","page_route":null,"variant_index":9,"viewport":{"intent":"isolated","css_width":1024},"aspect_ratio":"1:1","transparent_background":true,"extracted_from_item_id":"item_1","target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_1"],"direction":"Three thin-line social icons — LinkedIn, Instagram, and Twitter/X — in dark charcoal, matching the footer''s minimal icon row.","exact_copy":[],"reference_asset_ids":[],"depends_on":["item_1"],"image_prompt":"Extract ONLY the three small social media icons from the bottom-right footer of the attached reference UI image: the LinkedIn, Instagram, and Twitter/X glyphs in thin dark charcoal line style. Isolate each glyph on a clean 100% transparent background with the footer background and all text removed, faithfully reproducing stroke weight and proportions.","implementation_notes":["Render on a 100% transparent PNG background and layer in the component/page layout"],"behaviors":[],"responsive_rules":["Scale proportionally across breakpoints"],"asset_instructions":["Place these icons in the footer''s bottom-right social row as inline vectorial SVG graphics with hover color shift to the brand orange. Keep them as icons, not embedded raster images."],"is_visual_anchor":false}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T20:14:47.619270+00:00', '2026-10-24T20:16:15.244297+00:00', 'full-website', 'Full Website', 'example-out-elevate-studio-5', 1, 'example-out-elevate-studio-5');
INSERT INTO thread_messages (id, thread_id, role, content, created_at) VALUES ('example-message-elevate-studio-1', 'example-thread-elevate-studio', 'user', 'Tagline: Digital Experiences That Make an Impact.

A professional website for a creative agency offering web design, branding, and digital strategy. Includes dedicated Home, About, Services, Portfolio, and Contact pages, with cohesive branding and engaging layouts.', '2026-10-24T20:13:07.680267+00:00');
INSERT INTO thread_messages (id, thread_id, role, content, linked_run_id, created_at) VALUES ('example-message-elevate-studio-2', 'example-thread-elevate-studio', 'assistant', 'Elevate Studio showcase, including the finished designs and extracted assets. Open any image to inspect it or copy its implementation prompt.', 'example-run-elevate-studio', '2026-10-24T20:16:15.257209+00:00');

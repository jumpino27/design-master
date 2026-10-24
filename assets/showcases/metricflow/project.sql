INSERT INTO projects (id, name, description, purpose, industry, target_audience, audience_needs, desired_user_action, archive_status, created_at, updated_at, is_example) VALUES ('example-metricflow', 'MetricFlow', 'Tagline: Your Business. Every Insight. One Dashboard.

An advanced business analytics dashboard for tracking revenue, customer growth, projects, and performance. Includes interactive charts, KPI cards, activity feeds, data tables, reporting tools, and sidebar navigation.', 'A unified business analytics dashboard that consolidates revenue, customer growth, project status, and performance KPIs into one interactive, real-time command center with charts, activity feeds, data tables, and reporting tools.', 'SaaS / Business Intelligence & Analytics', 'Founders, operations leads, product managers, and data-driven teams at startups and mid-size companies who need fast, self-serve visibility into business health.', 'Quick at-a-glance KPIs, drill-down charts, clean data tables, shareable reports, and a calm, low-noise interface that keeps dense information scannable without visual fatigue.', 'Log in daily to monitor KPIs, explore insights via interactive charts, and generate/share reports — driving habitual engagement and conversion to a paid plan.', 'active', '2026-10-24T19:24:50.137122+00:00', '2026-10-24T19:24:50.137122+00:00', 1);
INSERT INTO brand_versions (id, project_id, version, brand_json, created_at) VALUES ('example-brand-metricflow-1', 'example-metricflow', 1, '{"brand_personality": "Precise, confident, and quietly intelligent — a trusted analytical co-pilot, not a flashy toy.", "design_direction": "Semi-flat dashboard aesthetic: clean flat surfaces with subtle layered elevation, bento-grid KPI cards, crisp 1px hairlines, and a single confident indigo accent reserved for data highlights and primary actions. Charts and numbers are the heroes; chrome stays neutral and quiet.", "tone_of_voice": "Clear, direct, and data-forward. Short labels, active verbs, no hype. ''Revenue up 12.4%'' — not ''Amazing growth!''.", "content_language": "English", "ui_density": "comfortable", "logo": {"mode": "temporary_monogram", "monogram_text": "M", "primary_asset_id": null, "mark_only_asset_id": null, "light_variant_asset_id": null, "dark_variant_asset_id": null, "preferred_usage": "Top-left header or card header mark with clear breathing room", "inclusion_policy": "allowed"}, "colors": {"light": {"primary": "#4F46E5", "accent": "#06B6D4", "background": "#F8FAFC", "surface": "#FFFFFF", "text": "#0F172A", "muted_text": "#64748B", "border": "#E2E8F0", "success": "#2B7A4B", "warning": "#B7791F", "danger": "#C53030"}, "dark": {"primary": "#818CF8", "accent": "#22D3EE", "background": "#0B1120", "surface": "#151E31", "text": "#F1F5F9", "muted_text": "#94A3B8", "border": "#27324A", "success": "#38A169", "warning": "#D69E2E", "danger": "#E53E3E"}}, "typography": {"heading_family": "Inter, 600–700 weight, tight tracking (-0.02em); scale 48/32/24/18px", "body_family": "Inter, 400–500 weight, 14–15px, 1.5 line-height for tables and feeds", "mono_family": "IBM Plex Mono, for metric values, IDs, and timestamps", "fallback_stack": "-apple-system, BlinkMacSystemFont, ''Segoe UI'', sans-serif", "weights": [400, 500, 600, 700], "size_scale": "12px / 14px / 16px / 20px / 28px / 40px", "line_heights": "Heading 1.15, Body 1.5, Compact UI 1.3", "font_source": "google_fonts", "uploaded_font_asset_ids": []}, "layout_tokens": {"spacing_scale": "4px / 8px / 16px / 24px / 32px / 48px / 64px", "content_width": "1200px max container with 24px padding", "grid_columns": "12-column responsive grid, 24px gap", "radii": "12px cards, 8px buttons, 6px inputs and badges", "borders": "1px solid subtle warm border (#E2DED5)", "shadows": "0 8px 24px rgba(0,0,0,0.06); hover lift 0 12px 32px rgba(0,0,0,0.10)", "icon_style": "2px crisp geometric stroke icons (Lucide-style), 20px grid", "button_treatment": "Solid high-contrast primary CTA with subtle arrow affordance"}, "brand_rules": ["Use the indigo primary strictly for primary actions, active nav state, and key data series; cyan accent only for secondary data series and positive deltas.", "Compose the dashboard as a bento grid of KPI cards with consistent 24px gutters; every card carries a label, mono value, delta badge, and sparkline.", "Apply subtle multi-layered elevation shadows on interactive cards only; static panels use 1px hairline borders.", "Support full light/dark parity — dark mode surfaces must keep WCAG AA contrast for all text and chart labels.", "Keep chart palettes to primary, accent, and two neutral grays; never exceed four series per chart.", "Sidebar navigation stays fixed at 240px with grouped, icon+label items and a collapsible state."], "mandatory_inclusions": [], "forbidden_treatments": ["No heavy gradients, glassmorphism blur, or neon glow effects on data surfaces.", "No more than one accent color per chart series group; no rainbow charts.", "No decorative illustrations inside KPI or data-table cards.", "No hard offset shadows, thick borders, or neobrutalist styling.", "No serif or display fonts in the dashboard UI; no all-caps body text."], "approved_reference_asset_ids": [], "target_context": {"target_stack": "Next.js App Router + TypeScript + Tailwind CSS", "repository_root": "", "preferred_asset_dir": "public/assets", "component_library_notes": "Accessible Radix primitives with Lucide icons"}, "design_style": {"style_id": "11-semi-flat-design", "style_number": 11, "style_name": "Semi-flat Design", "definition": "Keeps the simplicity of flat design while adding subtle shadows, layering, or gradients to communicate depth and interactivity.", "category": "Dimensional & Tactile", "visual_rules": "Maintain clean flat surfaces enhanced with subtle multi-layered ambient shadows (0 8px 24px rgba(15,23,42,0.08)), floating interactive cards, micro-gradients on primary CTAs, and clear z-index elevation cues.", "keywords": ["semi-flat", "flat 2.0", "layered", "subtle shadows", "elevation", "depth", "interactive", "material", "modern ui", "cards"], "modifiers": [], "combined_brief": "Style brief: \"semi-flat design\". Primary style #11 Semi-flat Design (Dimensional & Tactile): Keeps the simplicity of flat design while adding subtle shadows, layering, or gradients to communicate depth and interactivity. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Maintain clean flat surfaces enhanced with subtle multi-layered ambient shadows (0 8px 24px rgba(15,23,42,0.08)), floating interactive cards, micro-gradients on primary CTAs, and clear z-index elevation cues."}}', '2026-10-24T19:24:50.137122+00:00');
INSERT INTO threads (id, project_id, title, archive_status, created_at, updated_at) VALUES ('example-thread-metricflow', 'example-metricflow', 'Main Design Workspace', 'active', '2026-10-24T19:24:50.137122+00:00', '2026-10-24T20:10:36.843032+00:00');
INSERT INTO runs (id, project_id, thread_id, idempotency_key, mode, scope_preset, user_prompt, brand_version, brand_snapshot_json, selected_references_json, model_settings_json, plan_json, consistency_report_json, status, estimated_cost_micros, actual_cost_micros, cost_is_unknown, is_test_adapter, error_message, created_at, updated_at, completed_at, target_tab_key, target_tab_label, target_output_id, highlight_annotations_json) VALUES ('example-run-metricflow', 'example-metricflow', 'example-thread-metricflow', NULL, 'component', 'page_view', 'Design the main dashboard screen for MetricFlow, a SaaS business analytics platform (tagline: ''Your Business. Every Insight. One Dashboard.'') — a unified real-time command center for founders, ops leads, and product managers. LIGHT THEME, clean and airy: background #F8FAFC, white surfaces #FFFFFF, primary indigo #4F46E5 reserved strictly for data highlights and the primary CTA, cyan accent #06B6D4 for secondary data series. Style: Semi-flat Design (#11) — clean flat surfaces with subtle layered elevation (soft small shadows on hover-level cards), crisp 1px hairlines (#E2E8F0) separating regions, no heavy gradients, no purple-blue SaaS gradient slop, no random floating blobs. Layout: left sidebar navigation (logo, nav items with 2px crisp geometric Lucide-style 20px stroke icons: Overview active, Revenue, Customers, Projects, Reports, Settings; user avatar at bottom) + main content area with a slim top bar (search, date-range selector, ''Export report'' solid indigo primary button with subtle arrow affordance, notification bell). Main content: a bento-grid of KPI cards (Monthly Revenue, Active Customers, Projects On Track, Churn Rate) with IBM Plex Mono metric values, delta indicators, and one mini sparkline each; a large primary revenue-over-time area chart as the unmistakable #1 focal point with indigo line + cyan comparison series; a secondary customer-growth bar chart; a right-column activity feed with timestamps in mono; and a clean dense data table of recent projects with status pills and sortable column headers. Typography: Inter — headings 600–700 weight, tight tracking (-0.02em), scale 24/18px for in-app use; body 400–500, 14–15px, 1.5 line-height for tables and feeds; IBM Plex Mono for all metric values, IDs, and timestamps. Anti-AI-slop craft (pbakaus/impeccable standard): no card-in-card nesting, intentional whitespace, asymmetric bento proportions, one clear focal chart, generous breathing room, low-noise calm chrome so dense data stays scannable; authentic analytics terminology (MRR, ARR, retention, pipeline), no fake filler copy. Desktop 1440px app screen, 16:9. UI mockup only — no code.', 1, '{"brand_personality": "Precise, confident, and quietly intelligent — a trusted analytical co-pilot, not a flashy toy.", "design_direction": "Semi-flat dashboard aesthetic: clean flat surfaces with subtle layered elevation, bento-grid KPI cards, crisp 1px hairlines, and a single confident indigo accent reserved for data highlights and primary actions. Charts and numbers are the heroes; chrome stays neutral and quiet.", "tone_of_voice": "Clear, direct, and data-forward. Short labels, active verbs, no hype. ''Revenue up 12.4%'' — not ''Amazing growth!''.", "content_language": "English", "ui_density": "comfortable", "logo": {"mode": "temporary_monogram", "monogram_text": "M", "primary_asset_id": null, "mark_only_asset_id": null, "light_variant_asset_id": null, "dark_variant_asset_id": null, "preferred_usage": "Top-left header or card header mark with clear breathing room", "inclusion_policy": "allowed"}, "colors": {"light": {"primary": "#4F46E5", "accent": "#06B6D4", "background": "#F8FAFC", "surface": "#FFFFFF", "text": "#0F172A", "muted_text": "#64748B", "border": "#E2E8F0", "success": "#2B7A4B", "warning": "#B7791F", "danger": "#C53030"}, "dark": {"primary": "#818CF8", "accent": "#22D3EE", "background": "#0B1120", "surface": "#151E31", "text": "#F1F5F9", "muted_text": "#94A3B8", "border": "#27324A", "success": "#38A169", "warning": "#D69E2E", "danger": "#E53E3E"}}, "typography": {"heading_family": "Inter, 600–700 weight, tight tracking (-0.02em); scale 48/32/24/18px", "body_family": "Inter, 400–500 weight, 14–15px, 1.5 line-height for tables and feeds", "mono_family": "IBM Plex Mono, for metric values, IDs, and timestamps", "fallback_stack": "-apple-system, BlinkMacSystemFont, ''Segoe UI'', sans-serif", "weights": [400, 500, 600, 700], "size_scale": "12px / 14px / 16px / 20px / 28px / 40px", "line_heights": "Heading 1.15, Body 1.5, Compact UI 1.3", "font_source": "google_fonts", "uploaded_font_asset_ids": []}, "layout_tokens": {"spacing_scale": "4px / 8px / 16px / 24px / 32px / 48px / 64px", "content_width": "1200px max container with 24px padding", "grid_columns": "12-column responsive grid, 24px gap", "radii": "12px cards, 8px buttons, 6px inputs and badges", "borders": "1px solid subtle warm border (#E2DED5)", "shadows": "0 8px 24px rgba(0,0,0,0.06); hover lift 0 12px 32px rgba(0,0,0,0.10)", "icon_style": "2px crisp geometric stroke icons (Lucide-style), 20px grid", "button_treatment": "Solid high-contrast primary CTA with subtle arrow affordance"}, "brand_rules": ["Use the indigo primary strictly for primary actions, active nav state, and key data series; cyan accent only for secondary data series and positive deltas.", "Compose the dashboard as a bento grid of KPI cards with consistent 24px gutters; every card carries a label, mono value, delta badge, and sparkline.", "Apply subtle multi-layered elevation shadows on interactive cards only; static panels use 1px hairline borders.", "Support full light/dark parity — dark mode surfaces must keep WCAG AA contrast for all text and chart labels.", "Keep chart palettes to primary, accent, and two neutral grays; never exceed four series per chart.", "Sidebar navigation stays fixed at 240px with grouped, icon+label items and a collapsible state."], "mandatory_inclusions": [], "forbidden_treatments": ["No heavy gradients, glassmorphism blur, or neon glow effects on data surfaces.", "No more than one accent color per chart series group; no rainbow charts.", "No decorative illustrations inside KPI or data-table cards.", "No hard offset shadows, thick borders, or neobrutalist styling.", "No serif or display fonts in the dashboard UI; no all-caps body text."], "approved_reference_asset_ids": [], "target_context": {"target_stack": "Next.js App Router + TypeScript + Tailwind CSS", "repository_root": "", "preferred_asset_dir": "public/assets", "component_library_notes": "Accessible Radix primitives with Lucide icons"}, "design_style": {"style_id": "11-semi-flat-design", "style_number": 11, "style_name": "Semi-flat Design", "definition": "Keeps the simplicity of flat design while adding subtle shadows, layering, or gradients to communicate depth and interactivity.", "category": "Dimensional & Tactile", "visual_rules": "Maintain clean flat surfaces enhanced with subtle multi-layered ambient shadows (0 8px 24px rgba(15,23,42,0.08)), floating interactive cards, micro-gradients on primary CTAs, and clear z-index elevation cues.", "keywords": ["semi-flat", "flat 2.0", "layered", "subtle shadows", "elevation", "depth", "interactive", "material", "modern ui", "cards"], "modifiers": [], "combined_brief": "Style brief: \"semi-flat design\". Primary style #11 Semi-flat Design (Dimensional & Tactile): Keeps the simplicity of flat design while adding subtle shadows, layering, or gradients to communicate depth and interactivity. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Maintain clean flat surfaces enhanced with subtle multi-layered ambient shadows (0 8px 24px rgba(15,23,42,0.08)), floating interactive cards, micro-gradients on primary CTAs, and clear z-index elevation cues."}}', '[]', '{"planner_model":"z-ai/glm-5.3-flash","image_model":"openai/gpt-image-2.5-sunburst","quality":"medium","aspect_ratio":"16:9","output_size":"provider_default","scope_preset":"page_view","custom_dimensions":null,"review_plan_first":false,"asset_options":null,"target_tab_key":"dashboard","target_tab_label":"Dashboard","target_existing_output_id":null,"viewport_intent":"desktop","highlight_annotations":[],"single_focused_output":true}', '{
  "schema_version": 1,
  "mode": "component",
  "request_summary": "Design the main dashboard screen for MetricFlow, a SaaS business analytics platform — a light-theme, semi-flat 1440px desktop app view with left sidebar navigation, slim top bar, asymmetric bento-grid KPI cards with mono metric values and sparklines, a dominant revenue-over-time area chart as the #1 focal point, a secondary customer-growth bar chart, an activity feed, and a dense recent-projects data table.",
  "requirements": [
    {
      "id": "req_1",
      "description": "Light theme: background #F8FAFC, white surfaces #FFFFFF, indigo primary #4F46E5 reserved for primary CTA and key data series, cyan accent #06B6D4 for secondary data series and positive deltas",
      "source": "user"
    },
    {
      "id": "req_2",
      "description": "Semi-flat Design (#11): flat surfaces with subtle layered elevation (0 8px 24px rgba(15,23,42,0.08)), crisp 1px hairlines #E2E8F0, no heavy gradients, no glassmorphism, no neon glow, no floating blobs",
      "source": "user"
    },
    {
      "id": "req_3",
      "description": "Left sidebar navigation at 240px: monogram logo, nav items with 2px crisp geometric Lucide-style 20px stroke icons (Overview active, Revenue, Customers, Projects, Reports, Settings), user avatar at bottom, collapsible",
      "source": "user"
    },
    {
      "id": "req_4",
      "description": "Slim top bar: search input, date-range selector, ''Export report'' solid indigo primary button with subtle arrow affordance, notification bell",
      "source": "user"
    },
    {
      "id": "req_5",
      "description": "Bento-grid of 4 KPI cards (Monthly Revenue, Active Customers, Projects On Track, Churn Rate) with IBM Plex Mono values, delta badges, and one mini sparkline each; 24px gutters; no decorative illustrations inside cards",
      "source": "user"
    },
    {
      "id": "req_6",
      "description": "Large revenue-over-time area chart as the unmistakable #1 visual anchor with indigo primary line and cyan comparison series; secondary customer-growth bar chart; max four series per chart, no rainbow charts",
      "source": "user"
    },
    {
      "id": "req_7",
      "description": "Right-column activity feed with mono timestamps and a dense recent-projects data table with status pills and sortable column headers",
      "source": "user"
    },
    {
      "id": "req_8",
      "description": "Typography: Inter headings 600–700 tight tracking (-0.02em) at 24/18px in-app scale; body 400–500 at 14–15px with 1.5 line-height for tables and feeds; IBM Plex Mono for all metric values, IDs, and timestamps; left-aligned data; tabular numerals",
      "source": "user"
    },
    {
      "id": "req_9",
      "description": "Anti-AI-slop craft: no card-in-card nesting, asymmetric bento proportions, one clear focal chart, generous whitespace, calm low-noise chrome, authentic analytics terminology (MRR, ARR, retention, pipeline), no filler copy",
      "source": "user"
    }
  ],
  "assumptions": [
    "This is a single self-contained dashboard screen component (app shell + main content), not a multi-page website, so only one canonical UI item is generated.",
    "All icons use Lucide conventions and will be implemented as SVG in code, so no separate icon-sheet asset image is needed.",
    "The temporary monogram logo is simple enough to be coded as SVG, so no extracted transparent asset image is generated.",
    "Sample KPI values, activity feed entries, and project rows are placeholder data structured for future database integration."
  ],
  "shared_design_spec": {
    "colors": {
      "background": "#F8FAFC",
      "text": "#0F172A",
      "surface": "#FFFFFF",
      "primary": "#4F46E5"
    },
    "typography": {
      "mono": "IBM Plex Mono for metric values, IDs, and timestamps with tabular numerals",
      "heading": "Inter, 600–700 weight, tight tracking -0.02em, in-app scale 24px/18px",
      "body": "Inter, 400–500 weight, 14–15px, 1.5 line-height for tables and feeds"
    },
    "spacing": {
      "base": "4px / 8px / 16px / 24px / 32px / 48px scale; 24px gutters in the bento grid"
    },
    "radii": {
      "card": "12px",
      "button": "8px"
    },
    "shadows": {
      "card": "0 8px 24px rgba(15,23,42,0.08) on interactive cards only; static panels use 1px solid #E2E8F0 hairline borders with no shadow; hover lift to 0 12px 32px rgba(15,23,42,0.10)"
    },
    "layout_rules": [
      "STICKY VS NON-STICKY: The left sidebar navigation (240px) and the slim top bar stay fixed/anchored within the app shell while the main content area scrolls naturally beneath the top bar; the KPI bento grid, charts, activity feed, and data table all scroll in normal flow.",
      "The indigo primary color is used strictly for the primary CTA button, the active nav state, and the primary chart data series; cyan appears only as the secondary chart series and positive delta indicators.",
      "The revenue-over-time area chart occupies the largest bento cell (roughly two-thirds width) and is the single dominant focal point; all other modules are visually quieter.",
      "No nested cards inside cards; regions are separated by 1px hairlines and intentional negative space rather than stacked bordered boxes."
    ],
    "asset_rules": [
      "NATIVE TRANSPARENT PNG ASSETS: Any standalone extracted asset in this tab must set transparent_background=true so Sunburst 2.5 renders it on a 100% native transparent alpha background with no chroma key.",
      "For this dashboard, no separate asset images are required: the monogram logo and all nav/UI icons are coded as crisp 2px-stroke SVG graphics matching Lucide conventions on a 20px grid.",
      "CODED SEPARATOR / BACKGROUND / 3D SPEC FROM IMAGE (MetricFlow Main Dashboard Screen (Light, Semi-Flat, 1440px Desktop)): Sidebar separator: the left sidebar is divided from the main content area by a thin, light gray vertical hairline rule (approximately one pixel, a very light cool gray) running the full height of the viewport. Recreate this 1:1 in code as a subtle border on the sidebar''s right edge, with no glow or gradient.",
      "CODED SEPARATOR / BACKGROUND / 3D SPEC FROM IMAGE (MetricFlow Main Dashboard Screen (Light, Semi-Flat, 1440px Desktop)): Card and table dividers: every dashboard card (KPI tiles, Revenue over time, Customer growth, Activity, Recent projects) sits on a white surface with a very light gray border and a soft, barely visible drop shadow, separated from one another by roughly 24 pixels of the page background. Inside the Recent projects table, each row is separated by a one-pixel light gray horizontal rule, and the table header row is separated from the body by a slightly darker gray rule with a very light gray header background. Recreate all of these rules 1:1 in code with the exact same thickness and colors.",
      "CODED SEPARATOR / BACKGROUND / 3D SPEC FROM IMAGE (MetricFlow Main Dashboard Screen (Light, Semi-Flat, 1440px Desktop)): Background canvas: the overall page background is a flat, very light cool off-white (a pale blue-tinted gray, around #F7F8FB), with no gradient, grid, dot texture, or ambient glow anywhere in the image. The sidebar uses the same light background with the active ''Overview'' item highlighted by a soft pale indigo pill with indigo text. Recreate the entire background as flat code colors matching these exact tones; do not use a background screenshot.",
      "CODED SEPARATOR / BACKGROUND / 3D SPEC FROM IMAGE (MetricFlow Main Dashboard Screen (Light, Semi-Flat, 1440px Desktop)): Charts and progress visuals: the KPI sparklines are smooth curved lines (indigo for revenue/projects, teal for customers/churn) with a soft gradient fill fading to transparent beneath the line; the Revenue over time chart uses a solid indigo line with a light indigo area fill plus a dashed teal comparison line over faint horizontal gridlines; the Customer growth chart uses stacked indigo and teal rounded-top bars; progress bars in the table are thin rounded indigo bars on light gray tracks. All of these must be drawn programmatically (SVG or canvas) to match the image exactly, including the dashed line style, gradient fades, and rounded bar caps.",
      "CODED SEPARATOR / BACKGROUND / 3D SPEC FROM IMAGE (MetricFlow Main Dashboard Screen (Light, Semi-Flat, 1440px Desktop)): 3D and animation: there are no 3D objects, mascots, or decorative props in this image, so no 3D reconstruction is required. The only motion-worthy elements are the chart lines and progress bars, which may animate by drawing/wiping in on load, and the status pills which may gently fade in; keep all animation subtle and consistent with the flat semi-flat style."
    ]
  },
  "items": [
    {
      "id": "item_1",
      "kind": "component",
      "name": "MetricFlow Main Dashboard Screen (Light, Semi-Flat, 1440px Desktop)",
      "tab_key": "dashboard",
      "tab_label": "Dashboard",
      "page_route": null,
      "variant_index": 1,
      "viewport": {
        "intent": "desktop",
        "css_width": 1440
      },
      "aspect_ratio": "16:9",
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
        "req_6",
        "req_7",
        "req_8",
        "req_9"
      ],
      "direction": "Calm, precise, data-forward semi-flat analytics command center. Airy light theme on #F8FAFC with white surfaces, crisp 1px #E2E8F0 hairlines, and subtle layered elevation only on interactive cards. One dominant indigo revenue area chart anchors the composition; everything else stays quiet and neutral so dense data remains scannable. Inter for chrome, IBM Plex Mono for every number, timestamp, and ID.",
      "exact_copy": [
        "MetricFlow",
        "Overview",
        "Revenue",
        "Customers",
        "Projects",
        "Reports",
        "Settings",
        "Search metrics, projects, reports…",
        "Last 30 days",
        "Export report",
        "Monthly Revenue",
        "$84,320",
        "+12.4%",
        "Active Customers",
        "2,847",
        "+3.1%",
        "Projects On Track",
        "18 / 24",
        "+2",
        "Churn Rate",
        "2.1%",
        "-0.4%",
        "Revenue over time",
        "MRR",
        "Previous period",
        "Customer growth",
        "New",
        "Retained",
        "Activity",
        "Report ''Q3 Revenue Summary'' shared with 4 teammates",
        "Pipeline deal ''Northwind Renewal'' moved to Negotiation",
        "Alert: Churn risk flagged on 3 enterprise accounts",
        "New customer cohort synced from billing",
        "Recent projects",
        "Project",
        "Owner",
        "Status",
        "Progress",
        "Due",
        "Website Redesign",
        "A. Chen",
        "On track",
        "72%",
        "Aug 14",
        "Billing Migration",
        "M. Ortiz",
        "At risk",
        "41%",
        "Aug 09",
        "Retention Dashboard",
        "S. Patel",
        "On track",
        "88%",
        "Aug 21",
        "Data Warehouse Sync",
        "J. Kim",
        "Blocked",
        "12%",
        "Aug 07",
        "Mobile Analytics SDK",
        "R. Duarte",
        "On track",
        "55%",
        "Sep 02"
      ],
      "reference_asset_ids": [],
      "depends_on": [],
      "image_prompt": "A 1440px-wide desktop SaaS analytics dashboard app screen for ''MetricFlow'', shown as a full widescreen 16:9 UI mockup, light theme, semi-flat design style. Overall canvas: very light gray-blue background #F8FAFC, white #FFFFFF surfaces, crisp 1px hairline borders #E2E8F0, subtle soft shadows only on interactive cards, absolutely no heavy gradients, no purple-blue SaaS gradient blobs, no glassmorphism, no neon glows, no floating decorative shapes. LEFT: a fixed 240px white sidebar with a small indigo rounded-square monogram logo and ''MetricFlow'' wordmark at top; below it a vertical nav list with 2px crisp geometric Lucide-style 20px stroke icons and 14px Inter labels — Overview (active, indigo #4F46E5 icon and text with a soft indigo-tinted pill background), Revenue, Customers, Projects, Reports, Settings (all muted slate #64748B); at the sidebar bottom a small circular user avatar with name ''Alex Morgan'' and role ''Founder'' in muted text. TOP: a slim white top bar spanning the main content area with a search input (magnifier icon, placeholder ''Search metrics, projects, reports…''), a date-range selector pill reading ''Last 30 days'', a solid indigo #4F46E5 primary button ''Export report'' with a small right-pointing arrow affordance, and a notification bell icon with a tiny indigo dot. MAIN CONTENT (top to bottom): first a page header row with ''Overview'' heading in Inter 600 at 24px tight tracking and a muted subline ''Your business at a glance · Updated 2 min ago''; then an asymmetric bento grid of four KPI cards with 24px gutters — Monthly Revenue ($84,320, delta badge +12.4% in cyan-tinted positive pill, small indigo sparkline), Active Customers (2,847, +3.1%, cyan sparkline), Projects On Track (18 / 24, +2, neutral sparkline), Churn Rate (2.1%, -0.4% shown as a positive improvement in cyan) — each card with a small muted uppercase-free label, large IBM Plex Mono metric value, delta badge, and one thin mini sparkline, no decorative illustrations inside cards; below, a two-column split where the left two-thirds cell holds the unmistakable #1 focal point: a large ''Revenue over time'' area chart card with an indigo #4F46E5 line and soft indigo area fill for MRR plus a cyan #06B6D4 comparison line for ''Previous period'', light horizontal gridlines, mono axis labels, and a small legend; the right one-third column stacks a compact ''Customer growth'' bar chart card (indigo ''New'' and cyan ''Retained'' bars per month, max two series) above an ''Activity'' feed card listing four entries with IBM Plex Mono timestamps and short data-forward sentences; at the bottom a full-width ''Recent projects'' data table card with sortable column headers (Project, Owner, Status, Progress, Due), five dense rows at 14px Inter with 1.5 line-height, left-aligned text, status pills (''On track'' in soft cyan-tint, ''At risk'' in soft amber-tint, ''Blocked'' in soft red-tint), mono progress percentages and due dates, and thin #E2E8F0 row separators. Typography discipline: Inter headings 600–700 with -0.02em tracking, body 400–500 at 14–15px, IBM Plex Mono for every metric value, timestamp, and ID with tabular numerals; high contrast text #0F172A on white, muted #64748B only for secondary labels; generous whitespace, calm low-noise chrome, one clear focal chart, no card-in-card nesting. Authentic analytics terminology only, no lorem ipsum, no hype copy. Clean, production-quality UI design mockup, pixel-crisp, flat with subtle depth.",
      "implementation_notes": [
        "POSITIONING (STICKY VS NON-STICKY): Build this as an app shell where the 240px left sidebar and the slim top bar remain fixed within the viewport while only the main content region (page header, KPI bento grid, charts, activity feed, and data table) scrolls naturally in normal flow. The sidebar keeps its collapsible state per the brand rules.",
        "PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: All KPI values, delta percentages, sparkline shapes, chart series data, activity feed entries, project rows, owner names, and the user avatar are placeholder sample data structured as clean typed objects for future database integration — no placeholder images or static cutouts should be generated for any of them. Implement every card, chart container, feed row, table, pill, button, input, and nav item 1:1 in HTML/CSS/TypeScript right now, wiring them to configurable data inputs.",
        "CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Build purely with code: all surfaces, hairline separators, chart gridlines, sparklines, area and bar charts (rendered with a charting library or SVG), status pills, shadows, and the ambient page background. Convert into vectorial SVG graphics: the MetricFlow monogram logo, all navigation and UI icons (2px crisp geometric strokes on a 20px grid, Lucide conventions), the search magnifier, bell, and arrow affordance. There is no 3D object in this design — do not invent one; this is a calm data surface with no decorative 3D props.",
        "FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: On mount, stagger-reveal the KPI cards with a subtle upward fade using exponential ease-out (cubic-bezier(0.22, 1, 0.36, 1)), then animate chart lines and bars drawing in from zero with the same easing. Interactive cards lift on hover with the elevated shadow and a 2px upward translate using a crisp spring (stiffness 320, damping 28 — never bounce or elastic). The active nav indicator slides smoothly between items on route change. Sortable table headers reflow rows with a quick layout animation; the date-range selector crossfades chart data on change. All motion respects reduced-motion preferences by falling back to instant state changes.",
        "REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No existing screenshot or previous component was attached, so this is a brand-new build; if a legacy dashboard screen exists in the codebase, replace it wholesale with this design while preserving existing routes, handlers, and data bindings.",
        "CODED SEPARATOR / BACKGROUND SPEC: Sidebar separator: the left sidebar is divided from the main content area by a thin, light gray vertical hairline rule (approximately one pixel, a very light cool gray) running the full height of the viewport. Recreate this 1:1 in code as a subtle border on the sidebar''s right edge, with no glow or gradient.",
        "CODED SEPARATOR / BACKGROUND SPEC: Card and table dividers: every dashboard card (KPI tiles, Revenue over time, Customer growth, Activity, Recent projects) sits on a white surface with a very light gray border and a soft, barely visible drop shadow, separated from one another by roughly 24 pixels of the page background. Inside the Recent projects table, each row is separated by a one-pixel light gray horizontal rule, and the table header row is separated from the body by a slightly darker gray rule with a very light gray header background. Recreate all of these rules 1:1 in code with the exact same thickness and colors.",
        "CODED SEPARATOR / BACKGROUND SPEC: Background canvas: the overall page background is a flat, very light cool off-white (a pale blue-tinted gray, around #F7F8FB), with no gradient, grid, dot texture, or ambient glow anywhere in the image. The sidebar uses the same light background with the active ''Overview'' item highlighted by a soft pale indigo pill with indigo text. Recreate the entire background as flat code colors matching these exact tones; do not use a background screenshot.",
        "CODED SEPARATOR / BACKGROUND SPEC: Charts and progress visuals: the KPI sparklines are smooth curved lines (indigo for revenue/projects, teal for customers/churn) with a soft gradient fill fading to transparent beneath the line; the Revenue over time chart uses a solid indigo line with a light indigo area fill plus a dashed teal comparison line over faint horizontal gridlines; the Customer growth chart uses stacked indigo and teal rounded-top bars; progress bars in the table are thin rounded indigo bars on light gray tracks. All of these must be drawn programmatically (SVG or canvas) to match the image exactly, including the dashed line style, gradient fades, and rounded bar caps.",
        "CODED SEPARATOR / BACKGROUND SPEC: 3D and animation: there are no 3D objects, mascots, or decorative props in this image, so no 3D reconstruction is required. The only motion-worthy elements are the chart lines and progress bars, which may animate by drawing/wiping in on load, and the status pills which may gently fade in; keep all animation subtle and consistent with the flat semi-flat style."
      ],
      "behaviors": [
        "HOVER STATE: Interactive KPI and chart cards lift with background staying #FFFFFF, border shifting to a slightly deeper slate hairline, and shadow elevating from 0 8px 24px rgba(15,23,42,0.08) to 0 12px 32px rgba(15,23,42,0.10) with a subtle 2px upward lift. Nav items on hover show a soft slate-tinted background (#F1F5F9) with text darkening toward #0F172A. The Export report button on hover deepens to a darker indigo (#4338CA) with the arrow nudging 2px right.",
        "CLICK / ACTIVE STATE: Pressed buttons compress subtly (scale down about 2%) with the darker indigo background held; the active nav item keeps its indigo #4F46E5 icon/text on a soft indigo-tinted pill (#EEF2FF) with no border. Sortable column headers show an indigo sort-direction arrow on the active column.",
        "FOCUS & DISABLED STATES: Keyboard focus shows a 2px indigo #4F46E5 focus ring with a 2px white offset on all interactive elements. Disabled controls (e.g., Export report while a report generates) render at 50% opacity with no shadow and cursor not-allowed."
      ],
      "responsive_rules": [
        "Desktop (1440px): full layout as designed — 240px sidebar, slim top bar, asymmetric bento grid with the revenue chart at two-thirds width and the right column stacking the bar chart and activity feed, full-width data table at bottom.",
        "Tablet (834px): the sidebar collapses to an icon-only 64px rail (or an off-canvas drawer behind a hamburger in the top bar); the bento grid reflows to two columns with the revenue chart spanning full width and the bar chart and activity feed side by side below; the table becomes horizontally scrollable with sticky headers.",
        "Mobile (390px): single-column stack — top bar condenses to logo, search icon, and bell with the date-range selector and Export report moving into the page header; KPI cards stack full-width in a 2-up then 1-up flow; charts render full-width with simplified axis labels; the table converts to compact stacked row cards showing project, status pill, and due date."
      ],
      "asset_instructions": [
        "No extracted transparent asset images are required for this component: the monogram logo and every icon must be recreated as scalable vectorial SVG graphics (2px crisp geometric strokes on a 20px grid, Lucide-style) directly in code for razor-sharp rendering at any size.",
        "If a logo asset is ever extracted separately, it must be a transparent-background PNG converted to SVG; never rasterize icons or the wordmark into flat bitmap layers.",
        "CODED SEPARATOR / BACKGROUND SPEC: Sidebar separator: the left sidebar is divided from the main content area by a thin, light gray vertical hairline rule (approximately one pixel, a very light cool gray) running the full height of the viewport. Recreate this 1:1 in code as a subtle border on the sidebar''s right edge, with no glow or gradient.",
        "CODED SEPARATOR / BACKGROUND SPEC: Card and table dividers: every dashboard card (KPI tiles, Revenue over time, Customer growth, Activity, Recent projects) sits on a white surface with a very light gray border and a soft, barely visible drop shadow, separated from one another by roughly 24 pixels of the page background. Inside the Recent projects table, each row is separated by a one-pixel light gray horizontal rule, and the table header row is separated from the body by a slightly darker gray rule with a very light gray header background. Recreate all of these rules 1:1 in code with the exact same thickness and colors.",
        "CODED SEPARATOR / BACKGROUND SPEC: Background canvas: the overall page background is a flat, very light cool off-white (a pale blue-tinted gray, around #F7F8FB), with no gradient, grid, dot texture, or ambient glow anywhere in the image. The sidebar uses the same light background with the active ''Overview'' item highlighted by a soft pale indigo pill with indigo text. Recreate the entire background as flat code colors matching these exact tones; do not use a background screenshot.",
        "CODED SEPARATOR / BACKGROUND SPEC: Charts and progress visuals: the KPI sparklines are smooth curved lines (indigo for revenue/projects, teal for customers/churn) with a soft gradient fill fading to transparent beneath the line; the Revenue over time chart uses a solid indigo line with a light indigo area fill plus a dashed teal comparison line over faint horizontal gridlines; the Customer growth chart uses stacked indigo and teal rounded-top bars; progress bars in the table are thin rounded indigo bars on light gray tracks. All of these must be drawn programmatically (SVG or canvas) to match the image exactly, including the dashed line style, gradient fades, and rounded bar caps.",
        "CODED SEPARATOR / BACKGROUND SPEC: 3D and animation: there are no 3D objects, mascots, or decorative props in this image, so no 3D reconstruction is required. The only motion-worthy elements are the chart lines and progress bars, which may animate by drawing/wiping in on load, and the status pills which may gently fade in; keep all animation subtle and consistent with the flat semi-flat style."
      ],
      "is_visual_anchor": true
    },
    {
      "id": "item_1_auto_asset_1",
      "kind": "asset",
      "name": "MetricFlow Brand Logo Mark & Wordmark (Transparent)",
      "tab_key": "dashboard",
      "tab_label": "Dashboard",
      "page_route": null,
      "variant_index": 2,
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
      "direction": "Flat semi-flat SaaS brand mark: a rounded-square indigo badge with a bold white letter M, paired with the dark navy MetricFlow wordmark in a modern geometric sans-serif.",
      "exact_copy": [],
      "reference_asset_ids": [],
      "depends_on": [
        "item_1"
      ],
      "image_prompt": "Extract ONLY the MetricFlow brand logo from the attached reference UI image, located at the top-left of the sidebar: the indigo rounded-square icon containing a bold white letter M, plus the dark navy ''MetricFlow'' wordmark text beside it. Isolate both on a clean 100% transparent background, removing all sidebar background, shadows, and unrelated UI elements. Reproduce the exact indigo color, corner radius, letterform weight, and wordmark typography faithfully.",
      "implementation_notes": [
        "Render on a 100% transparent PNG background and layer in the component/page layout"
      ],
      "behaviors": [],
      "responsive_rules": [
        "Scale proportionally across breakpoints"
      ],
      "asset_instructions": [
        "Place this logo at the top-left of the sidebar navigation, exactly as shown in the reference. Recreate it as a vectorial SVG graphic: the rounded-square badge as an SVG rect with the indigo fill and the M as a path or bold text, and the wordmark as SVG text in the matching geometric sans-serif, so it scales crisply at all sizes. Do not embed it as a raster screenshot."
      ],
      "is_visual_anchor": false
    }
  ],
  "shared_components": [
    {
      "id": "shared_1",
      "name": "Fixed Sidebar Navigation",
      "canonical_item_id": "item_1",
      "used_on_routes": [
        "/"
      ],
      "description": "240px collapsible left sidebar with monogram logo, icon+label nav items, and bottom user avatar"
    },
    {
      "id": "shared_2",
      "name": "Slim Top Bar",
      "canonical_item_id": "item_1",
      "used_on_routes": [
        "/"
      ],
      "description": "Fixed top bar with search, date-range selector, Export report primary CTA, and notification bell"
    }
  ],
  "page_composition": [
    {
      "page_name": "MetricFlow Dashboard Screen",
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
        "Export report"
      ],
      "states": [
        "default",
        "hover",
        "active",
        "focus",
        "disabled"
      ]
    }
  ]
}', NULL, 'completed', 0, 0, 0, 0, NULL, '2026-10-24T20:10:36.843032+00:00', '2026-10-24T20:12:01.141642+00:00', '2026-10-24T20:12:01.141642+00:00', 'dashboard', 'Dashboard', NULL, '[]');
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-metricflow-1', 'example-run-metricflow', 'example-metricflow', 'item_1', 1, 'component', 'MetricFlow Main Dashboard Screen (Light, Semi-Flat, 1440px Desktop)', 'Calm, precise, data-forward semi-flat analytics command center. Airy light theme on #F8FAFC with white surfaces, crisp 1px #E2E8F0 hairlines, and subtle layered elevation only on interactive cards. One dominant indigo revenue area chart anchors the composition; everything else stays quiet and neutral so dense data remains scannable. Inter for chrome, IBM Plex Mono for every number, timestamp, and ID.', 'visual_anchor', NULL, '16:9', 'completed', 'showcases/metricflow/01.png', 'showcases/metricflow/01.webp', 'image/png', 1536, 864, 1106093, 0, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: component (component)
- Direction / Item Title: MetricFlow Main Dashboard Screen (Light, Semi-Flat, 1440px Desktop) (Variant #1)
- Viewport & Aspect Ratio: desktop (1440px target width), Aspect Ratio 16:9
- Scope Rules: Focused UI component reference.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: MetricFlow — A unified business analytics dashboard that consolidates revenue, customer growth, project status, and performance KPIs into one interactive, real-time command center with charts, activity feeds, data tables, and reporting tools.
- Product / Application Description: Tagline: Your Business. Every Insight. One Dashboard.

An advanced business analytics dashboard for tracking revenue, customer growth, projects, and performance. Includes interactive charts, KPI cards, activity feeds, data tables, reporting tools, and sidebar navigation.
- Industry & Target Audience: SaaS / Business Intelligence & Analytics | Audience: Founders, operations leads, product managers, and data-driven teams at startups and mid-size companies who need fast, self-serve visibility into business health. (Quick at-a-glance KPIs, drill-down charts, clean data tables, shareable reports, and a calm, low-noise interface that keeps dense information scannable without visual fatigue.)
- Primary User Goal / Conversion Action: Log in daily to monitor KPIs, explore insights via interactive charts, and generate/share reports — driving habitual engagement and conversion to a paid plan.

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #11 Semi-flat Design
- Exact Style Definition: Keeps the simplicity of flat design while adding subtle shadows, layering, or gradients to communicate depth and interactivity.
- Mandatory Visual & CSS System Rules: Maintain clean flat surfaces enhanced with subtle multi-layered ambient shadows (0 8px 24px rgba(15,23,42,0.08)), floating interactive cards, micro-gradients on primary CTAs, and clear z-index elevation cues.
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "semi-flat design". Primary style #11 Semi-flat Design (Dimensional & Tactile): Keeps the simplicity of flat design while adding subtle shadows, layering, or gradients to communicate depth and interactivity. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Maintain clean flat surfaces enhanced with subtle multi-layered ambient shadows (0 8px 24px rgba(15,23,42,0.08)), floating interactive cards, micro-gradients on primary CTAs, and clear z-index elevation cues.

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Precise, confident, and quietly intelligent — a trusted analytical co-pilot, not a flashy toy. — Semi-flat dashboard aesthetic: clean flat surfaces with subtle layered elevation, bento-grid KPI cards, crisp 1px hairlines, and a single confident indigo accent reserved for data highlights and primary actions. Charts and numbers are the heroes; chrome stays neutral and quiet.
- UI Density & Tone: comfortable / Clear, direct, and data-forward. Short labels, active verbs, no hype. ''Revenue up 12.4%'' — not ''Amazing growth!''. (English)
- Color Palette: Primary #4F46E5, Accent #06B6D4, Background #F8FAFC, Surface #FFFFFF, Text #0F172A, Muted #64748B, Border #E2E8F0
- Typography Intent: Heading Inter, 600–700 weight, tight tracking (-0.02em); scale 48/32/24/18px, Body Inter, 400–500 weight, 14–15px, 1.5 line-height for tables and feeds, Mono IBM Plex Mono, for metric values, IDs, and timestamps
- Radii, Borders & Shadows: Radius 12px cards, 8px buttons, 6px inputs and badges, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(0,0,0,0.06); hover lift 0 12px 32px rgba(0,0,0,0.10)
- Logo Instruction: Use a clean temporary monogram ''M'' integrated with the brand name ''MetricFlow'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E2E8F0), and subtle surface luminance shifts (#F8FAFC vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#0F172A on #F8FAFC/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Calm, precise, data-forward semi-flat analytics command center. Airy light theme on #F8FAFC with white surfaces, crisp 1px #E2E8F0 hairlines, and subtle layered elevation only on interactive cards. One dominant indigo revenue area chart anchors the composition; everything else stays quiet and neutral so dense data remains scannable. Inter for chrome, IBM Plex Mono for every number, timestamp, and ID.

PLANNER IMAGE PROMPT:
A 1440px-wide desktop SaaS analytics dashboard app screen for ''MetricFlow'', shown as a full widescreen 16:9 UI mockup, light theme, semi-flat design style. Overall canvas: very light gray-blue background #F8FAFC, white #FFFFFF surfaces, crisp 1px hairline borders #E2E8F0, subtle soft shadows only on interactive cards, absolutely no heavy gradients, no purple-blue SaaS gradient blobs, no glassmorphism, no neon glows, no floating decorative shapes. LEFT: a fixed 240px white sidebar with a small indigo rounded-square monogram logo and ''MetricFlow'' wordmark at top; below it a vertical nav list with 2px crisp geometric Lucide-style 20px stroke icons and 14px Inter labels — Overview (active, indigo #4F46E5 icon and text with a soft indigo-tinted pill background), Revenue, Customers, Projects, Reports, Settings (all muted slate #64748B); at the sidebar bottom a small circular user avatar with name ''Alex Morgan'' and role ''Founder'' in muted text. TOP: a slim white top bar spanning the main content area with a search input (magnifier icon, placeholder ''Search metrics, projects, reports…''), a date-range selector pill reading ''Last 30 days'', a solid indigo #4F46E5 primary button ''Export report'' with a small right-pointing arrow affordance, and a notification bell icon with a tiny indigo dot. MAIN CONTENT (top to bottom): first a page header row with ''Overview'' heading in Inter 600 at 24px tight tracking and a muted subline ''Your business at a glance · Updated 2 min ago''; then an asymmetric bento grid of four KPI cards with 24px gutters — Monthly Revenue ($84,320, delta badge +12.4% in cyan-tinted positive pill, small indigo sparkline), Active Customers (2,847, +3.1%, cyan sparkline), Projects On Track (18 / 24, +2, neutral sparkline), Churn Rate (2.1%, -0.4% shown as a positive improvement in cyan) — each card with a small muted uppercase-free label, large IBM Plex Mono metric value, delta badge, and one thin mini sparkline, no decorative illustrations inside cards; below, a two-column split where the left two-thirds cell holds the unmistakable #1 focal point: a large ''Revenue over time'' area chart card with an indigo #4F46E5 line and soft indigo area fill for MRR plus a cyan #06B6D4 comparison line for ''Previous period'', light horizontal gridlines, mono axis labels, and a small legend; the right one-third column stacks a compact ''Customer growth'' bar chart card (indigo ''New'' and cyan ''Retained'' bars per month, max two series) above an ''Activity'' feed card listing four entries with IBM Plex Mono timestamps and short data-forward sentences; at the bottom a full-width ''Recent projects'' data table card with sortable column headers (Project, Owner, Status, Progress, Due), five dense rows at 14px Inter with 1.5 line-height, left-aligned text, status pills (''On track'' in soft cyan-tint, ''At risk'' in soft amber-tint, ''Blocked'' in soft red-tint), mono progress percentages and due dates, and thin #E2E8F0 row separators. Typography discipline: Inter headings 600–700 with -0.02em tracking, body 400–500 at 14–15px, IBM Plex Mono for every metric value, timestamp, and ID with tabular numerals; high contrast text #0F172A on white, muted #64748B only for secondary labels; generous whitespace, calm low-noise chrome, one clear focal chart, no card-in-card nesting. Authentic analytics terminology only, no lorem ipsum, no hype copy. Clean, production-quality UI design mockup, pixel-crisp, flat with subtle depth.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_1] Light theme: background #F8FAFC, white surfaces #FFFFFF, indigo primary #4F46E5 reserved for primary CTA and key data series, cyan accent #06B6D4 for secondary data series and positive deltas
- [req_2] Semi-flat Design (#11): flat surfaces with subtle layered elevation (0 8px 24px rgba(15,23,42,0.08)), crisp 1px hairlines #E2E8F0, no heavy gradients, no glassmorphism, no neon glow, no floating blobs
- [req_3] Left sidebar navigation at 240px: monogram logo, nav items with 2px crisp geometric Lucide-style 20px stroke icons (Overview active, Revenue, Customers, Projects, Reports, Settings), user avatar at bottom, collapsible
- [req_4] Slim top bar: search input, date-range selector, ''Export report'' solid indigo primary button with subtle arrow affordance, notification bell
- [req_5] Bento-grid of 4 KPI cards (Monthly Revenue, Active Customers, Projects On Track, Churn Rate) with IBM Plex Mono values, delta badges, and one mini sparkline each; 24px gutters; no decorative illustrations inside cards
- [req_6] Large revenue-over-time area chart as the unmistakable #1 visual anchor with indigo primary line and cyan comparison series; secondary customer-growth bar chart; max four series per chart, no rainbow charts
- [req_7] Right-column activity feed with mono timestamps and a dense recent-projects data table with status pills and sortable column headers
- [req_8] Typography: Inter headings 600–700 tight tracking (-0.02em) at 24/18px in-app scale; body 400–500 at 14–15px with 1.5 line-height for tables and feeds; IBM Plex Mono for all metric values, IDs, and timestamps; left-aligned data; tabular numerals
- [req_9] Anti-AI-slop craft: no card-in-card nesting, asymmetric bento proportions, one clear focal chart, generous whitespace, calm low-noise chrome, authentic analytics terminology (MRR, ARR, retention, pipeline), no filler copy

EXACT COPY & HIERARCHY:
MetricFlow | Overview | Revenue | Customers | Projects | Reports | Settings | Search metrics, projects, reports… | Last 30 days | Export report | Monthly Revenue | $84,320 | +12.4% | Active Customers | 2,847 | +3.1% | Projects On Track | 18 / 24 | +2 | Churn Rate | 2.1% | -0.4% | Revenue over time | MRR | Previous period | Customer growth | New | Retained | Activity | Report ''Q3 Revenue Summary'' shared with 4 teammates | Pipeline deal ''Northwind Renewal'' moved to Negotiation | Alert: Churn risk flagged on 3 enterprise accounts | New customer cohort synced from billing | Recent projects | Project | Owner | Status | Progress | Due | Website Redesign | A. Chen | On track | 72% | Aug 14 | Billing Migration | M. Ortiz | At risk | 41% | Aug 09 | Retention Dashboard | S. Patel | On track | 88% | Aug 21 | Data Warehouse Sync | J. Kim | Blocked | 12% | Aug 07 | Mobile Analytics SDK | R. Duarte | On track | 55% | Sep 02

ATTACHED REFERENCE ROLES:
- Workflow Mode [NEW COMPONENT FROM SCRATCH]: No old/current component screenshot was provided. Generate a brand-new component from scratch based on the user''s prompt, designing the appropriate buttons, controls, text labels, and icons (2px crisp geometric stroke icons (Lucide-style), 20px grid) strictly in the Project Brand Kit and Chosen Web Design Style.

INVARIANT RENDERING CONSTRAINTS:
- Direct, full-bleed viewport UI design reference (do NOT place the UI inside an angled laptop/phone hardware mockup or external desk scene, and do NOT include a browser URL bar).
- Strictly execute the Chosen Web Design Style definition and visual rules above in every surface, border, shadow, color contrast, and typographic detail.
- Render crisp, legible typography, accurate button alignment, accessible contrast, and realistic spacing.
- INTERNAL BREAKPOINT DIMENSIONS = DESKTOP (`1440px` DESKTOP VIEWPORT): Render with authentic `1440px` desktop container proportions, horizontal desktop alignment, and desktop typography scale.
- GENERATE COMPONENT FROM SCRATCH: Because no old screenshot is attached, design this component fresh from the user''s prompt and requirements (using text labels, icons, or icon+text appropriate to the style and prompt) while strictly obeying the Project Brand Kit and Chosen Web Design Style above.
', '[]', '[]', '{"id":"item_1","kind":"component","name":"MetricFlow Main Dashboard Screen (Light, Semi-Flat, 1440px Desktop)","tab_key":"dashboard","tab_label":"Dashboard","page_route":null,"variant_index":1,"viewport":{"intent":"desktop","css_width":1440},"aspect_ratio":"16:9","transparent_background":false,"extracted_from_item_id":null,"target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_1","req_2","req_3","req_4","req_5","req_6","req_7","req_8","req_9"],"direction":"Calm, precise, data-forward semi-flat analytics command center. Airy light theme on #F8FAFC with white surfaces, crisp 1px #E2E8F0 hairlines, and subtle layered elevation only on interactive cards. One dominant indigo revenue area chart anchors the composition; everything else stays quiet and neutral so dense data remains scannable. Inter for chrome, IBM Plex Mono for every number, timestamp, and ID.","exact_copy":["MetricFlow","Overview","Revenue","Customers","Projects","Reports","Settings","Search metrics, projects, reports…","Last 30 days","Export report","Monthly Revenue","$84,320","+12.4%","Active Customers","2,847","+3.1%","Projects On Track","18 / 24","+2","Churn Rate","2.1%","-0.4%","Revenue over time","MRR","Previous period","Customer growth","New","Retained","Activity","Report ''Q3 Revenue Summary'' shared with 4 teammates","Pipeline deal ''Northwind Renewal'' moved to Negotiation","Alert: Churn risk flagged on 3 enterprise accounts","New customer cohort synced from billing","Recent projects","Project","Owner","Status","Progress","Due","Website Redesign","A. Chen","On track","72%","Aug 14","Billing Migration","M. Ortiz","At risk","41%","Aug 09","Retention Dashboard","S. Patel","On track","88%","Aug 21","Data Warehouse Sync","J. Kim","Blocked","12%","Aug 07","Mobile Analytics SDK","R. Duarte","On track","55%","Sep 02"],"reference_asset_ids":[],"depends_on":[],"image_prompt":"A 1440px-wide desktop SaaS analytics dashboard app screen for ''MetricFlow'', shown as a full widescreen 16:9 UI mockup, light theme, semi-flat design style. Overall canvas: very light gray-blue background #F8FAFC, white #FFFFFF surfaces, crisp 1px hairline borders #E2E8F0, subtle soft shadows only on interactive cards, absolutely no heavy gradients, no purple-blue SaaS gradient blobs, no glassmorphism, no neon glows, no floating decorative shapes. LEFT: a fixed 240px white sidebar with a small indigo rounded-square monogram logo and ''MetricFlow'' wordmark at top; below it a vertical nav list with 2px crisp geometric Lucide-style 20px stroke icons and 14px Inter labels — Overview (active, indigo #4F46E5 icon and text with a soft indigo-tinted pill background), Revenue, Customers, Projects, Reports, Settings (all muted slate #64748B); at the sidebar bottom a small circular user avatar with name ''Alex Morgan'' and role ''Founder'' in muted text. TOP: a slim white top bar spanning the main content area with a search input (magnifier icon, placeholder ''Search metrics, projects, reports…''), a date-range selector pill reading ''Last 30 days'', a solid indigo #4F46E5 primary button ''Export report'' with a small right-pointing arrow affordance, and a notification bell icon with a tiny indigo dot. MAIN CONTENT (top to bottom): first a page header row with ''Overview'' heading in Inter 600 at 24px tight tracking and a muted subline ''Your business at a glance · Updated 2 min ago''; then an asymmetric bento grid of four KPI cards with 24px gutters — Monthly Revenue ($84,320, delta badge +12.4% in cyan-tinted positive pill, small indigo sparkline), Active Customers (2,847, +3.1%, cyan sparkline), Projects On Track (18 / 24, +2, neutral sparkline), Churn Rate (2.1%, -0.4% shown as a positive improvement in cyan) — each card with a small muted uppercase-free label, large IBM Plex Mono metric value, delta badge, and one thin mini sparkline, no decorative illustrations inside cards; below, a two-column split where the left two-thirds cell holds the unmistakable #1 focal point: a large ''Revenue over time'' area chart card with an indigo #4F46E5 line and soft indigo area fill for MRR plus a cyan #06B6D4 comparison line for ''Previous period'', light horizontal gridlines, mono axis labels, and a small legend; the right one-third column stacks a compact ''Customer growth'' bar chart card (indigo ''New'' and cyan ''Retained'' bars per month, max two series) above an ''Activity'' feed card listing four entries with IBM Plex Mono timestamps and short data-forward sentences; at the bottom a full-width ''Recent projects'' data table card with sortable column headers (Project, Owner, Status, Progress, Due), five dense rows at 14px Inter with 1.5 line-height, left-aligned text, status pills (''On track'' in soft cyan-tint, ''At risk'' in soft amber-tint, ''Blocked'' in soft red-tint), mono progress percentages and due dates, and thin #E2E8F0 row separators. Typography discipline: Inter headings 600–700 with -0.02em tracking, body 400–500 at 14–15px, IBM Plex Mono for every metric value, timestamp, and ID with tabular numerals; high contrast text #0F172A on white, muted #64748B only for secondary labels; generous whitespace, calm low-noise chrome, one clear focal chart, no card-in-card nesting. Authentic analytics terminology only, no lorem ipsum, no hype copy. Clean, production-quality UI design mockup, pixel-crisp, flat with subtle depth.","implementation_notes":["POSITIONING (STICKY VS NON-STICKY): Build this as an app shell where the 240px left sidebar and the slim top bar remain fixed within the viewport while only the main content region (page header, KPI bento grid, charts, activity feed, and data table) scrolls naturally in normal flow. The sidebar keeps its collapsible state per the brand rules.","PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: All KPI values, delta percentages, sparkline shapes, chart series data, activity feed entries, project rows, owner names, and the user avatar are placeholder sample data structured as clean typed objects for future database integration — no placeholder images or static cutouts should be generated for any of them. Implement every card, chart container, feed row, table, pill, button, input, and nav item 1:1 in HTML/CSS/TypeScript right now, wiring them to configurable data inputs.","CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Build purely with code: all surfaces, hairline separators, chart gridlines, sparklines, area and bar charts (rendered with a charting library or SVG), status pills, shadows, and the ambient page background. Convert into vectorial SVG graphics: the MetricFlow monogram logo, all navigation and UI icons (2px crisp geometric strokes on a 20px grid, Lucide conventions), the search magnifier, bell, and arrow affordance. There is no 3D object in this design — do not invent one; this is a calm data surface with no decorative 3D props.","FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: On mount, stagger-reveal the KPI cards with a subtle upward fade using exponential ease-out (cubic-bezier(0.22, 1, 0.36, 1)), then animate chart lines and bars drawing in from zero with the same easing. Interactive cards lift on hover with the elevated shadow and a 2px upward translate using a crisp spring (stiffness 320, damping 28 — never bounce or elastic). The active nav indicator slides smoothly between items on route change. Sortable table headers reflow rows with a quick layout animation; the date-range selector crossfades chart data on change. All motion respects reduced-motion preferences by falling back to instant state changes.","REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No existing screenshot or previous component was attached, so this is a brand-new build; if a legacy dashboard screen exists in the codebase, replace it wholesale with this design while preserving existing routes, handlers, and data bindings.","CODED SEPARATOR / BACKGROUND SPEC: Sidebar separator: the left sidebar is divided from the main content area by a thin, light gray vertical hairline rule (approximately one pixel, a very light cool gray) running the full height of the viewport. Recreate this 1:1 in code as a subtle border on the sidebar''s right edge, with no glow or gradient.","CODED SEPARATOR / BACKGROUND SPEC: Card and table dividers: every dashboard card (KPI tiles, Revenue over time, Customer growth, Activity, Recent projects) sits on a white surface with a very light gray border and a soft, barely visible drop shadow, separated from one another by roughly 24 pixels of the page background. Inside the Recent projects table, each row is separated by a one-pixel light gray horizontal rule, and the table header row is separated from the body by a slightly darker gray rule with a very light gray header background. Recreate all of these rules 1:1 in code with the exact same thickness and colors.","CODED SEPARATOR / BACKGROUND SPEC: Background canvas: the overall page background is a flat, very light cool off-white (a pale blue-tinted gray, around #F7F8FB), with no gradient, grid, dot texture, or ambient glow anywhere in the image. The sidebar uses the same light background with the active ''Overview'' item highlighted by a soft pale indigo pill with indigo text. Recreate the entire background as flat code colors matching these exact tones; do not use a background screenshot.","CODED SEPARATOR / BACKGROUND SPEC: Charts and progress visuals: the KPI sparklines are smooth curved lines (indigo for revenue/projects, teal for customers/churn) with a soft gradient fill fading to transparent beneath the line; the Revenue over time chart uses a solid indigo line with a light indigo area fill plus a dashed teal comparison line over faint horizontal gridlines; the Customer growth chart uses stacked indigo and teal rounded-top bars; progress bars in the table are thin rounded indigo bars on light gray tracks. All of these must be drawn programmatically (SVG or canvas) to match the image exactly, including the dashed line style, gradient fades, and rounded bar caps.","CODED SEPARATOR / BACKGROUND SPEC: 3D and animation: there are no 3D objects, mascots, or decorative props in this image, so no 3D reconstruction is required. The only motion-worthy elements are the chart lines and progress bars, which may animate by drawing/wiping in on load, and the status pills which may gently fade in; keep all animation subtle and consistent with the flat semi-flat style."],"behaviors":["HOVER STATE: Interactive KPI and chart cards lift with background staying #FFFFFF, border shifting to a slightly deeper slate hairline, and shadow elevating from 0 8px 24px rgba(15,23,42,0.08) to 0 12px 32px rgba(15,23,42,0.10) with a subtle 2px upward lift. Nav items on hover show a soft slate-tinted background (#F1F5F9) with text darkening toward #0F172A. The Export report button on hover deepens to a darker indigo (#4338CA) with the arrow nudging 2px right.","CLICK / ACTIVE STATE: Pressed buttons compress subtly (scale down about 2%) with the darker indigo background held; the active nav item keeps its indigo #4F46E5 icon/text on a soft indigo-tinted pill (#EEF2FF) with no border. Sortable column headers show an indigo sort-direction arrow on the active column.","FOCUS & DISABLED STATES: Keyboard focus shows a 2px indigo #4F46E5 focus ring with a 2px white offset on all interactive elements. Disabled controls (e.g., Export report while a report generates) render at 50% opacity with no shadow and cursor not-allowed."],"responsive_rules":["Desktop (1440px): full layout as designed — 240px sidebar, slim top bar, asymmetric bento grid with the revenue chart at two-thirds width and the right column stacking the bar chart and activity feed, full-width data table at bottom.","Tablet (834px): the sidebar collapses to an icon-only 64px rail (or an off-canvas drawer behind a hamburger in the top bar); the bento grid reflows to two columns with the revenue chart spanning full width and the bar chart and activity feed side by side below; the table becomes horizontally scrollable with sticky headers.","Mobile (390px): single-column stack — top bar condenses to logo, search icon, and bell with the date-range selector and Export report moving into the page header; KPI cards stack full-width in a 2-up then 1-up flow; charts render full-width with simplified axis labels; the table converts to compact stacked row cards showing project, status pill, and due date."],"asset_instructions":["No extracted transparent asset images are required for this component: the monogram logo and every icon must be recreated as scalable vectorial SVG graphics (2px crisp geometric strokes on a 20px grid, Lucide-style) directly in code for razor-sharp rendering at any size.","If a logo asset is ever extracted separately, it must be a transparent-background PNG converted to SVG; never rasterize icons or the wordmark into flat bitmap layers.","CODED SEPARATOR / BACKGROUND SPEC: Sidebar separator: the left sidebar is divided from the main content area by a thin, light gray vertical hairline rule (approximately one pixel, a very light cool gray) running the full height of the viewport. Recreate this 1:1 in code as a subtle border on the sidebar''s right edge, with no glow or gradient.","CODED SEPARATOR / BACKGROUND SPEC: Card and table dividers: every dashboard card (KPI tiles, Revenue over time, Customer growth, Activity, Recent projects) sits on a white surface with a very light gray border and a soft, barely visible drop shadow, separated from one another by roughly 24 pixels of the page background. Inside the Recent projects table, each row is separated by a one-pixel light gray horizontal rule, and the table header row is separated from the body by a slightly darker gray rule with a very light gray header background. Recreate all of these rules 1:1 in code with the exact same thickness and colors.","CODED SEPARATOR / BACKGROUND SPEC: Background canvas: the overall page background is a flat, very light cool off-white (a pale blue-tinted gray, around #F7F8FB), with no gradient, grid, dot texture, or ambient glow anywhere in the image. The sidebar uses the same light background with the active ''Overview'' item highlighted by a soft pale indigo pill with indigo text. Recreate the entire background as flat code colors matching these exact tones; do not use a background screenshot.","CODED SEPARATOR / BACKGROUND SPEC: Charts and progress visuals: the KPI sparklines are smooth curved lines (indigo for revenue/projects, teal for customers/churn) with a soft gradient fill fading to transparent beneath the line; the Revenue over time chart uses a solid indigo line with a light indigo area fill plus a dashed teal comparison line over faint horizontal gridlines; the Customer growth chart uses stacked indigo and teal rounded-top bars; progress bars in the table are thin rounded indigo bars on light gray tracks. All of these must be drawn programmatically (SVG or canvas) to match the image exactly, including the dashed line style, gradient fades, and rounded bar caps.","CODED SEPARATOR / BACKGROUND SPEC: 3D and animation: there are no 3D objects, mascots, or decorative props in this image, so no 3D reconstruction is required. The only motion-worthy elements are the chart lines and progress bars, which may animate by drawing/wiping in on load, and the status pills which may gently fade in; keep all animation subtle and consistent with the flat semi-flat style."],"is_visual_anchor":true}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T20:11:15.482298+00:00', '2026-10-24T20:11:43.574744+00:00', 'dashboard', 'Dashboard', NULL, 0, NULL);
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-metricflow-2', 'example-run-metricflow', 'example-metricflow', 'item_1_auto_asset_1', 2, 'asset', 'MetricFlow Brand Logo Mark & Wordmark (Transparent)', 'Flat semi-flat SaaS brand mark: a rounded-square indigo badge with a bold white letter M, paired with the dark navy MetricFlow wordmark in a modern geometric sans-serif.', 'asset', NULL, '1:1', 'completed', 'showcases/metricflow/02.png', 'showcases/metricflow/02.webp', 'image/png', 1024, 1024, 568502, 1, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: component (asset)
- Direction / Item Title: MetricFlow Brand Logo Mark & Wordmark (Transparent) (Variant #2)
- Viewport & Aspect Ratio: isolated (1024px target width), Aspect Ratio 1:1
- Scope Rules: Focused UI component reference.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: MetricFlow — A unified business analytics dashboard that consolidates revenue, customer growth, project status, and performance KPIs into one interactive, real-time command center with charts, activity feeds, data tables, and reporting tools.
- Product / Application Description: Tagline: Your Business. Every Insight. One Dashboard.

An advanced business analytics dashboard for tracking revenue, customer growth, projects, and performance. Includes interactive charts, KPI cards, activity feeds, data tables, reporting tools, and sidebar navigation.
- Industry & Target Audience: SaaS / Business Intelligence & Analytics | Audience: Founders, operations leads, product managers, and data-driven teams at startups and mid-size companies who need fast, self-serve visibility into business health. (Quick at-a-glance KPIs, drill-down charts, clean data tables, shareable reports, and a calm, low-noise interface that keeps dense information scannable without visual fatigue.)
- Primary User Goal / Conversion Action: Log in daily to monitor KPIs, explore insights via interactive charts, and generate/share reports — driving habitual engagement and conversion to a paid plan.

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #11 Semi-flat Design
- Exact Style Definition: Keeps the simplicity of flat design while adding subtle shadows, layering, or gradients to communicate depth and interactivity.
- Mandatory Visual & CSS System Rules: Maintain clean flat surfaces enhanced with subtle multi-layered ambient shadows (0 8px 24px rgba(15,23,42,0.08)), floating interactive cards, micro-gradients on primary CTAs, and clear z-index elevation cues.
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "semi-flat design". Primary style #11 Semi-flat Design (Dimensional & Tactile): Keeps the simplicity of flat design while adding subtle shadows, layering, or gradients to communicate depth and interactivity. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Maintain clean flat surfaces enhanced with subtle multi-layered ambient shadows (0 8px 24px rgba(15,23,42,0.08)), floating interactive cards, micro-gradients on primary CTAs, and clear z-index elevation cues.

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Precise, confident, and quietly intelligent — a trusted analytical co-pilot, not a flashy toy. — Semi-flat dashboard aesthetic: clean flat surfaces with subtle layered elevation, bento-grid KPI cards, crisp 1px hairlines, and a single confident indigo accent reserved for data highlights and primary actions. Charts and numbers are the heroes; chrome stays neutral and quiet.
- UI Density & Tone: comfortable / Clear, direct, and data-forward. Short labels, active verbs, no hype. ''Revenue up 12.4%'' — not ''Amazing growth!''. (English)
- Color Palette: Primary #4F46E5, Accent #06B6D4, Background #F8FAFC, Surface #FFFFFF, Text #0F172A, Muted #64748B, Border #E2E8F0
- Typography Intent: Heading Inter, 600–700 weight, tight tracking (-0.02em); scale 48/32/24/18px, Body Inter, 400–500 weight, 14–15px, 1.5 line-height for tables and feeds, Mono IBM Plex Mono, for metric values, IDs, and timestamps
- Radii, Borders & Shadows: Radius 12px cards, 8px buttons, 6px inputs and badges, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(0,0,0,0.06); hover lift 0 12px 32px rgba(0,0,0,0.10)
- Logo Instruction: Use a clean temporary monogram ''M'' integrated with the brand name ''MetricFlow'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E2E8F0), and subtle surface luminance shifts (#F8FAFC vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#0F172A on #F8FAFC/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Flat semi-flat SaaS brand mark: a rounded-square indigo badge with a bold white letter M, paired with the dark navy MetricFlow wordmark in a modern geometric sans-serif.

PLANNER IMAGE PROMPT:
Extract ONLY the MetricFlow brand logo from the attached reference UI image, located at the top-left of the sidebar: the indigo rounded-square icon containing a bold white letter M, plus the dark navy ''MetricFlow'' wordmark text beside it. Isolate both on a clean 100% transparent background, removing all sidebar background, shadows, and unrelated UI elements. Reproduce the exact indigo color, corner radius, letterform weight, and wordmark typography faithfully.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_1] Light theme: background #F8FAFC, white surfaces #FFFFFF, indigo primary #4F46E5 reserved for primary CTA and key data series, cyan accent #06B6D4 for secondary data series and positive deltas

EXACT COPY & HIERARCHY:
Use concise, realistic copy matching brand language.

ATTACHED REFERENCE ROLES:
- Workflow Mode [NEW COMPONENT FROM SCRATCH]: No old/current component screenshot was provided. Generate a brand-new component from scratch based on the user''s prompt, designing the appropriate buttons, controls, text labels, and icons (2px crisp geometric stroke icons (Lucide-style), 20px grid) strictly in the Project Brand Kit and Chosen Web Design Style.
- Prerequisite Hero/Component Reference Image [FOREGROUND ASSET EXTRACTION]: Extract and faithfully reproduce ONLY the specified foreground illustration, 3D object, icon set, or graphic element from this reference UI image onto a 100% transparent background (`background: transparent`). Strip away all background walls, gradients, UI buttons, and text!

INVARIANT RENDERING CONSTRAINTS:
- Render a clean, production-ready standalone website asset matching the chosen Web Design Style above.
- Do NOT wrap the asset in browser chrome, UI buttons, or device frames.
- NATIVE TRANSPARENT BACKGROUND (`background: "transparent"`): Isolate the subject (or organized icon sheet grid) on a 100% true transparent alpha background. Do NOT draw a fake checkerboard grid, do NOT add opaque background fills or vignettes, and keep clean alpha edges with a 10% safety margin around the silhouette.
- FOREGROUND EXTRACTION FROM COMPOSITE UI: Reproduce the exact foreground visual asset shown in the attached prerequisite Hero/Component reference image at full resolution on a transparent background, completely omitting the background environment, headlines, and UI buttons so this transparent PNG can be layered over the coded background.
', '[]', '["item_1"]', '{"id":"item_1_auto_asset_1","kind":"asset","name":"MetricFlow Brand Logo Mark & Wordmark (Transparent)","tab_key":"dashboard","tab_label":"Dashboard","page_route":null,"variant_index":2,"viewport":{"intent":"isolated","css_width":1024},"aspect_ratio":"1:1","transparent_background":true,"extracted_from_item_id":"item_1","target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_1"],"direction":"Flat semi-flat SaaS brand mark: a rounded-square indigo badge with a bold white letter M, paired with the dark navy MetricFlow wordmark in a modern geometric sans-serif.","exact_copy":[],"reference_asset_ids":[],"depends_on":["item_1"],"image_prompt":"Extract ONLY the MetricFlow brand logo from the attached reference UI image, located at the top-left of the sidebar: the indigo rounded-square icon containing a bold white letter M, plus the dark navy ''MetricFlow'' wordmark text beside it. Isolate both on a clean 100% transparent background, removing all sidebar background, shadows, and unrelated UI elements. Reproduce the exact indigo color, corner radius, letterform weight, and wordmark typography faithfully.","implementation_notes":["Render on a 100% transparent PNG background and layer in the component/page layout"],"behaviors":[],"responsive_rules":["Scale proportionally across breakpoints"],"asset_instructions":["Place this logo at the top-left of the sidebar navigation, exactly as shown in the reference. Recreate it as a vectorial SVG graphic: the rounded-square badge as an SVG rect with the indigo fill and the M as a path or bold text, and the wordmark as SVG text in the matching geometric sans-serif, so it scales crisply at all sizes. Do not embed it as a raster screenshot."],"is_visual_anchor":false}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T20:11:43.574744+00:00', '2026-10-24T20:12:01.128389+00:00', 'dashboard', 'Dashboard', 'example-out-metricflow-1', 1, 'example-out-metricflow-1');
INSERT INTO thread_messages (id, thread_id, role, content, created_at) VALUES ('example-message-metricflow-1', 'example-thread-metricflow', 'user', 'Tagline: Your Business. Every Insight. One Dashboard.

An advanced business analytics dashboard for tracking revenue, customer growth, projects, and performance. Includes interactive charts, KPI cards, activity feeds, data tables, reporting tools, and sidebar navigation.', '2026-10-24T20:10:36.843032+00:00');
INSERT INTO thread_messages (id, thread_id, role, content, linked_run_id, created_at) VALUES ('example-message-metricflow-2', 'example-thread-metricflow', 'assistant', 'MetricFlow showcase, including the finished designs and extracted assets. Open any image to inspect it or copy its implementation prompt.', 'example-run-metricflow', '2026-10-24T20:12:01.141642+00:00');

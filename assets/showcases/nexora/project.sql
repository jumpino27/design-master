INSERT INTO projects (id, name, description, purpose, industry, target_audience, audience_needs, desired_user_action, archive_status, created_at, updated_at, is_example) VALUES ('example-nexora', 'Nexora', 'Tagline: Turn Big Ideas Into Business Growth.

A premium SaaS landing page designed to convert visitors into customers. Features a compelling hero section, product benefits, interactive previews, testimonials, pricing plans, and strong calls to action.', 'A premium SaaS landing page for Nexora that converts visitors into customers through a compelling hero, clear product benefits, interactive previews, social proof, pricing, and strong CTAs.', 'B2B SaaS / Business Growth Software', 'Founders, startup teams, and growth-focused business decision-makers evaluating tools to scale revenue', 'Quickly understand the value proposition, see the product in action, trust the brand via testimonials, and compare pricing without friction', 'Sign up for a free trial or request a demo via the primary CTA', 'active', '2026-10-24T19:14:31.526870+00:00', '2026-10-24T19:14:31.526870+00:00', 1);
INSERT INTO brand_versions (id, project_id, version, brand_json, created_at) VALUES ('example-brand-nexora-1', 'example-nexora', 1, '{"brand_personality": "Confident, modern, and growth-oriented — polished and premium without feeling corporate or cold", "design_direction": "Contemporary SaaS aesthetic: crisp typography, generous whitespace, hairline borders, soft ambient elevation, and a single confident accent color guiding every CTA. Interactive product previews sit in elevated surface cards with refined micro-interactions.", "tone_of_voice": "Clear, aspirational, and benefit-driven — ''Turn Big Ideas Into Business Growth''", "content_language": "English", "ui_density": "comfortable", "logo": {"mode": "temporary_monogram", "monogram_text": "N", "primary_asset_id": null, "mark_only_asset_id": null, "light_variant_asset_id": null, "dark_variant_asset_id": null, "preferred_usage": "Top-left header or card header mark with clear breathing room", "inclusion_policy": "allowed"}, "colors": {"light": {"primary": "#4F46E5", "accent": "#06B6D4", "background": "#FAFAFC", "surface": "#FFFFFF", "text": "#0F172A", "muted_text": "#64748B", "border": "#E2E8F0", "success": "#2B7A4B", "warning": "#B7791F", "danger": "#C53030"}, "dark": {"primary": "#818CF8", "accent": "#22D3EE", "background": "#0B1120", "surface": "#151D2E", "text": "#F1F5F9", "muted_text": "#94A3B8", "border": "#26324A", "success": "#38A169", "warning": "#D69E2E", "danger": "#E53E3E"}}, "typography": {"heading_family": "Inter / Sora — SemiBold, tight tracking, 48px hero / 24px section scale", "body_family": "Inter — Regular, 15–16px, 1.6 line height", "mono_family": "JetBrains Mono — for code snippets, metrics, and pricing figures", "fallback_stack": "-apple-system, BlinkMacSystemFont, ''Segoe UI'', sans-serif", "weights": [400, 500, 600, 700], "size_scale": "12px / 14px / 16px / 20px / 28px / 40px", "line_heights": "Heading 1.15, Body 1.5, Compact UI 1.3", "font_source": "google_fonts", "uploaded_font_asset_ids": []}, "layout_tokens": {"spacing_scale": "4px / 8px / 16px / 24px / 32px / 48px / 64px / 96px", "content_width": "1200px max container with 24px padding", "grid_columns": "12-column responsive grid, 24px gap", "radii": "12–16px cards, 10px buttons, 999px pills", "borders": "1px solid subtle warm border (#E2DED5)", "shadows": "0 8px 24px rgba(15,23,42,0.06) light / 0 8px 24px rgba(0,0,0,0.4) dark", "icon_style": "2px crisp geometric stroke icons, 20–24px, rounded joins", "button_treatment": "Solid high-contrast primary CTA with subtle arrow affordance"}, "brand_rules": ["Sticky top navigation with a single high-contrast primary CTA in brand primary color", "Hero leads with the value proposition and one primary action; never compete with dual CTAs of equal weight", "Use 1px hairline borders with soft ambient elevation on feature and preview cards", "Restrained neutral surfaces with the indigo primary reserved for actions, links, and key highlights", "Cyan accent used sparingly for secondary highlights, badges, and data visualization", "48px–96px section padding with a 12-column grid and 60%+ open whitespace", "Pricing section uses a highlighted ''recommended'' plan card in brand primary", "Testimonials presented as clean quote cards with avatar, name, and role", "Support both light and dark modes with equal contrast ratios (WCAG AA minimum)"], "mandatory_inclusions": [], "forbidden_treatments": ["No decorative clutter, gradients-on-everything, or heavy skeuomorphic effects", "No more than one accent color competing with the primary CTA", "No thin light-gray text below 4.5:1 contrast on backgrounds", "No stock-photo clichés of handshakes or generic office scenes", "No aggressive popups, auto-playing video with sound, or dark patterns in pricing", "No serif display fonts or overly playful rounded type that undermines premium credibility"], "approved_reference_asset_ids": [], "target_context": {"target_stack": "Next.js App Router + TypeScript + Tailwind CSS", "repository_root": "", "preferred_asset_dir": "public/assets", "component_library_notes": "Accessible Radix primitives with Lucide icons"}, "design_style": {"style_id": "02-modern-contemporary", "style_number": 2, "style_name": "Modern / Contemporary", "definition": "A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time.", "category": "Minimal & Editorial", "visual_rules": "Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.", "keywords": ["modern", "contemporary", "current", "saas", "polished", "hierarchy", "balanced", "sleek", "product", "linear", "vercel", "stripe", "clean"], "modifiers": [], "combined_brief": "Style brief: \"modern / contemporary\". Primary style #2 Modern / Contemporary (Minimal & Editorial): A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states."}}', '2026-10-24T19:14:31.526870+00:00');
INSERT INTO threads (id, project_id, title, archive_status, created_at, updated_at) VALUES ('example-thread-nexora', 'example-nexora', 'Main Design Workspace', 'active', '2026-10-24T19:14:31.526870+00:00', '2026-10-24T19:16:09.163486+00:00');
INSERT INTO runs (id, project_id, thread_id, idempotency_key, mode, scope_preset, user_prompt, brand_version, brand_snapshot_json, selected_references_json, model_settings_json, plan_json, consistency_report_json, status, estimated_cost_micros, actual_cost_micros, cost_is_unknown, is_test_adapter, error_message, created_at, updated_at, completed_at, target_tab_key, target_tab_label, target_output_id, highlight_annotations_json) VALUES ('example-run-nexora', 'example-nexora', 'example-thread-nexora', NULL, 'full_page', 'full_page', 'Generate a premium SaaS landing page for Nexora — ''Turn Big Ideas Into Business Growth'' — as a single Desktop full-page scroll at 1440px width rendered top-to-bottom in a vertical 2:3 canvas. Style #2 Modern/Contemporary: crisp hierarchy, polished typography, generous whitespace, hairline 1px borders, soft ambient elevation, one confident accent color guiding every CTA. STRICT Brand Kit: Dark theme — Background #0B1120, Surface #151D2E, Primary #818CF8, Accent #22D3EE; typography: Sora/Inter SemiBold headings with tight tracking (48px hero, 24px sections), Inter Regular 15–16px body at 1.6 line height, JetBrains Mono for metrics, pricing figures, and any code snippets. Icons: 2px crisp geometric stroke, 20–24px, rounded joins. Buttons: solid high-contrast primary CTA with subtle arrow affordance. PAGE SECTIONS in order: (1) Sticky top Navbar — Nexora logo mark left, links (Product, Benefits, Pricing, Testimonials), ''Start Free Trial'' primary CTA right; (2) HERO — asymmetric editorial split: left column bold oversized display headline ''Turn Big Ideas Into Business Growth'' with supporting subcopy and dual CTAs (''Start Free Trial'' primary, ''Request a Demo'' ghost), right column a refined 3D growth emblem (abstract ascending arrow/bar sculpture in indigo-cyan gradient, soft ambient glow, floating on dark surface — NOT a cliché purple-blue SaaS gradient blob); (3) BENEFITS — asymmetric bento layout (not a boring 3-column icon-in-circle grid): mixed-size surface tiles with hairline borders, each with a stroke icon, short authentic benefit copy, and one tile featuring a JetBrains Mono metric callout; (4) INTERACTIVE PRODUCT PREVIEW — an elevated surface card showing a believable Nexora analytics dashboard mockup (charts, KPI figures in mono font, sidebar) with subtle micro-interaction affordances; (5) TESTIMONIALS — social proof row with realistic founder names, roles, and company wordmarks, varied card sizes, no filler copy; (6) PRICING — three tiers (Starter, Growth, Scale) with JetBrains Mono figures, highlighted recommended tier using the accent color, feature lists with stroke check icons; (7) FINAL CTA — full-width confident band with large headline and primary CTA; (8) FOOTER — logo, link columns, subtle hairline top border. ANTI-AI-SLOP MANDATE (pbakaus/impeccable standard): no card-in-card nesting, no repeated identical 3-column grids, no purple-to-blue SaaS gradients, no floating neon blobs, no low-contrast gray text on colored surfaces, no fake AI filler copy — pass the squint test with one unmistakable #1 focal point (hero headline + emblem), clear #2 secondary action, and generous breathing room. Also generate as separate transparent PNG assets in the same run: (a) the Nexora logo mark — abstract geometric monogram in Primary #818CF8 with Accent #22D3EE detail, isolated on transparent background; (b) a geometric stroke icon sheet — 8–10 icons (growth chart, bolt, shield, users, integration, target, rocket, clock) in 2px rounded-join stroke style, consistent 24px grid, isolated on transparent background, 1:1 asset layout.', 1, '{"brand_personality": "Confident, modern, and growth-oriented — polished and premium without feeling corporate or cold", "design_direction": "Contemporary SaaS aesthetic: crisp typography, generous whitespace, hairline borders, soft ambient elevation, and a single confident accent color guiding every CTA. Interactive product previews sit in elevated surface cards with refined micro-interactions.", "tone_of_voice": "Clear, aspirational, and benefit-driven — ''Turn Big Ideas Into Business Growth''", "content_language": "English", "ui_density": "comfortable", "logo": {"mode": "temporary_monogram", "monogram_text": "N", "primary_asset_id": null, "mark_only_asset_id": null, "light_variant_asset_id": null, "dark_variant_asset_id": null, "preferred_usage": "Top-left header or card header mark with clear breathing room", "inclusion_policy": "allowed"}, "colors": {"light": {"primary": "#4F46E5", "accent": "#06B6D4", "background": "#FAFAFC", "surface": "#FFFFFF", "text": "#0F172A", "muted_text": "#64748B", "border": "#E2E8F0", "success": "#2B7A4B", "warning": "#B7791F", "danger": "#C53030"}, "dark": {"primary": "#818CF8", "accent": "#22D3EE", "background": "#0B1120", "surface": "#151D2E", "text": "#F1F5F9", "muted_text": "#94A3B8", "border": "#26324A", "success": "#38A169", "warning": "#D69E2E", "danger": "#E53E3E"}}, "typography": {"heading_family": "Inter / Sora — SemiBold, tight tracking, 48px hero / 24px section scale", "body_family": "Inter — Regular, 15–16px, 1.6 line height", "mono_family": "JetBrains Mono — for code snippets, metrics, and pricing figures", "fallback_stack": "-apple-system, BlinkMacSystemFont, ''Segoe UI'', sans-serif", "weights": [400, 500, 600, 700], "size_scale": "12px / 14px / 16px / 20px / 28px / 40px", "line_heights": "Heading 1.15, Body 1.5, Compact UI 1.3", "font_source": "google_fonts", "uploaded_font_asset_ids": []}, "layout_tokens": {"spacing_scale": "4px / 8px / 16px / 24px / 32px / 48px / 64px / 96px", "content_width": "1200px max container with 24px padding", "grid_columns": "12-column responsive grid, 24px gap", "radii": "12–16px cards, 10px buttons, 999px pills", "borders": "1px solid subtle warm border (#E2DED5)", "shadows": "0 8px 24px rgba(15,23,42,0.06) light / 0 8px 24px rgba(0,0,0,0.4) dark", "icon_style": "2px crisp geometric stroke icons, 20–24px, rounded joins", "button_treatment": "Solid high-contrast primary CTA with subtle arrow affordance"}, "brand_rules": ["Sticky top navigation with a single high-contrast primary CTA in brand primary color", "Hero leads with the value proposition and one primary action; never compete with dual CTAs of equal weight", "Use 1px hairline borders with soft ambient elevation on feature and preview cards", "Restrained neutral surfaces with the indigo primary reserved for actions, links, and key highlights", "Cyan accent used sparingly for secondary highlights, badges, and data visualization", "48px–96px section padding with a 12-column grid and 60%+ open whitespace", "Pricing section uses a highlighted ''recommended'' plan card in brand primary", "Testimonials presented as clean quote cards with avatar, name, and role", "Support both light and dark modes with equal contrast ratios (WCAG AA minimum)"], "mandatory_inclusions": [], "forbidden_treatments": ["No decorative clutter, gradients-on-everything, or heavy skeuomorphic effects", "No more than one accent color competing with the primary CTA", "No thin light-gray text below 4.5:1 contrast on backgrounds", "No stock-photo clichés of handshakes or generic office scenes", "No aggressive popups, auto-playing video with sound, or dark patterns in pricing", "No serif display fonts or overly playful rounded type that undermines premium credibility"], "approved_reference_asset_ids": [], "target_context": {"target_stack": "Next.js App Router + TypeScript + Tailwind CSS", "repository_root": "", "preferred_asset_dir": "public/assets", "component_library_notes": "Accessible Radix primitives with Lucide icons"}, "design_style": {"style_id": "02-modern-contemporary", "style_number": 2, "style_name": "Modern / Contemporary", "definition": "A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time.", "category": "Minimal & Editorial", "visual_rules": "Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.", "keywords": ["modern", "contemporary", "current", "saas", "polished", "hierarchy", "balanced", "sleek", "product", "linear", "vercel", "stripe", "clean"], "modifiers": [], "combined_brief": "Style brief: \"modern / contemporary\". Primary style #2 Modern / Contemporary (Minimal & Editorial): A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states."}}', '[]', '{"planner_model":"z-ai/glm-5.3-flash","image_model":"openai/gpt-image-2.5-sunburst","quality":"medium","aspect_ratio":"2:3","output_size":"provider_default","scope_preset":"full_page","custom_dimensions":null,"review_plan_first":false,"asset_options":null,"target_tab_key":"full-landing-page","target_tab_label":"Full Landing Page","target_existing_output_id":null,"viewport_intent":"desktop","highlight_annotations":[],"single_focused_output":false}', '{
  "schema_version": 1,
  "mode": "full_page",
  "request_summary": "Premium dark-theme SaaS landing page for Nexora (''Turn Big Ideas Into Business Growth'') at 1440px desktop width, rendered top-to-bottom in a vertical 2:3 canvas. Style #2 Modern/Contemporary with strict Brand Kit dark tokens (#0B1120 bg, #151D2E surface, #818CF8 primary, #22D3EE accent), Sora/Inter typography, JetBrains Mono metrics, hairline borders, soft ambient elevation, asymmetric bento benefits, interactive product preview, testimonials, pricing with highlighted recommended tier, final CTA band, and footer. Plus two transparent PNG assets: Nexora logo monogram and a geometric stroke icon sheet.",
  "requirements": [
    {
      "id": "req_1",
      "description": "Single desktop full-page vertical scroll landing page at 1440px width in 2:3 canvas, dark theme Brand Kit colors",
      "source": "user"
    },
    {
      "id": "req_2",
      "description": "Sticky top navbar with logo, links (Product, Benefits, Pricing, Testimonials), and ''Start Free Trial'' primary CTA",
      "source": "user"
    },
    {
      "id": "req_3",
      "description": "Hero asymmetric editorial split with oversized headline, dual CTAs, and refined 3D growth emblem (indigo-cyan, not cliché gradient blob)",
      "source": "user"
    },
    {
      "id": "req_4",
      "description": "Asymmetric bento benefits grid with stroke icons and one JetBrains Mono metric callout tile",
      "source": "user"
    },
    {
      "id": "req_5",
      "description": "Interactive product preview card with believable analytics dashboard mockup (charts, mono KPIs, sidebar)",
      "source": "user"
    },
    {
      "id": "req_6",
      "description": "Testimonials row with realistic founder names, roles, company wordmarks, varied card sizes",
      "source": "user"
    },
    {
      "id": "req_7",
      "description": "Three pricing tiers (Starter, Growth, Scale) with mono figures and highlighted recommended tier in accent color",
      "source": "user"
    },
    {
      "id": "req_8",
      "description": "Final full-width CTA band and structured footer with hairline top border",
      "source": "user"
    },
    {
      "id": "req_9",
      "description": "Transparent PNG assets: Nexora geometric monogram logo and 8-10 icon stroke sheet on 24px grid",
      "source": "user"
    },
    {
      "id": "req_10",
      "description": "Anti-AI-slop mandate: no card-in-card nesting, no purple-blue gradient blobs, no filler copy, one unmistakable focal point",
      "source": "user"
    }
  ],
  "assumptions": [
    "Dark theme is the primary (and only) rendered theme in the generated image; light mode support is noted for implementation but not depicted",
    "Testimonial avatars and company wordmarks are placeholder sample content to be replaced by real data later",
    "Dashboard mockup inside the product preview is a static believable UI illustration, not a live charting library in the generated image",
    "Pricing figures are sample values in JetBrains Mono, structured as clean placeholder data"
  ],
  "shared_design_spec": {
    "colors": {
      "text": "#F1F5F9",
      "primary": "#818CF8",
      "surface": "#151D2E",
      "background": "#0B1120"
    },
    "typography": {
      "heading": "Sora / Inter SemiBold, tight tracking, 48px hero / 24px sections",
      "body": "Inter Regular, 15-16px, 1.6 line height",
      "mono": "JetBrains Mono for metrics, pricing figures, and code snippets"
    },
    "spacing": {
      "base": "4px / 8px / 16px / 24px / 32px / 48px / 64px / 96px scale with 48-96px section padding on a 12-column grid and 60%+ open whitespace"
    },
    "radii": {
      "button": "10px (999px for pills)",
      "card": "12-16px"
    },
    "shadows": {
      "card": "0 8px 24px rgba(0,0,0,0.4) with 1px solid #26324A hairline borders"
    },
    "layout_rules": [
      "STICKY VS NON-STICKY: The top navigation bar stays anchored at the top of the viewport on scroll with a translucent dark surface and a hairline bottom border; every other section (hero, benefits, preview, testimonials, pricing, final CTA, footer) scrolls naturally in normal page flow",
      "SQUINT TEST HIERARCHY: The hero headline plus 3D growth emblem is the single unmistakable primary visual anchor; the ''Start Free Trial'' primary CTA is the clear secondary action; all other sections use restrained neutral surfaces with indigo reserved for actions, links, and key highlights and cyan used sparingly for badges and data highlights",
      "ANTI-CARD-CLUSTER RULE: Never nest cards inside cards; use 1px hairline borders (#26324A), intentional negative space, and asymmetric bento or editorial split compositions instead of monotonous repeated 3-column icon-in-circle rows",
      "CONTRAST DISCIPLINE: All text meets WCAG AA on dark surfaces; body copy is left-aligned; metrics and pricing use tabular numerals in JetBrains Mono; never place low-contrast gray text on colored or translucent backgrounds",
      "NATIVE TRANSPARENT PNG ASSETS: All standalone icons, icon sheets, badges, 3D object references, and extracted foreground illustrations must set transparent_background=true so Sunburst 2.5 renders them on a 100% native transparent alpha background"
    ],
    "asset_rules": [
      "NATIVE TRANSPARENT PNG ASSETS: All standalone icons, icon sheets, badges, 3D object references, and extracted foreground illustrations must set transparent_background=true so Sunburst 2.5 renders them on a 100% native transparent alpha background",
      "Icons are generated together in one organized icon sheet image with clean spacing so they can be converted into vectorial SVG graphics or sliced directly",
      "The 3D growth emblem reference is extracted as a transparent asset so it can be reconstructed as an interactive rotating 3D object rather than a flat static image",
      "CODED SEPARATOR / BACKGROUND / 3D SPEC FROM IMAGE (Nexora Premium SaaS Landing Page (Full Desktop Scroll, Dark Theme)): Section separators: This page uses no hard rule lines. Between the hero and the features section there is a soft horizontal fade where the hero''s darker navy canvas blends into the slightly lighter dark-navy features background — recreate this as a smooth vertical gradient transition, not a line. Each subsequent section (dashboard, testimonials, pricing, CTA banner, footer) is separated only by generous vertical whitespace and subtle changes in surface elevation; the footer is divided from the page above by a faint darker band. Recreate all of these 1:1 in code as gradient/spacing transitions identical to the image.",
      "CODED SEPARATOR / BACKGROUND / 3D SPEC FROM IMAGE (Nexora Premium SaaS Landing Page (Full Desktop Scroll, Dark Theme)): Background layers: The entire page sits on a very dark navy (near-black blue) canvas. The hero has a large soft radial blue glow behind the 3D sculpture on the right and a subtle darker vignette on the left. The features section cards sit on slightly lighter navy surfaces with thin translucent borders. The final CTA banner is a rounded rectangle with a blue-to-indigo horizontal gradient glow. Add a very subtle grain/noise texture across the dark canvas. Recreate every glow, gradient, and texture layer in code (CSS gradients and effects) so the background matches the image identically — do not use a flat background screenshot.",
      "CODED SEPARATOR / BACKGROUND / 3D SPEC FROM IMAGE (Nexora Premium SaaS Landing Page (Full Desktop Scroll, Dark Theme)): 3D objects and animation: The hero''s 3D growth-bar-and-arrow sculpture must be built as an interactive 3D object from its extracted reference: three glossy extruded bars (purple, blue, cyan) ascending left to right on a dark pedestal, with a curved translucent cyan-to-blue arrow ribbon arcing above them. Give it a slow continuous idle rotation around the vertical axis, a gentle floating bob, and a pointer-following tilt on hover, with soft ambient blue lighting and a glow beneath it matching the image.",
      "CODED SEPARATOR / BACKGROUND / 3D SPEC FROM IMAGE (Nexora Premium SaaS Landing Page (Full Desktop Scroll, Dark Theme)): Card and glow treatments: All cards (feature tiles, testimonial cards, pricing cards, stat cards) use a slightly lighter navy fill, a one-pixel translucent light-blue border, and large rounded corners; the recommended Growth pricing card has a bright cyan border with a cyan ''Recommended'' pill overlapping its top edge. Recreate these card surfaces, borders, and the cyan highlight treatment exactly in code to match the image."
    ]
  },
  "items": [
    {
      "id": "item_1",
      "kind": "page",
      "name": "Nexora Premium SaaS Landing Page (Full Desktop Scroll, Dark Theme)",
      "tab_key": "full-landing-page",
      "tab_label": "Full Landing Page",
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
        "req_6",
        "req_7",
        "req_8",
        "req_10"
      ],
      "direction": "Style #2 Modern/Contemporary dark-theme premium SaaS landing page: crisp hierarchy, polished Sora/Inter typography, generous whitespace, 1px hairline borders (#26324A), soft ambient elevation, single confident indigo primary guiding every CTA with sparing cyan accent. Asymmetric editorial compositions, no card-in-card nesting, no gradient blobs.",
      "exact_copy": [
        "Nexora",
        "Product",
        "Benefits",
        "Pricing",
        "Testimonials",
        "Start Free Trial",
        "Turn Big Ideas Into Business Growth",
        "Nexora turns scattered growth experiments into a single, measurable engine — plan campaigns, track revenue impact, and scale what works, all from one workspace.",
        "Request a Demo",
        "Trusted by 2,400+ growth teams",
        "Everything You Need to Grow, Nothing You Don''t",
        "Revenue Attribution",
        "See exactly which experiments move revenue, down to the cohort.",
        "Rapid Experimentation",
        "Launch, measure, and iterate on growth tests in hours, not weeks.",
        "Team Alignment",
        "Shared roadmaps keep product, marketing, and sales on the same page.",
        "Enterprise-Grade Security",
        "SOC 2 Type II, SSO, and granular permissions out of the box.",
        "+38%",
        "average revenue lift within the first quarter for active teams",
        "Your Growth Engine, Live",
        "MRR $284,930",
        "Conversion 4.8%",
        "Active Experiments 27",
        "See Nexora in action",
        "Founders Who Ship Faster With Nexora",
        "We replaced four tools with Nexora and finally know what actually drives our pipeline. It paid for itself in the first month.",
        "Maya Chen",
        "Co-founder & CEO, Loopwave",
        "The experimentation cadence Nexora gave us turned guesswork into a repeatable growth system.",
        "Daniel Okafor",
        "VP Growth, Fernbook",
        "Simple Pricing That Scales With You",
        "Starter",
        "$29/mo",
        "For small teams validating their first growth loops",
        "3 seats included",
        "Core analytics dashboard",
        "10 active experiments",
        "Email support",
        "Growth",
        "$79/mo",
        "Recommended",
        "For teams scaling revenue with structured experimentation",
        "Unlimited seats",
        "Revenue attribution engine",
        "Unlimited experiments",
        "Priority support & onboarding",
        "Scale",
        "$199/mo",
        "For organizations running growth as a company-wide discipline",
        "SSO & advanced permissions",
        "Custom data residency",
        "Dedicated success manager",
        "Ready to Turn Your Big Idea Into Growth?",
        "Join 2,400+ teams already growing with Nexora. Free for 14 days, no credit card required.",
        "Product",
        "Features",
        "Integrations",
        "Changelog",
        "Company",
        "About",
        "Careers",
        "Blog",
        "Resources",
        "Docs",
        "Help Center",
        "Security",
        "© 2025 Nexora, Inc. All rights reserved.",
        "Privacy",
        "Terms"
      ],
      "reference_asset_ids": [],
      "depends_on": [],
      "image_prompt": "A tall vertical 2:3 top-to-bottom full-page scroll capture of a 1440px-wide premium dark-theme SaaS landing page for ''Nexora'' — Style #2 Modern/Contemporary, minimal and editorial. Strict dark palette: background #0B1120, surface cards #151D2E, hairline 1px borders #26324A, text #F1F5F9, muted #94A3B8, primary indigo #818CF8, accent cyan #22D3EE. Typography: Sora SemiBold tight-tracked headings (48px hero, 24px sections), Inter Regular 15-16px body, JetBrains Mono for metrics and pricing. Generous whitespace, 12-column grid, 12-16px card radii, soft ambient elevation. TOP: sticky translucent dark navbar with a small geometric monogram logo left, horizontal links ''Product, Benefits, Pricing, Testimonials'', and one solid indigo ''Start Free Trial'' button with subtle arrow affordance right. SECTION 1 HERO — asymmetric editorial split: left column oversized display headline ''Turn Big Ideas Into Business Growth'', supporting subcopy, primary indigo CTA ''Start Free Trial'' and ghost secondary ''Request a Demo'', small trust line ''Trusted by 2,400+ growth teams''; right column a refined sculptural 3D growth emblem — an abstract ascending arrow and bar sculpture in indigo-to-cyan material with soft ambient glow floating on the dark surface (a crafted physical object, NOT a glowing gradient blob). SECTION 2 BENEFITS — asymmetric bento grid of mixed-size surface tiles with hairline borders, each with a crisp 2px stroke icon and short benefit copy; one larger tile features a large JetBrains Mono metric callout ''+38%''. No identical 3-column icon-in-circle rows. SECTION 3 PRODUCT PREVIEW — one large elevated surface card containing a believable analytics dashboard mockup: left sidebar, KPI figures in mono font (MRR $284,930, Conversion 4.8%, Active Experiments 27), clean line and bar charts using indigo and cyan data colors. SECTION 4 TESTIMONIALS — varied-size quote cards with small circular avatars, founder names and roles, subtle company wordmarks. SECTION 5 PRICING — three tier cards (Starter $29/mo, Growth $79/mo, Scale $199/mo) with JetBrains Mono figures and stroke check icons; the middle ''Growth'' card is highlighted with a cyan ''Recommended'' pill and elevated border treatment. SECTION 6 FINAL CTA — full-width confident band with large headline ''Ready to Turn Your Big Idea Into Growth?'' and one solid indigo CTA. FOOTER — logo, four link columns, hairline top border, copyright row. Crisp, premium, Linear/Stripe-caliber craft; no purple-blue SaaS gradient soup, no floating neon blobs, no card-in-card clutter, no low-contrast gray text, no stock-photo clichés.",
      "implementation_notes": [
        "POSITIONING (STICKY VS NON-STICKY): The top navigation bar stays anchored at the top of the viewport for the entire scroll, rendered as a translucent dark surface over the page background with a 1px hairline bottom border; the hero, benefits, product preview, testimonials, pricing, final CTA band, and footer all scroll naturally in normal document flow",
        "ANTI-AI-SLOP LAYOUT & VISUAL HIERARCHY (pbakaus/impeccable & DESIGN.md RULES): The hero headline plus the 3D growth emblem is the single unmistakable primary visual anchor and the ''Start Free Trial'' button is the clear secondary action; avoid wrapping every section in boxed cards containing nested sub-cards — use 1px hairline borders in #26324A, intentional negative space, and asymmetric bento and editorial split compositions; apply semantic tokens exactly: background #0B1120, surfaces #151D2E, primary indigo #818CF8 reserved for actions, links, and key highlights, cyan #22D3EE used sparingly for the Recommended badge and data highlights, headings in Sora SemiBold with tight tracking, body in Inter at 1.6 line height left-aligned, and all metrics and pricing figures in JetBrains Mono with tabular numerals",
        "PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: The testimonial avatars, founder names, roles, and company wordmarks are placeholder sample content for a future database; the analytics dashboard mockup inside the product preview card (sidebar, KPI figures, charts) is a static believable illustration acting as a product preview placeholder; every card, tile, quote card, pricing tier, dashboard mockup frame, button, navbar, and footer layout must be implemented 1:1 in code right now with the sample data structured as clean placeholder data",
        "CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Build purely with code the section separators, hairline rule lines, ambient radial glows behind the hero emblem, subtle dot or grid backdrops, all surface cards, buttons, navbar, pricing tiers, and footer; convert the Nexora logo monogram and all icons from the extracted transparent icon sheet into scalable vectorial SVG graphics for razor-sharp quality; reconstruct the hero 3D growth emblem from its extracted transparent reference image as an interactive 3D object (Image-to-3D mesh or procedural 3D in Three.js / React Three Fiber) with continuous smooth rotation on its vertical axis, a gentle floating bobbing idle motion, and interactive cursor-tracking tilt with a hover spin and soft glow",
        "FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Use crisp physical spring physics with stiffness 320 and damping 28 (never bounce or elastic easing) for button and card hover and press feedback; staggered scroll reveals for bento tiles, testimonial cards, and pricing tiers using an exponential ease-out curve like cubic-bezier(0.22, 1, 0.36, 1); a shared animated underline indicator slides between active navbar links; the pricing recommended card lifts subtly on hover; the 3D emblem rotates continuously and tilts toward the cursor; all motion respects reduced-motion accessibility preferences by falling back to instant opacity transitions",
        "REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No existing screenshot or previous output was attached, so this is a brand-new build from scratch with no component replacement required",
        "CODED SEPARATOR / BACKGROUND SPEC: Section separators: This page uses no hard rule lines. Between the hero and the features section there is a soft horizontal fade where the hero''s darker navy canvas blends into the slightly lighter dark-navy features background — recreate this as a smooth vertical gradient transition, not a line. Each subsequent section (dashboard, testimonials, pricing, CTA banner, footer) is separated only by generous vertical whitespace and subtle changes in surface elevation; the footer is divided from the page above by a faint darker band. Recreate all of these 1:1 in code as gradient/spacing transitions identical to the image.",
        "CODED SEPARATOR / BACKGROUND SPEC: Background layers: The entire page sits on a very dark navy (near-black blue) canvas. The hero has a large soft radial blue glow behind the 3D sculpture on the right and a subtle darker vignette on the left. The features section cards sit on slightly lighter navy surfaces with thin translucent borders. The final CTA banner is a rounded rectangle with a blue-to-indigo horizontal gradient glow. Add a very subtle grain/noise texture across the dark canvas. Recreate every glow, gradient, and texture layer in code (CSS gradients and effects) so the background matches the image identically — do not use a flat background screenshot.",
        "CODED SEPARATOR / BACKGROUND SPEC: 3D objects and animation: The hero''s 3D growth-bar-and-arrow sculpture must be built as an interactive 3D object from its extracted reference: three glossy extruded bars (purple, blue, cyan) ascending left to right on a dark pedestal, with a curved translucent cyan-to-blue arrow ribbon arcing above them. Give it a slow continuous idle rotation around the vertical axis, a gentle floating bob, and a pointer-following tilt on hover, with soft ambient blue lighting and a glow beneath it matching the image.",
        "CODED SEPARATOR / BACKGROUND SPEC: Card and glow treatments: All cards (feature tiles, testimonial cards, pricing cards, stat cards) use a slightly lighter navy fill, a one-pixel translucent light-blue border, and large rounded corners; the recommended Growth pricing card has a bright cyan border with a cyan ''Recommended'' pill overlapping its top edge. Recreate these card surfaces, borders, and the cyan highlight treatment exactly in code to match the image."
      ],
      "behaviors": [
        "HOVER STATE: Primary CTA hover shifts background from #818CF8 to a slightly lighter indigo with a soft ambient glow and a subtle upward lift of 2px; ghost CTA hover fills with surface #151D2E and a #26324A border; nav links hover to full text color #F1F5F9 from muted #94A3B8; surface cards hover elevate shadow to 0 12px 32px rgba(0,0,0,0.5) with border brightening toward #3B4A6B",
        "CLICK / ACTIVE STATE: Primary CTA pressed state compresses scale slightly toward 0.98 with background deepened toward #6366F1 and no shadow; nav links show an active indigo underline indicator on the current section; pricing tier selection highlights the chosen card border in #22D3EE",
        "FOCUS & DISABLED STATES: Keyboard focus rings use a 2px #22D3EE outline with 2px offset on all interactive elements; disabled buttons render at 40% opacity with no hover or press response"
      ],
      "responsive_rules": [
        "Desktop (1440px): full 12-column layout exactly as depicted with asymmetric hero split and multi-column bento, testimonial, and pricing grids",
        "Tablet (834px): hero stacks vertically with the emblem below the headline, bento collapses to a 2-column grid, testimonials to 2 columns, pricing to a horizontal scroll or stacked cards, and the navbar condenses with links behind a hamburger menu",
        "Mobile (390px): single-column stacked layout, hero headline scales down to a 32-36px fluid size, CTAs become full-width, bento tiles stack, pricing tiers stack vertically with the recommended tier first, and the footer link columns collapse into accordions"
      ],
      "asset_instructions": [
        "Use the extracted transparent logo monogram as the navbar and footer brand mark, converted into a scalable vectorial SVG graphic for razor-sharp rendering at any size",
        "Use the extracted transparent icon sheet by slicing each icon and converting them into vectorial SVG graphics on a consistent 24px grid with 2px rounded-join strokes",
        "Use the extracted transparent 3D growth emblem reference to reconstruct an interactive rotating 3D object (Image-to-3D or procedural 3D) rather than layering it as a flat static image",
        "CODED SEPARATOR / BACKGROUND SPEC: Section separators: This page uses no hard rule lines. Between the hero and the features section there is a soft horizontal fade where the hero''s darker navy canvas blends into the slightly lighter dark-navy features background — recreate this as a smooth vertical gradient transition, not a line. Each subsequent section (dashboard, testimonials, pricing, CTA banner, footer) is separated only by generous vertical whitespace and subtle changes in surface elevation; the footer is divided from the page above by a faint darker band. Recreate all of these 1:1 in code as gradient/spacing transitions identical to the image.",
        "CODED SEPARATOR / BACKGROUND SPEC: Background layers: The entire page sits on a very dark navy (near-black blue) canvas. The hero has a large soft radial blue glow behind the 3D sculpture on the right and a subtle darker vignette on the left. The features section cards sit on slightly lighter navy surfaces with thin translucent borders. The final CTA banner is a rounded rectangle with a blue-to-indigo horizontal gradient glow. Add a very subtle grain/noise texture across the dark canvas. Recreate every glow, gradient, and texture layer in code (CSS gradients and effects) so the background matches the image identically — do not use a flat background screenshot.",
        "CODED SEPARATOR / BACKGROUND SPEC: 3D objects and animation: The hero''s 3D growth-bar-and-arrow sculpture must be built as an interactive 3D object from its extracted reference: three glossy extruded bars (purple, blue, cyan) ascending left to right on a dark pedestal, with a curved translucent cyan-to-blue arrow ribbon arcing above them. Give it a slow continuous idle rotation around the vertical axis, a gentle floating bob, and a pointer-following tilt on hover, with soft ambient blue lighting and a glow beneath it matching the image.",
        "CODED SEPARATOR / BACKGROUND SPEC: Card and glow treatments: All cards (feature tiles, testimonial cards, pricing cards, stat cards) use a slightly lighter navy fill, a one-pixel translucent light-blue border, and large rounded corners; the recommended Growth pricing card has a bright cyan border with a cyan ''Recommended'' pill overlapping its top edge. Recreate these card surfaces, borders, and the cyan highlight treatment exactly in code to match the image."
      ],
      "is_visual_anchor": true
    },
    {
      "id": "item_2",
      "kind": "asset",
      "name": "Nexora Logo Monogram (Transparent)",
      "tab_key": "full-landing-page",
      "tab_label": "Full Landing Page",
      "page_route": null,
      "variant_index": 1,
      "viewport": {
        "intent": "isolated",
        "css_width": 512
      },
      "aspect_ratio": "1:1",
      "transparent_background": true,
      "extracted_from_item_id": "item_1",
      "target_existing_output_id": null,
      "quality_override": null,
      "requirements_covered": [
        "req_9"
      ],
      "direction": "Abstract geometric monogram for Nexora: an upward-ascending mark built from crisp angular planes suggesting growth, in primary indigo #818CF8 with a single cyan #22D3EE detail facet, premium and minimal",
      "exact_copy": [],
      "reference_asset_ids": [],
      "depends_on": [
        "item_1"
      ],
      "image_prompt": "A single abstract geometric logo monogram for a premium SaaS brand named Nexora, isolated on a clean 100% transparent background with zero UI chrome, zero text, and zero background clutter. The mark is built from crisp angular geometric planes forming an abstract ascending arrow or upward staircase motif suggesting business growth, rendered in indigo #818CF8 with one sharp cyan #22D3EE accent facet. Flat vector-style emblem, minimal, confident, premium, perfectly centered with generous transparent padding around it.",
      "implementation_notes": [
        "POSITIONING (STICKY VS NON-STICKY): This is a standalone brand asset, not a positioned UI component; it is placed in the navbar top-left and footer with clear breathing room",
        "PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: This asset contains no placeholder content; it is the canonical brand mark to be used 1:1 wherever the logo appears",
        "CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Convert this flat geometric monogram into a scalable vectorial SVG graphic for razor-sharp quality at all sizes from 24px favicon to footer display; it is a flat mark and must not be rebuilt as a 3D object",
        "FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Optional subtle hover scale of 1.05 with spring physics stiffness 320 damping 28 when used as a clickable home link; respects reduced-motion preferences",
        "REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No existing logo reference was attached; this new monogram replaces any temporary placeholder mark in the codebase"
      ],
      "behaviors": [
        "HOVER STATE: When used as the navbar home link, the mark scales gently to 105% with a soft indigo glow",
        "CLICK / ACTIVE STATE: Pressed state compresses slightly toward 98% scale with no color change",
        "FOCUS & DISABLED STATES: Keyboard focus shows a 2px cyan #22D3EE outline ring with 2px offset around the mark"
      ],
      "responsive_rules": [
        "Renders at 28-32px in the navbar, 24px in the footer, and scales cleanly as a vector at any size on tablet and mobile"
      ],
      "asset_instructions": [
        "Convert this transparent monogram into a scalable vectorial SVG graphic; do not layer it as a raster cutout; use it in the navbar, footer, and favicon with clear breathing room"
      ],
      "is_visual_anchor": false
    },
    {
      "id": "item_3",
      "kind": "asset",
      "name": "Nexora Geometric Stroke Icon Sheet (Transparent)",
      "tab_key": "full-landing-page",
      "tab_label": "Full Landing Page",
      "page_route": null,
      "variant_index": 1,
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
        "req_9"
      ],
      "direction": "Organized icon sheet of 9 crisp geometric stroke icons on a consistent 24px grid: growth chart, bolt, shield, users, integration, target, rocket, clock, and check, in 2px rounded-join strokes, light stroke color for dark-theme use",
      "exact_copy": [],
      "reference_asset_ids": [],
      "depends_on": [
        "item_1"
      ],
      "image_prompt": "An organized icon sheet of nine minimal geometric stroke icons arranged in a clean evenly-spaced 3x3 grid, isolated on a clean 100% transparent background with zero UI chrome, zero labels, and zero background clutter. Icons: ascending growth chart, lightning bolt, shield, group of users, integration puzzle/nodes, target with arrow, rocket, clock, and a checkmark. Each icon drawn on a consistent 24px grid with crisp 2px strokes and rounded joins, uniform visual weight, stroke color a light neutral #F1F5F9 suitable for a dark theme, generous transparent spacing between icons.",
      "implementation_notes": [
        "POSITIONING (STICKY VS NON-STICKY): This is a standalone asset sheet, not a positioned UI component; individual icons are sliced and placed inside benefit tiles, pricing feature lists, and UI controls",
        "PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: This asset contains no placeholder content; each icon is a reusable UI graphic to be implemented 1:1 in code as SVG",
        "CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Slice each icon and convert it into a scalable vectorial SVG graphic on the 24px grid with 2px rounded-join strokes; these are flat stroke icons and must never be rendered as raster images or 3D objects",
        "FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Icons may draw-in their strokes with a short ease-out animation on scroll reveal of their parent tile; hover states on parent buttons may tint the icon stroke toward cyan #22D3EE; respects reduced-motion preferences",
        "REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No existing icon set reference was attached; these icons replace any generic placeholder icon usage in the codebase"
      ],
      "behaviors": [
        "HOVER STATE: Icons inherit the hover tint of their parent control, shifting stroke color from #F1F5F9 toward cyan #22D3EE",
        "CLICK / ACTIVE STATE: No independent click behavior; icons compress with their parent button press",
        "FOCUS & DISABLED STATES: Disabled icons render at 40% opacity; focus is handled by the parent interactive element''s cyan focus ring"
      ],
      "responsive_rules": [
        "Icons render at 20-24px on desktop, 20px on tablet, and 20px on mobile, scaling cleanly as vectors at all sizes"
      ],
      "asset_instructions": [
        "Slice each icon from the transparent sheet and convert it into a scalable vectorial SVG graphic; maintain the consistent 24px grid and 2px rounded-join stroke style across all usages"
      ],
      "is_visual_anchor": false
    },
    {
      "id": "item_1_auto_asset_1",
      "kind": "asset",
      "name": "Nexora Brand Logo Mark & Wordmark (Transparent)",
      "tab_key": "full-landing-page",
      "tab_label": "Full Landing Page",
      "page_route": null,
      "variant_index": 4,
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
      "direction": "Geometric ''N'' logo mark in blue-to-cyan gradient inside a rounded square, paired with white ''Nexora'' wordmark.",
      "exact_copy": [],
      "reference_asset_ids": [],
      "depends_on": [
        "item_1"
      ],
      "image_prompt": "Extract ONLY the Nexora brand logo from the attached reference UI image: the rounded-square ''N'' mark with a blue-to-cyan gradient and the adjacent white ''Nexora'' wordmark as seen in the top navigation bar and footer. Isolate it on a clean 100% transparent background, removing all surrounding nav links, buttons, and background surfaces. Reproduce it faithfully at high resolution.",
      "implementation_notes": [
        "Render on a 100% transparent PNG background and layer in the component/page layout"
      ],
      "behaviors": [],
      "responsive_rules": [
        "Scale proportionally across breakpoints"
      ],
      "asset_instructions": [
        "Place this logo in the top navigation bar (left side) and in the footer brand column. Convert it into a vectorial SVG graphic so it scales crisply; recreate the gradient fill of the N mark exactly and keep the wordmark as part of the SVG lockup."
      ],
      "is_visual_anchor": false
    },
    {
      "id": "item_1_auto_asset_2",
      "kind": "asset",
      "name": "Hero 3D Growth Bars & Arrow Sculpture (Transparent)",
      "tab_key": "full-landing-page",
      "tab_label": "Full Landing Page",
      "page_route": null,
      "variant_index": 5,
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
      "direction": "3D glossy bar chart of three ascending cubes (purple, blue, cyan) on a dark pedestal with a large curved cyan-to-blue growth arrow sweeping upward to the right.",
      "exact_copy": [],
      "reference_asset_ids": [],
      "depends_on": [
        "item_1"
      ],
      "image_prompt": "Extract ONLY the 3D centerpiece from the hero section of the attached reference UI image: the three glossy ascending 3D bar cubes (purple, mid-blue, cyan) sitting on a dark rounded pedestal, plus the large curved translucent growth arrow in cyan-to-blue gradient sweeping up and to the right. Isolate the entire sculpture on a clean 100% transparent background, removing the dark hero background, glow, and all text.",
      "implementation_notes": [
        "Render on a 100% transparent PNG background and layer in the component/page layout"
      ],
      "behaviors": [],
      "responsive_rules": [
        "Scale proportionally across breakpoints"
      ],
      "asset_instructions": [
        "Place this as the hero centerpiece on the right side of the hero section. Reconstruct it as an interactive 3D object from this reference image: three extruded glossy bars of increasing height on a pedestal with a curved arrow ribbon above. Animate it with a slow continuous rotation around its vertical axis, a gentle floating idle bob, and an interactive tilt toward the pointer on hover."
      ],
      "is_visual_anchor": false
    },
    {
      "id": "item_1_auto_asset_4",
      "kind": "asset",
      "name": "Dashboard Product Mockup Panel (Transparent)",
      "tab_key": "full-landing-page",
      "tab_label": "Full Landing Page",
      "page_route": null,
      "variant_index": 7,
      "viewport": {
        "intent": "isolated",
        "css_width": 1024
      },
      "aspect_ratio": "16:9",
      "transparent_background": true,
      "extracted_from_item_id": "item_1",
      "target_existing_output_id": null,
      "quality_override": null,
      "requirements_covered": [
        "req_1"
      ],
      "direction": "Dark SaaS dashboard UI mockup with sidebar nav, greeting header, MRR/conversion/experiments stat cards, revenue trend line chart, and experiment results bar chart.",
      "exact_copy": [],
      "reference_asset_ids": [],
      "depends_on": [
        "item_1"
      ],
      "image_prompt": "Extract ONLY the full dashboard product mockup panel from the ''Your Growth Engine, Live'' section of the attached reference UI image: the dark rounded dashboard with the Nexora sidebar, ''Good morning, Alex'' header, three stat cards, the Revenue Trend line chart, and the Experiment Results bar chart. Isolate the entire panel on a clean 100% transparent background, removing the section background and surrounding text.",
      "implementation_notes": [
        "Render on a 100% transparent PNG background and layer in the component/page layout"
      ],
      "behaviors": [],
      "responsive_rules": [
        "Scale proportionally across breakpoints"
      ],
      "asset_instructions": [
        "Place this mockup as the large product screenshot on the right side of the ''Your Growth Engine, Live'' section, slightly elevated with a soft shadow. Recreate it as a real coded UI panel rather than a flat image if feasible; if kept as an image asset, add a subtle scroll-triggered fade-and-rise entrance and a gentle parallax float."
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
        "/"
      ],
      "description": "Translucent dark sticky header with Nexora monogram, section anchor links, and solid indigo ''Start Free Trial'' CTA"
    },
    {
      "id": "shared_2",
      "name": "Site Footer",
      "canonical_item_id": "item_1",
      "used_on_routes": [
        "/"
      ],
      "description": "Hairline-top-border footer with logo, four link columns, and copyright row"
    }
  ],
  "page_composition": [
    {
      "page_name": "Full Landing Page",
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
        "Start Free Trial (primary, navbar + hero + final CTA)",
        "Request a Demo (ghost, hero)"
      ],
      "states": [
        "default"
      ]
    }
  ]
}', NULL, 'completed', 0, 0, 0, 0, NULL, '2026-10-24T19:16:09.163486+00:00', '2026-10-24T19:18:20.888460+00:00', '2026-10-24T19:18:20.888460+00:00', 'full-landing-page', 'Full Landing Page', NULL, '[]');
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-nexora-1', 'example-run-nexora', 'example-nexora', 'item_3', 3, 'asset', 'Nexora Geometric Stroke Icon Sheet (Transparent)', 'Organized icon sheet of 9 crisp geometric stroke icons on a consistent 24px grid: growth chart, bolt, shield, users, integration, target, rocket, clock, and check, in 2px rounded-join strokes, light stroke color for dark-theme use', 'asset', NULL, '1:1', 'completed', 'showcases/nexora/01.png', 'showcases/nexora/01.webp', 'image/png', 1024, 1024, 996516, 1, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (asset)
- Direction / Item Title: Nexora Geometric Stroke Icon Sheet (Transparent) (Variant #1)
- Viewport & Aspect Ratio: isolated (1024px target width), Aspect Ratio 1:1
- Scope Rules: Standalone visual site asset (icon sheet, illustration, 3D cutout, or foreground graphic) rendered on a 100% native transparent background so it can be layered directly into the coordinated page/component set.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Nexora — A premium SaaS landing page for Nexora that converts visitors into customers through a compelling hero, clear product benefits, interactive previews, social proof, pricing, and strong CTAs.
- Product / Application Description: Tagline: Turn Big Ideas Into Business Growth.

A premium SaaS landing page designed to convert visitors into customers. Features a compelling hero section, product benefits, interactive previews, testimonials, pricing plans, and strong calls to action.
- Industry & Target Audience: B2B SaaS / Business Growth Software | Audience: Founders, startup teams, and growth-focused business decision-makers evaluating tools to scale revenue (Quickly understand the value proposition, see the product in action, trust the brand via testimonials, and compare pricing without friction)
- Primary User Goal / Conversion Action: Sign up for a free trial or request a demo via the primary CTA

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #2 Modern / Contemporary
- Exact Style Definition: A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time.
- Mandatory Visual & CSS System Rules: Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "modern / contemporary". Primary style #2 Modern / Contemporary (Minimal & Editorial): A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, modern, and growth-oriented — polished and premium without feeling corporate or cold — Contemporary SaaS aesthetic: crisp typography, generous whitespace, hairline borders, soft ambient elevation, and a single confident accent color guiding every CTA. Interactive product previews sit in elevated surface cards with refined micro-interactions.
- UI Density & Tone: comfortable / Clear, aspirational, and benefit-driven — ''Turn Big Ideas Into Business Growth'' (English)
- Color Palette: Primary #4F46E5, Accent #06B6D4, Background #FAFAFC, Surface #FFFFFF, Text #0F172A, Muted #64748B, Border #E2E8F0
- Typography Intent: Heading Inter / Sora — SemiBold, tight tracking, 48px hero / 24px section scale, Body Inter — Regular, 15–16px, 1.6 line height, Mono JetBrains Mono — for code snippets, metrics, and pricing figures
- Radii, Borders & Shadows: Radius 12–16px cards, 10px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(15,23,42,0.06) light / 0 8px 24px rgba(0,0,0,0.4) dark
- Logo Instruction: Use a clean temporary monogram ''N'' integrated with the brand name ''Nexora'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E2E8F0), and subtle surface luminance shifts (#FAFAFC vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#0F172A on #FAFAFC/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Organized icon sheet of 9 crisp geometric stroke icons on a consistent 24px grid: growth chart, bolt, shield, users, integration, target, rocket, clock, and check, in 2px rounded-join strokes, light stroke color for dark-theme use

PLANNER IMAGE PROMPT:
An organized icon sheet of nine minimal geometric stroke icons arranged in a clean evenly-spaced 3x3 grid, isolated on a clean 100% transparent background with zero UI chrome, zero labels, and zero background clutter. Icons: ascending growth chart, lightning bolt, shield, group of users, integration puzzle/nodes, target with arrow, rocket, clock, and a checkmark. Each icon drawn on a consistent 24px grid with crisp 2px strokes and rounded joins, uniform visual weight, stroke color a light neutral #F1F5F9 suitable for a dark theme, generous transparent spacing between icons.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_9] Transparent PNG assets: Nexora geometric monogram logo and 8-10 icon stroke sheet on 24px grid

EXACT COPY & HIERARCHY:
Use concise, realistic copy matching brand language.

ATTACHED REFERENCE ROLES:
- Prerequisite Hero/Component Reference Image [FOREGROUND ASSET EXTRACTION]: Extract and faithfully reproduce ONLY the specified foreground illustration, 3D object, icon set, or graphic element from this reference UI image onto a 100% transparent background (`background: transparent`). Strip away all background walls, gradients, UI buttons, and text!

INVARIANT RENDERING CONSTRAINTS:
- Render a clean, production-ready standalone website asset matching the chosen Web Design Style above.
- Do NOT wrap the asset in browser chrome, UI buttons, or device frames.
- NATIVE TRANSPARENT BACKGROUND (`background: "transparent"`): Isolate the subject (or organized icon sheet grid) on a 100% true transparent alpha background. Do NOT draw a fake checkerboard grid, do NOT add opaque background fills or vignettes, and keep clean alpha edges with a 10% safety margin around the silhouette.
- FOREGROUND EXTRACTION FROM COMPOSITE UI: Reproduce the exact foreground visual asset shown in the attached prerequisite Hero/Component reference image at full resolution on a transparent background, completely omitting the background environment, headlines, and UI buttons so this transparent PNG can be layered over the coded background.
', '[]', '["item_1"]', '{"id":"item_3","kind":"asset","name":"Nexora Geometric Stroke Icon Sheet (Transparent)","tab_key":"full-landing-page","tab_label":"Full Landing Page","page_route":null,"variant_index":1,"viewport":{"intent":"isolated","css_width":1024},"aspect_ratio":"1:1","transparent_background":true,"extracted_from_item_id":"item_1","target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_9"],"direction":"Organized icon sheet of 9 crisp geometric stroke icons on a consistent 24px grid: growth chart, bolt, shield, users, integration, target, rocket, clock, and check, in 2px rounded-join strokes, light stroke color for dark-theme use","exact_copy":[],"reference_asset_ids":[],"depends_on":["item_1"],"image_prompt":"An organized icon sheet of nine minimal geometric stroke icons arranged in a clean evenly-spaced 3x3 grid, isolated on a clean 100% transparent background with zero UI chrome, zero labels, and zero background clutter. Icons: ascending growth chart, lightning bolt, shield, group of users, integration puzzle/nodes, target with arrow, rocket, clock, and a checkmark. Each icon drawn on a consistent 24px grid with crisp 2px strokes and rounded joins, uniform visual weight, stroke color a light neutral #F1F5F9 suitable for a dark theme, generous transparent spacing between icons.","implementation_notes":["POSITIONING (STICKY VS NON-STICKY): This is a standalone asset sheet, not a positioned UI component; individual icons are sliced and placed inside benefit tiles, pricing feature lists, and UI controls","PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: This asset contains no placeholder content; each icon is a reusable UI graphic to be implemented 1:1 in code as SVG","CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Slice each icon and convert it into a scalable vectorial SVG graphic on the 24px grid with 2px rounded-join strokes; these are flat stroke icons and must never be rendered as raster images or 3D objects","FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Icons may draw-in their strokes with a short ease-out animation on scroll reveal of their parent tile; hover states on parent buttons may tint the icon stroke toward cyan #22D3EE; respects reduced-motion preferences","REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No existing icon set reference was attached; these icons replace any generic placeholder icon usage in the codebase"],"behaviors":["HOVER STATE: Icons inherit the hover tint of their parent control, shifting stroke color from #F1F5F9 toward cyan #22D3EE","CLICK / ACTIVE STATE: No independent click behavior; icons compress with their parent button press","FOCUS & DISABLED STATES: Disabled icons render at 40% opacity; focus is handled by the parent interactive element''s cyan focus ring"],"responsive_rules":["Icons render at 20-24px on desktop, 20px on tablet, and 20px on mobile, scaling cleanly as vectors at all sizes"],"asset_instructions":["Slice each icon from the transparent sheet and convert it into a scalable vectorial SVG graphic; maintain the consistent 24px grid and 2px rounded-join stroke style across all usages"],"is_visual_anchor":false}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T19:16:42.012062+00:00', '2026-10-24T19:17:48.627900+00:00', 'full-landing-page', 'Full Landing Page', 'example-out-nexora-3', 1, 'example-out-nexora-3');
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-nexora-2', 'example-run-nexora', 'example-nexora', 'item_2', 2, 'asset', 'Nexora Logo Monogram (Transparent)', 'Abstract geometric monogram for Nexora: an upward-ascending mark built from crisp angular planes suggesting growth, in primary indigo #818CF8 with a single cyan #22D3EE detail facet, premium and minimal', 'asset', NULL, '1:1', 'completed', 'showcases/nexora/02.png', 'showcases/nexora/02.webp', 'image/png', 1024, 1024, 884993, 1, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (asset)
- Direction / Item Title: Nexora Logo Monogram (Transparent) (Variant #1)
- Viewport & Aspect Ratio: isolated (512px target width), Aspect Ratio 1:1
- Scope Rules: Standalone visual site asset (icon sheet, illustration, 3D cutout, or foreground graphic) rendered on a 100% native transparent background so it can be layered directly into the coordinated page/component set.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Nexora — A premium SaaS landing page for Nexora that converts visitors into customers through a compelling hero, clear product benefits, interactive previews, social proof, pricing, and strong CTAs.
- Product / Application Description: Tagline: Turn Big Ideas Into Business Growth.

A premium SaaS landing page designed to convert visitors into customers. Features a compelling hero section, product benefits, interactive previews, testimonials, pricing plans, and strong calls to action.
- Industry & Target Audience: B2B SaaS / Business Growth Software | Audience: Founders, startup teams, and growth-focused business decision-makers evaluating tools to scale revenue (Quickly understand the value proposition, see the product in action, trust the brand via testimonials, and compare pricing without friction)
- Primary User Goal / Conversion Action: Sign up for a free trial or request a demo via the primary CTA

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #2 Modern / Contemporary
- Exact Style Definition: A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time.
- Mandatory Visual & CSS System Rules: Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "modern / contemporary". Primary style #2 Modern / Contemporary (Minimal & Editorial): A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, modern, and growth-oriented — polished and premium without feeling corporate or cold — Contemporary SaaS aesthetic: crisp typography, generous whitespace, hairline borders, soft ambient elevation, and a single confident accent color guiding every CTA. Interactive product previews sit in elevated surface cards with refined micro-interactions.
- UI Density & Tone: comfortable / Clear, aspirational, and benefit-driven — ''Turn Big Ideas Into Business Growth'' (English)
- Color Palette: Primary #4F46E5, Accent #06B6D4, Background #FAFAFC, Surface #FFFFFF, Text #0F172A, Muted #64748B, Border #E2E8F0
- Typography Intent: Heading Inter / Sora — SemiBold, tight tracking, 48px hero / 24px section scale, Body Inter — Regular, 15–16px, 1.6 line height, Mono JetBrains Mono — for code snippets, metrics, and pricing figures
- Radii, Borders & Shadows: Radius 12–16px cards, 10px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(15,23,42,0.06) light / 0 8px 24px rgba(0,0,0,0.4) dark
- Logo Instruction: Use a clean temporary monogram ''N'' integrated with the brand name ''Nexora'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E2E8F0), and subtle surface luminance shifts (#FAFAFC vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#0F172A on #FAFAFC/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Abstract geometric monogram for Nexora: an upward-ascending mark built from crisp angular planes suggesting growth, in primary indigo #818CF8 with a single cyan #22D3EE detail facet, premium and minimal

PLANNER IMAGE PROMPT:
A single abstract geometric logo monogram for a premium SaaS brand named Nexora, isolated on a clean 100% transparent background with zero UI chrome, zero text, and zero background clutter. The mark is built from crisp angular geometric planes forming an abstract ascending arrow or upward staircase motif suggesting business growth, rendered in indigo #818CF8 with one sharp cyan #22D3EE accent facet. Flat vector-style emblem, minimal, confident, premium, perfectly centered with generous transparent padding around it.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_9] Transparent PNG assets: Nexora geometric monogram logo and 8-10 icon stroke sheet on 24px grid

EXACT COPY & HIERARCHY:
Use concise, realistic copy matching brand language.

ATTACHED REFERENCE ROLES:
- Prerequisite Hero/Component Reference Image [FOREGROUND ASSET EXTRACTION]: Extract and faithfully reproduce ONLY the specified foreground illustration, 3D object, icon set, or graphic element from this reference UI image onto a 100% transparent background (`background: transparent`). Strip away all background walls, gradients, UI buttons, and text!

INVARIANT RENDERING CONSTRAINTS:
- Render a clean, production-ready standalone website asset matching the chosen Web Design Style above.
- Do NOT wrap the asset in browser chrome, UI buttons, or device frames.
- NATIVE TRANSPARENT BACKGROUND (`background: "transparent"`): Isolate the subject (or organized icon sheet grid) on a 100% true transparent alpha background. Do NOT draw a fake checkerboard grid, do NOT add opaque background fills or vignettes, and keep clean alpha edges with a 10% safety margin around the silhouette.
- FOREGROUND EXTRACTION FROM COMPOSITE UI: Reproduce the exact foreground visual asset shown in the attached prerequisite Hero/Component reference image at full resolution on a transparent background, completely omitting the background environment, headlines, and UI buttons so this transparent PNG can be layered over the coded background.
', '[]', '["item_1"]', '{"id":"item_2","kind":"asset","name":"Nexora Logo Monogram (Transparent)","tab_key":"full-landing-page","tab_label":"Full Landing Page","page_route":null,"variant_index":1,"viewport":{"intent":"isolated","css_width":512},"aspect_ratio":"1:1","transparent_background":true,"extracted_from_item_id":"item_1","target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_9"],"direction":"Abstract geometric monogram for Nexora: an upward-ascending mark built from crisp angular planes suggesting growth, in primary indigo #818CF8 with a single cyan #22D3EE detail facet, premium and minimal","exact_copy":[],"reference_asset_ids":[],"depends_on":["item_1"],"image_prompt":"A single abstract geometric logo monogram for a premium SaaS brand named Nexora, isolated on a clean 100% transparent background with zero UI chrome, zero text, and zero background clutter. The mark is built from crisp angular geometric planes forming an abstract ascending arrow or upward staircase motif suggesting business growth, rendered in indigo #818CF8 with one sharp cyan #22D3EE accent facet. Flat vector-style emblem, minimal, confident, premium, perfectly centered with generous transparent padding around it.","implementation_notes":["POSITIONING (STICKY VS NON-STICKY): This is a standalone brand asset, not a positioned UI component; it is placed in the navbar top-left and footer with clear breathing room","PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: This asset contains no placeholder content; it is the canonical brand mark to be used 1:1 wherever the logo appears","CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Convert this flat geometric monogram into a scalable vectorial SVG graphic for razor-sharp quality at all sizes from 24px favicon to footer display; it is a flat mark and must not be rebuilt as a 3D object","FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Optional subtle hover scale of 1.05 with spring physics stiffness 320 damping 28 when used as a clickable home link; respects reduced-motion preferences","REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No existing logo reference was attached; this new monogram replaces any temporary placeholder mark in the codebase"],"behaviors":["HOVER STATE: When used as the navbar home link, the mark scales gently to 105% with a soft indigo glow","CLICK / ACTIVE STATE: Pressed state compresses slightly toward 98% scale with no color change","FOCUS & DISABLED STATES: Keyboard focus shows a 2px cyan #22D3EE outline ring with 2px offset around the mark"],"responsive_rules":["Renders at 28-32px in the navbar, 24px in the footer, and scales cleanly as a vector at any size on tablet and mobile"],"asset_instructions":["Convert this transparent monogram into a scalable vectorial SVG graphic; do not layer it as a raster cutout; use it in the navbar, footer, and favicon with clear breathing room"],"is_visual_anchor":false}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T19:16:42.012062+00:00', '2026-10-24T19:17:35.596379+00:00', 'full-landing-page', 'Full Landing Page', 'example-out-nexora-3', 1, 'example-out-nexora-3');
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-nexora-3', 'example-run-nexora', 'example-nexora', 'item_1', 1, 'page', 'Nexora Premium SaaS Landing Page (Full Desktop Scroll, Dark Theme)', 'Style #2 Modern/Contemporary dark-theme premium SaaS landing page: crisp hierarchy, polished Sora/Inter typography, generous whitespace, 1px hairline borders (#26324A), soft ambient elevation, single confident indigo primary guiding every CTA with sparing cyan accent. Asymmetric editorial compositions, no card-in-card nesting, no gradient blobs.', 'visual_anchor', '/', '2:3', 'completed', 'showcases/nexora/03.png', 'showcases/nexora/03.webp', 'image/png', 1024, 1536, 1664488, 0, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (page)
- Direction / Item Title: Nexora Premium SaaS Landing Page (Full Desktop Scroll, Dark Theme) (Variant #1)
- Viewport & Aspect Ratio: desktop (1440px target width), Aspect Ratio 2:3
- Scope Rules: VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NOT A HORIZONTAL POWERPOINT SLIDE): Complete coordinated web page flowing vertically from top to bottom (Navbar at top -> Hero -> Features -> Pricing -> Footer at bottom) with internal column grids and dimensions strictly matching the target viewport breakpoint.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Nexora — A premium SaaS landing page for Nexora that converts visitors into customers through a compelling hero, clear product benefits, interactive previews, social proof, pricing, and strong CTAs.
- Product / Application Description: Tagline: Turn Big Ideas Into Business Growth.

A premium SaaS landing page designed to convert visitors into customers. Features a compelling hero section, product benefits, interactive previews, testimonials, pricing plans, and strong calls to action.
- Industry & Target Audience: B2B SaaS / Business Growth Software | Audience: Founders, startup teams, and growth-focused business decision-makers evaluating tools to scale revenue (Quickly understand the value proposition, see the product in action, trust the brand via testimonials, and compare pricing without friction)
- Primary User Goal / Conversion Action: Sign up for a free trial or request a demo via the primary CTA

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #2 Modern / Contemporary
- Exact Style Definition: A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time.
- Mandatory Visual & CSS System Rules: Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "modern / contemporary". Primary style #2 Modern / Contemporary (Minimal & Editorial): A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, modern, and growth-oriented — polished and premium without feeling corporate or cold — Contemporary SaaS aesthetic: crisp typography, generous whitespace, hairline borders, soft ambient elevation, and a single confident accent color guiding every CTA. Interactive product previews sit in elevated surface cards with refined micro-interactions.
- UI Density & Tone: comfortable / Clear, aspirational, and benefit-driven — ''Turn Big Ideas Into Business Growth'' (English)
- Color Palette: Primary #4F46E5, Accent #06B6D4, Background #FAFAFC, Surface #FFFFFF, Text #0F172A, Muted #64748B, Border #E2E8F0
- Typography Intent: Heading Inter / Sora — SemiBold, tight tracking, 48px hero / 24px section scale, Body Inter — Regular, 15–16px, 1.6 line height, Mono JetBrains Mono — for code snippets, metrics, and pricing figures
- Radii, Borders & Shadows: Radius 12–16px cards, 10px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(15,23,42,0.06) light / 0 8px 24px rgba(0,0,0,0.4) dark
- Logo Instruction: Use a clean temporary monogram ''N'' integrated with the brand name ''Nexora'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E2E8F0), and subtle surface luminance shifts (#FAFAFC vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#0F172A on #FAFAFC/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Style #2 Modern/Contemporary dark-theme premium SaaS landing page: crisp hierarchy, polished Sora/Inter typography, generous whitespace, 1px hairline borders (#26324A), soft ambient elevation, single confident indigo primary guiding every CTA with sparing cyan accent. Asymmetric editorial compositions, no card-in-card nesting, no gradient blobs.

PLANNER IMAGE PROMPT:
A tall vertical 2:3 top-to-bottom full-page scroll capture of a 1440px-wide premium dark-theme SaaS landing page for ''Nexora'' — Style #2 Modern/Contemporary, minimal and editorial. Strict dark palette: background #0B1120, surface cards #151D2E, hairline 1px borders #26324A, text #F1F5F9, muted #94A3B8, primary indigo #818CF8, accent cyan #22D3EE. Typography: Sora SemiBold tight-tracked headings (48px hero, 24px sections), Inter Regular 15-16px body, JetBrains Mono for metrics and pricing. Generous whitespace, 12-column grid, 12-16px card radii, soft ambient elevation. TOP: sticky translucent dark navbar with a small geometric monogram logo left, horizontal links ''Product, Benefits, Pricing, Testimonials'', and one solid indigo ''Start Free Trial'' button with subtle arrow affordance right. SECTION 1 HERO — asymmetric editorial split: left column oversized display headline ''Turn Big Ideas Into Business Growth'', supporting subcopy, primary indigo CTA ''Start Free Trial'' and ghost secondary ''Request a Demo'', small trust line ''Trusted by 2,400+ growth teams''; right column a refined sculptural 3D growth emblem — an abstract ascending arrow and bar sculpture in indigo-to-cyan material with soft ambient glow floating on the dark surface (a crafted physical object, NOT a glowing gradient blob). SECTION 2 BENEFITS — asymmetric bento grid of mixed-size surface tiles with hairline borders, each with a crisp 2px stroke icon and short benefit copy; one larger tile features a large JetBrains Mono metric callout ''+38%''. No identical 3-column icon-in-circle rows. SECTION 3 PRODUCT PREVIEW — one large elevated surface card containing a believable analytics dashboard mockup: left sidebar, KPI figures in mono font (MRR $284,930, Conversion 4.8%, Active Experiments 27), clean line and bar charts using indigo and cyan data colors. SECTION 4 TESTIMONIALS — varied-size quote cards with small circular avatars, founder names and roles, subtle company wordmarks. SECTION 5 PRICING — three tier cards (Starter $29/mo, Growth $79/mo, Scale $199/mo) with JetBrains Mono figures and stroke check icons; the middle ''Growth'' card is highlighted with a cyan ''Recommended'' pill and elevated border treatment. SECTION 6 FINAL CTA — full-width confident band with large headline ''Ready to Turn Your Big Idea Into Growth?'' and one solid indigo CTA. FOOTER — logo, four link columns, hairline top border, copyright row. Crisp, premium, Linear/Stripe-caliber craft; no purple-blue SaaS gradient soup, no floating neon blobs, no card-in-card clutter, no low-contrast gray text, no stock-photo clichés.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_1] Single desktop full-page vertical scroll landing page at 1440px width in 2:3 canvas, dark theme Brand Kit colors
- [req_2] Sticky top navbar with logo, links (Product, Benefits, Pricing, Testimonials), and ''Start Free Trial'' primary CTA
- [req_3] Hero asymmetric editorial split with oversized headline, dual CTAs, and refined 3D growth emblem (indigo-cyan, not cliché gradient blob)
- [req_4] Asymmetric bento benefits grid with stroke icons and one JetBrains Mono metric callout tile
- [req_5] Interactive product preview card with believable analytics dashboard mockup (charts, mono KPIs, sidebar)
- [req_6] Testimonials row with realistic founder names, roles, company wordmarks, varied card sizes
- [req_7] Three pricing tiers (Starter, Growth, Scale) with mono figures and highlighted recommended tier in accent color
- [req_8] Final full-width CTA band and structured footer with hairline top border
- [req_10] Anti-AI-slop mandate: no card-in-card nesting, no purple-blue gradient blobs, no filler copy, one unmistakable focal point

EXACT COPY & HIERARCHY:
Nexora | Product | Benefits | Pricing | Testimonials | Start Free Trial | Turn Big Ideas Into Business Growth | Nexora turns scattered growth experiments into a single, measurable engine — plan campaigns, track revenue impact, and scale what works, all from one workspace. | Request a Demo | Trusted by 2,400+ growth teams | Everything You Need to Grow, Nothing You Don''t | Revenue Attribution | See exactly which experiments move revenue, down to the cohort. | Rapid Experimentation | Launch, measure, and iterate on growth tests in hours, not weeks. | Team Alignment | Shared roadmaps keep product, marketing, and sales on the same page. | Enterprise-Grade Security | SOC 2 Type II, SSO, and granular permissions out of the box. | +38% | average revenue lift within the first quarter for active teams | Your Growth Engine, Live | MRR $284,930 | Conversion 4.8% | Active Experiments 27 | See Nexora in action | Founders Who Ship Faster With Nexora | We replaced four tools with Nexora and finally know what actually drives our pipeline. It paid for itself in the first month. | Maya Chen | Co-founder & CEO, Loopwave | The experimentation cadence Nexora gave us turned guesswork into a repeatable growth system. | Daniel Okafor | VP Growth, Fernbook | Simple Pricing That Scales With You | Starter | $29/mo | For small teams validating their first growth loops | 3 seats included | Core analytics dashboard | 10 active experiments | Email support | Growth | $79/mo | Recommended | For teams scaling revenue with structured experimentation | Unlimited seats | Revenue attribution engine | Unlimited experiments | Priority support & onboarding | Scale | $199/mo | For organizations running growth as a company-wide discipline | SSO & advanced permissions | Custom data residency | Dedicated success manager | Ready to Turn Your Big Idea Into Growth? | Join 2,400+ teams already growing with Nexora. Free for 14 days, no credit card required. | Product | Features | Integrations | Changelog | Company | About | Careers | Blog | Resources | Docs | Help Center | Security | © 2025 Nexora, Inc. All rights reserved. | Privacy | Terms

ATTACHED REFERENCE ROLES:
None attached.

INVARIANT RENDERING CONSTRAINTS:
- Direct, full-bleed viewport UI design reference (do NOT place the UI inside an angled laptop/phone hardware mockup or external desk scene, and do NOT include a browser URL bar).
- Strictly execute the Chosen Web Design Style definition and visual rules above in every surface, border, shadow, color contrast, and typographic detail.
- Render crisp, legible typography, accurate button alignment, accessible contrast, and realistic spacing.
- VERTICAL TOP-TO-BOTTOM FULL-PAGE SCROLL CAPTURE (NEVER A HORIZONTAL POWERPOINT SLIDE): Render the entire page flowing vertically from top to bottom — starting with the top navigation bar at the very top edge, followed vertically by the Hero section, Feature sections, Pricing/Content sections, and ending with the Footer at the bottom edge.
- INTERNAL BREAKPOINT DIMENSIONS = DESKTOP (`1440px` WIDE DESKTOP GRID INSIDE VERTICAL FULL-PAGE SCROLL): Even though the image canvas is vertical (`2:3` / `9:16`) so the whole landing page is visible from top to bottom, the UI INSIDE the image MUST be a true `1440px`-wide Desktop layout — full horizontal desktop navbar (brand logo left, horizontal nav links center, CTA buttons right — NO mobile hamburger menu), 12-column desktop container with generous left/right desktop margins, 2-column desktop Hero, and 3-to-4 column side-by-side desktop Feature & Pricing cards in horizontal rows! Do NOT render a narrow single-column mobile phone layout!
- VISUAL ANCHOR PAGE (`item_1`): Establish the canonical sticky top navigation bar (brand logo left, clear route links for all pages of the webapp center, primary CTA right), brand surface palette, typography, border radius, shadow depth, and footer chrome that all subsequent pages in this multi-page set will inherit via Image-to-Image conditioning.
', '[]', '[]', '{"id":"item_1","kind":"page","name":"Nexora Premium SaaS Landing Page (Full Desktop Scroll, Dark Theme)","tab_key":"full-landing-page","tab_label":"Full Landing Page","page_route":"/","variant_index":1,"viewport":{"intent":"desktop","css_width":1440},"aspect_ratio":"2:3","transparent_background":false,"extracted_from_item_id":null,"target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_1","req_2","req_3","req_4","req_5","req_6","req_7","req_8","req_10"],"direction":"Style #2 Modern/Contemporary dark-theme premium SaaS landing page: crisp hierarchy, polished Sora/Inter typography, generous whitespace, 1px hairline borders (#26324A), soft ambient elevation, single confident indigo primary guiding every CTA with sparing cyan accent. Asymmetric editorial compositions, no card-in-card nesting, no gradient blobs.","exact_copy":["Nexora","Product","Benefits","Pricing","Testimonials","Start Free Trial","Turn Big Ideas Into Business Growth","Nexora turns scattered growth experiments into a single, measurable engine — plan campaigns, track revenue impact, and scale what works, all from one workspace.","Request a Demo","Trusted by 2,400+ growth teams","Everything You Need to Grow, Nothing You Don''t","Revenue Attribution","See exactly which experiments move revenue, down to the cohort.","Rapid Experimentation","Launch, measure, and iterate on growth tests in hours, not weeks.","Team Alignment","Shared roadmaps keep product, marketing, and sales on the same page.","Enterprise-Grade Security","SOC 2 Type II, SSO, and granular permissions out of the box.","+38%","average revenue lift within the first quarter for active teams","Your Growth Engine, Live","MRR $284,930","Conversion 4.8%","Active Experiments 27","See Nexora in action","Founders Who Ship Faster With Nexora","We replaced four tools with Nexora and finally know what actually drives our pipeline. It paid for itself in the first month.","Maya Chen","Co-founder & CEO, Loopwave","The experimentation cadence Nexora gave us turned guesswork into a repeatable growth system.","Daniel Okafor","VP Growth, Fernbook","Simple Pricing That Scales With You","Starter","$29/mo","For small teams validating their first growth loops","3 seats included","Core analytics dashboard","10 active experiments","Email support","Growth","$79/mo","Recommended","For teams scaling revenue with structured experimentation","Unlimited seats","Revenue attribution engine","Unlimited experiments","Priority support & onboarding","Scale","$199/mo","For organizations running growth as a company-wide discipline","SSO & advanced permissions","Custom data residency","Dedicated success manager","Ready to Turn Your Big Idea Into Growth?","Join 2,400+ teams already growing with Nexora. Free for 14 days, no credit card required.","Product","Features","Integrations","Changelog","Company","About","Careers","Blog","Resources","Docs","Help Center","Security","© 2025 Nexora, Inc. All rights reserved.","Privacy","Terms"],"reference_asset_ids":[],"depends_on":[],"image_prompt":"A tall vertical 2:3 top-to-bottom full-page scroll capture of a 1440px-wide premium dark-theme SaaS landing page for ''Nexora'' — Style #2 Modern/Contemporary, minimal and editorial. Strict dark palette: background #0B1120, surface cards #151D2E, hairline 1px borders #26324A, text #F1F5F9, muted #94A3B8, primary indigo #818CF8, accent cyan #22D3EE. Typography: Sora SemiBold tight-tracked headings (48px hero, 24px sections), Inter Regular 15-16px body, JetBrains Mono for metrics and pricing. Generous whitespace, 12-column grid, 12-16px card radii, soft ambient elevation. TOP: sticky translucent dark navbar with a small geometric monogram logo left, horizontal links ''Product, Benefits, Pricing, Testimonials'', and one solid indigo ''Start Free Trial'' button with subtle arrow affordance right. SECTION 1 HERO — asymmetric editorial split: left column oversized display headline ''Turn Big Ideas Into Business Growth'', supporting subcopy, primary indigo CTA ''Start Free Trial'' and ghost secondary ''Request a Demo'', small trust line ''Trusted by 2,400+ growth teams''; right column a refined sculptural 3D growth emblem — an abstract ascending arrow and bar sculpture in indigo-to-cyan material with soft ambient glow floating on the dark surface (a crafted physical object, NOT a glowing gradient blob). SECTION 2 BENEFITS — asymmetric bento grid of mixed-size surface tiles with hairline borders, each with a crisp 2px stroke icon and short benefit copy; one larger tile features a large JetBrains Mono metric callout ''+38%''. No identical 3-column icon-in-circle rows. SECTION 3 PRODUCT PREVIEW — one large elevated surface card containing a believable analytics dashboard mockup: left sidebar, KPI figures in mono font (MRR $284,930, Conversion 4.8%, Active Experiments 27), clean line and bar charts using indigo and cyan data colors. SECTION 4 TESTIMONIALS — varied-size quote cards with small circular avatars, founder names and roles, subtle company wordmarks. SECTION 5 PRICING — three tier cards (Starter $29/mo, Growth $79/mo, Scale $199/mo) with JetBrains Mono figures and stroke check icons; the middle ''Growth'' card is highlighted with a cyan ''Recommended'' pill and elevated border treatment. SECTION 6 FINAL CTA — full-width confident band with large headline ''Ready to Turn Your Big Idea Into Growth?'' and one solid indigo CTA. FOOTER — logo, four link columns, hairline top border, copyright row. Crisp, premium, Linear/Stripe-caliber craft; no purple-blue SaaS gradient soup, no floating neon blobs, no card-in-card clutter, no low-contrast gray text, no stock-photo clichés.","implementation_notes":["POSITIONING (STICKY VS NON-STICKY): The top navigation bar stays anchored at the top of the viewport for the entire scroll, rendered as a translucent dark surface over the page background with a 1px hairline bottom border; the hero, benefits, product preview, testimonials, pricing, final CTA band, and footer all scroll naturally in normal document flow","ANTI-AI-SLOP LAYOUT & VISUAL HIERARCHY (pbakaus/impeccable & DESIGN.md RULES): The hero headline plus the 3D growth emblem is the single unmistakable primary visual anchor and the ''Start Free Trial'' button is the clear secondary action; avoid wrapping every section in boxed cards containing nested sub-cards — use 1px hairline borders in #26324A, intentional negative space, and asymmetric bento and editorial split compositions; apply semantic tokens exactly: background #0B1120, surfaces #151D2E, primary indigo #818CF8 reserved for actions, links, and key highlights, cyan #22D3EE used sparingly for the Recommended badge and data highlights, headings in Sora SemiBold with tight tracking, body in Inter at 1.6 line height left-aligned, and all metrics and pricing figures in JetBrains Mono with tabular numerals","PLACEHOLDER CONTENT & 1:1 CODED UI COMPONENTS: The testimonial avatars, founder names, roles, and company wordmarks are placeholder sample content for a future database; the analytics dashboard mockup inside the product preview card (sidebar, KPI figures, charts) is a static believable illustration acting as a product preview placeholder; every card, tile, quote card, pricing tier, dashboard mockup frame, button, navbar, and footer layout must be implemented 1:1 in code right now with the sample data structured as clean placeholder data","CODE VS. 2D VECTOR/ASSET VS. INTERACTIVE 3D OBJECT SPLIT: Build purely with code the section separators, hairline rule lines, ambient radial glows behind the hero emblem, subtle dot or grid backdrops, all surface cards, buttons, navbar, pricing tiers, and footer; convert the Nexora logo monogram and all icons from the extracted transparent icon sheet into scalable vectorial SVG graphics for razor-sharp quality; reconstruct the hero 3D growth emblem from its extracted transparent reference image as an interactive 3D object (Image-to-3D mesh or procedural 3D in Three.js / React Three Fiber) with continuous smooth rotation on its vertical axis, a gentle floating bobbing idle motion, and interactive cursor-tracking tilt with a hover spin and soft glow","FRAMER MOTION, ANIMATIONS & INTERACTIVE EFFECTS: Use crisp physical spring physics with stiffness 320 and damping 28 (never bounce or elastic easing) for button and card hover and press feedback; staggered scroll reveals for bento tiles, testimonial cards, and pricing tiers using an exponential ease-out curve like cubic-bezier(0.22, 1, 0.36, 1); a shared animated underline indicator slides between active navbar links; the pricing recommended card lifts subtly on hover; the 3D emblem rotates continuously and tilts toward the cursor; all motion respects reduced-motion accessibility preferences by falling back to instant opacity transitions","REPLACE OLD HERO / HEADER / COMPONENT (WHEN REFERENCE PROVIDED): No existing screenshot or previous output was attached, so this is a brand-new build from scratch with no component replacement required","CODED SEPARATOR / BACKGROUND SPEC: Section separators: This page uses no hard rule lines. Between the hero and the features section there is a soft horizontal fade where the hero''s darker navy canvas blends into the slightly lighter dark-navy features background — recreate this as a smooth vertical gradient transition, not a line. Each subsequent section (dashboard, testimonials, pricing, CTA banner, footer) is separated only by generous vertical whitespace and subtle changes in surface elevation; the footer is divided from the page above by a faint darker band. Recreate all of these 1:1 in code as gradient/spacing transitions identical to the image.","CODED SEPARATOR / BACKGROUND SPEC: Background layers: The entire page sits on a very dark navy (near-black blue) canvas. The hero has a large soft radial blue glow behind the 3D sculpture on the right and a subtle darker vignette on the left. The features section cards sit on slightly lighter navy surfaces with thin translucent borders. The final CTA banner is a rounded rectangle with a blue-to-indigo horizontal gradient glow. Add a very subtle grain/noise texture across the dark canvas. Recreate every glow, gradient, and texture layer in code (CSS gradients and effects) so the background matches the image identically — do not use a flat background screenshot.","CODED SEPARATOR / BACKGROUND SPEC: 3D objects and animation: The hero''s 3D growth-bar-and-arrow sculpture must be built as an interactive 3D object from its extracted reference: three glossy extruded bars (purple, blue, cyan) ascending left to right on a dark pedestal, with a curved translucent cyan-to-blue arrow ribbon arcing above them. Give it a slow continuous idle rotation around the vertical axis, a gentle floating bob, and a pointer-following tilt on hover, with soft ambient blue lighting and a glow beneath it matching the image.","CODED SEPARATOR / BACKGROUND SPEC: Card and glow treatments: All cards (feature tiles, testimonial cards, pricing cards, stat cards) use a slightly lighter navy fill, a one-pixel translucent light-blue border, and large rounded corners; the recommended Growth pricing card has a bright cyan border with a cyan ''Recommended'' pill overlapping its top edge. Recreate these card surfaces, borders, and the cyan highlight treatment exactly in code to match the image."],"behaviors":["HOVER STATE: Primary CTA hover shifts background from #818CF8 to a slightly lighter indigo with a soft ambient glow and a subtle upward lift of 2px; ghost CTA hover fills with surface #151D2E and a #26324A border; nav links hover to full text color #F1F5F9 from muted #94A3B8; surface cards hover elevate shadow to 0 12px 32px rgba(0,0,0,0.5) with border brightening toward #3B4A6B","CLICK / ACTIVE STATE: Primary CTA pressed state compresses scale slightly toward 0.98 with background deepened toward #6366F1 and no shadow; nav links show an active indigo underline indicator on the current section; pricing tier selection highlights the chosen card border in #22D3EE","FOCUS & DISABLED STATES: Keyboard focus rings use a 2px #22D3EE outline with 2px offset on all interactive elements; disabled buttons render at 40% opacity with no hover or press response"],"responsive_rules":["Desktop (1440px): full 12-column layout exactly as depicted with asymmetric hero split and multi-column bento, testimonial, and pricing grids","Tablet (834px): hero stacks vertically with the emblem below the headline, bento collapses to a 2-column grid, testimonials to 2 columns, pricing to a horizontal scroll or stacked cards, and the navbar condenses with links behind a hamburger menu","Mobile (390px): single-column stacked layout, hero headline scales down to a 32-36px fluid size, CTAs become full-width, bento tiles stack, pricing tiers stack vertically with the recommended tier first, and the footer link columns collapse into accordions"],"asset_instructions":["Use the extracted transparent logo monogram as the navbar and footer brand mark, converted into a scalable vectorial SVG graphic for razor-sharp rendering at any size","Use the extracted transparent icon sheet by slicing each icon and converting them into vectorial SVG graphics on a consistent 24px grid with 2px rounded-join strokes","Use the extracted transparent 3D growth emblem reference to reconstruct an interactive rotating 3D object (Image-to-3D or procedural 3D) rather than layering it as a flat static image","CODED SEPARATOR / BACKGROUND SPEC: Section separators: This page uses no hard rule lines. Between the hero and the features section there is a soft horizontal fade where the hero''s darker navy canvas blends into the slightly lighter dark-navy features background — recreate this as a smooth vertical gradient transition, not a line. Each subsequent section (dashboard, testimonials, pricing, CTA banner, footer) is separated only by generous vertical whitespace and subtle changes in surface elevation; the footer is divided from the page above by a faint darker band. Recreate all of these 1:1 in code as gradient/spacing transitions identical to the image.","CODED SEPARATOR / BACKGROUND SPEC: Background layers: The entire page sits on a very dark navy (near-black blue) canvas. The hero has a large soft radial blue glow behind the 3D sculpture on the right and a subtle darker vignette on the left. The features section cards sit on slightly lighter navy surfaces with thin translucent borders. The final CTA banner is a rounded rectangle with a blue-to-indigo horizontal gradient glow. Add a very subtle grain/noise texture across the dark canvas. Recreate every glow, gradient, and texture layer in code (CSS gradients and effects) so the background matches the image identically — do not use a flat background screenshot.","CODED SEPARATOR / BACKGROUND SPEC: 3D objects and animation: The hero''s 3D growth-bar-and-arrow sculpture must be built as an interactive 3D object from its extracted reference: three glossy extruded bars (purple, blue, cyan) ascending left to right on a dark pedestal, with a curved translucent cyan-to-blue arrow ribbon arcing above them. Give it a slow continuous idle rotation around the vertical axis, a gentle floating bob, and a pointer-following tilt on hover, with soft ambient blue lighting and a glow beneath it matching the image.","CODED SEPARATOR / BACKGROUND SPEC: Card and glow treatments: All cards (feature tiles, testimonial cards, pricing cards, stat cards) use a slightly lighter navy fill, a one-pixel translucent light-blue border, and large rounded corners; the recommended Growth pricing card has a bright cyan border with a cyan ''Recommended'' pill overlapping its top edge. Recreate these card surfaces, borders, and the cyan highlight treatment exactly in code to match the image."],"is_visual_anchor":true}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T19:16:42.012062+00:00', '2026-10-24T19:17:27.089599+00:00', 'full-landing-page', 'Full Landing Page', NULL, 0, NULL);
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-nexora-4', 'example-run-nexora', 'example-nexora', 'item_1_auto_asset_2', 5, 'asset', 'Hero 3D Growth Bars & Arrow Sculpture (Transparent)', '3D glossy bar chart of three ascending cubes (purple, blue, cyan) on a dark pedestal with a large curved cyan-to-blue growth arrow sweeping upward to the right.', 'asset', NULL, '1:1', 'completed', 'showcases/nexora/04.png', 'showcases/nexora/04.webp', 'image/png', 1024, 1024, 976576, 1, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (asset)
- Direction / Item Title: Hero 3D Growth Bars & Arrow Sculpture (Transparent) (Variant #5)
- Viewport & Aspect Ratio: isolated (1024px target width), Aspect Ratio 1:1
- Scope Rules: Standalone visual site asset (icon sheet, illustration, 3D cutout, or foreground graphic) rendered on a 100% native transparent background so it can be layered directly into the coordinated page/component set.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Nexora — A premium SaaS landing page for Nexora that converts visitors into customers through a compelling hero, clear product benefits, interactive previews, social proof, pricing, and strong CTAs.
- Product / Application Description: Tagline: Turn Big Ideas Into Business Growth.

A premium SaaS landing page designed to convert visitors into customers. Features a compelling hero section, product benefits, interactive previews, testimonials, pricing plans, and strong calls to action.
- Industry & Target Audience: B2B SaaS / Business Growth Software | Audience: Founders, startup teams, and growth-focused business decision-makers evaluating tools to scale revenue (Quickly understand the value proposition, see the product in action, trust the brand via testimonials, and compare pricing without friction)
- Primary User Goal / Conversion Action: Sign up for a free trial or request a demo via the primary CTA

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #2 Modern / Contemporary
- Exact Style Definition: A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time.
- Mandatory Visual & CSS System Rules: Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "modern / contemporary". Primary style #2 Modern / Contemporary (Minimal & Editorial): A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, modern, and growth-oriented — polished and premium without feeling corporate or cold — Contemporary SaaS aesthetic: crisp typography, generous whitespace, hairline borders, soft ambient elevation, and a single confident accent color guiding every CTA. Interactive product previews sit in elevated surface cards with refined micro-interactions.
- UI Density & Tone: comfortable / Clear, aspirational, and benefit-driven — ''Turn Big Ideas Into Business Growth'' (English)
- Color Palette: Primary #4F46E5, Accent #06B6D4, Background #FAFAFC, Surface #FFFFFF, Text #0F172A, Muted #64748B, Border #E2E8F0
- Typography Intent: Heading Inter / Sora — SemiBold, tight tracking, 48px hero / 24px section scale, Body Inter — Regular, 15–16px, 1.6 line height, Mono JetBrains Mono — for code snippets, metrics, and pricing figures
- Radii, Borders & Shadows: Radius 12–16px cards, 10px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(15,23,42,0.06) light / 0 8px 24px rgba(0,0,0,0.4) dark
- Logo Instruction: Use a clean temporary monogram ''N'' integrated with the brand name ''Nexora'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E2E8F0), and subtle surface luminance shifts (#FAFAFC vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#0F172A on #FAFAFC/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
3D glossy bar chart of three ascending cubes (purple, blue, cyan) on a dark pedestal with a large curved cyan-to-blue growth arrow sweeping upward to the right.

PLANNER IMAGE PROMPT:
Extract ONLY the 3D centerpiece from the hero section of the attached reference UI image: the three glossy ascending 3D bar cubes (purple, mid-blue, cyan) sitting on a dark rounded pedestal, plus the large curved translucent growth arrow in cyan-to-blue gradient sweeping up and to the right. Isolate the entire sculpture on a clean 100% transparent background, removing the dark hero background, glow, and all text.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_1] Single desktop full-page vertical scroll landing page at 1440px width in 2:3 canvas, dark theme Brand Kit colors

EXACT COPY & HIERARCHY:
Use concise, realistic copy matching brand language.

ATTACHED REFERENCE ROLES:
- Prerequisite Hero/Component Reference Image [FOREGROUND ASSET EXTRACTION]: Extract and faithfully reproduce ONLY the specified foreground illustration, 3D object, icon set, or graphic element from this reference UI image onto a 100% transparent background (`background: transparent`). Strip away all background walls, gradients, UI buttons, and text!

INVARIANT RENDERING CONSTRAINTS:
- Render a clean, production-ready standalone website asset matching the chosen Web Design Style above.
- Do NOT wrap the asset in browser chrome, UI buttons, or device frames.
- NATIVE TRANSPARENT BACKGROUND (`background: "transparent"`): Isolate the subject (or organized icon sheet grid) on a 100% true transparent alpha background. Do NOT draw a fake checkerboard grid, do NOT add opaque background fills or vignettes, and keep clean alpha edges with a 10% safety margin around the silhouette.
- FOREGROUND EXTRACTION FROM COMPOSITE UI: Reproduce the exact foreground visual asset shown in the attached prerequisite Hero/Component reference image at full resolution on a transparent background, completely omitting the background environment, headlines, and UI buttons so this transparent PNG can be layered over the coded background.
', '[]', '["item_1"]', '{"id":"item_1_auto_asset_2","kind":"asset","name":"Hero 3D Growth Bars & Arrow Sculpture (Transparent)","tab_key":"full-landing-page","tab_label":"Full Landing Page","page_route":null,"variant_index":5,"viewport":{"intent":"isolated","css_width":1024},"aspect_ratio":"1:1","transparent_background":true,"extracted_from_item_id":"item_1","target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_1"],"direction":"3D glossy bar chart of three ascending cubes (purple, blue, cyan) on a dark pedestal with a large curved cyan-to-blue growth arrow sweeping upward to the right.","exact_copy":[],"reference_asset_ids":[],"depends_on":["item_1"],"image_prompt":"Extract ONLY the 3D centerpiece from the hero section of the attached reference UI image: the three glossy ascending 3D bar cubes (purple, mid-blue, cyan) sitting on a dark rounded pedestal, plus the large curved translucent growth arrow in cyan-to-blue gradient sweeping up and to the right. Isolate the entire sculpture on a clean 100% transparent background, removing the dark hero background, glow, and all text.","implementation_notes":["Render on a 100% transparent PNG background and layer in the component/page layout"],"behaviors":[],"responsive_rules":["Scale proportionally across breakpoints"],"asset_instructions":["Place this as the hero centerpiece on the right side of the hero section. Reconstruct it as an interactive 3D object from this reference image: three extruded glossy bars of increasing height on a pedestal with a curved arrow ribbon above. Animate it with a slow continuous rotation around its vertical axis, a gentle floating idle bob, and an interactive tilt toward the pointer on hover."],"is_visual_anchor":false}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T19:17:27.089599+00:00', '2026-10-24T19:18:12.561137+00:00', 'full-landing-page', 'Full Landing Page', 'example-out-nexora-3', 1, 'example-out-nexora-3');
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-nexora-5', 'example-run-nexora', 'example-nexora', 'item_1_auto_asset_4', 7, 'asset', 'Dashboard Product Mockup Panel (Transparent)', 'Dark SaaS dashboard UI mockup with sidebar nav, greeting header, MRR/conversion/experiments stat cards, revenue trend line chart, and experiment results bar chart.', 'asset', NULL, '16:9', 'completed', 'showcases/nexora/05.png', 'showcases/nexora/05.webp', 'image/png', 1536, 864, 1101521, 1, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (asset)
- Direction / Item Title: Dashboard Product Mockup Panel (Transparent) (Variant #7)
- Viewport & Aspect Ratio: isolated (1024px target width), Aspect Ratio 16:9
- Scope Rules: Standalone visual site asset (icon sheet, illustration, 3D cutout, or foreground graphic) rendered on a 100% native transparent background so it can be layered directly into the coordinated page/component set.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Nexora — A premium SaaS landing page for Nexora that converts visitors into customers through a compelling hero, clear product benefits, interactive previews, social proof, pricing, and strong CTAs.
- Product / Application Description: Tagline: Turn Big Ideas Into Business Growth.

A premium SaaS landing page designed to convert visitors into customers. Features a compelling hero section, product benefits, interactive previews, testimonials, pricing plans, and strong calls to action.
- Industry & Target Audience: B2B SaaS / Business Growth Software | Audience: Founders, startup teams, and growth-focused business decision-makers evaluating tools to scale revenue (Quickly understand the value proposition, see the product in action, trust the brand via testimonials, and compare pricing without friction)
- Primary User Goal / Conversion Action: Sign up for a free trial or request a demo via the primary CTA

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #2 Modern / Contemporary
- Exact Style Definition: A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time.
- Mandatory Visual & CSS System Rules: Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "modern / contemporary". Primary style #2 Modern / Contemporary (Minimal & Editorial): A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, modern, and growth-oriented — polished and premium without feeling corporate or cold — Contemporary SaaS aesthetic: crisp typography, generous whitespace, hairline borders, soft ambient elevation, and a single confident accent color guiding every CTA. Interactive product previews sit in elevated surface cards with refined micro-interactions.
- UI Density & Tone: comfortable / Clear, aspirational, and benefit-driven — ''Turn Big Ideas Into Business Growth'' (English)
- Color Palette: Primary #4F46E5, Accent #06B6D4, Background #FAFAFC, Surface #FFFFFF, Text #0F172A, Muted #64748B, Border #E2E8F0
- Typography Intent: Heading Inter / Sora — SemiBold, tight tracking, 48px hero / 24px section scale, Body Inter — Regular, 15–16px, 1.6 line height, Mono JetBrains Mono — for code snippets, metrics, and pricing figures
- Radii, Borders & Shadows: Radius 12–16px cards, 10px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(15,23,42,0.06) light / 0 8px 24px rgba(0,0,0,0.4) dark
- Logo Instruction: Use a clean temporary monogram ''N'' integrated with the brand name ''Nexora'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E2E8F0), and subtle surface luminance shifts (#FAFAFC vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#0F172A on #FAFAFC/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Dark SaaS dashboard UI mockup with sidebar nav, greeting header, MRR/conversion/experiments stat cards, revenue trend line chart, and experiment results bar chart.

PLANNER IMAGE PROMPT:
Extract ONLY the full dashboard product mockup panel from the ''Your Growth Engine, Live'' section of the attached reference UI image: the dark rounded dashboard with the Nexora sidebar, ''Good morning, Alex'' header, three stat cards, the Revenue Trend line chart, and the Experiment Results bar chart. Isolate the entire panel on a clean 100% transparent background, removing the section background and surrounding text.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_1] Single desktop full-page vertical scroll landing page at 1440px width in 2:3 canvas, dark theme Brand Kit colors

EXACT COPY & HIERARCHY:
Use concise, realistic copy matching brand language.

ATTACHED REFERENCE ROLES:
- Prerequisite Hero/Component Reference Image [FOREGROUND ASSET EXTRACTION]: Extract and faithfully reproduce ONLY the specified foreground illustration, 3D object, icon set, or graphic element from this reference UI image onto a 100% transparent background (`background: transparent`). Strip away all background walls, gradients, UI buttons, and text!

INVARIANT RENDERING CONSTRAINTS:
- Render a clean, production-ready standalone website asset matching the chosen Web Design Style above.
- Do NOT wrap the asset in browser chrome, UI buttons, or device frames.
- NATIVE TRANSPARENT BACKGROUND (`background: "transparent"`): Isolate the subject (or organized icon sheet grid) on a 100% true transparent alpha background. Do NOT draw a fake checkerboard grid, do NOT add opaque background fills or vignettes, and keep clean alpha edges with a 10% safety margin around the silhouette.
- FOREGROUND EXTRACTION FROM COMPOSITE UI: Reproduce the exact foreground visual asset shown in the attached prerequisite Hero/Component reference image at full resolution on a transparent background, completely omitting the background environment, headlines, and UI buttons so this transparent PNG can be layered over the coded background.
', '[]', '["item_1"]', '{"id":"item_1_auto_asset_4","kind":"asset","name":"Dashboard Product Mockup Panel (Transparent)","tab_key":"full-landing-page","tab_label":"Full Landing Page","page_route":null,"variant_index":7,"viewport":{"intent":"isolated","css_width":1024},"aspect_ratio":"16:9","transparent_background":true,"extracted_from_item_id":"item_1","target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_1"],"direction":"Dark SaaS dashboard UI mockup with sidebar nav, greeting header, MRR/conversion/experiments stat cards, revenue trend line chart, and experiment results bar chart.","exact_copy":[],"reference_asset_ids":[],"depends_on":["item_1"],"image_prompt":"Extract ONLY the full dashboard product mockup panel from the ''Your Growth Engine, Live'' section of the attached reference UI image: the dark rounded dashboard with the Nexora sidebar, ''Good morning, Alex'' header, three stat cards, the Revenue Trend line chart, and the Experiment Results bar chart. Isolate the entire panel on a clean 100% transparent background, removing the section background and surrounding text.","implementation_notes":["Render on a 100% transparent PNG background and layer in the component/page layout"],"behaviors":[],"responsive_rules":["Scale proportionally across breakpoints"],"asset_instructions":["Place this mockup as the large product screenshot on the right side of the ''Your Growth Engine, Live'' section, slightly elevated with a soft shadow. Recreate it as a real coded UI panel rather than a flat image if feasible; if kept as an image asset, add a subtle scroll-triggered fade-and-rise entrance and a gentle parallax float."],"is_visual_anchor":false}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T19:17:27.089599+00:00', '2026-10-24T19:18:20.876991+00:00', 'full-landing-page', 'Full Landing Page', 'example-out-nexora-3', 1, 'example-out-nexora-3');
INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, estimated_cost_micros, actual_cost_micros, error_message, save_journal_path, created_at, updated_at, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id) VALUES ('example-out-nexora-6', 'example-run-nexora', 'example-nexora', 'item_1_auto_asset_1', 4, 'asset', 'Nexora Brand Logo Mark & Wordmark (Transparent)', 'Geometric ''N'' logo mark in blue-to-cyan gradient inside a rounded square, paired with white ''Nexora'' wordmark.', 'asset', NULL, '1:1', 'completed', 'showcases/nexora/06.png', 'showcases/nexora/06.webp', 'image/png', 1024, 1024, 643993, 1, 'DESIGN MASTER IMAGE PROMPT COMPILER (v1 — IMPECCABLE ANTI-AI-SLOP EDITION)
==========================================================================
OUTPUT SCOPE & CROP:
- Mode: full_page (asset)
- Direction / Item Title: Nexora Brand Logo Mark & Wordmark (Transparent) (Variant #4)
- Viewport & Aspect Ratio: isolated (1024px target width), Aspect Ratio 1:1
- Scope Rules: Standalone visual site asset (icon sheet, illustration, 3D cutout, or foreground graphic) rendered on a 100% native transparent background so it can be layered directly into the coordinated page/component set.

WHAT WE ARE BUILDING INSIDE THIS PROJECT:
- Project: Nexora — A premium SaaS landing page for Nexora that converts visitors into customers through a compelling hero, clear product benefits, interactive previews, social proof, pricing, and strong CTAs.
- Product / Application Description: Tagline: Turn Big Ideas Into Business Growth.

A premium SaaS landing page designed to convert visitors into customers. Features a compelling hero section, product benefits, interactive previews, testimonials, pricing plans, and strong calls to action.
- Industry & Target Audience: B2B SaaS / Business Growth Software | Audience: Founders, startup teams, and growth-focused business decision-makers evaluating tools to scale revenue (Quickly understand the value proposition, see the product in action, trust the brand via testimonials, and compare pricing without friction)
- Primary User Goal / Conversion Action: Sign up for a free trial or request a demo via the primary CTA

CHOSEN WEB DESIGN STYLE (MANDATORY AESTHETIC & SYSTEM RULES):
- Primary Web Design Style: #2 Modern / Contemporary
- Exact Style Definition: A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time.
- Mandatory Visual & CSS System Rules: Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.
- Combinable Layout / System Modifiers: None
- Combined Style Brief: Style brief: "modern / contemporary". Primary style #2 Modern / Contemporary (Minimal & Editorial): A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions. It changes over time. Visual rules (apply using the User''s Brand Kit colors, logo & typography): Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.

BRAND KIT & DESIGN.MD TOKENS:
- Brand Personality & Direction: Confident, modern, and growth-oriented — polished and premium without feeling corporate or cold — Contemporary SaaS aesthetic: crisp typography, generous whitespace, hairline borders, soft ambient elevation, and a single confident accent color guiding every CTA. Interactive product previews sit in elevated surface cards with refined micro-interactions.
- UI Density & Tone: comfortable / Clear, aspirational, and benefit-driven — ''Turn Big Ideas Into Business Growth'' (English)
- Color Palette: Primary #4F46E5, Accent #06B6D4, Background #FAFAFC, Surface #FFFFFF, Text #0F172A, Muted #64748B, Border #E2E8F0
- Typography Intent: Heading Inter / Sora — SemiBold, tight tracking, 48px hero / 24px section scale, Body Inter — Regular, 15–16px, 1.6 line height, Mono JetBrains Mono — for code snippets, metrics, and pricing figures
- Radii, Borders & Shadows: Radius 12–16px cards, 10px buttons, 999px pills, Border 1px solid subtle warm border (#E2DED5), Shadow 0 8px 24px rgba(15,23,42,0.06) light / 0 8px 24px rgba(0,0,0,0.4) dark
- Logo Instruction: Use a clean temporary monogram ''N'' integrated with the brand name ''Nexora'' (Top-left header or card header mark with clear breathing room). Do not invent an unrelated complex emblem.

ANTI-AI-SLOP VISUAL CRAFTSMANSHIP RULES (IMPECCABLE STANDARD — STRICTLY ENFORCED):
- PASS THE SQUINT TEST: Establish one unmistakable #1 primary focal point (e.g., an authentic interactive product/canvas showcase, architectural diagram, or bold editorial typographic anchor), a clear #2 secondary action, and generous negative space.
- BAN CARD-IN-CARD HELL ("CARDOCALYPSE"): Do NOT wrap every section in a boxed card containing nested sub-cards with heavy borders. Use proximity, intentional whitespace, crisp 1px architectural hairlines (#E2E8F0), and subtle surface luminance shifts (#FAFAFC vs #FFFFFF). Avoid repetitive 3-column "icon-in-a-circle" boilerplate grids; prefer asymmetric bento grids, split editorial layouts, or live product previews.
- BAN CLICHÉ AI SLOP GRADIENTS & NEON BLOB SOUP: Do NOT render generic purple-to-blue SaaS gradients, random floating glowing spheres/blobs, meaningless sci-fi HUD rings on non-gaming apps, or cheap rainbow borders. Strictly use the Brand Kit palette above with tinted neutrals and purposeful accents.
- HIGH-CONTRAST TYPOGRAPHY & ZERO AI FILLER COPY: Render razor-sharp, high-contrast text (#0F172A on #FAFAFC/#FFFFFF — never unreadable faint gray text on colored or translucent backgrounds). Left-align body copy and data metrics; use tabular numerals for numbers. Never render generic AI filler phrases ("Revolutionize your workflow", "Lorem ipsum", or fake SaaS pricing on open-source tools).

CREATIVE ART DIRECTION FOR THIS OUTPUT:
Geometric ''N'' logo mark in blue-to-cyan gradient inside a rounded square, paired with white ''Nexora'' wordmark.

PLANNER IMAGE PROMPT:
Extract ONLY the Nexora brand logo from the attached reference UI image: the rounded-square ''N'' mark with a blue-to-cyan gradient and the adjacent white ''Nexora'' wordmark as seen in the top navigation bar and footer. Isolate it on a clean 100% transparent background, removing all surrounding nav links, buttons, and background surfaces. Reproduce it faithfully at high resolution.

SCREENSHOT IMPROVEMENTS & REQUIRED CONTROLS:
- [req_1] Single desktop full-page vertical scroll landing page at 1440px width in 2:3 canvas, dark theme Brand Kit colors

EXACT COPY & HIERARCHY:
Use concise, realistic copy matching brand language.

ATTACHED REFERENCE ROLES:
- Prerequisite Hero/Component Reference Image [FOREGROUND ASSET EXTRACTION]: Extract and faithfully reproduce ONLY the specified foreground illustration, 3D object, icon set, or graphic element from this reference UI image onto a 100% transparent background (`background: transparent`). Strip away all background walls, gradients, UI buttons, and text!

INVARIANT RENDERING CONSTRAINTS:
- Render a clean, production-ready standalone website asset matching the chosen Web Design Style above.
- Do NOT wrap the asset in browser chrome, UI buttons, or device frames.
- NATIVE TRANSPARENT BACKGROUND (`background: "transparent"`): Isolate the subject (or organized icon sheet grid) on a 100% true transparent alpha background. Do NOT draw a fake checkerboard grid, do NOT add opaque background fills or vignettes, and keep clean alpha edges with a 10% safety margin around the silhouette.
- FOREGROUND EXTRACTION FROM COMPOSITE UI: Reproduce the exact foreground visual asset shown in the attached prerequisite Hero/Component reference image at full resolution on a transparent background, completely omitting the background environment, headlines, and UI buttons so this transparent PNG can be layered over the coded background.
', '[]', '["item_1"]', '{"id":"item_1_auto_asset_1","kind":"asset","name":"Nexora Brand Logo Mark & Wordmark (Transparent)","tab_key":"full-landing-page","tab_label":"Full Landing Page","page_route":null,"variant_index":4,"viewport":{"intent":"isolated","css_width":1024},"aspect_ratio":"1:1","transparent_background":true,"extracted_from_item_id":"item_1","target_existing_output_id":null,"quality_override":null,"requirements_covered":["req_1"],"direction":"Geometric ''N'' logo mark in blue-to-cyan gradient inside a rounded square, paired with white ''Nexora'' wordmark.","exact_copy":[],"reference_asset_ids":[],"depends_on":["item_1"],"image_prompt":"Extract ONLY the Nexora brand logo from the attached reference UI image: the rounded-square ''N'' mark with a blue-to-cyan gradient and the adjacent white ''Nexora'' wordmark as seen in the top navigation bar and footer. Isolate it on a clean 100% transparent background, removing all surrounding nav links, buttons, and background surfaces. Reproduce it faithfully at high resolution.","implementation_notes":["Render on a 100% transparent PNG background and layer in the component/page layout"],"behaviors":[],"responsive_rules":["Scale proportionally across breakpoints"],"asset_instructions":["Place this logo in the top navigation bar (left side) and in the footer brand column. Convert it into a vectorial SVG graphic so it scales crisply; recreate the gradient fill of the N mark exactly and keep the wordmark as part of the SVG lockup."],"is_visual_anchor":false}', '{}', NULL, 0, 0, NULL, NULL, '2026-10-24T19:17:27.089599+00:00', '2026-10-24T19:17:57.347830+00:00', 'full-landing-page', 'Full Landing Page', 'example-out-nexora-3', 1, 'example-out-nexora-3');
INSERT INTO thread_messages (id, thread_id, role, content, created_at) VALUES ('example-message-nexora-1', 'example-thread-nexora', 'user', 'Tagline: Turn Big Ideas Into Business Growth.

A premium SaaS landing page designed to convert visitors into customers. Features a compelling hero section, product benefits, interactive previews, testimonials, pricing plans, and strong calls to action.', '2026-10-24T19:16:09.163486+00:00');
INSERT INTO thread_messages (id, thread_id, role, content, linked_run_id, created_at) VALUES ('example-message-nexora-2', 'example-thread-nexora', 'assistant', 'Nexora showcase, including the finished designs and extracted assets. Open any image to inspect it or copy its implementation prompt.', 'example-run-nexora', '2026-10-24T19:18:20.888460+00:00');

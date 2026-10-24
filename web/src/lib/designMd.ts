import { BrandSpec } from "./api";

export interface DesignMdProjectInfo {
  name: string;
  description?: string;
  purpose?: string;
  industry?: string;
  target_audience?: string;
  audience_needs?: string;
  desired_user_action?: string;
}

/**
 * Generates a comprehensive, production-grade `DESIGN.md` document following the
 * `nexu-io/open-design` canonical brand schema and `pbakaus/impeccable` Anti-AI-Slop rules.
 */
export function generateDesignMd(
  project: DesignMdProjectInfo,
  brand: BrandSpec,
  brandVersion: number = 1
): string {
  const ds = brand.design_style;
  const styleTitle =
    ds && ds.style_name?.trim()
      ? `#${ds.style_number || 1} ${ds.style_name}`
      : brand.brand_personality || "Custom Studio System";
  const styleDef =
    ds && ds.definition?.trim()
      ? ds.definition
      : brand.design_direction || "High-craft dark studio visual architecture.";
  const styleRules =
    ds && ds.visual_rules?.trim()
      ? ds.visual_rules
      : "Enforce 1px architectural hairlines, high-contrast scannable typography, and generous negative space.";
  const styleModifiers =
    ds && ds.modifiers && ds.modifiers.length > 0
      ? ds.modifiers.join(" + ")
      : "None";

  const d = brand.colors.dark;
  const l = brand.colors.light;
  const t = brand.typography;
  const lt = brand.layout_tokens;
  const tc = brand.target_context;

  const doRules =
    brand.brand_rules && brand.brand_rules.length > 0
      ? brand.brand_rules.map((r) => `- ${r}`).join("\n")
      : "- Maintain strict visual consistency across all routes, components, and interactive states.\n- Ensure WCAG AA/AAA text contrast on every surface.";

  const inclusions =
    brand.mandatory_inclusions && brand.mandatory_inclusions.length > 0
      ? brand.mandatory_inclusions.map((r) => `- ${r}`).join("\n")
      : "- Canonical brand mark / monogram and consistent primary action buttons.";

  const forbidden =
    brand.forbidden_treatments && brand.forbidden_treatments.length > 0
      ? brand.forbidden_treatments.map((r) => `- ${r}`).join("\n")
      : "- No generic AI slop gradients, nested card-in-card clutter, or low-contrast gray text on colored surfaces.";

  return `# DESIGN.md — ${project.name || "Project Design System"}

> Canonical AI Design System & Brand Contract (\`nexu-io/open-design\` + \`pbakaus/impeccable\` Anti-AI-Slop Standard · Brand Version v${brandVersion})

---

## 1. Visual Theme, Atmosphere & Product Identity ("The Why")

- **Project Name**: ${project.name || "Untitled Project"}
- **Product Description**: ${project.description || "High-craft digital product interface."}
- **Purpose & Industry**: ${project.purpose || "Web / Product UI"} (${project.industry || "Software"})
- **Target Audience & Needs**: ${project.target_audience || "Users & Developers"}${
    project.audience_needs ? ` — ${project.audience_needs}` : ""
  }
- **Primary Conversion / User Action**: ${project.desired_user_action || "Primary workflow action"}
- **Chosen Design Style**: **${styleTitle}**
- **Style Philosophy & Atmosphere**: ${styleDef}
- **Mandatory Visual System Rules**: ${styleRules}
- **Combinable Layout / System Modifiers**: ${styleModifiers}
- **Brand Personality & Direction**: ${brand.brand_personality} — ${brand.design_direction}
- **Tone of Voice & UI Density**: ${brand.tone_of_voice} | Density: \`${brand.ui_density}\` | Language: \`${brand.content_language}\`

---

## 2. Color Palette & Component Semantic Roles (\`/colorize\`)

Colors are defined strictly by functional UI role so coding agents map every surface, border, navbar, and button deterministically without guessing or defaulting to generic AI gradients.

### 2.1 Core Atmospheric Palette (Dark & Light Polarity)

| Semantic Token | Dark Mode (Primary Polarity) | Light Mode Polarity | Functional Usage & Role |
| :--- | :--- | :--- | :--- |
| **Primary Brand Action** | \`${d.primary}\` | \`${l.primary}\` | Primary CTA buttons, active route indicators, focus rings, key interactive highlights |
| **Secondary / Accent** | \`${d.accent}\` | \`${l.accent}\` | Interactive badges, secondary highlights, telemetry accents, subtle spark nodes |
| **Canvas Background** | \`${d.background}\` | \`${l.background}\` | Root viewport canvas (\`body\` / main background) — tinted neutral, never dead \`#000000\` |
| **Elevated Surface / Card** | \`${d.surface}\` | \`${l.surface}\` | Sticky navbar shell, elevated bento panels, cards, modal dialogs, inspector drawers |
| **Primary Ink (Text)** | \`${d.text}\` | \`${l.text}\` | Headlines, primary body copy, active navigation labels, form input values |
| **Secondary / Muted Ink** | \`${d.muted_text}\` | \`${l.muted_text}\` | Supporting descriptions, metadata captions, timestamps, axis labels |
| **Architectural Hairline Border** | \`${d.border}\` | \`${l.border}\` | \`1px\` structural grid dividers, card borders, input outlines, navbar bottom border |
| **Success Semantic** | \`${d.success}\` | \`${l.success}\` | Completed states, verified alpha badges, positive delta indicators |
| **Warning Semantic** | \`${d.warning}\` | \`${l.warning}\` | Caution banners, pending/in-progress indicators |
| **Danger / Destructive** | \`${d.danger}\` | \`${l.danger}\` | Error states, destructive actions, validation alerts |

### 2.2 Component-Specific Color Allocation Matrix (Navbar, Buttons, Cards, Inputs, Badges)

| UI Component | Background / Fill | Text / Icon Color | Border Token | Hover / Active / Focus State |
| :--- | :--- | :--- | :--- | :--- |
| **Sticky Top Navbar** | \`${d.surface}\` (\`85%\` alpha + \`backdrop-filter: blur(16px)\`) | Active link: \`${d.text}\` · Inactive link: \`${d.muted_text}\` | Bottom \`1px solid ${d.border}\` | Hover shifts link to \`${d.text}\`; active route uses shared indicator in \`${d.primary}\`; right CTA uses Primary Button spec |
| **Primary Action Button** | Solid \`${d.primary}\` (or subtle linear sheen \`${d.primary}\` → \`${d.accent}\`) | High-contrast \`#FFFFFF\` | \`1px solid ${d.primary}\` | Hover lifts \`-1px\`, brightens fill \`+8%\`, shadow \`0 8px 24px -6px ${d.primary}66\`; active press scales \`0.98\` |
| **Secondary / Outline Button** | \`${d.surface}\` | \`${d.text}\` | \`1px solid ${d.border}\` | Hover shifts border to \`${d.primary}\`, tints surface with \`12% ${d.primary}\`, text stays \`${d.text}\` |
| **Ghost / Toolbar Button** | \`transparent\` | \`${d.muted_text}\` | \`1px solid transparent\` | Hover fills \`${d.surface}\` with \`1px solid ${d.border}\`, text brightens to \`${d.text}\` |
| **Destructive Button** | \`${d.danger}22\` (14% tint at rest) | \`${d.danger}\` | \`1px solid ${d.danger}66\` | Hover transitions to solid \`${d.danger}\` fill with \`#FFFFFF\` text |
| **Feature / Bento Card** | \`${d.surface}\` | Title: \`${d.text}\` · Body: \`${d.muted_text}\` | \`1px solid ${d.border}\` | Hover highlights border toward \`${d.primary}66\` with subtle elevation |
| **Form Input / Chat Box** | \`${d.background}\` | Value: \`${d.text}\` · Placeholder: \`${d.muted_text}\` | \`1px solid ${d.border}\` | Focus ring \`2px solid ${d.primary}\`, border \`${d.primary}\` |
| **Status / Category Badge** | \`${d.primary}22\` or \`${d.accent}22\` | \`${d.text}\` or \`${d.accent}\` | \`1px solid ${d.primary}55\` | Crisp pill geometry with \`tabular-nums\` counters |

---

## 3. Typography & Scannable Hierarchy (\`/typeset\`)

- **Heading / Display Font Family**: \`${t.heading_family}\`
  - Allowed Weights: \`${(t.weights && t.weights.length ? t.weights : [500, 600, 700]).join(", ")}\`
  - Tracking & Leading: Tight optical letter-spacing (\`-0.025em\` to \`-0.04em\`) with crisp display line-height (\`1.08\`–\`1.18\`).
- **Body / UI Prose Font Family**: \`${t.body_family}\`
  - Hierarchy Rules: \`400\` Regular for long-form prose, \`500\` Medium for navigation links and interactive controls, \`600\` SemiBold for key callouts.
  - Measure & Leading: Generous \`1.55\`–\`1.65\` line-height; constrain prose blocks to \`60ch–70ch\` maximum width and always left-align multi-line copy.
- **Monospace / Code / Telemetry Font**: \`${t.mono_family}\`
  - Numeric Discipline: Enforce \`font-variant-numeric: tabular-nums\` on all metrics, prices, counters, badges, and coordinates.
- **Fallback Stack**: \`${t.fallback_stack}\`
- **Type Scale System**: \`${t.size_scale}\`
- **Line-Height Scale**: \`${t.line_heights}\`
- **Font Source**: \`${t.font_source}\`

---

## 4. Layout, Spatial Architecture & Component Specifications (\`/layout\`)

- **Spacing Scale**: \`${lt.spacing_scale}\`
- **Container & Content Width**: \`${lt.content_width}\`
- **Grid Architecture**: \`${lt.grid_columns}\`
- **Corner Radii System**: \`${lt.radii}\`
- **Borders & Dividers**: \`${lt.borders}\`
- **Shadow & Depth Elevation**: \`${lt.shadows}\`
- **Iconography System**: \`${lt.icon_style}\` (All brand marks, UI icons, and geometric badges must be implemented as crisp vectorial SVG graphics)
- **Button Anatomy & Treatment**: \`${lt.button_treatment}\`
- **Logo Policy**: Mode=\`${brand.logo.mode}\` | Monogram=\`${brand.logo.monogram_text || "DM"}\` | Preferred Usage: ${brand.logo.preferred_usage} | Inclusion Policy: \`${brand.logo.inclusion_policy}\`
- **Target Stack & Repository Context**: Stack: \`${tc.target_stack || "Next.js / React / Tailwind CSS"}\`${
    tc.preferred_asset_dir ? ` | Asset Directory: \`${tc.preferred_asset_dir}\`` : ""
  }${tc.component_library_notes ? ` | Notes: ${tc.component_library_notes}` : ""}

---

## 5. Anti-AI-Slop Engineering & Design Guardrails (\`pbakaus/impeccable\` Standard)

1. **The Squint Test (Mandatory Hierarchy Check)**:
   - When squinting at or blurring any screen, there must be an unmistakable **#1 Primary Focal Point** (e.g., an interactive product preview, architectural canvas showcase, or bold typographic thesis), a clear **#2 Secondary Action**, and distinct spatial grouping achieved through generous whitespace rather than heavy boxes.
2. **No "Card-in-Card Hell" ("Cardocalypse")**:
   - Never wrap a card inside another card with redundant borders and drop shadows. Use spatial proximity, \`1px\` architectural hairlines (\`${d.border}\`), and subtle surface luminance shifts (\`${d.background}\` vs \`${d.surface}\`).
   - Never use monotonous 3-column "icon-in-a-circle" boilerplate feature rows for non-equivalent content; use asymmetric bento grids, split editorial layouts, or interactive live product previews.
3. **No Cliché AI Slop Gradients or Floating Neon Blobs**:
   - Ban generic purple-to-blue SaaS gradients, random glowing floating spheres behind text, or meaningless sci-fi rings on non-gaming apps. Rely on OKLCH-inspired tinted neutrals and purposeful brand accents.
4. **Strict Contrast & Legibility (\`/colorize\`)**:
   - Never place low-contrast gray text on colored, gradient, or translucent backgrounds. Every text element must pass WCAG AA/AAA contrast against its immediate surface.
5. **Authentic Domain Copy & Zero Slop Text**:
   - Ban generic AI marketing filler ("Revolutionize your workflow", "Unleash the power of AI", "Lorem ipsum") and never invent mismatched business sections (e.g., never add paid SaaS pricing tables to open-source projects).
6. **Physical Motion Discipline**:
   - Ban dated \`bounce\` or exaggerated \`elastic\` easing curves. Use physical spring physics (\`stiffness: 320, damping: 28\`) or exponential ease-out (\`cubic-bezier(0.22, 1, 0.36, 1)\`) with \`150ms–240ms\` micro-interaction timing.

### Project-Specific Brand Rules

**Do Rules**:
${doRules}

**Mandatory Inclusions**:
${inclusions}

**Forbidden Treatments (Never Do)**:
${forbidden}
`;
}

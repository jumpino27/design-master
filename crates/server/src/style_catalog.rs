use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct StyleCatalogEntry {
    pub number: u32,
    pub id: &'static str,
    pub name: &'static str,
    pub category: &'static str,
    pub definition: &'static str,
    pub visual_rules: &'static str,
    pub best_fit: &'static str,
    pub essential_screens: &'static str,
    pub keywords: &'static [&'static str],
}

pub const MARKETPLACE_ESSENTIAL_SCREENS: &str =
    "Search and filters; category pages; listing details; seller profiles; ratings; saved items; and a purchase, booking, or inquiry flow.";
pub const DROPSHIPPING_ESSENTIAL_SCREENS: &str =
    "Product benefits and specifications; useful product media; variant selection; pricing; clear delivery and returns information; cart; and checkout.";
pub const SHOP_ESSENTIAL_SCREENS: &str =
    "Category navigation; search; collection filters; product pages; variants and availability; cart; checkout; order information; and customer support.";
pub const SOCIAL_ESSENTIAL_SCREENS: &str =
    "Feed or discovery view; profiles; publishing tools; comments and reactions; search; notifications; privacy controls; and community moderation tools.";
pub const ANIME_ESSENTIAL_SCREENS: &str =
    "Poster search; genre and format filters; title pages; watching status; episode progress; ratings; custom lists; and seasonal or release-calendar views.";
pub const MOVIE_ESSENTIAL_SCREENS: &str =
    "Search and filters; film pages; watched and watchlist status; ratings and reviews; viewing dates; cast and crew information; and custom collections.";
pub const GAME_ESSENTIAL_SCREENS: &str =
    "Search; platform and genre filters; game pages; owned and wishlist status; backlog and completion status; personal ratings; and configurable playtime or progress fields.";
pub const GENERAL_WEB_SCREENS: &str =
    "Sticky top navigation; hero value proposition & primary CTA; core feature/benefit cards; interactive proof or directory/product showcase; pricing or conversion flow; and structured footer.";

pub static STYLE_CATALOG: &[StyleCatalogEntry] = &[
    // =========================================================================
    // FOUNDATIONAL WEB DESIGN STYLES (#1 - #50)
    // =========================================================================
    StyleCatalogEntry {
        number: 1,
        id: "01-minimalism",
        name: "Minimalism",
        category: "Minimal & Editorial",
        definition: "Uses few elements, generous whitespace, restrained colors, and clear typography to focus attention on essential content.",
        visual_rules: "Enforce 60%+ open whitespace, razor-thin 1px borders, restrained neutral surfaces paired with the user's brand primary accent, zero decorative clutter, and generous 48px–96px section padding.",
        best_fit: "Design-forward SaaS, architecture, studios, portfolios, and focused digital products.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["minimalism", "minimal", "whitespace", "clean", "restrained", "simple", "essential", "zen", "airy"],
    },
    StyleCatalogEntry {
        number: 2,
        id: "02-modern-contemporary",
        name: "Modern / Contemporary",
        category: "Minimal & Editorial",
        definition: "A broad label for design that feels current: clear hierarchy, polished typography, balanced spacing, and considered interactions.",
        visual_rules: "Use crisp 12px–16px border radii, subtle 1px hairline borders with soft ambient elevation, clear typographic scale (48px/24px/15px), balanced 12-column grid, and refined micro-interaction states.",
        best_fit: "Modern SaaS platforms, web apps, startups, developer tools, and general digital products.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["modern", "contemporary", "saas", "polished", "hierarchy", "balanced", "sleek", "product", "startup"],
    },
    StyleCatalogEntry {
        number: 3,
        id: "03-swiss-international",
        name: "Swiss / International Style",
        category: "Minimal & Editorial",
        definition: "Uses precise grids, strong alignment, sans-serif typography, and clear visual hierarchy. Emphasizes order and readability.",
        visual_rules: "Expose mathematical multi-column grid lines (1px solid), left-aligned flush neo-grotesque typography, high-contrast brand accent blocks, numerical index labels (01, 02, 03), and strict geometric proportion.",
        best_fit: "Directories, agencies, cultural institutions, documentation, and information-dense platforms.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["swiss", "international", "grid", "helvetica", "alignment", "order", "objective", "modular", "typographic"],
    },
    StyleCatalogEntry {
        number: 4,
        id: "04-editorial",
        name: "Editorial",
        category: "Minimal & Editorial",
        definition: "Borrows from magazines and newspapers: expressive headlines, carefully arranged text, photography, and layouts built around storytelling.",
        visual_rules: "Pair high-contrast display serif headlines with crisp editorial body columns, pull-quotes, issue/volume metadata headers, hairline column rules, and art-directed photography crops.",
        best_fit: "Publications, storytelling brands, blogs, curated directories, and content-rich platforms.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["editorial", "magazine", "newspaper", "publication", "serif", "storytelling", "journal", "articles"],
    },
    StyleCatalogEntry {
        number: 5,
        id: "05-typography-led",
        name: "Typography-led",
        category: "Minimal & Editorial",
        definition: "Makes typography the main visual attraction through oversized text, expressive fonts, unusual arrangements, or animated lettering.",
        visual_rules: "Make monumental display typography (72px–120px scale, tight -0.04em tracking) the dominant hero visual and structural framing device, contrasting ultra-bold display headings with technical monospace metadata.",
        best_fit: "Creative studios, portfolios, events, manifestos, and bold brand launches.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["typography", "typography-led", "oversized text", "lettering", "display font", "monumental", "poster"],
    },
    StyleCatalogEntry {
        number: 6,
        id: "06-luxury-premium",
        name: "Luxury / Premium",
        category: "Minimal & Editorial",
        definition: "Communicates exclusivity through refined typography, careful spacing, restrained details, and exceptional imagery. Can be light or dark.",
        visual_rules: "Use deep velvet or warm silk surfaces, fine metallic/brand-accent hairline borders, wide-tracked uppercase micro-labels (0.18em letter-spacing), high-contrast couture serif headings, and museum-grade spacing.",
        best_fit: "High-end brands, hospitality, real estate, concierge services, and prestige products.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["luxury", "premium", "exclusive", "bespoke", "refined", "couture", "high-end", "prestige", "vip"],
    },
    StyleCatalogEntry {
        number: 7,
        id: "07-corporate-professional",
        name: "Corporate / Professional",
        category: "Minimal & Editorial",
        definition: "Prioritizes credibility, consistency, and clarity through familiar layouts, controlled colors, and straightforward presentation.",
        visual_rules: "Use structured 12-column enterprise grid, high-trust KPI metric cards, compliance/trust badges, clear contrast ratios, and authoritative typography.",
        best_fit: "B2B SaaS, fintech, enterprise software, consulting, healthcare, and legal services.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["corporate", "professional", "enterprise", "b2b", "fintech", "business", "trust", "credibility"],
    },
    StyleCatalogEntry {
        number: 8,
        id: "08-scandinavian",
        name: "Scandinavian",
        category: "Minimal & Editorial",
        definition: "Combines simplicity with warmth: light backgrounds, muted colors, natural imagery, and practical, approachable layouts.",
        visual_rules: "Use warm daylight surfaces, soft organic corners (14px–20px), calm breathable spacing, natural lifestyle photography, and understated functional controls.",
        best_fit: "Home goods, wellness, sustainable brands, lifestyle apps, and calm consumer tools.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["scandinavian", "nordic", "warm minimal", "cozy", "calm", "natural", "approachable", "hygge"],
    },
    StyleCatalogEntry {
        number: 9,
        id: "09-japandi",
        name: "Japandi",
        category: "Minimal & Editorial",
        definition: "Blends Japanese restraint with Scandinavian warmth through earthy colors, natural textures, balanced composition, and minimal decoration.",
        visual_rules: "Use warm stone and timber-inspired neutrals combined with user brand tokens, architectural asymmetry, delicate 1px horizontal dividers, and serene tactile balance.",
        best_fit: "Architecture, interior design, artisanal products, mindfulness, and boutique hospitality.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["japandi", "japanese", "nordic", "earthy", "zen", "architectural", "serene", "craft"],
    },
    StyleCatalogEntry {
        number: 10,
        id: "10-flat-design",
        name: "Flat Design",
        category: "Dimensional & Tactile",
        definition: "Uses simple shapes, solid colors, and minimal texture or dimensional effects. Elements generally avoid realistic shadows and surfaces.",
        visual_rules: "Use pure 2D vector geometry with zero drop shadows and zero complex gradients, relying on crisp solid color blocks, sharp typography, and flat geometric icons.",
        best_fit: "Utility web apps, educational tools, fast-loading directories, and clean digital interfaces.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["flat", "flat design", "2d", "solid color", "vector", "no shadow", "simple", "geometric"],
    },
    StyleCatalogEntry {
        number: 11,
        id: "11-semi-flat-design",
        name: "Semi-flat Design",
        category: "Dimensional & Tactile",
        definition: "Keeps the simplicity of flat design while adding subtle shadows, layering, or gradients to communicate depth and interactivity.",
        visual_rules: "Use clean flat surfaces enhanced with subtle multi-layered ambient elevation shadows (`0 8px 24px rgba(0,0,0,0.06)`), smooth hover lift, and crisp card hierarchy.",
        best_fit: "SaaS dashboards, consumer web platforms, marketplaces, and interactive tools.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["semi-flat", "flat 2.0", "subtle shadow", "elevation", "layered", "modern ui", "clean cards"],
    },
    StyleCatalogEntry {
        number: 12,
        id: "12-skeuomorphism",
        name: "Skeuomorphism",
        category: "Dimensional & Tactile",
        definition: "Makes digital elements resemble physical objects or materials, such as leather notebooks, mechanical switches, or realistic buttons.",
        visual_rules: "Craft tactile hardware-inspired controls: brushed metal/matte-plastic bezels, physical bevels, recessed LED indicators, knurled dials, and realistic press states.",
        best_fit: "Audio/music tools, synthesizers, hardware companions, calculators, and tactile creative apps.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["skeuomorphism", "skeuomorphic", "tactile", "hardware", "physical", "switches", "knobs", "realistic"],
    },
    StyleCatalogEntry {
        number: 13,
        id: "13-neumorphism-soft-ui",
        name: "Neumorphism / Soft UI",
        category: "Dimensional & Tactile",
        definition: "Uses matching surface colors and paired light and dark shadows to make controls appear raised from or pressed into their background.",
        visual_rules: "Extrude cards and controls from a unified matte surface using paired top-left light highlights and bottom-right dark shadows, with crisp brand-colored active indicators.",
        best_fit: "Smart home controllers, media players, timers, wellness trackers, and focused control panels.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["neumorphism", "soft ui", "extruded", "embossed", "debossed", "tactile", "monochrome surface"],
    },
    StyleCatalogEntry {
        number: 14,
        id: "14-glassmorphism",
        name: "Glassmorphism",
        category: "Dimensional & Tactile",
        definition: "Uses translucent surfaces, background blur, fine borders, and layering to create a frosted-glass appearance.",
        visual_rules: "Layer translucent frosted panels (`backdrop-filter: blur(16px)`, semi-transparent surface fills) with 1px specular borders over rich brand-colored ambient mesh/glow backdrops.",
        best_fit: "AI platforms, fintech, modern SaaS, media apps, and spatial dashboards.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["glassmorphism", "frosted glass", "translucent", "blur", "backdrop-filter", "specular", "layered"],
    },
    StyleCatalogEntry {
        number: 15,
        id: "15-claymorphism",
        name: "Claymorphism",
        category: "Dimensional & Tactile",
        definition: "Uses rounded, inflated shapes and soft shading to make elements resemble clay or soft molded objects.",
        visual_rules: "Build UI cards, buttons, and 3D props with pill-rounded 24px–32px corners, puffy inflated volumes, dual inner shadows (top-left bright rim + bottom-right clay shade), and joyful matte finishes in the user's brand palette.",
        best_fit: "Creator platforms, link-in-bio directories, friendly SaaS, Web3/community apps, education, and consumer products.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["claymorphism", "clay", "3d clay", "inflated", "puffy", "rounded", "playful", "tactile", "friendly", "creator", "link-in-bio"],
    },
    StyleCatalogEntry {
        number: 16,
        id: "16-liquid-glass",
        name: "Liquid Glass",
        category: "Dimensional & Tactile",
        definition: "Uses glossy, fluid, refractive surfaces that look like curved glass or moving liquid.",
        visual_rules: "Render specular curved-glass highlights, chromatic edge refraction, fluid droplet contours, and deep translucent depth over luminous brand-tinted backdrops.",
        best_fit: "Next-gen hardware showcases, AI interfaces, spatial computing, and creative tech brands.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["liquid glass", "refractive", "glossy", "fluid", "chromatic", "specular", "spatial", "apple vision"],
    },
    StyleCatalogEntry {
        number: 17,
        id: "17-3d-dimensional-web",
        name: "3D / Dimensional Web Design",
        category: "Dimensional & Tactile",
        definition: "Integrates three-dimensional objects, scenes, or camera movement as a central part of the interface.",
        visual_rules: "Stage a striking 3D studio centerpiece with realistic studio lighting, perspective depth, floating interactive UI cards anchored in 3D space, and smooth parallax depth.",
        best_fit: "Product launches, interactive showcases, gaming/tech platforms, and immersive brand experiences.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["3d", "dimensional", "webgl", "three.js", "spline", "spatial", "perspective", "render"],
    },
    StyleCatalogEntry {
        number: 18,
        id: "18-material-tactile-surfaces",
        name: "Material / Tactile Surfaces",
        category: "Dimensional & Tactile",
        definition: "Uses textures and surface qualities inspired by paper, stone, metal, fabric, or glass to make digital layouts feel physical.",
        visual_rules: "Incorporate fine physical surface grain (linen, brushed aluminum, matte stone, or embossed cardstock), crisp 1px stitched/machined borders, and grounded contact shadows.",
        best_fit: "Craft brands, industrial design, boutique studios, hardware, and physical product platforms.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["material", "tactile", "texture", "paper", "stone", "metal", "linen", "physical", "grain"],
    },
    StyleCatalogEntry {
        number: 19,
        id: "19-brutalism",
        name: "Brutalism",
        category: "Bold & Avant-Garde",
        definition: "Uses raw, unpolished layouts, exposed structure, stark contrasts, and default or utilitarian elements to reject conventional polish.",
        visual_rules: "Expose raw 2px solid structural borders, zero border-radius (`0px`), stark high-contrast surfaces, monospace system/tabular typography, and unapologetic architectural honesty.",
        best_fit: "Experimental art, developer tools, cultural archives, underground fashion, and bold manifestos.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["brutalism", "raw", "exposed grid", "monospace", "stark", "unpolished", "architectural", "0px"],
    },
    StyleCatalogEntry {
        number: 20,
        id: "20-neobrutalism",
        name: "Neobrutalism",
        category: "Bold & Avant-Garde",
        definition: "A cleaner, more playful form of brutalism with thick borders, solid offset shadows, bright colors, and bold typography.",
        visual_rules: "Enforce crisp 2px–3px solid dark ink borders (`#111111`), hard non-blurred offset box shadows (`4px 4px 0px #111111`), punchy brand color blocks, pill sticker badges, and bold grotesque headings.",
        best_fit: "Creator tools, Gumroad/Figma-style marketplaces, indie SaaS, link-in-bio platforms, and youth brands.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["neobrutalism", "neo-brutalism", "hard shadow", "thick border", "playful", "gumroad", "indie", "creator"],
    },
    StyleCatalogEntry {
        number: 21,
        id: "21-anti-design",
        name: "Anti-design",
        category: "Bold & Avant-Garde",
        definition: "Deliberately breaks standard usability or visual harmony through clashing colors, awkward layouts, overlapping elements, and visual tension.",
        visual_rules: "Create controlled avant-garde tension with overlapping sticker/window layers, rotated callout badges, asymmetrical type collisions, while keeping core CTAs and navigation functional.",
        best_fit: "Avant-garde fashion, music festivals, experimental art collectives, and counter-culture zines.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["anti-design", "avant-garde", "experimental", "overlapping", "clashing", "rebellious", "art"],
    },
    StyleCatalogEntry {
        number: 22,
        id: "22-maximalism",
        name: "Maximalism",
        category: "Bold & Avant-Garde",
        definition: "Embraces abundance through rich color, layered graphics, mixed typography, patterns, and dense visual detail.",
        visual_rules: "Layer rich decorative patterns, mixed display serif + bold sans typography, expressive badge stickers, and vibrant brand color combinations within a repeatable card structure.",
        best_fit: "Creative festivals, expressive lifestyle brands, entertainment platforms, and bold ecommerce.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["maximalism", "maximalist", "abundant", "rich", "layered", "patterns", "eclectic", "vibrant"],
    },
    StyleCatalogEntry {
        number: 23,
        id: "23-dopamine-design",
        name: "Dopamine Design",
        category: "Bold & Avant-Garde",
        definition: "Uses saturated colors, high contrast, playful shapes, and energetic visuals to create an immediate upbeat impression.",
        visual_rules: "Use high-joy saturated brand accents, bouncy pill buttons, celebratory sticker badges, playful wavy dividers, and high-energy micro-interactions.",
        best_fit: "Consumer apps, food & beverage, creator communities, social apps, and Gen-Z brands.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["dopamine", "energetic", "saturated", "upbeat", "joyful", "vibrant", "playful", "colorful"],
    },
    StyleCatalogEntry {
        number: 24,
        id: "24-pop-art",
        name: "Pop Art",
        category: "Bold & Avant-Garde",
        definition: "Draws from mid-century pop culture and print graphics: Ben-Day dots, comic framing, bold outlines, and bright primary colors.",
        visual_rules: "Use bold 3px ink outlines, Ben-Day halftone dot patterns, starburst price/action badges, comic-inspired framing, and punchy high-contrast brand colors.",
        best_fit: "Entertainment, snacks/beverages, viral campaigns, merch stores, and playful communities.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["pop art", "halftone", "ben-day", "warhol", "lichtenstein", "comic", "bold outlines"],
    },
    StyleCatalogEntry {
        number: 25,
        id: "25-high-contrast-design",
        name: "High-contrast Design",
        category: "Bold & Avant-Garde",
        definition: "Uses strong light-dark or color opposition for visual drama, clarity, and emphasis.",
        visual_rules: "Maximize luminance contrast (AAA 7:1+ legibility), pairing stark dark/light blocks with razor-sharp borders, unmistakable focus rings, and ultra-crisp typographic hierarchy.",
        best_fit: "Accessible public platforms, developer tools, sports brands, and high-clarity utilities.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["high-contrast", "contrast", "accessible", "wcag", "stark", "bold", "legible", "clarity"],
    },
    StyleCatalogEntry {
        number: 26,
        id: "26-color-blocking",
        name: "Color Blocking",
        category: "Bold & Avant-Garde",
        definition: "Divides the layout into large, distinct fields of solid color to organize content and create bold contrast.",
        visual_rules: "Partition sections and bento cards into full-bleed solid color compartments drawn from the user's brand palette, with crisp edge-to-edge alignment and high-contrast text.",
        best_fit: "Design studios, conferences, modern ecommerce, creative directories, and bold portfolios.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["color blocking", "color blocks", "compartments", "mondrian", "bold sections", "solid fields"],
    },
    StyleCatalogEntry {
        number: 27,
        id: "27-aurora-mesh-gradients",
        name: "Aurora / Mesh Gradients",
        category: "Bold & Avant-Garde",
        definition: "Uses soft, multi-color blended gradients to create atmospheric backgrounds and luminous accents.",
        visual_rules: "Weave smooth multi-point mesh gradients using the user's primary and accent brand hues as luminous atmospheric backdrops beneath crisp high-contrast UI cards.",
        best_fit: "AI products, fintech, cloud platforms, music/creative apps, and modern SaaS.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["aurora", "mesh gradient", "gradients", "luminous", "atmospheric", "stripe", "ai", "glow"],
    },
    StyleCatalogEntry {
        number: 28,
        id: "28-bauhaus",
        name: "Bauhaus",
        category: "Geometric & Historic",
        definition: "Inspired by the early-20th-century design school: geometric shapes, primary colors, asymmetric balance, and functional typography.",
        visual_rules: "Combine elemental geometric primitives (circles, triangles, strict rectangles), architectural grid rules, asymmetric balance, and geometric sans-serif typography.",
        best_fit: "Architecture, design schools, museums, modern furniture, and cultural platforms.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["bauhaus", "geometric", "modernist", "constructivist", "elementary shapes", "functional"],
    },
    StyleCatalogEntry {
        number: 29,
        id: "29-art-deco",
        name: "Art Deco",
        category: "Geometric & Historic",
        definition: "Uses symmetry, geometric ornament, metallic accents, stepped forms, and decorative elegance inspired by 1920s–1930s design.",
        visual_rules: "Frame content with symmetrical stepped geometric borders, sunburst motifs, fine metallic/brand-accent hairlines, and tall poised display typography.",
        best_fit: "Luxury hospitality, cocktail/spirits brands, cinema clubs, jewelry, and gala events.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["art deco", "gatsby", "symmetrical", "stepped border", "gold", "ornamental", "1920s", "theatre"],
    },
    StyleCatalogEntry {
        number: 30,
        id: "30-memphis",
        name: "Memphis",
        category: "Geometric & Historic",
        definition: "Inspired by the 1980s Memphis Group: squiggles, terrazzo-like patterns, geometric shapes, and clashing bright colors.",
        visual_rules: "Accent structured cards with playful geometricconfetti, squiggle dividers, polka-dot/terrazzo fills, hard offset shadows, and upbeat 80s geometric energy.",
        best_fit: "Creative agencies, youth education, playful SaaS, events, and lifestyle products.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["memphis", "80s geometric", "squiggles", "terrazzo", "confetti", "postmodern", "playful"],
    },
    StyleCatalogEntry {
        number: 31,
        id: "31-retro-vintage",
        name: "Retro / Vintage",
        category: "Retro & Futuristic",
        definition: "Borrows visual cues from past decades, such as aged paper tones, classic typography, print textures, or old signage.",
        visual_rules: "Use warm archival paper tones, vintage badge/stamp borders, expressive period display lettering, subtle halftone/print grain, and classic ticket or label frames.",
        best_fit: "Craft food & beverage, coffee roasters, heritage apparel, barber/tattoo studios, and record shops.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["retro", "vintage", "heritage", "nostalgic", "aged paper", "badge", "classic", "americana"],
    },
    StyleCatalogEntry {
        number: 32,
        id: "32-y2k",
        name: "Y2K",
        category: "Retro & Futuristic",
        definition: "Draws from late-1990s and early-2000s digital culture: metallic gradients, chrome, bubbly type, stars, stickers, and techno-optimism.",
        visual_rules: "Use liquid-chrome 3D bevels, translucent polycarbonate panels, four-point sparkle stars, pixel/techno badges, and futuristic early-2000s UI chrome.",
        best_fit: "Gen-Z fashion, music artists, streetwear, social apps, and pop-culture platforms.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["y2k", "2000s", "chrome", "metallic", "cyber y2k", "bubbly", "sparkles", "futuristic pop"],
    },
    StyleCatalogEntry {
        number: 33,
        id: "33-frutiger-aero",
        name: "Frutiger Aero",
        category: "Retro & Futuristic",
        definition: "Inspired by mid-2000s interfaces: glossy buttons, translucent panels, sky and water imagery, lens flares, and optimistic tech themes.",
        visual_rules: "Render aqua-gloss gel buttons with top specular shine, translucent glass frames, fresh sky/water/nature optimism, and smooth humanistic sans typography.",
        best_fit: "Consumer tech, nostalgic communities, eco-tech, and refreshing digital experiences.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["frutiger aero", "glossy", "aqua", "aero glass", "2000s ui", "water", "optimistic tech"],
    },
    StyleCatalogEntry {
        number: 34,
        id: "34-retrofuturism",
        name: "Retrofuturism",
        category: "Retro & Futuristic",
        definition: "Imagines the future through the visual language of an earlier era, such as space-age graphics or vintage computer interfaces.",
        visual_rules: "Blend Apollo-era space-age control panels, warm console bezels, CRT amber/orange telemetry readouts, orbital vector diagrams, and technical stencil typography.",
        best_fit: "AI labs, hardware startups, sci-fi media, developer tools, and synth/audio products.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["retrofuturism", "space age", "cassette futurism", "apollo", "crt", "telemetry", "atomic"],
    },
    StyleCatalogEntry {
        number: 35,
        id: "35-synthwave-outrun",
        name: "Synthwave / Outrun",
        category: "Retro & Futuristic",
        definition: "Uses neon pinks and blues, glowing grids, sunset gradients, and imagery inspired by 1980s electronic culture.",
        visual_rules: "Render glowing perspective grid lines, neon bloom borders, segmented sunset gradients, and sleek 1980s arcade/synth display typography.",
        best_fit: "Gaming, synth music, night-life events, arcade platforms, and retro-tech brands.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["synthwave", "outrun", "retrowave", "80s neon", "laser grid", "arcade", "neon"],
    },
    StyleCatalogEntry {
        number: 36,
        id: "36-vaporwave",
        name: "Vaporwave",
        category: "Retro & Futuristic",
        definition: "Mixes pastel colors, classical statues, old computer windows, surreal collages, and nostalgic consumer imagery.",
        visual_rules: "Combine nostalgic OS window frames (beveled title bars), pastel sunset gradients, checkerboard perspective floors, and surreal digital collage motifs.",
        best_fit: "Net-art, indie music labels, digital fashion, and nostalgic internet communities.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["vaporwave", "pastel cyber", "win95", "checkerboard", "nostalgic", "surreal", "aesthetic"],
    },
    StyleCatalogEntry {
        number: 37,
        id: "37-cyberpunk",
        name: "Cyberpunk",
        category: "Retro & Futuristic",
        definition: "Combines dark environments, electric colors, glitches, dense interfaces, and industrial imagery to suggest a technological dystopia.",
        visual_rules: "Use chamfered 45-degree notched card corners, high-voltage neon accents over deep carbon surfaces, barcode/telemetry strips, and dense tactical HUD panels.",
        best_fit: "Gaming, cybersecurity, crypto/Web3, sci-fi communities, and high-octane tech.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["cyberpunk", "chamfered", "neon", "dystopian", "hud", "glitch", "night city", "tech"],
    },
    StyleCatalogEntry {
        number: 38,
        id: "38-sci-fi-hud",
        name: "Sci-fi / HUD",
        category: "Retro & Futuristic",
        definition: "Resembles fictional control panels through technical diagrams, instrument displays, fine lines, data readouts, and targeting graphics.",
        visual_rules: "Construct a precision FUI command deck with 1px vector reticle corner brackets, radial telemetry gauges, live data readouts, and monospace coordinate labels.",
        best_fit: "Aerospace, analytics dashboards, defense/robotics, AI command centers, and sci-fi games.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["sci-fi", "hud", "fui", "telemetry", "control panel", "command deck", "wireframe", "radar"],
    },
    StyleCatalogEntry {
        number: 39,
        id: "39-pixel-art-8-bit",
        name: "Pixel Art / 8-bit",
        category: "Retro & Futuristic",
        definition: "Uses deliberately pixelated illustrations, bitmap typography, and limited palettes inspired by early video games and computers.",
        visual_rules: "Render icons, badges, and stepped card borders on a crisp integer pixel grid with bitmap display headers, RPG dialogue frames, and readable body text.",
        best_fit: "Indie games, retro communities, pixel art marketplaces, and gamified learning.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["pixel art", "8-bit", "16-bit", "bitmap", "arcade", "retro game", "sprite"],
    },
    StyleCatalogEntry {
        number: 40,
        id: "40-grunge",
        name: "Grunge",
        category: "Expressive & Narrative",
        definition: "Uses distressed textures, scratches, rough edges, ink effects, and imperfect typography to create a worn or rebellious appearance.",
        visual_rules: "Apply raw photocopy grain, torn paper/tape edges, ink stamp badges, halftone imagery, and bold stencil/condensed display typography.",
        best_fit: "Streetwear, rock/indie music, skate brands, zines, and alternative culture.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["grunge", "distressed", "zine", "halftone", "torn paper", "ink", "skate", "rebellious"],
    },
    StyleCatalogEntry {
        number: 41,
        id: "41-psychedelic",
        name: "Psychedelic",
        category: "Expressive & Narrative",
        definition: "Features warped lettering, swirling patterns, saturated colors, and surreal imagery inspired by psychedelic art.",
        visual_rules: "Use fluid wavy section dividers, expressive display lettering, kaleidoscopic color harmony in the user's brand palette, and imaginative surreal spot art.",
        best_fit: "Music festivals, creative beverages, art experiences, and expressive lifestyle brands.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["psychedelic", "groovy", "wavy", "liquid type", "surreal", "festival", "kaleidoscope"],
    },
    StyleCatalogEntry {
        number: 42,
        id: "42-collage-scrapbook",
        name: "Collage / Scrapbook",
        category: "Expressive & Narrative",
        definition: "Combines cutout photographs, paper textures, stickers, handwritten notes, and overlapping fragments into layered compositions.",
        visual_rules: "Frame content like a tactile moodboard with paper-rimmed photo cutouts, washi-tape strips, pinned polaroid cards, sticker badges, and handwritten annotations.",
        best_fit: "Creator portfolios, personal link-in-bio pages, fan clubs, fashion lookbooks, and creative journals.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["collage", "scrapbook", "cutout", "stickers", "polaroid", "washi tape", "moodboard", "zine"],
    },
    StyleCatalogEntry {
        number: 43,
        id: "43-handmade-hand-drawn",
        name: "Handmade / Hand-drawn",
        category: "Expressive & Narrative",
        definition: "Uses sketches, doodles, uneven lines, and handwritten lettering to make the interface feel personal and human.",
        visual_rules: "Incorporate organic hand-inked borders, sketchy arrow callouts, marker highlight accents, warm paper surfaces, and personal human spot illustrations.",
        best_fit: "Indie makers, bakeries/cafes, personal blogs, children's brands, and creative tools.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["handmade", "hand-drawn", "sketch", "doodle", "human", "personal", "indie", "organic"],
    },
    StyleCatalogEntry {
        number: 44,
        id: "44-illustration-led",
        name: "Illustration-led",
        category: "Expressive & Narrative",
        definition: "Makes custom illustration the central visual language for explaining concepts, expressing personality, and connecting sections.",
        visual_rules: "Anchor the hero and feature sections with cohesive custom editorial/brand illustrations that visually explain workflows and give the product a memorable character.",
        best_fit: "SaaS onboarding, fintech, education, community directories, and friendly consumer platforms.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["illustration-led", "illustration", "custom art", "vector art", "storytelling", "brand characters"],
    },
    StyleCatalogEntry {
        number: 45,
        id: "45-cartoon-comic",
        name: "Cartoon / Comic",
        category: "Expressive & Narrative",
        definition: "Uses expressive characters, bold outlines, speech bubbles, panel structures, and exaggerated visuals inspired by cartoons and comics.",
        visual_rules: "Structure sections into crisp comic-strip panels with 3px ink borders, speech-bubble callouts, halftone accents, and expressive character artwork.",
        best_fit: "Webcomics, gaming communities, playful food/merch brands, and entertainment.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["cartoon", "comic", "panels", "speech bubble", "cel-shaded", "bold outlines", "manga"],
    },
    StyleCatalogEntry {
        number: 46,
        id: "46-kawaii-cute",
        name: "Kawaii / Cute",
        category: "Expressive & Narrative",
        definition: "Uses rounded forms, cheerful characters, gentle colors, and playful details to create a friendly, adorable identity.",
        visual_rules: "Use puffy 28px+ pill corners, soft pastel surfaces paired with user brand colors, charming mascot/sparkle badges, and bouncy rounded typography.",
        best_fit: "Cute stationery/plush shops, anime/vtuber profiles, cozy apps, and friendly communities.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["kawaii", "cute", "pastel", "adorable", "mascot", "chibi", "bubbly", "rounded"],
    },
    StyleCatalogEntry {
        number: 47,
        id: "47-organic-nature-inspired",
        name: "Organic / Nature-inspired",
        category: "Expressive & Narrative",
        definition: "Uses flowing shapes, botanical imagery, earthy colors, and natural textures rather than rigid, mechanical forms.",
        visual_rules: "Use soft organic arch/pebble masks, curved section dividers, botanical accents, and grounded earthy harmony adapted to the user's brand palette.",
        best_fit: "Sustainability, eco-products, agriculture, skincare, wellness retreats, and outdoor brands.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["organic", "nature", "biophilic", "botanical", "earthy", "sustainable", "wellness", "eco"],
    },
    StyleCatalogEntry {
        number: 48,
        id: "48-wabi-sabi-inspired",
        name: "Wabi-sabi-inspired",
        category: "Expressive & Narrative",
        definition: "Emphasizes quiet simplicity, asymmetry, weathered textures, and visible imperfections borrowed from the Japanese aesthetic tradition.",
        visual_rules: "Use tactile unbleached paper/clay textures, poetic asymmetrical spacing, understated serif typography, and calm contemplative visual pacing.",
        best_fit: "Ceramics, tea, slow fashion, architecture, and meditative cultural projects.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["wabi-sabi", "imperfection", "asymmetry", "ceramic", "quiet", "meditative", "patina"],
    },
    StyleCatalogEntry {
        number: 49,
        id: "49-photography-led",
        name: "Photography-led",
        category: "Expressive & Narrative",
        definition: "Makes photography the main source of visual impact, with layout and typography supporting the images.",
        visual_rules: "Dedicate 70%+ of visual weight to high-impact photography with clean gradient scrims, generous gallery framing, and restrained overlay typography.",
        best_fit: "Travel, real estate, fashion lookbooks, portfolios, hospitality, and visual directories.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["photography-led", "photography", "full-bleed", "gallery", "portfolio", "visual"],
    },
    StyleCatalogEntry {
        number: 50,
        id: "50-cinematic",
        name: "Cinematic",
        category: "Expressive & Narrative",
        definition: "Uses dramatic imagery, film-like composition, video, lighting, and paced transitions to create a narrative atmosphere.",
        visual_rules: "Frame sections with widescreen theatrical pacing, chiaroscuro lighting depth, subtle film grain, and dramatic spotlight hierarchy.",
        best_fit: "Film/media studios, luxury automotive, game launches, and narrative product reveals.",
        essential_screens: GENERAL_WEB_SCREENS,
        keywords: &["cinematic", "film", "widescreen", "dramatic lighting", "atmospheric", "theatrical", "storytelling"],
    },

    // =========================================================================
    // CATEGORY 1: MARKETPLACE (#51 - #60)
    // =========================================================================
    StyleCatalogEntry {
        number: 51,
        id: "51-marketplace-minimal-utility",
        name: "Marketplace — Minimal Utility",
        category: "Marketplace",
        definition: "A restrained interface with neutral surfaces, clear sans-serif type, compact filters, and consistent listing cards. Product images, prices, seller names, and ratings form an easy-to-scan hierarchy.",
        visual_rules: "Build a high-clarity multi-seller discovery layout using the user's brand palette: prominent search + faceted filter bar, category pill strip, and standardized listing cards displaying thumbnail, title, price/status, verified seller avatar/name, and star rating in a scannable grid.",
        best_fit: "Broad product catalogues, service directories, creator/freelancer directories, and frequent comparison marketplaces.",
        essential_screens: MARKETPLACE_ESSENTIAL_SCREENS,
        keywords: &["marketplace", "minimal utility", "directory", "multi-seller", "listings", "search and filters", "comparison", "sellers", "catalogue", "profiles", "ratings"],
    },
    StyleCatalogEntry {
        number: 52,
        id: "52-marketplace-editorial-craft",
        name: "Marketplace — Editorial Craft",
        category: "Marketplace",
        definition: "A magazine-like presentation with warm backgrounds, serif headlines, close-up photography, and spacious feature stories. Seller profiles explain the people, materials, and processes behind the products.",
        visual_rules: "Combine warm editorial storytelling with marketplace discovery: pair serif headlines and featured maker/creator spotlight stories (highlighting biography, craft process, and provenance) alongside curated listing grids and seller profile cards in the user's brand colors.",
        best_fit: "Handmade goods, independent makers, creator portfolios/directories, and curated vintage marketplaces.",
        essential_screens: MARKETPLACE_ESSENTIAL_SCREENS,
        keywords: &["marketplace", "editorial craft", "handmade", "makers", "artisans", "vintage", "curated", "seller profiles", "creator directory", "storytelling"],
    },
    StyleCatalogEntry {
        number: 53,
        id: "53-marketplace-neobrutalist-creative",
        name: "Marketplace — Neobrutalist Creative",
        category: "Marketplace",
        definition: "Thick borders, solid offset shadows, bold headings, and bright accent colors create a playful identity. Listing cards remain consistent while seller badges and category graphics add personality.",
        visual_rules: "Frame marketplace listing cards, search bars, and category tiles with crisp 2px–3px dark borders and solid offset shadows (`4px 4px 0px`), using the user's brand primary/accent colors for seller flair badges, featured placement banners, and category pills while keeping listing metadata (title, creator, price/links, rating) strictly aligned.",
        best_fit: "Creative assets, digital goods, indie creator directories, link-in-bio discovery platforms, unusual goods, and independent brands.",
        essential_screens: MARKETPLACE_ESSENTIAL_SCREENS,
        keywords: &["marketplace", "neobrutalist creative", "creative assets", "indie", "creators", "link-in-bio", "directory", "gumroad", "playful", "badges", "banners"],
    },
    StyleCatalogEntry {
        number: 54,
        id: "54-marketplace-swiss-catalogue",
        name: "Marketplace — Swiss Catalogue",
        category: "Marketplace",
        definition: "A strict grid, strong alignment, restrained colors, and precise typography organize large amounts of information. Structured specification blocks make differences between listings easy to identify.",
        visual_rules: "Organize listings inside an exposed 1px structural Swiss grid with flush-left typography, structured key-value specification blocks on every card, tabular comparison alignment, and crisp brand-accent active filter states.",
        best_fit: "Equipment, specialist products, B2B marketplaces, technical directories, and business purchasing.",
        essential_screens: MARKETPLACE_ESSENTIAL_SCREENS,
        keywords: &["marketplace", "swiss catalogue", "b2b", "equipment", "specifications", "structured grid", "comparison", "specialist", "directory"],
    },
    StyleCatalogEntry {
        number: 55,
        id: "55-marketplace-luxury-gallery",
        name: "Marketplace — Luxury Gallery",
        category: "Marketplace",
        definition: "Large photographs, refined typography, quiet colors, and generous spacing create a selective gallery atmosphere. Provenance, condition, and seller credentials receive prominent placement.",
        visual_rules: "Present marketplace listings like a curated auction house or private gallery: generous whitespace, high-contrast serif titles, large uncropped imagery, prominent authentication/provenance badges, condition grades, and verified dealer/seller credentials.",
        best_fit: "Art, collectible furniture, watches, luxury real estate, and high-end resale marketplaces.",
        essential_screens: MARKETPLACE_ESSENTIAL_SCREENS,
        keywords: &["marketplace", "luxury gallery", "art", "collectibles", "watches", "auction", "resale", "provenance", "curated"],
    },
    StyleCatalogEntry {
        number: 56,
        id: "56-marketplace-organic-community",
        name: "Marketplace — Organic Community",
        category: "Marketplace",
        definition: "Earthy colors, soft shapes, natural photography, and approachable type create a neighborhood feeling. Locality labels, pickup information, and seller introductions connect the visual identity to people and place.",
        visual_rules: "Use warm approachable rounded cards (16px–24px radius), friendly typography, neighborhood/location distance pills, local pickup or availability tags, and warm seller/member introduction cards styled in the user's brand palette.",
        best_fit: "Local produce, sustainable goods, community exchanges, local services, and neighborhood directories.",
        essential_screens: MARKETPLACE_ESSENTIAL_SCREENS,
        keywords: &["marketplace", "organic community", "local", "community", "sustainable", "neighborhood", "farmers", "services", "directory", "people"],
    },
    StyleCatalogEntry {
        number: 57,
        id: "57-marketplace-industrial-technical",
        name: "Marketplace — Industrial Technical",
        category: "Marketplace",
        definition: "Squared components, utilitarian typography, muted gray surfaces, and diagram-style imagery emphasize practical detail. Specification tables, condition labels, and documentation anchors shape the pages.",
        visual_rules: "Use squared heavy-duty card borders, monospace part numbers and spec tables, schematic/diagram previews, condition/stock badges, and high-contrast action buttons.",
        best_fit: "Tools, machinery, electronic components, automotive parts, and trade supplies.",
        essential_screens: MARKETPLACE_ESSENTIAL_SCREENS,
        keywords: &["marketplace", "industrial technical", "tools", "machinery", "parts", "hardware", "trade", "specs"],
    },
    StyleCatalogEntry {
        number: 58,
        id: "58-marketplace-streetwear-collage",
        name: "Marketplace — Streetwear Collage",
        category: "Marketplace",
        definition: "Oversized typography, cutout product images, stickers, and layered photography create a fashion-culture aesthetic. Size, condition, seller identity, and price stay in predictable positions within expressive layouts.",
        visual_rules: "Pair bold condensed culture typography, cutout product silhouettes, and drop/hype stickers with disciplined listing card footers that always keep size, condition, verified seller handle, and price in predictable, scannable positions.",
        best_fit: "Sneakers, streetwear, vintage apparel, Depop/Grailed-style fashion resale, and youth marketplaces.",
        essential_screens: MARKETPLACE_ESSENTIAL_SCREENS,
        keywords: &["marketplace", "streetwear collage", "sneakers", "streetwear", "fashion resale", "depop", "grailed", "drops", "youth"],
    },
    StyleCatalogEntry {
        number: 59,
        id: "59-marketplace-futuristic-digital",
        name: "Marketplace — Futuristic Digital",
        category: "Marketplace",
        definition: "Dark surfaces, controlled neon accents, fine technical lines, and animated preview areas suggest a digital studio. Compatibility, file formats, and license information use compact, readable detail panels.",
        visual_rules: "Use sleek dark studio surfaces with luminous brand-colored borders, interactive preview frames, and compact technical metadata pills for file formats, engine compatibility, version, license tier, and creator rating.",
        best_fit: "Digital assets, 3D models, game resources, UI templates, prompts, plugins, and software tools.",
        essential_screens: MARKETPLACE_ESSENTIAL_SCREENS,
        keywords: &["marketplace", "futuristic digital", "digital assets", "templates", "3d models", "plugins", "software", "game assets", "studio"],
    },
    StyleCatalogEntry {
        number: 60,
        id: "60-marketplace-soft-illustrated",
        name: "Marketplace — Soft Illustrated",
        category: "Marketplace",
        definition: "Rounded cards, friendly illustrations, gentle colors, and simple icons make discovery approachable. Illustrated category tiles and consistent seller summaries give a varied catalogue a coherent identity.",
        visual_rules: "Use friendly rounded 20px–28px cards, custom spot-illustrated category tiles, approachable icons, clear step-by-step 'How It Works' graphics, and warm seller/creator summary cards using the user's brand palette.",
        best_fit: "Everyday services, family products, creator/community directories, link-in-bio platforms, and beginner-friendly marketplaces.",
        essential_screens: MARKETPLACE_ESSENTIAL_SCREENS,
        keywords: &["marketplace", "soft illustrated", "friendly", "services", "family", "approachable", "illustrated categories", "directory", "creators", "link-in-bio"],
    },

    // =========================================================================
    // CATEGORY 2: DROPSHIPPING STORE (#61 - #70)
    // =========================================================================
    StyleCatalogEntry {
        number: 61,
        id: "61-dropshipping-modern-product-spotlight",
        name: "Dropshipping — Modern Product Spotlight",
        category: "Dropshipping Store",
        definition: "A clean layout with one strong accent color, large product imagery, clear headings, and compact benefit sections. The main product appears consistently across the hero, demonstrations, and purchase area.",
        visual_rules: "Center the entire page hierarchy on the flagship product: hero product showcase with instant benefit bullets, variant/bundle selector, trust/shipping guarantee badges, before/after or feature breakdown cards, and a high-contrast sticky/anchored purchase CTA in the user's brand primary color.",
        best_fit: "One flagship product, hero DTC offer, or a small curated product family.",
        essential_screens: DROPSHIPPING_ESSENTIAL_SCREENS,
        keywords: &["dropshipping", "product spotlight", "single product", "flagship", "dtc", "conversion", "funnel", "benefits", "variants"],
    },
    StyleCatalogEntry {
        number: 62,
        id: "62-dropshipping-technical-demonstrator",
        name: "Dropshipping — Technical Demonstrator",
        category: "Dropshipping Store",
        definition: "Precise typography, close-up renders, specification diagrams, and orderly comparison blocks make the product's operation visible. Neutral backgrounds and restrained accents create an engineering-oriented visual identity.",
        visual_rules: "Showcase how the product works through macro close-up renders, annotated engineering callout lines, side-by-side 'Us vs. Others' comparison matrices, technical spec tables, and a precision buy box.",
        best_fit: "Gadgets, tech accessories, smart devices, and practical problem-solving tools.",
        essential_screens: DROPSHIPPING_ESSENTIAL_SCREENS,
        keywords: &["dropshipping", "technical demonstrator", "gadgets", "tech", "accessories", "devices", "specs", "comparison", "engineering"],
    },
    StyleCatalogEntry {
        number: 63,
        id: "63-dropshipping-organic-lifestyle",
        name: "Dropshipping — Organic Lifestyle",
        category: "Dropshipping Store",
        definition: "Warm neutrals, botanical accents, natural textures, and photography in everyday settings give the store a calm, tactile identity. Rounded components soften product details and routine-based storytelling.",
        visual_rules: "Frame the product inside a calm daily ritual using warm tactile surfaces, soft 20px rounded benefit cards, ingredient/material transparency badges, step-by-step routine storytelling, and subscription/bundle options.",
        best_fit: "Personal care accessories, skincare, wellness devices, home comfort, and eco lifestyle products.",
        essential_screens: DROPSHIPPING_ESSENTIAL_SCREENS,
        keywords: &["dropshipping", "organic lifestyle", "personal care", "wellness", "home comfort", "routine", "skincare", "natural"],
    },
    StyleCatalogEntry {
        number: 64,
        id: "64-dropshipping-neobrutalist-pop",
        name: "Dropshipping — Neobrutalist Pop",
        category: "Dropshipping Store",
        definition: "Bold color blocks, thick outlines, oversized type, and hard shadows make a simple offer visually distinctive. Short sections and strong product silhouettes carry the design without relying on dense decoration.",
        visual_rules: "Use high-impact color blocking in the user's brand palette, 3px dark borders, hard offset shadows, punchy 3-word benefit headlines, sticker social-proof badges, and an unmistakable tiered bundle selector card.",
        best_fit: "Playful accessories, viral TikTok/social products, snacks/drinks, and visually unusual goods.",
        essential_screens: DROPSHIPPING_ESSENTIAL_SCREENS,
        keywords: &["dropshipping", "neobrutalist pop", "viral", "playful", "accessories", "pop", "bold", "bundles"],
    },
    StyleCatalogEntry {
        number: 65,
        id: "65-dropshipping-editorial-lifestyle",
        name: "Dropshipping — Editorial Lifestyle",
        category: "Dropshipping Store",
        definition: "Magazine-style photography, expressive headline typography, and spacious layouts place products within a desirable daily routine. Product details follow visual stories about use, styling, and context.",
        visual_rules: "Combine magazine-grade serif headlines and aspirational context photography with seamless inline 'Shop the Story' variant cards, styling notes, and a refined slide-over cart/checkout flow.",
        best_fit: "Fashion accessories, eyewear, bags, home decor, and curated lifestyle goods.",
        essential_screens: DROPSHIPPING_ESSENTIAL_SCREENS,
        keywords: &["dropshipping", "editorial lifestyle", "fashion accessories", "home decor", "curated", "lookbook", "lifestyle"],
    },
    StyleCatalogEntry {
        number: 66,
        id: "66-dropshipping-cinematic-demonstration",
        name: "Dropshipping — Cinematic Demonstration",
        category: "Dropshipping Store",
        definition: "Large video or image sequences, dramatic lighting, minimal surrounding decoration, and carefully paced sections show the product in action. Purchase controls remain visually clear beside the storytelling.",
        visual_rules: "Stage dramatic widescreen product-in-action sequences with chiaroscuro studio lighting, step-by-step visual demonstration frames, and a high-contrast floating purchase module with clear delivery/returns info.",
        best_fit: "Lighting products, projectors, audio gear, fitness equipment, and products best understood through visual demonstration.",
        essential_screens: DROPSHIPPING_ESSENTIAL_SCREENS,
        keywords: &["dropshipping", "cinematic demonstration", "video", "demonstration", "dramatic", "in action", "product launch"],
    },
    StyleCatalogEntry {
        number: 67,
        id: "67-dropshipping-luxury-boutique",
        name: "Dropshipping — Luxury Boutique",
        category: "Dropshipping Store",
        definition: "Restrained colors, elegant type, carefully art-directed photography, and generous margins present a small assortment with polish. Packaging, materials, dimensions, and finishing details receive close attention.",
        visual_rules: "Present a focused assortment with atelier polish: refined serif headings, generous margins, macro detail callouts for materials/finishes/packaging, gift-ready presentation notes, and discreet high-trust checkout controls.",
        best_fit: "Premium-positioned accessories, jewelry, leather goods, fragrance, and elevated home decor.",
        essential_screens: DROPSHIPPING_ESSENTIAL_SCREENS,
        keywords: &["dropshipping", "luxury boutique", "premium", "packaging", "jewelry", "decor", "boutique", "craftsmanship"],
    },
    StyleCatalogEntry {
        number: 68,
        id: "68-dropshipping-friendly-illustrated",
        name: "Dropshipping — Friendly Illustrated",
        category: "Dropshipping Store",
        definition: "Soft colors, custom characters, simple diagrams, and rounded typography explain the product through approachable visual stories. Illustrations connect feature explanations, instructions, and support content.",
        visual_rules: "Pair product photography with friendly custom spot illustrations and simple 1-2-3 how-to diagrams, rounded 24px cards, cheerful FAQ accordions, and clear family/pet-friendly trust badges.",
        best_fit: "Pet accessories, baby/family products, household helpers, and playful gifts.",
        essential_screens: DROPSHIPPING_ESSENTIAL_SCREENS,
        keywords: &["dropshipping", "friendly illustrated", "pet", "household", "gifts", "family", "illustrated", "how it works"],
    },
    StyleCatalogEntry {
        number: 69,
        id: "69-dropshipping-retro-novelty",
        name: "Dropshipping — Retro Novelty",
        category: "Dropshipping Store",
        definition: "Vintage display lettering, halftone textures, nostalgic color combinations, and poster-like product compositions give the store a specific period identity. Modern forms and controls keep purchase steps straightforward.",
        visual_rules: "Frame the product in a nostalgic vintage poster aesthetic with retro display typography, badge stamps, and halftone accents, while keeping variant selectors, pricing, shipping timers, and checkout controls modern and frictionless.",
        best_fit: "Nostalgic gifts, novelty items, retro gadgets, games, and themed apparel.",
        essential_screens: DROPSHIPPING_ESSENTIAL_SCREENS,
        keywords: &["dropshipping", "retro novelty", "vintage", "gifts", "novelty", "nostalgic", "poster"],
    },
    StyleCatalogEntry {
        number: 70,
        id: "70-dropshipping-colorful-maximalist",
        name: "Dropshipping — Colorful Maximalist",
        category: "Dropshipping Store",
        definition: "Layered graphics, contrasting backgrounds, expressive typography, and abundant product photography create an energetic store. Repeated card structures and a consistent purchase area keep the visual abundance organized.",
        visual_rules: "Combine high-energy contrasting brand color sections, expressive display type, and starburst review callouts with a strictly consistent, repeatable product card & purchase box layout.",
        best_fit: "Party products, cosmetics, seasonal collections, snacks, and expressive gifts.",
        essential_screens: DROPSHIPPING_ESSENTIAL_SCREENS,
        keywords: &["dropshipping", "colorful maximalist", "party", "seasonal", "gifts", "energetic", "cosmetics", "bundles"],
    },

    // =========================================================================
    // CATEGORY 3: SHOP / BRANDED ECOMMERCE (#71 - #80)
    // =========================================================================
    StyleCatalogEntry {
        number: 71,
        id: "71-shop-minimal-product-gallery",
        name: "Shop / Ecommerce — Minimal Product Gallery",
        category: "Shop / Branded Ecommerce",
        definition: "Quiet backgrounds, restrained typography, generous spacing, and consistent product photography give the catalogue a gallery-like quality. Compact controls let product form and detail lead each page.",
        visual_rules: "Treat every product card like a gallery object: uniform aspect-ratio studio frames, generous grid gutters, minimal typography (title + price + subtle color swatch dots), compact filter drawer, and clean slide-out cart.",
        best_fit: "Design objects, minimalist apparel, tech accessories, and carefully curated brand catalogues.",
        essential_screens: SHOP_ESSENTIAL_SCREENS,
        keywords: &["shop", "ecommerce", "store", "minimal product gallery", "catalogue", "design objects", "curated", "clean"],
    },
    StyleCatalogEntry {
        number: 72,
        id: "72-shop-fashion-editorial",
        name: "Shop / Ecommerce — Fashion Editorial",
        category: "Shop / Branded Ecommerce",
        definition: "Large campaign photography, dramatic type scale, asymmetric compositions, and magazine-like stories connect products to a visual world. Collection pages alternate editorial moments with orderly shopping grids.",
        visual_rules: "Alternate full-bleed campaign lookbook hero moments and asymmetric editorial story blocks with disciplined 3- or 4-column product collection grids, quick-size hover selectors, and crisp serif/sans pairings.",
        best_fit: "Fashion brands, apparel, jewelry, eyewear, and seasonal campaign stores.",
        essential_screens: SHOP_ESSENTIAL_SCREENS,
        keywords: &["shop", "ecommerce", "fashion editorial", "apparel", "clothing", "lookbook", "campaign", "collection"],
    },
    StyleCatalogEntry {
        number: 73,
        id: "73-shop-luxury-couture",
        name: "Shop / Ecommerce — Luxury Couture",
        category: "Shop / Branded Ecommerce",
        definition: "Refined serif and sans-serif pairings, controlled contrasts, subtle motion, and close-up detail photography emphasize craftsmanship. Product pages use deliberate pacing and a restrained amount of visual decoration.",
        visual_rules: "Use couture serif headlines, wide-tracked uppercase category navigation, macro craftsmanship photography, discreet availability/concierge indicators, and an elevated atelier bag & checkout experience.",
        best_fit: "Luxury fashion, fine jewelry, haute fragrance, watches, and prestige leather goods.",
        essential_screens: SHOP_ESSENTIAL_SCREENS,
        keywords: &["shop", "ecommerce", "luxury couture", "fine jewelry", "fragrance", "maison", "prestige", "atelier"],
    },
    StyleCatalogEntry {
        number: 74,
        id: "74-shop-scandinavian-home",
        name: "Shop / Ecommerce — Scandinavian Home",
        category: "Shop / Branded Ecommerce",
        definition: "Light warm surfaces, muted colors, practical typography, and natural interior photography create a calm domestic atmosphere. Material swatches and dimension diagrams fit naturally into the presentation.",
        visual_rules: "Pair sunlit room-context photography with clean studio silhouettes, interactive fabric/wood material swatches, clear architectural dimension diagrams, delivery lead-time badges, and calm Nordic spacing.",
        best_fit: "Furniture, lighting, textiles, kitchenware, and homeware brands.",
        essential_screens: SHOP_ESSENTIAL_SCREENS,
        keywords: &["shop", "ecommerce", "scandinavian home", "furniture", "homeware", "interior", "lighting", "textiles"],
    },
    StyleCatalogEntry {
        number: 75,
        id: "75-shop-artisan-botanical",
        name: "Shop / Ecommerce — Artisan Botanical",
        category: "Shop / Branded Ecommerce",
        definition: "Earthy palettes, paper-like textures, botanical drawings, and tactile close-ups connect the store to making and materials. Small ornamental details support an otherwise clear product hierarchy.",
        visual_rules: "Use warm paper-toned surfaces, fine botanical line-art accents, apothecary-style ingredient/origin labels, tactile close-up textures, and clear collection filtering.",
        best_fit: "Ceramics, specialty tea/coffee, candles, natural skincare, stationery, and artisan goods.",
        essential_screens: SHOP_ESSENTIAL_SCREENS,
        keywords: &["shop", "ecommerce", "artisan botanical", "ceramics", "tea", "candles", "apothecary", "skincare", "stationery"],
    },
    StyleCatalogEntry {
        number: 76,
        id: "76-shop-industrial-utility",
        name: "Shop / Ecommerce — Industrial Utility",
        category: "Shop / Branded Ecommerce",
        definition: "Robust typography, squared layouts, high-contrast controls, and practical product photography communicate functional purpose. Technical specifications and comparison tables are central visual elements.",
        visual_rules: "Use rugged squared grids, bold condensed headings, technical specification matrices (materials, weight, ratings), compatibility filters, and high-visibility add-to-cart controls.",
        best_fit: "Tools, workwear, outdoor/camping equipment, cycling/overland gear, and hardware.",
        essential_screens: SHOP_ESSENTIAL_SCREENS,
        keywords: &["shop", "ecommerce", "industrial utility", "outdoor gear", "workwear", "tools", "hardware", "rugged"],
    },
    StyleCatalogEntry {
        number: 77,
        id: "77-shop-bauhaus-geometric",
        name: "Shop / Ecommerce — Bauhaus Geometric",
        category: "Shop / Branded Ecommerce",
        definition: "Primary-color accents, simple geometric shapes, strong alignment, and bold typography create a graphic, design-conscious identity. Product grids use disciplined structure with occasional playful compositions.",
        visual_rules: "Structure the store on a crisp geometric grid with bold brand-color blocks, circular/triangular category badges, architectural alignment, and graphic typography.",
        best_fit: "Design goods, modern furniture, art prints, books, museum shops, and stationery.",
        essential_screens: SHOP_ESSENTIAL_SCREENS,
        keywords: &["shop", "ecommerce", "bauhaus geometric", "design shop", "prints", "stationery", "museum store", "geometric"],
    },
    StyleCatalogEntry {
        number: 78,
        id: "78-shop-retro-hobby-shop",
        name: "Shop / Ecommerce — Retro Hobby Shop",
        category: "Shop / Branded Ecommerce",
        definition: "Period-inspired graphics, nostalgic display fonts, illustrated labels, and warm or arcade-like colors evoke a particular hobby culture. Standardized product cards keep collectibles and variations easy to browse.",
        visual_rules: "Pair nostalgic hobby badge headers and illustrated category banners with standardized collectible product cards showing edition/condition, stock count, release series, and quick pre-order/cart actions.",
        best_fit: "Vinyl records, comics, tabletop games, trading cards, toys, and collectibles.",
        essential_screens: SHOP_ESSENTIAL_SCREENS,
        keywords: &["shop", "ecommerce", "retro hobby shop", "records", "vinyl", "comics", "toys", "collectibles", "tabletop"],
    },
    StyleCatalogEntry {
        number: 79,
        id: "79-shop-maximalist-concept-store",
        name: "Shop / Ecommerce — Maximalist Concept Store",
        category: "Shop / Branded Ecommerce",
        definition: "Expressive color, mixed typography, patterns, collage, and unconventional photography make browsing feel like exploring a curated exhibition. Repeated navigation and product-card conventions connect the varied sections.",
        visual_rules: "Create an exhibition-like browsing experience with expressive color combinations, mixed display fonts, and curated thematic rooms while keeping navigation, price tags, and product cards consistent.",
        best_fit: "Eclectic fashion, street-culture boutiques, art objects, and experimental lifestyle brands.",
        essential_screens: SHOP_ESSENTIAL_SCREENS,
        keywords: &["shop", "ecommerce", "maximalist concept store", "concept store", "eclectic", "boutique", "exhibition"],
    },
    StyleCatalogEntry {
        number: 80,
        id: "80-shop-bold-athletic",
        name: "Shop / Ecommerce — Bold Athletic",
        category: "Shop / Branded Ecommerce",
        definition: "Large condensed headlines, dynamic photography, sharp color contrasts, and directional graphics create a sense of movement. Product pages foreground fit, performance details, and clear variant selection.",
        visual_rules: "Use high-energy condensed headlines, dynamic action photography, diagonal/kinetic accents, performance metric badges (breathability, weight, support), and fast size/color matrix selection.",
        best_fit: "Sportswear, athletic footwear, gym/training equipment, supplements, and active lifestyle brands.",
        essential_screens: SHOP_ESSENTIAL_SCREENS,
        keywords: &["shop", "ecommerce", "bold athletic", "sportswear", "fitness", "activewear", "running", "training", "athletic"],
    },

    // =========================================================================
    // CATEGORY 4: SOCIAL MEDIA / COMMUNITY PLATFORM (#81 - #90)
    // =========================================================================
    StyleCatalogEntry {
        number: 81,
        id: "81-social-minimal-social-utility",
        name: "Social / Community — Minimal Social Utility",
        category: "Social Media / Community",
        definition: "Neutral surfaces, restrained accents, clear type, and predictable post structures make content easy to scan. Visual weight belongs to posts and conversations, with navigation and controls kept consistent.",
        visual_rules: "Organize feeds, profiles, and discussions with calm neutral surfaces, 1px subtle dividers, clear author avatar + handle headers, predictable action bars (reply, repost, bookmark, share), and distraction-free reading hierarchy.",
        best_fit: "General communities, microblogging, Q&A platforms, and frequent daily interaction.",
        essential_screens: SOCIAL_ESSENTIAL_SCREENS,
        keywords: &["social", "community", "minimal social utility", "feed", "posts", "profiles", "discussion", "network"],
    },
    StyleCatalogEntry {
        number: 82,
        id: "82-social-editorial-publishing",
        name: "Social / Community — Editorial Publishing",
        category: "Social Media / Community",
        definition: "Readable text columns, serif or expressive headline typography, generous spacing, and large article imagery give the feed a publication-like character. Author identity and topic structure connect posts.",
        visual_rules: "Combine literary serif headlines, comfortable 65ch reading columns, author bio & subscribe cards, inline reader highlights/comments, and curated topic channels.",
        best_fit: "Essays, newsletters, Substack/Medium-style writing platforms, and thoughtful discussion communities.",
        essential_screens: SOCIAL_ESSENTIAL_SCREENS,
        keywords: &["social", "community", "editorial publishing", "newsletter", "essays", "writing", "blogging", "authors"],
    },
    StyleCatalogEntry {
        number: 83,
        id: "83-social-photographic-gallery",
        name: "Social / Community — Photographic Gallery",
        category: "Social Media / Community",
        definition: "Large image areas, understated chrome, precise cropping, and carefully spaced grids make visual work the focus. Profiles resemble portfolios, while captions and conversation appear in supporting panels.",
        visual_rules: "Let visual work dominate via clean masonry/aspect-ratio grids and understated UI chrome; structure profiles as curated portfolios with collections, gear/process tags, and supporting appreciation/comment drawers.",
        best_fit: "Photography, visual art, architecture, travel sharing, and creative portfolio directories.",
        essential_screens: SOCIAL_ESSENTIAL_SCREENS,
        keywords: &["social", "community", "photographic gallery", "portfolio", "photography", "visual art", "creators", "gallery"],
    },
    StyleCatalogEntry {
        number: 84,
        id: "84-social-creator-pop",
        name: "Social / Community — Creator Pop",
        category: "Social Media / Community",
        definition: "Bright accent colors, expressive avatars, playful badges, and bold profile headers give creators a recognizable presence. Post cards remain consistent while profiles provide controlled room for personalization.",
        visual_rules: "Give creators a vibrant, customizable stage using the user's brand palette: expressive profile hero cards (avatar, bio, category tags, social links, featured projects, and view/follower badges), searchable discovery directory grids, and discreet sponsored/banner slots.",
        best_fit: "Link-in-bio platforms, creator discovery directories, fan communities, talent hubs, and entertainment networks.",
        essential_screens: SOCIAL_ESSENTIAL_SCREENS,
        keywords: &["social", "community", "creator pop", "link-in-bio", "creators", "profiles", "directory", "discover", "bio page", "portfolio", "influencers"],
    },
    StyleCatalogEntry {
        number: 85,
        id: "85-social-neobrutalist-community",
        name: "Social / Community — Neobrutalist Community",
        category: "Social Media / Community",
        definition: "Thick outlines, hard shadows, strong type, and colorful blocks create an informal, outspoken identity. Polls, topic labels, and post actions become distinctive components with clear visual states.",
        visual_rules: "Use 2px–3px ink borders, hard offset shadows, bold topic tags, interactive poll/upvote modules, creator showcase cards, and high-contrast active states in the user's brand palette.",
        best_fit: "Design communities, indie hackers, student networks, creator directories, and collaborative clubs.",
        essential_screens: SOCIAL_ESSENTIAL_SCREENS,
        keywords: &["social", "community", "neobrutalist community", "indie", "polls", "upvotes", "creators", "design community", "directory"],
    },
    StyleCatalogEntry {
        number: 86,
        id: "86-social-retro-forum",
        name: "Social / Community — Retro Forum",
        category: "Social Media / Community",
        definition: "Compact rows, visible borders, topic icons, distinctive usernames, and restrained nostalgic accents recall classic message boards. Thread structure, recent activity, and reply counts define the layout.",
        visual_rules: "Structure discussions into high-density tabular category boards and thread rows with visible 1px borders, status icons, user flair/rank badges, reply/view counters, and chronological quote-reply trees.",
        best_fit: "Hobby discussions, technical communities, enthusiast clubs, and knowledge archives.",
        essential_screens: SOCIAL_ESSENTIAL_SCREENS,
        keywords: &["social", "community", "retro forum", "forum", "message board", "threads", "bulletin board", "hobby"],
    },
    StyleCatalogEntry {
        number: 87,
        id: "87-social-cozy-illustrated",
        name: "Social / Community — Cozy Illustrated",
        category: "Social Media / Community",
        definition: "Warm colors, soft illustrations, rounded shapes, and gentle typography create a relaxed social atmosphere. Group spaces and milestones use illustration while conversation text remains straightforward.",
        visual_rules: "Combine warm inviting surfaces, soft 24px rounded cards, custom spot illustrations for club rooms and member milestones, and clear readable conversation threads.",
        best_fit: "Book clubs, crafting groups, language exchange, local neighborhoods, and small interest communities.",
        essential_screens: SOCIAL_ESSENTIAL_SCREENS,
        keywords: &["social", "community", "cozy illustrated", "book club", "crafting", "friendly", "warm", "groups"],
    },
    StyleCatalogEntry {
        number: 88,
        id: "88-social-swiss-professional",
        name: "Social / Community — Swiss Professional",
        category: "Social Media / Community",
        definition: "Precise grids, strong alignment, neutral colors, and disciplined typography organize profiles, posts, and credentials. Clear metadata and restrained visual decoration support information-heavy exchanges.",
        visual_rules: "Use a disciplined Swiss grid to organize verified member profiles, role/skill matrices, work history, structured directory filters, and signal-rich industry discussions.",
        best_fit: "Professional networks, freelancer/agency directories, research groups, and B2B communities.",
        essential_screens: SOCIAL_ESSENTIAL_SCREENS,
        keywords: &["social", "community", "swiss professional", "professional network", "freelancers", "directory", "credentials", "b2b"],
    },
    StyleCatalogEntry {
        number: 89,
        id: "89-social-cyberpunk-gaming",
        name: "Social / Community — Cyberpunk Gaming",
        category: "Social Media / Community",
        definition: "Dark panels, electric accents, technical motifs, and strong avatar imagery create a gaming identity. Compact activity widgets and channel lists sit beside readable conversation areas.",
        visual_rules: "Build a dark tactical community hub with neon-accented squad/channel sidebars, framed player avatars with live status/rank badges, match tickers, and high-contrast chat/forum panels.",
        best_fit: "Gaming clans, esports communities, streaming hubs, and sci-fi/tech fandoms.",
        essential_screens: SOCIAL_ESSENTIAL_SCREENS,
        keywords: &["social", "community", "cyberpunk gaming", "gaming community", "esports", "guild", "discord", "clans"],
    },
    StyleCatalogEntry {
        number: 90,
        id: "90-social-soft-glass-social",
        name: "Social / Community — Soft Glass Social",
        category: "Social Media / Community",
        definition: "Translucent panels, gentle gradients, subtle blur, and rounded controls create a layered digital appearance. More opaque surfaces support longer text, while glass effects distinguish navigation and overlays.",
        visual_rules: "Use translucent frosted glass (`backdrop-filter: blur(16px)`) for sticky navigation, profile headers, and floating action docks, paired with high-legibility opaque cards for posts, bio links, and comments.",
        best_fit: "Music communities, creative circles, link-in-bio profiles, and visual social apps.",
        essential_screens: SOCIAL_ESSENTIAL_SCREENS,
        keywords: &["social", "community", "soft glass social", "glassmorphism", "music community", "profiles", "link-in-bio", "creative"],
    },

    // =========================================================================
    // CATEGORY 5: ANIME LIST / ANIME DISCOVERY (#91 - #100)
    // =========================================================================
    StyleCatalogEntry {
        number: 91,
        id: "91-anime-minimal-tracker",
        name: "Anime Discovery — Minimal Anime Tracker",
        category: "Anime List / Discovery",
        definition: "Light or dark neutral surfaces, compact poster cards, quiet typography, and restrained status colors emphasize tracking. Episode counts and watch status stay visible in both list and grid views.",
        visual_rules: "Prioritize fast library tracking: 2:3 anime poster grids and compact table rows with inline `+1 EP` progress buttons, episode bars (`8/12`), color-coded status dots (Watching, Completed, Plan to Watch, Dropped), and seasonal/format filter pills.",
        best_fit: "Large personal anime libraries, seasonal trackers, and frequent episode progress updates.",
        essential_screens: ANIME_ESSENTIAL_SCREENS,
        keywords: &["anime", "anime list", "minimal anime tracker", "watchlist", "episode tracker", "seasonal anime", "mal", "anilist"],
    },
    StyleCatalogEntry {
        number: 92,
        id: "92-anime-manga-monochrome",
        name: "Anime Discovery — Manga Monochrome",
        category: "Anime List / Discovery",
        definition: "Black-and-white panels, strong outlines, halftone accents, and expressive display lettering borrow from manga pages. Poster art supplies color while tracking controls use consistent, simple shapes.",
        visual_rules: "Frame the interface in crisp black-and-white manga panel borders with screentone dot textures and bold display headers, allowing full-color anime/manga cover art and brand accent badges to pop dramatically.",
        best_fit: "Manga & anime trackers, chapter/episode logs, and graphic editorial fan communities.",
        essential_screens: ANIME_ESSENTIAL_SCREENS,
        keywords: &["anime", "manga", "manga monochrome", "black and white", "screentone", "panels", "chapter tracker", "watchlist"],
    },
    StyleCatalogEntry {
        number: 93,
        id: "93-anime-pastel-kawaii",
        name: "Anime Discovery — Pastel Kawaii",
        category: "Anime List / Discovery",
        definition: "Soft pinks, lilacs, blues, rounded cards, cute icons, and playful illustrations create a gentle mood. Progress indicators and empty states carry the personality without crowding the posters.",
        visual_rules: "Use soft rounded 24px poster cards, gentle pastel surfaces harmonized with the user's brand colors, cute star/heart rating chips, and playful mascot empty states while keeping poster art unobstructed.",
        best_fit: "Slice-of-life anime discovery, cozy watchlists, and personalizable fan collections.",
        essential_screens: ANIME_ESSENTIAL_SCREENS,
        keywords: &["anime", "pastel kawaii", "kawaii", "cute", "slice of life", "watchlist", "cozy", "pastel"],
    },
    StyleCatalogEntry {
        number: 94,
        id: "94-anime-shonen-neobrutalism",
        name: "Anime Discovery — Shonen Neobrutalism",
        category: "Anime List / Discovery",
        definition: "Bold accent colors, heavy typography, thick card borders, hard shadows, and energetic graphic marks suggest action. Ranking numbers and status badges become strong visual anchors.",
        visual_rules: "Use 3px dark borders, hard offset shadows, oversized ranking numbers (`#01`, `#02`), high-voltage brand accent blocks, and punchy score/episode badges for high-energy seasonal rankings.",
        best_fit: "Action/shonen anime discovery, top-chart rankings, and active fan voting communities.",
        essential_screens: ANIME_ESSENTIAL_SCREENS,
        keywords: &["anime", "shonen neobrutalism", "shonen", "action", "rankings", "charts", "neobrutalism", "watchlist"],
    },
    StyleCatalogEntry {
        number: 95,
        id: "95-anime-cinematic-gallery",
        name: "Anime Discovery — Cinematic Anime Gallery",
        category: "Anime List / Discovery",
        definition: "Large key art, dark backgrounds, dramatic title sections, and spacious poster collections turn discovery into a visual experience. Synopsis, episode count, score, and watchlist controls follow a clear hierarchy.",
        visual_rules: "Stage widescreen seasonal key-art hero banners over deep obsidian surfaces with studio/staff metadata, airing countdowns, one-click watchlist status selectors, and luminous poster carousels.",
        best_fit: "Seasonal anime discovery, streaming guides, and visually rich anime catalogues.",
        essential_screens: ANIME_ESSENTIAL_SCREENS,
        keywords: &["anime", "cinematic anime gallery", "seasonal chart", "key art", "streaming", "catalogue", "discovery"],
    },
    StyleCatalogEntry {
        number: 96,
        id: "96-anime-mecha-cyberpunk",
        name: "Anime Discovery — Mecha Cyberpunk",
        category: "Anime List / Discovery",
        definition: "Dark technical panels, cyan or magenta accents, fine grid lines, and compact metadata create a futuristic control-room aesthetic. Tables and tracking controls remain orderly beneath the decorative layer.",
        visual_rules: "Frame anime database tables and poster grids inside precision mecha-HUD telemetry panels with chamfered corners, monospace airing/studio readouts, and glowing progress bars.",
        best_fit: "Sci-fi, mecha, and cyberpunk anime databases and technical tracking power-users.",
        essential_screens: ANIME_ESSENTIAL_SCREENS,
        keywords: &["anime", "mecha cyberpunk", "mecha", "sci-fi", "hud", "database", "telemetry", "tracker"],
    },
    StyleCatalogEntry {
        number: 97,
        id: "97-anime-retro-pixel-club",
        name: "Anime Discovery — Retro Pixel Club",
        category: "Anime List / Discovery",
        definition: "Pixel icons, limited color accents, cartridge-like labels, and nostalgic interface details evoke older games and fan sites. Readable body typography supports modern search and episode tracking.",
        visual_rules: "Pair nostalgic pixel-art badges, VHS/cartridge status tabs, and stepped borders with crisp modern body typography for synopses, episode counters, and fast search filters.",
        best_fit: "Classic 80s/90s/2000s anime archives, retro OVA catalogues, and nostalgic fan clubs.",
        essential_screens: ANIME_ESSENTIAL_SCREENS,
        keywords: &["anime", "retro pixel club", "retro anime", "pixel", "90s anime", "vhs", "fan club"],
    },
    StyleCatalogEntry {
        number: 98,
        id: "98-anime-editorial-journal",
        name: "Anime Discovery — Editorial Anime Journal",
        category: "Anime List / Discovery",
        definition: "Magazine-style features, thoughtful typography, review excerpts, and generous spacing emphasize criticism and discovery. Personal lists can include short essays, annotations, and themed introductions.",
        visual_rules: "Combine magazine-style serif headlines, long-form review essays, studio/director spotlights, and annotated personal list cards where users attach notes to each ranked title.",
        best_fit: "Anime criticism, recommendation essays, director retrospectives, and curated thematic lists.",
        essential_screens: ANIME_ESSENTIAL_SCREENS,
        keywords: &["anime", "editorial anime journal", "reviews", "essays", "curated lists", "criticism", "recommendations"],
    },
    StyleCatalogEntry {
        number: 99,
        id: "99-anime-expressive-fandom-collage",
        name: "Anime Discovery — Expressive Fandom Collage",
        category: "Anime List / Discovery",
        definition: "Character cutouts, stickers, layered artwork, bold type, and personalized banners create a fan-made scrapbook atmosphere. Standardized title cards keep ratings and watch progress recognizable.",
        visual_rules: "Allow expressive scrapbook headers, sticker badges, and favorite-character cutouts around standardized, easy-to-scan anime title cards with clear ratings and episode progress.",
        best_fit: "Customizable fan profiles, character shrines, and themed social watchlists.",
        essential_screens: ANIME_ESSENTIAL_SCREENS,
        keywords: &["anime", "fandom collage", "scrapbook", "stickers", "custom profile", "characters", "watchlist"],
    },
    StyleCatalogEntry {
        number: 100,
        id: "100-anime-dreamy-glass-library",
        name: "Anime Discovery — Dreamy Glass Library",
        category: "Anime List / Discovery",
        definition: "Soft atmospheric gradients, translucent navigation, luminous accents, and floating poster cards create a dreamy library. Clear content panels keep synopses, scores, and episode controls readable.",
        visual_rules: "Set floating 2:3 anime poster cards and frosted glass navigation over twilight sky/aurora gradients, keeping synopses, scores, and episode increment controls on high-contrast panels.",
        best_fit: "Personal anime libraries, atmospheric seasonal discovery, and soundtrack/visual catalogues.",
        essential_screens: ANIME_ESSENTIAL_SCREENS,
        keywords: &["anime", "dreamy glass library", "glassmorphism", "atmospheric", "library", "watchlist", "posters"],
    },

    // =========================================================================
    // CATEGORY 6: MOVIE LIST / FILM DISCOVERY (#101 - #110)
    // =========================================================================
    StyleCatalogEntry {
        number: 101,
        id: "101-movie-dark-cinematic",
        name: "Film Discovery — Dark Cinematic",
        category: "Movie List / Film Discovery",
        definition: "Deep backgrounds, large film stills, restrained accents, and dramatic typography give the catalogue a cinema-like mood. Posters and key art dominate while ratings and watchlist actions stay clear.",
        visual_rules: "Use deep theater-black surfaces (`#08090C`), widescreen 2.39:1 hero film stills with smooth bottom vignette scrims, 2:3 poster shelves, and crisp watched/liked/rated/watchlist action controls.",
        best_fit: "Mainstream film discovery, Letterboxd-style viewing diaries, and visually rich movie collections.",
        essential_screens: MOVIE_ESSENTIAL_SCREENS,
        keywords: &["movie", "film", "cinema", "dark cinematic", "watchlist", "letterboxd", "posters", "film discovery"],
    },
    StyleCatalogEntry {
        number: 102,
        id: "102-movie-editorial-cinephile",
        name: "Film Discovery — Editorial Cinephile",
        category: "Movie List / Film Discovery",
        definition: "Elegant typography, generous reading space, still-image essays, and review excerpts make the platform feel like a film journal. Director collections and themed lists become editorial features.",
        visual_rules: "Pair Criterion/Sight-&-Sound inspired serif typography with curated director retrospectives, film-still essays, viewing diary timestamps, and thoughtful review cards.",
        best_fit: "Film criticism, cinephile journals, personal reviews, and curated director/festival collections.",
        essential_screens: MOVIE_ESSENTIAL_SCREENS,
        keywords: &["movie", "film", "editorial cinephile", "criterion", "reviews", "film journal", "directors", "essays"],
    },
    StyleCatalogEntry {
        number: 103,
        id: "103-movie-minimal-poster-gallery",
        name: "Film Discovery — Minimal Poster Gallery",
        category: "Movie List / Film Discovery",
        definition: "A clean grid, neutral backgrounds, sparse decoration, and consistent poster proportions make the collection itself the main visual element. Compact overlays reveal ratings and watched status.",
        visual_rules: "Center the layout on a precision-spaced grid of uniform 2:3 film posters with minimal UI chrome, compact corner status badges (watched eye, star score, list rank), and instant filter pills.",
        best_fit: "Personal film collections, annual viewing challenges, and browsing large watchlists.",
        essential_screens: MOVIE_ESSENTIAL_SCREENS,
        keywords: &["movie", "film", "minimal poster gallery", "posters", "watchlist", "collection", "grid"],
    },
    StyleCatalogEntry {
        number: 104,
        id: "104-movie-vintage-cinema",
        name: "Film Discovery — Vintage Cinema",
        category: "Movie List / Film Discovery",
        definition: "Warm paper tones, restrained grain, classic display lettering, ticket motifs, and archival imagery evoke older cinema culture. Modern list controls sit within a period-inspired visual frame.",
        visual_rules: "Use warm archival paper surfaces, perforated cinema-ticket stub cards for logged viewing dates, marquee display headers, and clean modern rating/filter controls.",
        best_fit: "Classic films, archival projects, repertory cinema calendars, and film-history clubs.",
        essential_screens: MOVIE_ESSENTIAL_SCREENS,
        keywords: &["movie", "film", "vintage cinema", "classic film", "ticket", "repertory", "archival", "diary"],
    },
    StyleCatalogEntry {
        number: 105,
        id: "105-movie-art-deco-theatre",
        name: "Film Discovery — Art Deco Theatre",
        category: "Movie List / Film Discovery",
        definition: "Symmetrical composition, geometric ornament, stepped borders, elegant lettering, and controlled gold accents reference historic movie palaces. Decorative elements frame film imagery and curated collections.",
        visual_rules: "Frame featured films and award-season lists inside symmetrical stepped Art Deco borders with fine metallic/brand-accent lines and poised theatrical typography.",
        best_fit: "Classic cinema, awards-season trackers (Oscars/Cannes), and premiere film clubs.",
        essential_screens: MOVIE_ESSENTIAL_SCREENS,
        keywords: &["movie", "film", "art deco theatre", "awards", "oscars", "movie palace", "classic cinema", "club"],
    },
    StyleCatalogEntry {
        number: 106,
        id: "106-movie-swiss-film-database",
        name: "Film Discovery — Swiss Film Database",
        category: "Movie List / Film Discovery",
        definition: "Precise alignment, clear sans-serif type, compact metadata, and orderly tables emphasize catalogue depth. Cast, crew, runtime, release year, and personal status share a consistent information structure.",
        visual_rules: "Structure filmographies, cast/crew credits, release year, country, runtime, aspect ratio, and personal watch status in an ultra-clean Swiss grid with toggleable table and card views.",
        best_fit: "Film research, complete director/actor filmographies, and extensive catalog databases.",
        essential_screens: MOVIE_ESSENTIAL_SCREENS,
        keywords: &["movie", "film", "swiss film database", "filmography", "cast and crew", "metadata", "archive", "database"],
    },
    StyleCatalogEntry {
        number: 107,
        id: "107-movie-neobrutalist-review-club",
        name: "Film Discovery — Neobrutalist Review Club",
        category: "Movie List / Film Discovery",
        definition: "Bold outlines, hard shadows, colorful rating blocks, and expressive headlines give reviews a playful, social presence. List covers and review cards use strong, repeatable graphic patterns.",
        visual_rules: "Use 3px dark borders, hard offset shadows, bold color-coded rating blocks, hot-take review cards, friend watch-party polls, and punchy list covers.",
        best_fit: "Friends' movie clubs, podcast communities, social ratings, and group watchlists.",
        essential_screens: MOVIE_ESSENTIAL_SCREENS,
        keywords: &["movie", "film", "neobrutalist review club", "film club", "reviews", "social", "ratings", "friends"],
    },
    StyleCatalogEntry {
        number: 108,
        id: "108-movie-monochrome-noir",
        name: "Film Discovery — Monochrome Noir",
        category: "Movie List / Film Discovery",
        definition: "Black, white, and charcoal dominate the interface, supported by dramatic cropping and restrained typography. Limited accent color highlights actions while imagery supplies atmosphere and contrast.",
        visual_rules: "Use stark black, charcoal, and chalk-white surfaces with high-contrast typography and a single sharp brand accent color for watched/rating states.",
        best_fit: "Film noir, arthouse cinema, horror/thriller catalogues, and auteur collections.",
        essential_screens: MOVIE_ESSENTIAL_SCREENS,
        keywords: &["movie", "film", "monochrome noir", "noir", "arthouse", "black and white", "auteur"],
    },
    StyleCatalogEntry {
        number: 109,
        id: "109-movie-soft-glass-watchlist",
        name: "Film Discovery — Soft Glass Watchlist",
        category: "Movie List / Film Discovery",
        definition: "Blurred backdrop imagery, translucent panels, rounded cards, and subtle highlights create a layered personal-library aesthetic. A solid reading area holds reviews and detailed film information.",
        visual_rules: "Use blurred cinema-still ambient backdrops behind frosted translucent filter/navigation bars and rounded poster cards, paired with a solid high-contrast reading panel for reviews and cast details.",
        best_fit: "Personal streaming watchlists, home-theater dashboards, and polished film trackers.",
        essential_screens: MOVIE_ESSENTIAL_SCREENS,
        keywords: &["movie", "film", "soft glass watchlist", "apple tv", "streaming", "watchlist", "dashboard"],
    },
    StyleCatalogEntry {
        number: 110,
        id: "110-movie-independent-festival-collage",
        name: "Film Discovery — Independent Festival Collage",
        category: "Movie List / Film Discovery",
        definition: "Expressive typography, poster fragments, contrasting colors, unconventional crops, and asymmetric feature blocks evoke independent film programmes. Consistent film cards organize the experimental presentation.",
        visual_rules: "Combine indie festival programme energy—asymmetric feature blocks, laurel stamps, and bold display type—with standardized film cards displaying screening/watch status and ratings.",
        best_fit: "Independent cinema, film festivals, zine-style film discovery, and themed screenings.",
        essential_screens: MOVIE_ESSENTIAL_SCREENS,
        keywords: &["movie", "film", "independent festival collage", "indie film", "festival", "screenings", "Experimental"],
    },

    // =========================================================================
    // CATEGORY 7: GAME LIST / GAME LIBRARY (#111 - #120)
    // =========================================================================
    StyleCatalogEntry {
        number: 111,
        id: "111-game-dark-launcher-library",
        name: "Game Library — Dark Launcher Library",
        category: "Game List / Game Library",
        definition: "Dark neutral surfaces, orderly cover grids, compact navigation, and a clear game-detail panel resemble a familiar desktop game library. Ownership, platform, and play status appear alongside each title.",
        visual_rules: "Structure the interface like a flagship desktop game launcher: dark slate surfaces, collection sidebar, featured game hero banner with playtime hours & completion %, platform chips (PC, PS5, Switch, Xbox), and vertical box-art grid.",
        best_fit: "Cross-platform game collections, Steam/Playnite-style libraries, and everyday backlog management.",
        essential_screens: GAME_ESSENTIAL_SCREENS,
        keywords: &["game", "gaming", "game library", "dark launcher", "steam", "backlog", "playtime", "collection", "platforms"],
    },
    StyleCatalogEntry {
        number: 112,
        id: "112-game-cyberpunk-hud",
        name: "Game Library — Cyberpunk HUD",
        category: "Game List / Game Library",
        definition: "Electric accents, angular panels, technical lines, condensed headings, and controlled animation create a futuristic interface. Status tags and library statistics use readable, repeatable modules.",
        visual_rules: "Frame game covers, achievement completion rings, and playtime telemetry inside chamfered HUD modules with crisp 1px neon brand accents over deep carbon surfaces.",
        best_fit: "Sci-fi/action game trackers, achievement hunters, and futuristic gaming communities.",
        essential_screens: GAME_ESSENTIAL_SCREENS,
        keywords: &["game", "gaming", "cyberpunk hud", "achievements", "telemetry", "sci-fi", "tracker", "hud"],
    },
    StyleCatalogEntry {
        number: 113,
        id: "113-game-pixel-arcade",
        name: "Game Library — Pixel Arcade",
        category: "Game List / Game Library",
        definition: "Pixel illustrations, bitmap accents, bright limited palettes, and arcade-inspired badges evoke classic games. Standard body typography keeps descriptions and library controls comfortable to read.",
        visual_rules: "Use pixel-art trophy/status badges, arcade cabinet color accents, and bitmap section headers paired with clean sans-serif body typography for readable reviews, platform tags, and backlog filters.",
        best_fit: "Retro gaming collections, emulation libraries, indie pixel games, and arcade leaderboards.",
        essential_screens: GAME_ESSENTIAL_SCREENS,
        keywords: &["game", "gaming", "pixel arcade", "retro games", "arcade", "emulation", "8-bit", "16-bit", "collection"],
    },
    StyleCatalogEntry {
        number: 114,
        id: "114-game-esports-neobrutalism",
        name: "Game Library — Esports Neobrutalism",
        category: "Game List / Game Library",
        definition: "Heavy typography, strong color contrasts, thick borders, hard shadows, and large ranking numbers create a competitive visual tone. Genre labels and personal scores use bold, consistent markers.",
        visual_rules: "Use 3px solid borders, hard offset shadows, bold S/A/B tier-list rank badges, high-contrast score blocks, and condensed competitive headlines in the user's brand palette.",
        best_fit: "Competitive gaming rankings, game tier lists, community ratings, and esports hubs.",
        essential_screens: GAME_ESSENTIAL_SCREENS,
        keywords: &["game", "gaming", "esports neobrutalism", "tier list", "rankings", "competitive", "scores", "reviews"],
    },
    StyleCatalogEntry {
        number: 115,
        id: "115-game-editorial-indie-journal",
        name: "Game Library — Editorial Indie Journal",
        category: "Game List / Game Library",
        definition: "Expressive but restrained typography, generous screenshots, developer stories, and annotated lists create a magazine-like discovery experience. Personal reviews and thematic collections receive prominent space.",
        visual_rules: "Pair editorial serif headlines and wide gameplay screenshot galleries with personal play-diary essays, developer notes, and curated thematic game lists.",
        best_fit: "Indie game discovery, long-form game criticism, play diaries, and curated recommendations.",
        essential_screens: GAME_ESSENTIAL_SCREENS,
        keywords: &["game", "gaming", "editorial indie journal", "indie games", "play diary", "reviews", "journal"],
    },
    StyleCatalogEntry {
        number: 116,
        id: "116-game-fantasy-parchment",
        name: "Game Library — Fantasy Parchment",
        category: "Game List / Game Library",
        definition: "Warm textured surfaces, ornamental frames, illustrated icons, and restrained display lettering resemble a fantasy journal. Modern tables and controls organize game entries within the themed presentation.",
        visual_rules: "Frame the game backlog like an adventurer's quest compendium with warm parchment/leather tones, ornamental corner rules, genre crest icons, and clean modern status/progress tables.",
        best_fit: "RPGs, tabletop/fantasy collections, quest logs, and lore-rich gaming libraries.",
        essential_screens: GAME_ESSENTIAL_SCREENS,
        keywords: &["game", "gaming", "fantasy parchment", "rpg", "quest log", "fantasy", "compendium", "tabletop"],
    },
    StyleCatalogEntry {
        number: 117,
        id: "117-game-cozy-illustrated-library",
        name: "Game Library — Cozy Illustrated Library",
        category: "Game List / Game Library",
        definition: "Pastel or earthy colors, soft illustrations, rounded cards, and friendly icons create a relaxed digital collection. Progress indicators feel gentle, and covers remain the main browsing content.",
        visual_rules: "Use soft rounded 24px game cards, warm cozy pastel/earthy surfaces, charming illustrated status icons (Currently Playing sprout, Completed star), and gentle progress bars.",
        best_fit: "Cozy games, life/farming simulations, Nintendo Switch collections, and casual game libraries.",
        essential_screens: GAME_ESSENTIAL_SCREENS,
        keywords: &["game", "gaming", "cozy illustrated library", "cozy games", "life sim", "casual", "relaxed", "collection"],
    },
    StyleCatalogEntry {
        number: 118,
        id: "118-game-swiss-backlog-manager",
        name: "Game Library — Swiss Backlog Manager",
        category: "Game List / Game Library",
        definition: "A disciplined grid, compact rows, neutral colors, and precise typography prioritize efficient organization. Platform, status, priority, rating, and progress are available in configurable table or card views.",
        visual_rules: "Organize large game backlogs inside a precision 1px Swiss grid with sortable columns for Platform, Status (Backlog, Playing, Beaten, 100%), Priority, Time-to-Beat, and Personal Score.",
        best_fit: "Large game backlogs, completionist tracking, and data-driven collection management.",
        essential_screens: GAME_ESSENTIAL_SCREENS,
        keywords: &["game", "gaming", "swiss backlog manager", "backlog", "hltb", "completionist", "table", "tracker"],
    },
    StyleCatalogEntry {
        number: 119,
        id: "119-game-sci-fi-glass-dashboard",
        name: "Game Library — Sci-Fi Glass Dashboard",
        category: "Game List / Game Library",
        definition: "Translucent panels, atmospheric gradients, luminous edges, and large game artwork create a futuristic dashboard. Solid content surfaces support filters, statistics, and detailed library information.",
        visual_rules: "Combine next-gen console-style key-art ambient glow and frosted glass navigation with solid high-contrast panels for library analytics, playtime charts, and cover grids.",
        best_fit: "Next-gen console dashboards, personal gaming showcases, and visual collection overviews.",
        essential_screens: GAME_ESSENTIAL_SCREENS,
        keywords: &["game", "gaming", "sci-fi glass dashboard", "console dashboard", "glassmorphism", "showcase", "library"],
    },
    StyleCatalogEntry {
        number: 120,
        id: "120-game-retro-console-interface",
        name: "Game Library — Retro Console Interface",
        category: "Game List / Game Library",
        definition: "Chunky menu shapes, cartridge or disc motifs, nostalgic colors, and era-inspired iconography reference a chosen console generation. Consistent navigation connects collection shelves, details, and status controls.",
        visual_rules: "Use tactile chunky console-menu tabs, physical cartridge/disc case shelf proportions, memory-card slot progress indicators, and clean collection filters.",
        best_fit: "Physical game collectors, retro console libraries, and nostalgic gaming projects.",
        essential_screens: GAME_ESSENTIAL_SCREENS,
        keywords: &["game", "gaming", "retro console interface", "cartridge", "physical games", "collector", "console"],
    },
];

pub fn find_style_by_number(num: u32) -> Option<&'static StyleCatalogEntry> {
    STYLE_CATALOG.iter().find(|s| s.number == num)
}

pub fn find_style_by_id_or_name(q: &str) -> Option<&'static StyleCatalogEntry> {
    let clean = q.trim().to_ascii_lowercase();
    if clean.is_empty() {
        return None;
    }
    STYLE_CATALOG.iter().find(|s| {
        s.id.eq_ignore_ascii_case(&clean)
            || s.name.eq_ignore_ascii_case(&clean)
            || clean.contains(&format!("#{}", s.number))
    })
}

/// Tokenizes a query string into lowercase alphanumeric stems (`len >= 2`).
fn tokenize_query(text: &str) -> Vec<String> {
    text.to_ascii_lowercase()
        .split(|c: char| !c.is_alphanumeric() && c != '-')
        .map(|w| w.trim_matches('-').to_string())
        .filter(|w| w.len() >= 2)
        .filter(|w| {
            !matches!(
                w.as_str(),
                "the"
                    | "and"
                    | "for"
                    | "with"
                    | "that"
                    | "this"
                    | "from"
                    | "your"
                    | "are"
                    | "can"
                    | "will"
                    | "into"
                    | "their"
                    | "have"
                    | "has"
                    | "our"
                    | "you"
                    | "about"
                    | "website"
                    | "page"
                    | "site"
                    | "platform"
                    | "app"
                    | "design"
            )
        })
        .collect()
}

/// Smart Keyword & Product-Intent Search Engine over all 120 Web Design Styles.
/// Returns the Top `limit` most relevant styles so the AI planner / brand compiler
/// only reads the styles that actually matter most for the user's product and keywords,
/// saving 85–90% of the context window while preserving full rules for every matched style.
pub fn search_top_design_styles(
    query_text: &str,
    preferred_hint: Option<&str>,
    limit: usize,
) -> Vec<&'static StyleCatalogEntry> {
    let combined = format!("{} {}", query_text, preferred_hint.unwrap_or(""));
    let lower = combined.to_ascii_lowercase();
    let tokens = tokenize_query(&combined);

    // Detect high-level product/domain intent so category-specific styles (#51-#120) get boosted
    // when the user's product structure matches (e.g. a link-in-bio directory or multi-seller platform -> Marketplace & Social/Community).
    let is_marketplace_like = lower.contains("marketplace")
        || lower.contains("directory")
        || lower.contains("discover")
        || lower.contains("multi-seller")
        || lower.contains("seller")
        || lower.contains("vendor")
        || lower.contains("listing")
        || lower.contains("link-in-bio")
        || lower.contains("link in bio")
        || lower.contains("banner")
        || lower.contains("advertis")
        || lower.contains("booking")
        || lower.contains("freelancer")
        || lower.contains("catalog");

    let is_dropshipping_like = lower.contains("dropship")
        || lower.contains("single product")
        || lower.contains("flagship product")
        || lower.contains("gadget")
        || lower.contains("funnel")
        || lower.contains("bundle");

    let is_shop_like = lower.contains("shop")
        || lower.contains("store")
        || lower.contains("ecommerce")
        || lower.contains("e-commerce")
        || lower.contains("cart")
        || lower.contains("checkout")
        || lower.contains("retail")
        || lower.contains("merch")
        || lower.contains("apparel")
        || lower.contains("fashion")
        || lower.contains("homeware")
        || lower.contains("furniture");

    let is_social_like = lower.contains("social")
        || lower.contains("community")
        || lower.contains("creator")
        || lower.contains("profile")
        || lower.contains("link-in-bio")
        || lower.contains("link in bio")
        || lower.contains("feed")
        || lower.contains("forum")
        || lower.contains("member")
        || lower.contains("discussion")
        || lower.contains("newsletter")
        || lower.contains("followers");

    let is_anime_like = lower.contains("anime")
        || lower.contains("manga")
        || lower.contains("otaku")
        || lower.contains("shonen")
        || lower.contains("episode")
        || lower.contains("seasonal");

    let is_movie_like = lower.contains("movie")
        || lower.contains("film")
        || lower.contains("cinema")
        || lower.contains("watchlist")
        || lower.contains("director")
        || lower.contains("cinephile")
        || lower.contains("box office")
        || lower.contains("theatre");

    let is_game_like = lower.contains("game")
        || lower.contains("gaming")
        || lower.contains("backlog")
        || lower.contains("playtime")
        || lower.contains("esports")
        || lower.contains("steam")
        || lower.contains("rpg")
        || lower.contains("arcade")
        || lower.contains("console");

    let pinned_style = preferred_hint.and_then(find_style_by_id_or_name);

    let mut scored: Vec<(i64, &'static StyleCatalogEntry)> = STYLE_CATALOG
        .iter()
        .map(|entry| {
            let mut score: i64 = 0;

            if let Some(pinned) = pinned_style {
                if pinned.number == entry.number {
                    score += 10_000;
                }
            }

            // Category intent boosts
            match entry.category {
                "Marketplace" if is_marketplace_like => score += 45,
                "Dropshipping Store" if is_dropshipping_like => score += 50,
                "Shop / Branded Ecommerce" if is_shop_like => score += 45,
                "Social Media / Community" if is_social_like => score += 45,
                "Anime List / Discovery" if is_anime_like => score += 65,
                "Movie List / Film Discovery" if is_movie_like => score += 65,
                "Game List / Game Library" if is_game_like => score += 65,
                _ => {}
            }

            let entry_name_lower = entry.name.to_ascii_lowercase();
            let entry_cat_lower = entry.category.to_ascii_lowercase();
            let entry_def_lower = entry.definition.to_ascii_lowercase();
            let entry_fit_lower = entry.best_fit.to_ascii_lowercase();
            let entry_rules_lower = entry.visual_rules.to_ascii_lowercase();

            if lower.contains(&entry_name_lower) {
                score += 120;
            }

            for kw in entry.keywords {
                let kw_l = kw.to_ascii_lowercase();
                if lower.contains(&kw_l) {
                    score += if kw_l.contains(' ') { 36 } else { 22 };
                }
            }

            for tok in &tokens {
                if entry.keywords.iter().any(|k| k.eq_ignore_ascii_case(tok)) {
                    score += 20;
                } else if entry
                    .keywords
                    .iter()
                    .any(|k| k.to_ascii_lowercase().contains(tok.as_str()))
                {
                    score += 12;
                }
                if entry_name_lower.contains(tok.as_str()) {
                    score += 18;
                }
                if entry_cat_lower.contains(tok.as_str()) {
                    score += 14;
                }
                if entry_fit_lower.contains(tok.as_str()) {
                    score += 15;
                }
                if entry_def_lower.contains(tok.as_str()) {
                    score += 8;
                }
                if entry_rules_lower.contains(tok.as_str()) {
                    score += 5;
                }
            }

            (score, entry)
        })
        .collect();

    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.number.cmp(&b.1.number)));

    let mut result: Vec<&'static StyleCatalogEntry> = Vec::new();
    for (score, entry) in &scored {
        if result.len() >= limit {
            break;
        }
        if *score > 0 || result.len() < 6 {
            result.push(*entry);
        }
    }

    // Ensure at least a couple of versatile foundational styles are present if empty
    if result.is_empty() {
        for fallback_num in [2, 15, 20, 51, 84, 4] {
            if let Some(s) = find_style_by_number(fallback_num) {
                result.push(s);
            }
        }
    }

    result
}

/// Formats the shortlisted styles for the AI Brand & Style Compiler prompt so the LLM
/// reads complete rules for the top relevant styles without bloating the context window.
pub fn format_shortlisted_styles_for_ai(
    query_text: &str,
    preferred_hint: Option<&str>,
    limit: usize,
) -> String {
    let top_styles = search_top_design_styles(query_text, preferred_hint, limit);
    let mut lines = Vec::with_capacity(top_styles.len() + 2);
    lines.push(format!(
        "SHORTLISTED WEB DESIGN STYLES (Top {} of 120 styles matched by keyword & product intent search — pick the best `recommended_style_number` from these):",
        top_styles.len()
    ));
    for s in top_styles {
        lines.push(format!(
            "- Style #{num}: {name} [Category: {cat}]\n  * Definition: {def}\n  * Best Fit: {fit}\n  * Essential Screens/Features: {screens}\n  * Concrete Visual Rules (apply using the User's Brand Palette & Logo): {rules}",
            num = s.number,
            name = s.name,
            cat = s.category,
            def = s.definition,
            fit = s.best_fit,
            screens = s.essential_screens,
            rules = s.visual_rules
        ));
    }
    lines.join("\n")
}

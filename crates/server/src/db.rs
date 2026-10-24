use crate::error::{AppError, AppResult};
use sqlx::{
    sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions, SqliteSynchronous},
    SqlitePool,
};
use std::{path::Path, str::FromStr, time::Duration};

pub async fn init_db_pool(data_dir: &Path) -> AppResult<SqlitePool> {
    std::fs::create_dir_all(data_dir)?;
    let db_path = data_dir.join("design-gen.sqlite");
    let db_url = format!("sqlite://{}?mode=rwc", db_path.display());

    let connect_opts = SqliteConnectOptions::from_str(&db_url)
        .map_err(|e| AppError::Internal(format!("Invalid SQLite URL: {}", e)))?
        .create_if_missing(true)
        .foreign_keys(true)
        .journal_mode(SqliteJournalMode::Wal)
        .synchronous(SqliteSynchronous::Normal)
        .busy_timeout(Duration::from_secs(5));

    let pool = SqlitePoolOptions::new()
        .max_connections(16)
        .acquire_timeout(Duration::from_secs(10))
        .connect_with(connect_opts)
        .await?;

    // Run initial schema migration
    let migration_sql = include_str!("../../../migrations/0001_initial.sql");
    sqlx::raw_sql(migration_sql).execute(&pool).await?;

    // Run chat & multi-tab canvas migration
    let migration_0002_sql = include_str!("../../../migrations/0002_chat_and_tabs.sql");
    sqlx::raw_sql(migration_0002_sql).execute(&pool).await?;

    // Ensure new columns exist on `outputs` and `runs` for existing databases
    ensure_column(&pool, "outputs", "tab_key", "TEXT NOT NULL DEFAULT 'main'").await?;
    ensure_column(
        &pool,
        "outputs",
        "tab_label",
        "TEXT NOT NULL DEFAULT 'Main'",
    )
    .await?;
    ensure_column(&pool, "outputs", "parent_output_id", "TEXT").await?;
    ensure_column(
        &pool,
        "outputs",
        "transparent_background",
        "INTEGER NOT NULL DEFAULT 0",
    )
    .await?;
    ensure_column(&pool, "outputs", "extracted_from_output_id", "TEXT").await?;

    ensure_column(&pool, "runs", "target_tab_key", "TEXT").await?;
    ensure_column(&pool, "runs", "target_tab_label", "TEXT").await?;
    ensure_column(&pool, "runs", "target_output_id", "TEXT").await?;
    ensure_column(
        &pool,
        "runs",
        "highlight_annotations_json",
        "TEXT NOT NULL DEFAULT '[]'",
    )
    .await?;
    ensure_column(
        &pool,
        "projects",
        "is_example",
        "INTEGER NOT NULL DEFAULT 0",
    )
    .await?;

    Ok(pool)
}

async fn ensure_column(
    pool: &SqlitePool,
    table: &str,
    column: &str,
    definition: &str,
) -> AppResult<()> {
    let pragma_sql = format!("PRAGMA table_info({})", table);
    let rows = sqlx::query(&pragma_sql).fetch_all(pool).await?;
    let exists = rows.iter().any(|r| {
        use sqlx::Row;
        r.try_get::<String, _>("name")
            .map(|n| n.eq_ignore_ascii_case(column))
            .unwrap_or(false)
    });
    if !exists {
        let alter_sql = format!("ALTER TABLE {} ADD COLUMN {} {}", table, column, definition);
        sqlx::query(&alter_sql).execute(pool).await?;
    }
    Ok(())
}

/// Safely creates a consistent backup snapshot of the live WAL database using SQLite's `VACUUM INTO`.
pub async fn backup_database(pool: &SqlitePool, target_file: &Path) -> AppResult<()> {
    if target_file.exists() {
        std::fs::remove_file(target_file)?;
    }
    let target_str = target_file
        .to_str()
        .ok_or_else(|| AppError::BadRequest("Invalid backup target path".into()))?;
    sqlx::query("VACUUM INTO ?")
        .bind(target_str)
        .execute(pool)
        .await?;
    Ok(())
}

struct SeedOutputDef<'a> {
    output_id: &'a str,
    plan_item_id: &'a str,
    title: &'a str,
    kind: &'a str,
    role: &'a str,
    aspect_ratio: &'a str,
    transparent_background: bool,
    extracted_from: Option<&'a str>,
    embedded_bytes: &'static [u8],
    prompt_used: &'a str,
}

/// Ensures the 2 built-in, non-deletable showcase example projects exist in the database and on disk:
/// 1. **Aetheris Chronicles — RPG Game UI & HUD** (`example-aetheris-chronicles`)
/// 2. **Jumpino View — Creator Discovery Platform** (`example-jumpino-view`)
/// Users can open, inspect, modify, and continue generating in these projects, while deletion is blocked.
pub async fn ensure_seed_example_projects(
    pool: &SqlitePool,
    config: &crate::config::AppConfig,
) -> AppResult<()> {
    use crate::{
        image_util::{atomic_write_file, validate_and_thumbnail_output_bytes},
        models::{
            BrandColors, BrandLayoutTokens, BrandLogoConfig, BrandSpec, BrandTypography,
            ColorPalette, DesignStyleSelection, PlanItem, PlanRequirement, PlannerPlan,
            RunModelSettings, SharedDesignSpec, TargetContext, ViewportSpec,
        },
    };
    use std::collections::HashMap;

    // ------------------------------------------------------------------------
    // Example 1: Aetheris Chronicles — RPG Game UI & HUD
    // ------------------------------------------------------------------------
    let aetheris_id = "example-aetheris-chronicles";
    let aetheris_exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM projects WHERE id = ?")
        .bind(aetheris_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0);

    if aetheris_exists == 0 {
        let now = chrono::Utc::now().to_rfc3339();
        let brand = BrandSpec {
            brand_personality: "Tactical Arcane Obsidian & Gilded Dark Fantasy".into(),
            design_direction: "Ornate dark-fantasy RPG HUD & equipment interface with precision obsidian-glass panels, engraved gold filigree borders, luminous mana/health globes, and modular inventory slot frames.".into(),
            tone_of_voice: "Heroic, tactical, crisp RPG terminology".into(),
            ui_density: "Compact tactical HUD".into(),
            content_language: "en".into(),
            colors: BrandColors {
                light: ColorPalette {
                    primary: "#7C3AED".into(),
                    accent: "#F59E0B".into(),
                    background: "#0B0D14".into(),
                    surface: "#131724".into(),
                    text: "#F8FAFC".into(),
                    muted_text: "#94A3B8".into(),
                    border: "#2E364F".into(),
                    success: "#10B981".into(),
                    warning: "#F59E0B".into(),
                    danger: "#EF4444".into(),
                },
                dark: ColorPalette {
                    primary: "#8B5CF6".into(),
                    accent: "#F59E0B".into(),
                    background: "#08090F".into(),
                    surface: "#121521".into(),
                    text: "#F8FAFC".into(),
                    muted_text: "#94A3B8".into(),
                    border: "#2A314A".into(),
                    success: "#10B981".into(),
                    warning: "#F59E0B".into(),
                    danger: "#EF4444".into(),
                },
            },
            typography: BrandTypography {
                heading_family: "Cinzel".into(),
                body_family: "Plus Jakarta Sans".into(),
                mono_family: "JetBrains Mono".into(),
                fallback_stack: "serif, system-ui".into(),
                weights: vec![400, 600, 700],
                size_scale: "HUD Compact (12px / 14px / 18px / 24px / 32px)".into(),
                line_heights: "1.2 heading / 1.45 HUD stats".into(),
                font_source: "google_fonts".into(),
                uploaded_font_asset_ids: vec![],
            },
            layout_tokens: BrandLayoutTokens {
                spacing_scale: "4px / 8px / 12px / 16px / 24px".into(),
                content_width: "100vw HUD viewport overlay".into(),
                grid_columns: "12-column tactical HUD & 6x5 inventory slot matrix".into(),
                radii: "6px chamfered HUD frames, 4px item slots".into(),
                borders: "1px metallic gold-obsidian inner/outer bevel border".into(),
                shadows: "0 16px 40px rgba(0,0,0,0.78) + arcane violet glow".into(),
                icon_style: "Faceted metallic & arcane crystal RPG iconography".into(),
                button_treatment: "Beveled obsidian-gold socketed action buttons".into(),
            },
            logo: BrandLogoConfig {
                mode: "temporary_monogram".into(),
                monogram_text: Some("AC".into()),
                primary_asset_id: None,
                mark_only_asset_id: None,
                light_variant_asset_id: None,
                dark_variant_asset_id: None,
                preferred_usage: "Top-left HUD crest anchor & inventory header".into(),
                inclusion_policy: "allowed".into(),
            },
            brand_rules: vec![
                "Treat all 3D character viewport renders and world backgrounds behind the HUD as runtime engine placeholders; build the HUD frames, inventory slots, health/mana globes, and hotbar 1:1 in code.".into(),
                "Render ornate panel dividers and glowing border filigree in code (CSS/SVG), while using the extracted transparent PNG assets for the Guild Crest and RPG Icon Sheet.".into(),
            ],
            mandatory_inclusions: vec![
                "Top-left character level crest with crimson health and arcane mana bars".into(),
                "Center equipment paperdoll & 6x5 rarity-coded inventory grid with item tooltip".into(),
                "Bottom 8-slot tactical skill hotbar with keybind badges (1-4, Q, E, R, Space)".into(),
            ],
            forbidden_treatments: vec![
                "Do not hardcode the background 3D character model as a static flat wallpaper in production; keep the HUD overlay modular.".into(),
            ],
            approved_reference_asset_ids: vec![],
            target_context: TargetContext {
                target_stack: "React + TypeScript + Tailwind CSS + Three.js / React Three Fiber".into(),
                repository_root: "./src/ui/hud".into(),
                preferred_asset_dir: "public/assets/hud".into(),
                component_library_notes: "Game UI overlay components with keyboard shortcut bindings and drag-and-drop inventory slots.".into(),
            },
            design_style: Some(DesignStyleSelection {
                style_id: "tactical-game-hud".into(),
                style_number: 18,
                style_name: "Tactical Game HUD & Dark Arcane Interface".into(),
                category: "Gaming & Interactive 3D UI".into(),
                definition: "High-contrast dark obsidian HUD frames, socketed inventory grids, glowing resource globes, and tactile keybind action bars.".into(),
                visual_rules: "1px metallic borders, translucent obsidian panels (backdrop-blur-md), rarity color-coding (Common Slate, Rare Cyan, Epic Violet, Legendary Gold).".into(),
                modifiers: vec!["Dark Mode Native".into(), "Socketed Grid".into(), "3D Viewport Overlay".into()],
                combined_brief: "Dark fantasy action-RPG HUD and inventory interface with ornate gold-obsidian frames, rarity-coded item slots, and extracted transparent crest & icon assets.".into(),
                keywords: vec!["game-ui".into(), "hud".into(), "inventory".into(), "rpg".into()],
            }),
        };

        let thread_id = "example-thread-aetheris";
        let run_id = "example-run-aetheris";

        sqlx::query(
            "INSERT INTO projects (id, name, description, purpose, industry, target_audience, audience_needs, desired_user_action, archive_status, is_example, created_at, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'active', 1, ?, ?)"
        )
        .bind(aetheris_id)
        .bind("Aetheris Chronicles — RPG Game UI & HUD")
        .bind("Tactical dark-fantasy RPG game HUD, character equipment slots, ornate item inventory grid, and interactive skill hotbar with extracted transparent Guild Crest and RPG Icon Sheet.")
        .bind("Dark Fantasy RPG Game Interface, HUD & Inventory Screen")
        .bind("Gaming / Action RPG")
        .bind("PC & Console Action-RPG Players")
        .bind("Instant combat readability, fast drag-and-drop loot management, and crisp stat comparison tooltips")
        .bind("Equip legendary gear and trigger hotbar abilities")
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query(
            "INSERT INTO brand_versions (id, project_id, version, brand_json, created_at) VALUES (?, ?, 1, ?, ?)"
        )
        .bind("example-brand-aetheris-v1")
        .bind(aetheris_id)
        .bind(serde_json::to_string(&brand).unwrap_or_default())
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query(
            "INSERT INTO threads (id, project_id, title, archive_status, created_at, updated_at) VALUES (?, ?, ?, 'active', ?, ?)"
        )
        .bind(thread_id)
        .bind(aetheris_id)
        .bind("HUD & Inventory Screen")
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        let user_prompt = "Design a 16:9 dark-fantasy RPG Game UI HUD and Inventory overlay for Aetheris Chronicles. Include the top-left player level crest with health/mana bars, a central Character Equipment & 6x5 Rarity-Coded Inventory window with a Legendary sword comparison tooltip, a top-right circular minimap frame, and a bottom 8-slot skill hotbar. Also extract the ornate Guild Crest and the RPG HUD Icon Sheet as separate transparent PNG assets in the same tab.";

        let plan = PlannerPlan {
            schema_version: 1,
            mode: "full_page".into(),
            request_summary: "16:9 Dark Fantasy RPG HUD & Inventory Interface + 2 Extracted Transparent Game UI Assets (Guild Crest & HUD Icon Sheet)".into(),
            assumptions: vec![],
            shared_design_spec: SharedDesignSpec {
                colors: HashMap::new(),
                typography: HashMap::new(),
                spacing: HashMap::new(),
                radii: HashMap::new(),
                shadows: HashMap::new(),
                layout_rules: vec![
                    "Treat the background 3D character render and world scene behind the HUD as runtime game engine placeholders; implement only the HUD frames, bars, slots, and tooltips 1:1 in code.".into(),
                ],
                asset_rules: vec![
                    "Use the extracted transparent Guild Crest PNG (item_2_guild_crest_asset) and RPG HUD Icon Sheet (item_3_hud_icons_asset), and convert flat icons to crisp vectorial SVGs.".into(),
                ],
            },
            requirements: vec![
                PlanRequirement {
                    id: "req_1".into(),
                    description: "Top-left player status unit frame with ornate Guild Crest, Level 42 badge, Crimson Health bar (2,840/2,840), and Arcane Mana bar (1,450/1,600).".into(),
                    source: "user".into(),
                },
                PlanRequirement {
                    id: "req_2".into(),
                    description: "Central split Character Equipment paperdoll + 6x5 rarity-coded Inventory Grid + hover comparison tooltip for 'Starfall Greatsword (Legendary)'.".into(),
                    source: "user".into(),
                },
                PlanRequirement {
                    id: "req_3".into(),
                    description: "Bottom 8-slot tactical ability hotbar with keybind badges (1-4, Q, E, R, Space) flanked by glowing Crimson Health and Violet Mana globes.".into(),
                    source: "user".into(),
                },
            ],
            page_composition: vec![],
            shared_components: vec![],
            items: vec![
                PlanItem {
                    id: "item_1".into(),
                    name: "Aetheris RPG HUD & Equipment Inventory Screen".into(),
                    kind: "page".into(),
                    tab_key: "hud-inventory".into(),
                    tab_label: "HUD & Inventory".into(),
                    page_route: Some("/hud/inventory".into()),
                    variant_index: 1,
                    viewport: ViewportSpec {
                        intent: "desktop".into(),
                        css_width: 1920,
                    },
                    aspect_ratio: "16:9".into(),
                    direction: "16:9 Dark Fantasy Action-RPG HUD & Equipment Inventory Screen with obsidian-gold frames, rarity-coded loot slots, and ability hotbar.".into(),
                    image_prompt: user_prompt.into(),
                    exact_copy: vec![
                        "AETHERIS CHRONICLES | LVL 42 ARCHMAGE KNIGHT".into(),
                        "EQUIPMENT & INVENTORY | STARFALL GREATSWORD — LEGENDARY GREATSWORD (842 DMG)".into(),
                    ],
                    behaviors: vec![
                        "Hovering any inventory slot reveals the glassmorphic item comparison tooltip with stat deltas (+142 Attack Power, +18% Crit Strike).".into(),
                        "Center character equipment slot supports interactive 3D model rotation on pointer drag and gentle idle floating animation.".into(),
                        "Coded Separators & Background Split: Render the engraved gold linear dividers, corner diamond nodes, and dark translucent obsidian panel backgrounds directly in CSS/SVG code.".into(),
                    ],
                    responsive_rules: vec![
                        "Scale HUD safe-zone anchors proportionally across 16:9 and 21:9 ultrawide monitors while keeping the central inventory modal max-w-5xl.".into(),
                    ],
                    asset_instructions: vec![
                        "Pair with item_2_guild_crest_asset and item_3_hud_icons_asset in the same tab.".into(),
                    ],
                    implementation_notes: vec![
                        "Do not hardcode the placeholder 3D character render behind the HUD; mount a live Three.js / engine canvas slot inside the equipment preview frame.".into(),
                    ],
                    reference_asset_ids: vec![],
                    depends_on: vec![],
                    is_visual_anchor: true,
                    transparent_background: false,
                    quality_override: Some("medium".into()),
                    extracted_from_item_id: None,
                    target_existing_output_id: None,
                    requirements_covered: vec!["req_1".into(), "req_2".into(), "req_3".into()],
                },
                PlanItem {
                    id: "item_2_guild_crest_asset".into(),
                    name: "Extracted Asset: Aetheris Ornate Gold & Obsidian Guild Crest".into(),
                    kind: "asset".into(),
                    tab_key: "hud-inventory".into(),
                    tab_label: "HUD & Inventory".into(),
                    page_route: None,
                    variant_index: 1,
                    viewport: ViewportSpec {
                        intent: "isolated".into(),
                        css_width: 1024,
                    },
                    aspect_ratio: "1:1".into(),
                    direction: "Standalone isolated transparent-background PNG of the ornate gold, obsidian, and glowing violet Aetheris Guild Crest emblem.".into(),
                    image_prompt: "Isolated standalone transparent-background PNG of the ornate dark-fantasy Aetheris Guild Crest & Level Emblem with faceted gold wings, obsidian shield, and glowing violet crystal core, 100% transparent alpha background, zero UI bars.".into(),
                    exact_copy: vec![],
                    behaviors: vec![
                        "Subtle ambient arcane pulse glow on level-up or active buff state.".into(),
                    ],
                    responsive_rules: vec![],
                    asset_instructions: vec![
                        "Use as the top-left player portrait crest frame and guild emblem; convert outer filigree rings to vectorial SVG where applicable.".into(),
                    ],
                    implementation_notes: vec![],
                    reference_asset_ids: vec![],
                    depends_on: vec!["item_1".into()],
                    is_visual_anchor: false,
                    transparent_background: true,
                    quality_override: Some("medium".into()),
                    extracted_from_item_id: Some("item_1".into()),
                    target_existing_output_id: None,
                    requirements_covered: vec!["req_1".into()],
                },
                PlanItem {
                    id: "item_3_hud_icons_asset".into(),
                    name: "Extracted Asset: RPG Tactical HUD & Loot Category Icon Sheet".into(),
                    kind: "asset".into(),
                    tab_key: "hud-inventory".into(),
                    tab_label: "HUD & Inventory".into(),
                    page_route: None,
                    variant_index: 1,
                    viewport: ViewportSpec {
                        intent: "isolated".into(),
                        css_width: 1024,
                    },
                    aspect_ratio: "1:1".into(),
                    direction: "Standalone isolated 3x3 icon sheet on a 100% transparent background featuring the RPG weapon, armor, potion, spellbook, rune, and currency icons.".into(),
                    image_prompt: "Isolated 3x3 RPG HUD & Inventory Icon Sheet on a 100% transparent background: arcane greatsword, gilded shield, crimson health elixir, azure mana crystal, ancient spell tome, dragon crest ring, rune stone, lockpick key, and gold coin stack.".into(),
                    exact_copy: vec![],
                    behaviors: vec![],
                    responsive_rules: vec![],
                    asset_instructions: vec![
                        "Slice or vectorize individual category icons into crisp SVGs or high-DPI sprite icons for inventory tabs and hotbar slots.".into(),
                    ],
                    implementation_notes: vec![],
                    reference_asset_ids: vec![],
                    depends_on: vec!["item_1".into()],
                    is_visual_anchor: false,
                    transparent_background: true,
                    quality_override: Some("medium".into()),
                    extracted_from_item_id: Some("item_1".into()),
                    target_existing_output_id: None,
                    requirements_covered: vec!["req_2".into(), "req_3".into()],
                },
            ],
        };

        let model_settings = RunModelSettings {
            planner_model: "google/gemini-3.1-pro-preview".into(),
            image_model: "openai/gpt-5-image".into(),
            quality: "medium".into(),
            aspect_ratio: "16:9".into(),
            output_size: "auto".into(),
            scope_preset: "page_view".into(),
            custom_dimensions: None,
            review_plan_first: false,
            asset_options: None,
            target_tab_key: Some("hud-inventory".into()),
            target_tab_label: Some("HUD & Inventory".into()),
            target_existing_output_id: None,
            viewport_intent: Some("desktop".into()),
            highlight_annotations: vec![],
            single_focused_output: false,
        };

        sqlx::query(
            "INSERT INTO runs (id, project_id, thread_id, status, mode, scope_preset, target_tab_key, target_tab_label, user_prompt, brand_version, brand_snapshot_json, selected_references_json, plan_json, model_settings_json, estimated_cost_micros, actual_cost_micros, cost_is_unknown, created_at, updated_at, completed_at) \
             VALUES (?, ?, ?, 'completed', 'full_page', 'page_view', 'hud-inventory', 'HUD & Inventory', ?, 1, ?, '[]', ?, ?, 0, 0, 0, ?, ?, ?)"
        )
        .bind(run_id)
        .bind(aetheris_id)
        .bind(thread_id)
        .bind(user_prompt)
        .bind(serde_json::to_string(&brand).unwrap_or_default())
        .bind(serde_json::to_string(&plan).unwrap_or_default())
        .bind(serde_json::to_string(&model_settings).unwrap_or_default())
        .bind(&now)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query(
            "INSERT INTO thread_messages (id, thread_id, role, content, linked_run_id, target_tab_key, target_tab_label, created_at) VALUES (?, ?, 'user', ?, ?, 'hud-inventory', 'HUD & Inventory', ?)"
        )
        .bind("example-msg-aetheris-1")
        .bind(thread_id)
        .bind(user_prompt)
        .bind(run_id)
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query(
            "INSERT INTO thread_messages (id, thread_id, role, content, linked_run_id, target_tab_key, target_tab_label, created_at) VALUES (?, ?, 'assistant', ?, ?, 'hud-inventory', 'HUD & Inventory', ?)"
        )
        .bind("example-msg-aetheris-2")
        .bind(thread_id)
        .bind("I have generated the **Aetheris Chronicles — RPG Game UI & HUD** in 16:9 widescreen along with its 2 standalone transparent PNG assets (**Ornate Guild Crest** and **RPG Tactical HUD Icon Sheet**) in the **HUD & Inventory** tab. Click **Copy Prompt** on any card or the full set to get the pure human-language Game UI handoff prompt (with 1:1 component framing, code-rendered filigree separators, and 3D rotation instructions).")
        .bind(run_id)
        .bind(&now)
        .execute(pool)
        .await?;

        let aetheris_outputs = [
            SeedOutputDef {
                output_id: "example-out-aetheris-1",
                plan_item_id: "item_1",
                title: "Aetheris RPG HUD & Equipment Inventory Screen",
                kind: "page",
                role: "game_hud_inventory",
                aspect_ratio: "16:9",
                transparent_background: false,
                extracted_from: None,
                embedded_bytes: include_bytes!("../../../assets/seed_examples/aetheris/01_hud_inventory.png"),
                prompt_used: user_prompt,
            },
            SeedOutputDef {
                output_id: "example-out-aetheris-2",
                plan_item_id: "item_2_guild_crest_asset",
                title: "Extracted Asset: Aetheris Ornate Gold & Obsidian Guild Crest",
                kind: "asset",
                role: "extracted_asset",
                aspect_ratio: "1:1",
                transparent_background: true,
                extracted_from: Some("example-out-aetheris-1"),
                embedded_bytes: include_bytes!("../../../assets/seed_examples/aetheris/02_guild_crest_transparent.png"),
                prompt_used: "Isolated standalone transparent-background PNG of the ornate dark-fantasy Aetheris Guild Crest & Level Emblem with faceted gold wings, obsidian shield, and glowing violet crystal core.",
            },
            SeedOutputDef {
                output_id: "example-out-aetheris-3",
                plan_item_id: "item_3_hud_icons_asset",
                title: "Extracted Asset: RPG Tactical HUD & Loot Category Icon Sheet",
                kind: "asset",
                role: "extracted_asset",
                aspect_ratio: "1:1",
                transparent_background: true,
                extracted_from: Some("example-out-aetheris-1"),
                embedded_bytes: include_bytes!("../../../assets/seed_examples/aetheris/03_hud_icons_transparent.png"),
                prompt_used: "Isolated 3x3 RPG HUD & Inventory Icon Sheet on a 100% transparent background: arcane greatsword, gilded shield, crimson health elixir, azure mana crystal, ancient spell tome, dragon crest ring, rune stone, lockpick key, and gold coin stack.",
            },
        ];

        for (idx, def) in aetheris_outputs.iter().enumerate() {
            let bytes = def.embedded_bytes.to_vec();
            if !bytes.is_empty() {
                if let Ok(processed) = validate_and_thumbnail_output_bytes(bytes.clone()).await {
                    let orig_rel = format!(
                        "projects/{}/runs/{}/outputs/{}/original.png",
                        aetheris_id, run_id, def.plan_item_id
                    );
                    let thumb_rel = format!(
                        "projects/{}/runs/{}/outputs/{}/thumbnail.webp",
                        aetheris_id, run_id, def.plan_item_id
                    );
                    if let (Ok(abs_orig), Ok(abs_thumb)) = (
                        config.resolve_rel_path(&orig_rel),
                        config.resolve_rel_path(&thumb_rel),
                    ) {
                        let _ = atomic_write_file(&abs_orig, &bytes);
                        let _ = atomic_write_file(&abs_thumb, &processed.thumbnail_webp);
                    }
                    let item_spec_json = plan
                        .items
                        .iter()
                        .find(|it| it.id == def.plan_item_id)
                        .and_then(|it| serde_json::to_string(it).ok())
                        .unwrap_or_else(|| "{}".to_string());
                    let deps_json = if def.extracted_from.is_some() {
                        "[\"item_1\"]"
                    } else {
                        "[]"
                    };
                    sqlx::query(
                        "INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, tab_key, tab_label, transparent_background, extracted_from_output_id, aspect_ratio, status, compiled_prompt, item_spec_json, depends_on_json, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, actual_cost_micros, created_at, updated_at) \
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'hud-inventory', 'HUD & Inventory', ?, ?, ?, 'completed', ?, ?, ?, ?, ?, 'image/png', ?, ?, ?, ?, 0, ?, ?)"
                    )
                    .bind(def.output_id)
                    .bind(run_id)
                    .bind(aetheris_id)
                    .bind(def.plan_item_id)
                    .bind((idx + 1) as i64)
                    .bind(def.kind)
                    .bind(def.title)
                    .bind(def.prompt_used)
                    .bind(def.role)
                    .bind(if def.kind == "page" { Some("/hud/inventory") } else { None })
                    .bind(if def.transparent_background { 1 } else { 0 })
                    .bind(def.extracted_from)
                    .bind(def.aspect_ratio)
                    .bind(def.prompt_used)
                    .bind(item_spec_json)
                    .bind(deps_json)
                    .bind(&orig_rel)
                    .bind(&thumb_rel)
                    .bind(processed.width as i64)
                    .bind(processed.height as i64)
                    .bind(processed.byte_size as i64)
                    .bind(if processed.has_alpha || def.transparent_background { 1 } else { 0 })
                    .bind(&now)
                    .bind(&now)
                    .execute(pool)
                    .await?;
                }
            }
        }
    }

    // ------------------------------------------------------------------------
    // Example 2: Jumpino View — Creator Discovery Platform
    // ------------------------------------------------------------------------
    let jumpino_id = "example-jumpino-view";
    let jumpino_exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM projects WHERE id = ?")
        .bind(jumpino_id)
        .fetch_one(pool)
        .await
        .unwrap_or(0);

    if jumpino_exists == 0 {
        let now = chrono::Utc::now().to_rfc3339();
        let brand = BrandSpec {
            brand_personality: "Warm Tactile Claymorphism & Editorial Studio".into(),
            design_direction: "High-craft creator discovery landing page blending warm cream editorial surfaces (#FAF6F0), terracotta & indigo accents (#E15B3E / #4F46E5), tactile 3D clay botanical & spark props, and modular bento showcase cards.".into(),
            tone_of_voice: "Inspiring, creator-first, human and inviting".into(),
            ui_density: "Balanced editorial bento".into(),
            content_language: "en".into(),
            colors: BrandColors {
                light: ColorPalette {
                    primary: "#E15B3E".into(),
                    accent: "#4F46E5".into(),
                    background: "#FAF6F0".into(),
                    surface: "#FFFFFF".into(),
                    text: "#18181B".into(),
                    muted_text: "#64748B".into(),
                    border: "#E6DFD5".into(),
                    success: "#16A34A".into(),
                    warning: "#D97706".into(),
                    danger: "#DC2626".into(),
                },
                dark: ColorPalette {
                    primary: "#F97316".into(),
                    accent: "#6366F1".into(),
                    background: "#0F111A".into(),
                    surface: "#181B28".into(),
                    text: "#F8FAFC".into(),
                    muted_text: "#94A3B8".into(),
                    border: "#272C40".into(),
                    success: "#22C55E".into(),
                    warning: "#F59E0B".into(),
                    danger: "#EF4444".into(),
                },
            },
            typography: BrandTypography {
                heading_family: "Fraunces".into(),
                body_family: "Plus Jakarta Sans".into(),
                mono_family: "JetBrains Mono".into(),
                fallback_stack: "serif, system-ui".into(),
                weights: vec![400, 600, 700],
                size_scale: "Editorial Display (14px / 16px / 20px / 32px / 52px)".into(),
                line_heights: "1.12 display / 1.55 body".into(),
                font_source: "google_fonts".into(),
                uploaded_font_asset_ids: vec![],
            },
            layout_tokens: BrandLayoutTokens {
                spacing_scale: "8px / 16px / 24px / 40px / 64px".into(),
                content_width: "1280px max-w-7xl container".into(),
                grid_columns: "12-column responsive bento grid (gap-6)".into(),
                radii: "24px bento cards, 9999px pill CTAs".into(),
                borders: "1px warm sand border (#E6DFD5) with soft inner highlight".into(),
                shadows: "0 20px 45px -15px rgba(24,24,27,0.08)".into(),
                icon_style: "Tactile 3D clay & crisp vector duo-tone icons".into(),
                button_treatment: "Pill CTA buttons with warm terracotta gradient & spring hover lift".into(),
            },
            logo: BrandLogoConfig {
                mode: "temporary_monogram".into(),
                monogram_text: Some("JV".into()),
                primary_asset_id: None,
                mark_only_asset_id: None,
                light_variant_asset_id: None,
                dark_variant_asset_id: None,
                preferred_usage: "Top-left sticky navbar & footer brand column".into(),
                inclusion_policy: "allowed".into(),
            },
            brand_rules: vec![
                "Build all bento cards, creator spotlight badges, and stat pills 1:1 in code while treating sample creator names, view counts, and portrait photos inside cards as dynamic database placeholders.".into(),
                "Use the extracted transparent PNGs for the Jumpino View Brand Logo, the 3D Clay Potted Plant & Spark Tile, and the Clay Feature Icon Sheet.".into(),
            ],
            mandatory_inclusions: vec![
                "Sticky glassmorphic top navbar with Jumpino View logo, Explore/Creators/Pricing links, and 'Start Creating' CTA".into(),
                "Split hero with 'Where Visual Creators Turn Ideas Into Living Worlds' headline, 3D clay plant & spark composition, and live creator metrics".into(),
                "3-card Bento feature grid + Creator Spotlight strip + bottom conversion banner".into(),
            ],
            forbidden_treatments: vec![
                "Never crop text or buttons as flat raster images; render all typography, pill badges, and section dividers in semantic HTML/CSS.".into(),
            ],
            approved_reference_asset_ids: vec![],
            target_context: TargetContext {
                target_stack: "Next.js 15 App Router + TypeScript + Tailwind CSS + Framer Motion".into(),
                repository_root: "./src/app".into(),
                preferred_asset_dir: "public/assets/jumpino".into(),
                component_library_notes: "Responsive bento landing page with Framer Motion spring hover physics and interactive 3D tilt cards.".into(),
            },
            design_style: Some(DesignStyleSelection {
                style_id: "tactile-clay-bento".into(),
                style_number: 11,
                style_name: "Tactile Claymorphism & Editorial Bento Studio".into(),
                category: "Creator Economy & Consumer Web".into(),
                definition: "Warm editorial typography paired with tactile 3D clay illustrations, soft rounded 24px bento cards, and playful spring micro-interactions.".into(),
                visual_rules: "Warm cream canvas (#FAF6F0), crisp white bento cards (#FFFFFF), 1px warm borders (#E6DFD5), terracotta primary CTAs (#E15B3E), and extracted transparent 3D clay props.".into(),
                modifiers: vec!["Bento Grid".into(), "3D Clay Props".into(), "Editorial Serif".into()],
                combined_brief: "Full vertical desktop landing page for Jumpino View creator discovery platform with warm editorial bento cards and extracted transparent brand logo, 3D clay potted plant, and icon sheet.".into(),
                keywords: vec!["landing-page".into(), "creator".into(), "bento".into(), "claymorphism".into()],
            }),
        };

        let thread_id = "example-thread-jumpino";
        let run_id = "example-run-jumpino";

        sqlx::query(
            "INSERT INTO projects (id, name, description, purpose, industry, target_audience, audience_needs, desired_user_action, archive_status, is_example, created_at, updated_at) \
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'active', 1, ?, ?)"
        )
        .bind(jumpino_id)
        .bind("Jumpino View — Creator Discovery Platform")
        .bind("Full desktop landing page for a creator discovery and interactive showcase platform featuring tactile 3D claymorphism, bento cards, and extracted transparent Brand Logo, 3D Clay Plant, and Icon Sheet.")
        .bind("Full Desktop Landing Page & Brand Asset Kit")
        .bind("Creator Economy & Interactive Media")
        .bind("Independent digital artists, 3D designers, and interactive storytellers")
        .bind("A beautiful portfolio hub to publish interactive work, grow an audience, and monetize memberships")
        .bind("Click 'Start Your Studio' or explore trending creator worlds")
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query(
            "INSERT INTO brand_versions (id, project_id, version, brand_json, created_at) VALUES (?, ?, 1, ?, ?)"
        )
        .bind("example-brand-jumpino-v1")
        .bind(jumpino_id)
        .bind(serde_json::to_string(&brand).unwrap_or_default())
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query(
            "INSERT INTO threads (id, project_id, title, archive_status, created_at, updated_at) VALUES (?, ?, ?, 'active', ?, ?)"
        )
        .bind(thread_id)
        .bind(jumpino_id)
        .bind("Landing Page & Brand Assets")
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        let user_prompt = "Design a full vertical desktop landing page (2:3) for Jumpino View, a creator discovery and interactive studio platform. Include the sticky top navbar with the Jumpino View brand logo, hero section with the tactile 3D clay potted plant and floating spark tile, 3-card Bento feature grid, Creator Spotlight cards, and footer CTA. Also extract the Brand Logo, the 3D Clay Potted Plant, and the Clay Feature Icon Sheet as separate transparent-background PNGs in the same tab.";

        let plan = PlannerPlan {
            schema_version: 1,
            mode: "full_page".into(),
            request_summary: "Full 2:3 Desktop Landing Page for Jumpino View + 3 Extracted Transparent PNG Assets (Brand Logo, 3D Clay Potted Plant, and Clay Feature Icon Sheet)".into(),
            assumptions: vec![],
            shared_design_spec: SharedDesignSpec {
                colors: HashMap::new(),
                typography: HashMap::new(),
                spacing: HashMap::new(),
                radii: HashMap::new(),
                shadows: HashMap::new(),
                layout_rules: vec![
                    "Build all bento cards, stat badges, and navigation bars 1:1 in code; treat creator names, follower counts, and sample thumbnails as dynamic database placeholders.".into(),
                ],
                asset_rules: vec![
                    "Use the 3 extracted transparent PNG assets (item_2_brand_logo_asset, item_3_3d_clay_plant_asset, item_4_icon_sheet_asset) and convert flat logos/icons to vectorial SVGs.".into(),
                ],
            },
            requirements: vec![
                PlanRequirement {
                    id: "req_1".into(),
                    description: "Sticky top navigation bar with Jumpino View logo mark, Explore, Creators, Studio, Pricing links, and 'Start Your Studio' pill CTA.".into(),
                    source: "user".into(),
                },
                PlanRequirement {
                    id: "req_2".into(),
                    description: "Split editorial hero section with headline, dual CTAs, live creator stats, and the tactile 3D clay potted plant & floating spark tile.".into(),
                    source: "user".into(),
                },
                PlanRequirement {
                    id: "req_3".into(),
                    description: "3-card Bento feature grid + Creator Spotlight cards + bottom conversion banner.".into(),
                    source: "user".into(),
                },
            ],
            page_composition: vec![],
            shared_components: vec![],
            items: vec![
                PlanItem {
                    id: "item_1".into(),
                    name: "Jumpino View — Full Desktop Landing Page".into(),
                    kind: "page".into(),
                    tab_key: "landing-page".into(),
                    tab_label: "Landing Page".into(),
                    page_route: Some("/".into()),
                    variant_index: 1,
                    viewport: ViewportSpec {
                        intent: "desktop".into(),
                        css_width: 1440,
                    },
                    aspect_ratio: "2:3".into(),
                    direction: "Warm tactile claymorphic & editorial desktop landing page with hero, 3-card bento grid, creator spotlight strip, and conversion footer.".into(),
                    image_prompt: user_prompt.into(),
                    exact_copy: vec![
                        "Jumpino View | Where Visual Creators Turn Ideas Into Living Worlds".into(),
                        "Start Your Studio | Explore Trending Worlds | 48,000+ Active Creators".into(),
                    ],
                    behaviors: vec![
                        "Bento cards lift by -4px with Framer Motion spring physics (stiffness 320, damping 24) on hover.".into(),
                        "The 3D Clay Potted Plant & Floating Spark Tile (item_3_3d_clay_plant_asset) features a gentle floating y-axis bob and interactive 3D tilt on pointer move.".into(),
                        "Coded Separators & Background Split: Render the warm cream ambient radial background gradient and 1px sand section dividers directly in CSS.".into(),
                    ],
                    responsive_rules: vec![
                        "12-column desktop bento grid collapses to 2 columns on tablet and 1 column stack on mobile (<768px).".into(),
                    ],
                    asset_instructions: vec![
                        "Mount the extracted transparent Brand Logo (item_2_brand_logo_asset), 3D Clay Plant (item_3_3d_clay_plant_asset), and Clay Icon Sheet (item_4_icon_sheet_asset).".into(),
                    ],
                    implementation_notes: vec![],
                    reference_asset_ids: vec![],
                    depends_on: vec![],
                    is_visual_anchor: true,
                    transparent_background: false,
                    quality_override: Some("medium".into()),
                    extracted_from_item_id: None,
                    target_existing_output_id: None,
                    requirements_covered: vec!["req_1".into(), "req_2".into(), "req_3".into()],
                },
                PlanItem {
                    id: "item_2_brand_logo_asset".into(),
                    name: "Extracted Asset: Jumpino View Brand Logo & Wordmark".into(),
                    kind: "asset".into(),
                    tab_key: "landing-page".into(),
                    tab_label: "Landing Page".into(),
                    page_route: None,
                    variant_index: 1,
                    viewport: ViewportSpec {
                        intent: "isolated".into(),
                        css_width: 1024,
                    },
                    aspect_ratio: "1:1".into(),
                    direction: "Standalone isolated transparent-background PNG of the Jumpino View geometric prism-lens emblem and wordmark.".into(),
                    image_prompt: "Isolated standalone transparent-background PNG of the Jumpino View brand logo emblem and crisp geometric wordmark in warm terracotta and indigo, 100% transparent alpha background.".into(),
                    exact_copy: vec![],
                    behaviors: vec![],
                    responsive_rules: vec![],
                    asset_instructions: vec![
                        "Convert the clean geometric logo mark into a crisp inline SVG for the sticky navbar and footer.".into(),
                    ],
                    implementation_notes: vec![],
                    reference_asset_ids: vec![],
                    depends_on: vec!["item_1".into()],
                    is_visual_anchor: false,
                    transparent_background: true,
                    quality_override: Some("medium".into()),
                    extracted_from_item_id: Some("item_1".into()),
                    target_existing_output_id: None,
                    requirements_covered: vec!["req_1".into()],
                },
                PlanItem {
                    id: "item_3_3d_clay_plant_asset".into(),
                    name: "Extracted Asset: 3D Clay Potted Monstera Plant & Floating Spark Tile".into(),
                    kind: "asset".into(),
                    tab_key: "landing-page".into(),
                    tab_label: "Landing Page".into(),
                    page_route: None,
                    variant_index: 1,
                    viewport: ViewportSpec {
                        intent: "isolated".into(),
                        css_width: 1024,
                    },
                    aspect_ratio: "1:1".into(),
                    direction: "Standalone isolated transparent-background PNG of the tactile 3D clay potted monstera plant and floating terracotta spark badge from the hero section.".into(),
                    image_prompt: "Isolated standalone transparent-background PNG of the tactile matte 3D clay potted monstera plant in a ribbed ceramic pot with a floating warm terracotta spark tile, 100% transparent alpha background.".into(),
                    exact_copy: vec![],
                    behaviors: vec![
                        "Animate with a continuous subtle floating y-axis oscillation and interactive 3D tilt on cursor move.".into(),
                    ],
                    responsive_rules: vec![],
                    asset_instructions: vec![
                        "Mount in the right column of the hero showcase card as a high-DPI transparent WebP/PNG layer.".into(),
                    ],
                    implementation_notes: vec![],
                    reference_asset_ids: vec![],
                    depends_on: vec!["item_1".into()],
                    is_visual_anchor: false,
                    transparent_background: true,
                    quality_override: Some("medium".into()),
                    extracted_from_item_id: Some("item_1".into()),
                    target_existing_output_id: None,
                    requirements_covered: vec!["req_2".into()],
                },
                PlanItem {
                    id: "item_4_icon_sheet_asset".into(),
                    name: "Extracted Asset: Tactile 3D Clay Feature & Navigation Icon Sheet".into(),
                    kind: "asset".into(),
                    tab_key: "landing-page".into(),
                    tab_label: "Landing Page".into(),
                    page_route: None,
                    variant_index: 1,
                    viewport: ViewportSpec {
                        intent: "isolated".into(),
                        css_width: 1024,
                    },
                    aspect_ratio: "1:1".into(),
                    direction: "Standalone isolated 2x3 icon sheet on a 100% transparent background featuring the tactile 3D clay bento feature icons.".into(),
                    image_prompt: "Isolated 2x3 tactile 3D clay icon sheet on a 100% transparent background: discovery compass, creator spark wand, interactive layers cube, community heart badge, analytics growth prism, and verified shield.".into(),
                    exact_copy: vec![],
                    behaviors: vec![],
                    responsive_rules: vec![],
                    asset_instructions: vec![
                        "Slice individual clay icons for the 3-card Bento feature grid and category pills.".into(),
                    ],
                    implementation_notes: vec![],
                    reference_asset_ids: vec![],
                    depends_on: vec!["item_1".into()],
                    is_visual_anchor: false,
                    transparent_background: true,
                    quality_override: Some("medium".into()),
                    extracted_from_item_id: Some("item_1".into()),
                    target_existing_output_id: None,
                    requirements_covered: vec!["req_3".into()],
                },
            ],
        };

        let model_settings = RunModelSettings {
            planner_model: "google/gemini-3.1-pro-preview".into(),
            image_model: "openai/gpt-5-image".into(),
            quality: "medium".into(),
            aspect_ratio: "2:3".into(),
            output_size: "auto".into(),
            scope_preset: "page_view".into(),
            custom_dimensions: None,
            review_plan_first: false,
            asset_options: None,
            target_tab_key: Some("landing-page".into()),
            target_tab_label: Some("Landing Page".into()),
            target_existing_output_id: None,
            viewport_intent: Some("desktop".into()),
            highlight_annotations: vec![],
            single_focused_output: false,
        };

        sqlx::query(
            "INSERT INTO runs (id, project_id, thread_id, status, mode, scope_preset, target_tab_key, target_tab_label, user_prompt, brand_version, brand_snapshot_json, selected_references_json, plan_json, model_settings_json, estimated_cost_micros, actual_cost_micros, cost_is_unknown, created_at, updated_at, completed_at) \
             VALUES (?, ?, ?, 'completed', 'full_page', 'page_view', 'landing-page', 'Landing Page', ?, 1, ?, '[]', ?, ?, 0, 0, 0, ?, ?, ?)"
        )
        .bind(run_id)
        .bind(jumpino_id)
        .bind(thread_id)
        .bind(user_prompt)
        .bind(serde_json::to_string(&brand).unwrap_or_default())
        .bind(serde_json::to_string(&plan).unwrap_or_default())
        .bind(serde_json::to_string(&model_settings).unwrap_or_default())
        .bind(&now)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query(
            "INSERT INTO thread_messages (id, thread_id, role, content, linked_run_id, target_tab_key, target_tab_label, created_at) VALUES (?, ?, 'user', ?, ?, 'landing-page', 'Landing Page', ?)"
        )
        .bind("example-msg-jumpino-1")
        .bind(thread_id)
        .bind(user_prompt)
        .bind(run_id)
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query(
            "INSERT INTO thread_messages (id, thread_id, role, content, linked_run_id, target_tab_key, target_tab_label, created_at) VALUES (?, ?, 'assistant', ?, ?, 'landing-page', 'Landing Page', ?)"
        )
        .bind("example-msg-jumpino-2")
        .bind(thread_id)
        .bind("I have generated the **Jumpino View — Full Desktop Landing Page** (`1024x1536`) along with 3 standalone transparent-background PNG assets in the **Landing Page** tab: **Brand Logo & Wordmark**, **3D Clay Potted Monstera Plant & Floating Spark Tile**, and **Tactile Clay Feature Icon Sheet**. You can inspect each asset or copy the complete AI Coding Agent handoff prompt.")
        .bind(run_id)
        .bind(&now)
        .execute(pool)
        .await?;

        let jumpino_outputs = [
            SeedOutputDef {
                output_id: "example-out-jumpino-1",
                plan_item_id: "item_1",
                title: "Jumpino View — Full Desktop Landing Page",
                kind: "page",
                role: "landing_page",
                aspect_ratio: "2:3",
                transparent_background: false,
                extracted_from: None,
                embedded_bytes: include_bytes!("../../../assets/seed_examples/jumpino_view/01_landing_page.png"),
                prompt_used: user_prompt,
            },
            SeedOutputDef {
                output_id: "example-out-jumpino-2",
                plan_item_id: "item_2_brand_logo_asset",
                title: "Extracted Asset: Jumpino View Brand Logo & Wordmark",
                kind: "asset",
                role: "extracted_asset",
                aspect_ratio: "1:1",
                transparent_background: true,
                extracted_from: Some("example-out-jumpino-1"),
                embedded_bytes: include_bytes!("../../../assets/seed_examples/jumpino_view/02_brand_logo_transparent.png"),
                prompt_used: "Isolated standalone transparent-background PNG of the Jumpino View brand logo emblem and crisp geometric wordmark in warm terracotta and indigo.",
            },
            SeedOutputDef {
                output_id: "example-out-jumpino-3",
                plan_item_id: "item_3_3d_clay_plant_asset",
                title: "Extracted Asset: 3D Clay Potted Monstera Plant & Floating Spark Tile",
                kind: "asset",
                role: "extracted_asset",
                aspect_ratio: "1:1",
                transparent_background: true,
                extracted_from: Some("example-out-jumpino-1"),
                embedded_bytes: include_bytes!("../../../assets/seed_examples/jumpino_view/03_3d_clay_plant_transparent.png"),
                prompt_used: "Isolated standalone transparent-background PNG of the tactile matte 3D clay potted monstera plant in a ribbed ceramic pot with a floating warm terracotta spark tile.",
            },
            SeedOutputDef {
                output_id: "example-out-jumpino-4",
                plan_item_id: "item_4_icon_sheet_asset",
                title: "Extracted Asset: Tactile 3D Clay Feature & Navigation Icon Sheet",
                kind: "asset",
                role: "extracted_asset",
                aspect_ratio: "1:1",
                transparent_background: true,
                extracted_from: Some("example-out-jumpino-1"),
                embedded_bytes: include_bytes!("../../../assets/seed_examples/jumpino_view/04_icon_sheet_transparent.png"),
                prompt_used: "Isolated 2x3 tactile 3D clay icon sheet on a 100% transparent background: discovery compass, creator spark wand, interactive layers cube, community heart badge, analytics growth prism, and verified shield.",
            },
        ];

        for (idx, def) in jumpino_outputs.iter().enumerate() {
            let bytes = def.embedded_bytes.to_vec();
            if !bytes.is_empty() {
                if let Ok(processed) = validate_and_thumbnail_output_bytes(bytes.clone()).await {
                    let orig_rel = format!(
                        "projects/{}/runs/{}/outputs/{}/original.png",
                        jumpino_id, run_id, def.plan_item_id
                    );
                    let thumb_rel = format!(
                        "projects/{}/runs/{}/outputs/{}/thumbnail.webp",
                        jumpino_id, run_id, def.plan_item_id
                    );
                    if let (Ok(abs_orig), Ok(abs_thumb)) = (
                        config.resolve_rel_path(&orig_rel),
                        config.resolve_rel_path(&thumb_rel),
                    ) {
                        let _ = atomic_write_file(&abs_orig, &bytes);
                        let _ = atomic_write_file(&abs_thumb, &processed.thumbnail_webp);
                    }
                    let item_spec_json = plan
                        .items
                        .iter()
                        .find(|it| it.id == def.plan_item_id)
                        .and_then(|it| serde_json::to_string(it).ok())
                        .unwrap_or_else(|| "{}".to_string());
                    let deps_json = if def.extracted_from.is_some() {
                        "[\"item_1\"]"
                    } else {
                        "[]"
                    };
                    sqlx::query(
                        "INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, tab_key, tab_label, transparent_background, extracted_from_output_id, aspect_ratio, status, compiled_prompt, item_spec_json, depends_on_json, original_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, has_alpha, actual_cost_micros, created_at, updated_at) \
                         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'landing-page', 'Landing Page', ?, ?, ?, 'completed', ?, ?, ?, ?, ?, 'image/png', ?, ?, ?, ?, 0, ?, ?)"
                    )
                    .bind(def.output_id)
                    .bind(run_id)
                    .bind(jumpino_id)
                    .bind(def.plan_item_id)
                    .bind((idx + 1) as i64)
                    .bind(def.kind)
                    .bind(def.title)
                    .bind(def.prompt_used)
                    .bind(def.role)
                    .bind(if def.kind == "page" { Some("/") } else { None })
                    .bind(if def.transparent_background { 1 } else { 0 })
                    .bind(def.extracted_from)
                    .bind(def.aspect_ratio)
                    .bind(def.prompt_used)
                    .bind(item_spec_json)
                    .bind(deps_json)
                    .bind(&orig_rel)
                    .bind(&thumb_rel)
                    .bind(processed.width as i64)
                    .bind(processed.height as i64)
                    .bind(processed.byte_size as i64)
                    .bind(if processed.has_alpha || def.transparent_background { 1 } else { 0 })
                    .bind(&now)
                    .bind(&now)
                    .execute(pool)
                    .await?;
                }
            }
        }
    }

    // Ensure all active projects have their organized folders (`images/` + `prompts.md`) synced on disk
    let _ = crate::handoff::sync_all_projects_organized_folders(pool, config).await;

    Ok(())
}

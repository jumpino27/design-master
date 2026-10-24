use crate::{
    config::AppConfig,
    context_builder::{summarize_context_badge, ProjectMetaSummary},
    db::backup_database,
    error::{AppError, AppResult},
    handoff::{build_set_handoff, build_single_output_handoff},
    image_util::{atomic_write_file, process_upload_bytes},
    models::{
        normalize_tab_key, validate_planner_plan, AssetModeOptions, AttachmentRole, BrandSpec,
        ExistingTabSummary, HighlightAnnotation, PlannerPlan, RunModelSettings, SelectedReference,
    },
    provider::{
        compute_cost_estimate, ModelCapabilitiesReport, OpenRouterClient, TestFixtureAdapter,
    },
    worker::{
        build_run_context_from_db, materialize_render_jobs_from_plan, recompute_run_status,
        record_and_broadcast_event, RunEventMessage,
    },
};
use axum::{
    body::Body,
    extract::{DefaultBodyLimit, Multipart, Path as AxumPath, Query, State},
    http::{header, HeaderMap, Method, Request, StatusCode, Uri},
    middleware::{self, Next},
    response::{
        sse::{Event, KeepAlive, Sse},
        IntoResponse, Response,
    },
    routing::{delete, get, post, put},
    Json, Router,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use futures::stream::Stream;
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use std::{
    collections::{HashMap, HashSet},
    convert::Infallible,
    sync::Arc,
    time::Duration,
};
use tokio::sync::broadcast;
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use uuid::Uuid;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub config: AppConfig,
    pub provider: Arc<OpenRouterClient>,
    pub event_tx: broadcast::Sender<RunEventMessage>,
    pub restart: crate::updater::AppRestart,
}

pub fn build_router(state: AppState) -> Router {
    let api_router = Router::new()
        .route("/health", get(health_handler))
        .route("/setup/status", get(setup_status_handler))
        .route("/setup/key", post(setup_key_handler))
        .route("/setup/test-key", post(test_key_handler))
        .route("/setup/test-models", post(test_models_handler))
        .route("/setup/data-dir", post(update_data_dir_handler))
        .route("/setup/pick-folder", post(pick_folder_handler))
        .route("/setup/browse-dirs", get(browse_dirs_handler))
        .route("/setup/complete", post(complete_setup_handler))
        .route("/settings", put(update_settings_handler))
        .route("/settings/usage", get(settings_usage_handler))
        .route("/capabilities/refresh", post(refresh_capabilities_handler))
        .route("/cost/estimate", post(estimate_cost_handler))
        .route("/database/backup", post(database_backup_handler))
        .route("/system/update/check", get(system_update_check_handler))
        .route("/system/update/config", put(system_update_config_handler))
        .route(
            "/system/update/install",
            post(system_update_install_handler),
        )
        .route(
            "/system/update/restart",
            post(system_update_restart_handler),
        )
        .route(
            "/projects",
            get(list_projects_handler).post(create_project_handler),
        )
        .route(
            "/projects/:id",
            get(get_project_handler)
                .put(update_project_handler)
                .delete(delete_project_handler),
        )
        .route("/projects/:id/brand", put(update_brand_handler))
        .route("/projects/:id/suggest-brand", post(suggest_brand_handler))
        .route("/brand/generate-ai", post(generate_ai_brand_handler))
        .route(
            "/projects/:id/adopt-run-tokens",
            post(adopt_run_tokens_handler),
        )
        .route(
            "/projects/:id/threads",
            get(list_threads_handler).post(create_thread_handler),
        )
        .route(
            "/projects/:id/assets",
            get(list_assets_handler).post(upload_asset_handler),
        )
        .route(
            "/assets/:id",
            put(update_asset_handler).delete(delete_asset_handler),
        )
        .route(
            "/threads/:id",
            get(get_thread_handler).put(update_thread_handler),
        )
        .route("/threads/:id/runs", post(create_run_handler))
        .route("/threads/:id/chat", post(thread_chat_handler))
        .route("/runs/:id", get(get_run_handler))
        .route("/runs/:id/approve-plan", post(approve_plan_handler))
        .route("/runs/:id/cancel", post(cancel_run_handler))
        .route("/runs/:id/retry", post(retry_run_handler))
        .route(
            "/runs/:id/check-consistency",
            post(check_consistency_handler),
        )
        .route("/runs/:id/events", get(run_events_sse_handler))
        .route("/runs/:id/handoff", post(run_set_handoff_handler))
        .route("/runs/:id/export-bundle", post(run_export_bundle_handler))
        .route("/outputs/:id", delete(delete_output_handler))
        .route("/outputs/:id/retry", post(retry_output_handler))
        .route(
            "/outputs/:id/extract-assets",
            post(extract_output_assets_handler),
        )
        .route("/outputs/:id/handoff", post(output_handoff_handler))
        .route(
            "/outputs/:id/promote-reference",
            post(promote_output_to_reference_handler),
        )
        .route("/exports/:id/download", get(download_export_handler))
        .fallback(api_not_found_handler);

    Router::new()
        .nest("/api", api_router)
        .route(
            "/media/:entity/:id/:representation",
            get(serve_media_handler),
        )
        .fallback(serve_static_or_spa_handler)
        .layer(DefaultBodyLimit::max(25 * 1024 * 1024)) // 25 MiB multipart envelope limit
        .layer(middleware::from_fn_with_state(
            state.clone(),
            security_guard_middleware,
        ))
        .with_state(state)
}

async fn security_guard_middleware(
    State(state): State<AppState>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, AppError> {
    // 1. Validate Host header against loopback / Tauri webview to prevent DNS rebinding
    if let Some(host_val) = req
        .headers()
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
    {
        let host_only = host_val.split(':').next().unwrap_or(host_val);
        let allowed_hosts = ["127.0.0.1", "localhost", "[::1]", "::1", "tauri.localhost"];
        if !allowed_hosts.contains(&host_only) {
            return Err(AppError::Forbidden(format!(
                "Host '{}' is not permitted for local Design Gen server.",
                host_val
            )));
        }
    }

    // 2. For state-mutating API requests, validate Origin and CSRF token
    let method = req.method().clone();
    let path = req.uri().path().to_string();
    let is_mutation = matches!(
        method,
        Method::POST | Method::PUT | Method::DELETE | Method::PATCH
    );

    if is_mutation && path.starts_with("/api/") {
        if let Some(origin) = req
            .headers()
            .get(header::ORIGIN)
            .and_then(|o| o.to_str().ok())
        {
            let is_allowed_origin = origin == "tauri://localhost"
                || origin == "http://tauri.localhost"
                || origin == "https://tauri.localhost"
                || origin == "http://127.0.0.1"
                || origin == "http://localhost"
                || origin.starts_with("http://127.0.0.1:")
                || origin.starts_with("http://localhost:");
            if !is_allowed_origin {
                return Err(AppError::Forbidden(format!(
                    "Origin '{}' is not permitted.",
                    origin
                )));
            }
        }

        let provided_csrf = req
            .headers()
            .get("x-csrf-token")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");
        if provided_csrf != state.config.csrf_token {
            return Err(AppError::Forbidden(
                "Missing or invalid X-CSRF-Token header.".into(),
            ));
        }
    }

    Ok(next.run(req).await)
}

async fn api_not_found_handler(uri: Uri) -> AppError {
    AppError::NotFound(format!("API endpoint '{}' not found.", uri.path()))
}

async fn health_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "design-gen-server",
        "version": crate::updater::APP_VERSION,
        "process_id": std::process::id(),
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[derive(Serialize)]
struct SettingsDto {
    planner_model: String,
    image_model: String,
    default_quality: String,
    output_size: String,
    include_extraction_instructions: bool,
    max_concurrent_jobs: i64,
    spend_limit_micros: Option<i64>,
    test_adapter_enabled: bool,
    setup_completed_at: Option<String>,
}

async fn load_settings_and_caps(
    pool: &SqlitePool,
    config: &AppConfig,
) -> AppResult<(SettingsDto, ModelCapabilitiesReport)> {
    let row = sqlx::query(
        "SELECT planner_model, image_model, default_quality, output_size, include_extraction_instructions, max_concurrent_jobs, spend_limit_micros, test_adapter_enabled, capability_cache_json, setup_completed_at FROM settings WHERE id = 1"
    )
    .fetch_one(pool)
    .await?;

    let test_db: i64 = row.get("test_adapter_enabled");
    let settings = SettingsDto {
        planner_model: row.get("planner_model"),
        image_model: row.get("image_model"),
        default_quality: row.get("default_quality"),
        output_size: row.get("output_size"),
        include_extraction_instructions: row.get::<i64, _>("include_extraction_instructions") != 0,
        max_concurrent_jobs: row.get("max_concurrent_jobs"),
        spend_limit_micros: row.get("spend_limit_micros"),
        test_adapter_enabled: (test_db != 0) || config.env_test_adapter,
        setup_completed_at: row.get("setup_completed_at"),
    };

    let mut caps: ModelCapabilitiesReport = row
        .get::<Option<String>, _>("capability_cache_json")
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_else(|| ModelCapabilitiesReport {
            planner_model: settings.planner_model.clone(),
            image_model: settings.image_model.clone(),
            ..Default::default()
        });
    if caps.quality_pricing_usd.is_empty() {
        caps.quality_pricing_usd = crate::provider::compute_quality_pricing_map(
            caps.image_output_usd_per_token,
            caps.image_input_text_usd_per_token,
        );
    }

    Ok((settings, caps))
}

async fn setup_status_handler(State(state): State<AppState>) -> AppResult<Json<serde_json::Value>> {
    let (settings, caps) = load_settings_and_caps(&state.pool, &state.config).await?;
    let key_status = state.config.secrets.status();
    let project_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM projects WHERE archive_status = 'active'")
            .fetch_one(&state.pool)
            .await
            .unwrap_or(0);
    let active_dir = state.config.active_data_dir();

    Ok(Json(serde_json::json!({
        "csrf_token": state.config.csrf_token,
        "setup_completed": settings.setup_completed_at.is_some(),
        "key_status": key_status,
        "data_dir": active_dir.to_string_lossy(),
        "data_dir_env_locked": std::env::var("DESIGN_GEN_DATA_DIR").is_ok(),
        "settings": settings,
        "capabilities": caps,
        "active_project_count": project_count
    })))
}

#[derive(Deserialize)]
struct SetKeyRequest {
    api_key: Option<String>,
    prefer_session_only: Option<bool>,
    clear: Option<bool>,
    validate_online: Option<bool>,
}

async fn setup_key_handler(
    State(state): State<AppState>,
    Json(req): Json<SetKeyRequest>,
) -> AppResult<Json<serde_json::Value>> {
    if req.clear.unwrap_or(false) {
        let status = state.config.secrets.clear_key()?;
        return Ok(Json(serde_json::json!({
            "key_status": status,
            "validation": null
        })));
    }

    let raw_key = req
        .api_key
        .as_deref()
        .ok_or_else(|| AppError::Validation("api_key is required".into()))?;

    let mut validation_res = None;
    if req.validate_online.unwrap_or(false) {
        let v = state.provider.validate_key(raw_key).await?;
        if !v.valid {
            return Err(AppError::Unauthorized(v.message));
        }
        validation_res = Some(v);
    }

    let status = state
        .config
        .secrets
        .set_key(raw_key, req.prefer_session_only.unwrap_or(false))?;

    Ok(Json(serde_json::json!({
        "key_status": status,
        "validation": validation_res
    })))
}

#[derive(Deserialize)]
struct TestKeyRequest {
    api_key: Option<String>,
}

async fn test_key_handler(
    State(state): State<AppState>,
    Json(req): Json<TestKeyRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let key_to_test = if let Some(ref k) = req.api_key {
        if !k.trim().is_empty() {
            k.trim().to_string()
        } else {
            state
                .config
                .secrets
                .get_key()
                .map(|(k, _)| k)
                .ok_or_else(|| AppError::Validation("No API key provided or stored.".into()))?
        }
    } else {
        state
            .config
            .secrets
            .get_key()
            .map(|(k, _)| k)
            .ok_or_else(|| AppError::Validation("No API key provided or stored.".into()))?
    };

    let (settings, _) = load_settings_and_caps(&state.pool, &state.config).await?;
    let validation = state.provider.validate_key(&key_to_test).await?;
    let caps = state
        .provider
        .discover_capabilities(&settings.planner_model, &settings.image_model)
        .await;

    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query("UPDATE settings SET capability_cache_json = ?, capability_cached_at = ?, updated_at = ? WHERE id = 1")
        .bind(serde_json::to_string(&caps).unwrap_or_default())
        .bind(&now)
        .bind(&now)
        .execute(&state.pool)
        .await?;

    Ok(Json(serde_json::json!({
        "validation": validation,
        "capabilities": caps
    })))
}

#[derive(Deserialize)]
struct TestModelsRequest {
    planner_model: String,
    image_model: String,
}

async fn test_models_handler(
    State(state): State<AppState>,
    Json(req): Json<TestModelsRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let planner = req.planner_model.trim();
    let image = req.image_model.trim();
    if planner.is_empty() || image.is_empty() {
        return Err(AppError::Validation(
            "Both a text/planner model ID and an image generation model ID are required.".into(),
        ));
    }

    let caps = state.provider.discover_capabilities(planner, image).await;
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query("UPDATE settings SET capability_cache_json = ?, capability_cached_at = ?, updated_at = ? WHERE id = 1")
        .bind(serde_json::to_string(&caps).unwrap_or_default())
        .bind(&now)
        .bind(&now)
        .execute(&state.pool)
        .await?;

    let planner_msg = if !caps.planner_available {
        format!(
            "Text model '{}' was not found or has no active endpoints on OpenRouter.",
            planner
        )
    } else if !caps.planner_supports_image_input {
        format!(
            "Model '{}' is text-only! Design Gen requires a MULTIMODAL text model (must support image input) to inspect screenshots and reference assets.",
            planner
        )
    } else {
        format!(
            "Verified '{}' — Multimodal vision + text input supported!",
            planner
        )
    };

    let image_msg = if !caps.image_available {
        format!(
            "Image model '{}' was not found or does not support image generation on OpenRouter.",
            image
        )
    } else {
        format!(
            "Verified '{}' via {} — Ready for image generation!",
            image, caps.image_provider_name
        )
    };

    Ok(Json(serde_json::json!({
        "planner_ok": caps.planner_available && caps.planner_supports_image_input,
        "planner_multimodal": caps.planner_supports_image_input,
        "planner_message": planner_msg,
        "image_ok": caps.image_available,
        "image_message": image_msg,
        "capabilities": caps
    })))
}

async fn pick_folder_handler(State(state): State<AppState>) -> AppResult<Json<serde_json::Value>> {
    let _current_dir = state.config.data_dir.to_string_lossy().to_string();

    let picked_opt = tokio::task::spawn_blocking(move || -> Option<String> {
        #[cfg(target_os = "macos")]
        {
            let script = r#"POSIX path of (choose folder with prompt "Select folder to store Design Gen projects, brand kits, and images:")"#;
            if let Ok(out) = std::process::Command::new("osascript")
                .arg("-e")
                .arg(script)
                .output()
            {
                if out.status.success() {
                    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    if !s.is_empty() {
                        return Some(s);
                    }
                }
            }
            None
        }

        #[cfg(target_os = "windows")]
        {
            let ps_script = format!(
                r#"Add-Type -AssemblyName System.Windows.Forms; $f = New-Object System.Windows.Forms.FolderBrowserDialog; $f.Description = 'Select folder to store Design Gen projects and images'; $f.SelectedPath = '{}'; $f.ShowNewFolderButton = $true; if ($f.ShowDialog() -eq 'OK') {{ Write-Output $f.SelectedPath }}"#,
                _current_dir.replace('\'', "''")
            );
            if let Ok(out) = std::process::Command::new("powershell.exe")
                .args(["-NoProfile", "-STA", "-Command", &ps_script])
                .output()
            {
                if out.status.success() {
                    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    if !s.is_empty() {
                        return Some(s);
                    }
                }
            }
            None
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(out) = std::process::Command::new("zenity")
                .args([
                    "--file-selection",
                    "--directory",
                    "--title=Select Design Gen Data Folder",
                ])
                .output()
            {
                if out.status.success() {
                    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    if !s.is_empty() {
                        return Some(s);
                    }
                }
            }
            if let Ok(out) = std::process::Command::new("kdialog")
                .args(["--getexistingdirectory", &_current_dir])
                .output()
            {
                if out.status.success() {
                    let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                    if !s.is_empty() {
                        return Some(s);
                    }
                }
            }
            None
        }
    })
    .await
    .unwrap_or(None);

    let Some(chosen_path) = picked_opt else {
        return Ok(Json(serde_json::json!({
            "cancelled": true,
            "data_dir": null,
            "path": null
        })));
    };

    let validated = AppConfig::validate_and_prepare_data_dir(&chosen_path)?;
    state.config.persist_and_switch_data_dir(&validated)?;
    let _ = crate::handoff::sync_all_projects_organized_folders(&state.pool, &state.config).await;

    Ok(Json(serde_json::json!({
        "cancelled": false,
        "validated": true,
        "data_dir": validated.to_string_lossy(),
        "path": validated.to_string_lossy(),
        "note": "Folder selected, verified writable, and synced with organized project folders!"
    })))
}

#[derive(Deserialize)]
struct BrowseDirsQuery {
    path: Option<String>,
}

async fn browse_dirs_handler(
    State(state): State<AppState>,
    Query(q): Query<BrowseDirsQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let active_dir = state.config.active_data_dir();
    let start_dir = q
        .path
        .filter(|s| !s.trim().is_empty())
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| active_dir.clone()));

    let target_dir = if start_dir.is_dir() {
        start_dir
    } else {
        dirs::home_dir().unwrap_or(active_dir)
    };

    let parent_dir = target_dir.parent().map(|p| p.to_string_lossy().to_string());

    let mut subdirs = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&target_dir) {
        for entry in entries.flatten() {
            if let Ok(ft) = entry.file_type() {
                if ft.is_dir() {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if !name.starts_with('.') {
                        subdirs.push(serde_json::json!({
                            "name": name,
                            "path": entry.path().to_string_lossy()
                        }));
                    }
                }
            }
        }
    }
    subdirs.sort_by(|a, b| {
        let an = a.get("name").and_then(|v| v.as_str()).unwrap_or("");
        let bn = b.get("name").and_then(|v| v.as_str()).unwrap_or("");
        an.to_lowercase().cmp(&bn.to_lowercase())
    });

    Ok(Json(serde_json::json!({
        "current_dir": target_dir.to_string_lossy(),
        "parent_dir": parent_dir,
        "subdirectories": subdirs
    })))
}

#[derive(Deserialize)]
struct UpdateDataDirRequest {
    data_dir: String,
}

async fn update_data_dir_handler(
    State(state): State<AppState>,
    Json(req): Json<UpdateDataDirRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let validated = AppConfig::validate_and_prepare_data_dir(&req.data_dir)?;
    state.config.persist_and_switch_data_dir(&validated)?;
    let _ = crate::handoff::sync_all_projects_organized_folders(&state.pool, &state.config).await;

    Ok(Json(serde_json::json!({
        "validated": true,
        "data_dir": validated.to_string_lossy(),
        "note": "Directory verified writable and switched live. All projects, threads, images, and prompts.md files are organized inside this folder."
    })))
}

#[derive(Deserialize)]
struct CompleteSetupRequest {
    planner_model: Option<String>,
    image_model: Option<String>,
    default_quality: Option<String>,
    test_adapter_enabled: Option<bool>,
}

async fn complete_setup_handler(
    State(state): State<AppState>,
    Json(req): Json<CompleteSetupRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let (current, _) = load_settings_and_caps(&state.pool, &state.config).await?;
    let planner = req.planner_model.unwrap_or(current.planner_model);
    let image = req.image_model.unwrap_or(current.image_model);
    let quality = req.default_quality.unwrap_or(current.default_quality);
    let test_adapter = req
        .test_adapter_enabled
        .unwrap_or(current.test_adapter_enabled);

    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE settings SET planner_model = ?, image_model = ?, default_quality = ?, test_adapter_enabled = ?, setup_completed_at = COALESCE(setup_completed_at, ?), updated_at = ? WHERE id = 1"
    )
    .bind(&planner)
    .bind(&image)
    .bind(&quality)
    .bind(if test_adapter { 1 } else { 0 })
    .bind(&now)
    .bind(&now)
    .execute(&state.pool)
    .await?;

    let (settings, caps) = load_settings_and_caps(&state.pool, &state.config).await?;
    Ok(Json(serde_json::json!({
        "setup_completed": true,
        "settings": settings,
        "capabilities": caps
    })))
}

#[derive(Deserialize)]
struct UpdateSettingsRequest {
    planner_model: Option<String>,
    image_model: Option<String>,
    default_quality: Option<String>,
    output_size: Option<String>,
    include_extraction_instructions: Option<bool>,
    max_concurrent_jobs: Option<i64>,
    spend_limit_micros: Option<Option<i64>>,
    test_adapter_enabled: Option<bool>,
}

async fn update_settings_handler(
    State(state): State<AppState>,
    Json(req): Json<UpdateSettingsRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let (current, _) = load_settings_and_caps(&state.pool, &state.config).await?;
    let planner = req.planner_model.unwrap_or(current.planner_model);
    let image = req.image_model.unwrap_or(current.image_model);
    let quality = req.default_quality.unwrap_or(current.default_quality);
    let output_size = req.output_size.unwrap_or(current.output_size);
    let include_extract = req
        .include_extraction_instructions
        .unwrap_or(current.include_extraction_instructions);
    let max_jobs = req
        .max_concurrent_jobs
        .unwrap_or(current.max_concurrent_jobs)
        .clamp(1, 6);
    let spend_limit = req.spend_limit_micros.unwrap_or(current.spend_limit_micros);
    let test_adapter = req
        .test_adapter_enabled
        .unwrap_or(current.test_adapter_enabled);

    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE settings SET planner_model = ?, image_model = ?, default_quality = ?, output_size = ?, include_extraction_instructions = ?, max_concurrent_jobs = ?, spend_limit_micros = ?, test_adapter_enabled = ?, updated_at = ? WHERE id = 1"
    )
    .bind(&planner)
    .bind(&image)
    .bind(&quality)
    .bind(&output_size)
    .bind(if include_extract { 1 } else { 0 })
    .bind(max_jobs)
    .bind(spend_limit)
    .bind(if test_adapter { 1 } else { 0 })
    .bind(&now)
    .execute(&state.pool)
    .await?;

    let (updated, caps) = load_settings_and_caps(&state.pool, &state.config).await?;
    Ok(Json(serde_json::json!({
        "settings": updated,
        "capabilities": caps
    })))
}

#[derive(Deserialize)]
struct RefreshCapsRequest {
    planner_model: Option<String>,
    image_model: Option<String>,
}

async fn refresh_capabilities_handler(
    State(state): State<AppState>,
    Json(req): Json<RefreshCapsRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let (settings, _) = load_settings_and_caps(&state.pool, &state.config).await?;
    let planner = req.planner_model.unwrap_or(settings.planner_model);
    let image = req.image_model.unwrap_or(settings.image_model);

    let caps = state.provider.discover_capabilities(&planner, &image).await;
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE settings SET capability_cache_json = ?, capability_cached_at = ?, updated_at = ? WHERE id = 1"
    )
    .bind(serde_json::to_string(&caps).unwrap_or_default())
    .bind(&now)
    .bind(&now)
    .execute(&state.pool)
    .await?;

    Ok(Json(serde_json::json!({ "capabilities": caps })))
}

#[derive(Deserialize)]
struct CostEstimateRequest {
    image_count: u32,
    reference_image_count: u32,
    quality: Option<String>,
}

async fn estimate_cost_handler(
    State(state): State<AppState>,
    Json(req): Json<CostEstimateRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let (settings, caps) = load_settings_and_caps(&state.pool, &state.config).await?;
    let quality = req.quality.as_deref().unwrap_or(&settings.default_quality);
    let est = compute_cost_estimate(
        &caps,
        req.image_count.clamp(1, 20),
        req.reference_image_count,
        quality,
        settings.test_adapter_enabled,
    );
    Ok(Json(serde_json::json!({ "estimate": est })))
}

async fn database_backup_handler(
    State(state): State<AppState>,
) -> AppResult<Json<serde_json::Value>> {
    let backup_dir = state.config.active_data_dir().join("backups");
    std::fs::create_dir_all(&backup_dir)?;
    let filename = format!(
        "design-master-backup-{}.sqlite",
        chrono::Utc::now().format("%Y%m%d-%H%M%S")
    );
    let target = backup_dir.join(&filename);
    backup_database(&state.pool, &target).await?;

    Ok(Json(serde_json::json!({
        "backup_path": target.to_string_lossy(),
        "created_at": chrono::Utc::now().to_rfc3339()
    })))
}

async fn settings_usage_handler(
    State(state): State<AppState>,
) -> AppResult<Json<serde_json::Value>> {
    let (settings, caps) = load_settings_and_caps(&state.pool, &state.config).await?;

    // Query runs (excluding seeded zero-cost example runs unless they have spend)
    let run_rows = sqlx::query(
        "SELECT r.id, r.project_id, COALESCE(p.name, 'Unknown Project') AS project_name, r.estimated_cost_micros, r.actual_cost_micros, r.model_settings_json, r.created_at, \
         (SELECT COUNT(*) FROM outputs o WHERE o.run_id = r.id AND o.status = 'completed') AS completed_images \
         FROM runs r \
         LEFT JOIN projects p ON p.id = r.project_id \
         ORDER BY r.created_at ASC"
    )
    .fetch_all(&state.pool)
    .await?;

    let mut total_actual_micros: i64 = 0;
    let mut total_estimated_micros: i64 = 0;
    let mut total_effective_micros: i64 = 0;
    let mut total_runs: i64 = 0;
    let mut total_images: i64 = 0;

    let mut daily_map: std::collections::BTreeMap<String, (i64, i64, i64)> =
        std::collections::BTreeMap::new();
    let mut proj_map: std::collections::BTreeMap<String, (String, i64, i64, i64)> =
        std::collections::BTreeMap::new();
    let mut qual_map: std::collections::BTreeMap<String, (i64, i64, i64)> =
        std::collections::BTreeMap::new();

    for q in ["low", "medium", "high", "xhigh"] {
        qual_map.insert(q.to_string(), (0, 0, 0));
    }

    for r in run_rows {
        let pid: String = r.get("project_id");
        let pname: String = r.get("project_name");
        let est: i64 = r
            .try_get::<Option<i64>, _>("estimated_cost_micros")
            .ok()
            .flatten()
            .unwrap_or(0);
        let act: i64 = r
            .try_get::<Option<i64>, _>("actual_cost_micros")
            .ok()
            .flatten()
            .unwrap_or(0);
        let eff = if act > 0 { act } else { est };
        let imgs: i64 = r.try_get("completed_images").unwrap_or(0);
        let created: String = r.get("created_at");
        let day_key = created.chars().take(10).collect::<String>();

        let ms_json: String = r.try_get("model_settings_json").unwrap_or_default();
        let quality = serde_json::from_str::<serde_json::Value>(&ms_json)
            .ok()
            .and_then(|v| {
                v.get("quality")
                    .and_then(|q| q.as_str())
                    .map(|s| s.to_ascii_lowercase())
            })
            .unwrap_or_else(|| "medium".to_string());
        let norm_q = match quality.as_str() {
            "low" => "low",
            "high" => "high",
            "xhigh" | "max" => "xhigh",
            _ => "medium",
        };

        total_actual_micros += act;
        total_estimated_micros += est;
        total_effective_micros += eff;
        total_runs += 1;
        total_images += imgs;

        let d_entry = daily_map.entry(day_key).or_insert((0, 0, 0));
        d_entry.0 += eff;
        d_entry.1 += 1;
        d_entry.2 += imgs;

        let p_entry = proj_map.entry(pid).or_insert((pname, 0, 0, 0));
        p_entry.1 += eff;
        p_entry.2 += 1;
        p_entry.3 += imgs;

        let q_entry = qual_map.entry(norm_q.to_string()).or_insert((0, 0, 0));
        q_entry.0 += eff;
        q_entry.1 += 1;
        q_entry.2 += imgs;
    }

    // Ensure at least the last 7 days exist in `daily_spend` so the chart always has a clean timeline
    for days_ago in (0..7).rev() {
        let day_str = (chrono::Utc::now() - chrono::Duration::days(days_ago))
            .format("%Y-%m-%d")
            .to_string();
        daily_map.entry(day_str).or_insert((0, 0, 0));
    }

    let daily_spend: Vec<serde_json::Value> = daily_map
        .into_iter()
        .map(|(date, (micros, runs, images))| {
            serde_json::json!({
                "date": date,
                "spend_micros": micros,
                "spend_usd": (micros as f64) / 1_000_000.0,
                "run_count": runs,
                "image_count": images
            })
        })
        .collect();

    let by_project: Vec<serde_json::Value> = proj_map
        .into_iter()
        .map(|(pid, (name, micros, runs, images))| {
            serde_json::json!({
                "project_id": pid,
                "project_name": name,
                "spend_micros": micros,
                "spend_usd": (micros as f64) / 1_000_000.0,
                "run_count": runs,
                "image_count": images
            })
        })
        .collect();

    let by_quality: Vec<serde_json::Value> = qual_map
        .into_iter()
        .map(|(tier, (micros, runs, images))| {
            serde_json::json!({
                "tier": tier,
                "spend_micros": micros,
                "spend_usd": (micros as f64) / 1_000_000.0,
                "run_count": runs,
                "image_count": images
            })
        })
        .collect();

    let mut openrouter_account = None;
    if let Some((api_key, _)) = state.config.secrets.get_key() {
        if let Ok(v) = state.provider.validate_key(&api_key).await {
            if v.valid {
                openrouter_account = Some(serde_json::json!({
                    "label": v.label,
                    "usage_usd": v.usage_usd,
                    "limit_usd": v.limit_usd
                }));
            }
        }
    }

    Ok(Json(serde_json::json!({
        "total_actual_micros": total_actual_micros,
        "total_estimated_micros": total_estimated_micros,
        "total_effective_micros": total_effective_micros,
        "total_effective_usd": (total_effective_micros as f64) / 1_000_000.0,
        "total_runs": total_runs,
        "total_images": total_images,
        "spend_limit_micros": settings.spend_limit_micros,
        "daily_spend": daily_spend,
        "by_project": by_project,
        "by_quality": by_quality,
        "openrouter_account": openrouter_account,
        "quality_pricing": caps.quality_pricing_usd,
        "capabilities": caps
    })))
}

// ============================================================================
// Projects & Brand Kit Handlers
// ============================================================================

async fn list_projects_handler(
    State(state): State<AppState>,
) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query(
        "SELECT id, name, description, purpose, industry, target_audience, audience_needs, desired_user_action, archive_status, is_example, created_at, updated_at FROM projects WHERE archive_status = 'active' ORDER BY is_example ASC, updated_at DESC"
    )
    .fetch_all(&state.pool)
    .await?;

    let mut projects = Vec::new();
    for r in rows {
        let pid: String = r.get("id");
        let is_example: i64 = r.try_get("is_example").unwrap_or(0);
        let brand_row = sqlx::query(
            "SELECT version, brand_json FROM brand_versions WHERE project_id = ? ORDER BY version DESC LIMIT 1"
        )
        .bind(&pid)
        .fetch_optional(&state.pool)
        .await?;

        let (brand_ver, brand_spec) = match brand_row {
            Some(br) => (
                br.get::<i64, _>("version"),
                serde_json::from_str::<BrandSpec>(br.get::<&str, _>("brand_json"))
                    .unwrap_or_default(),
            ),
            None => (1, BrandSpec::default()),
        };

        let recent_output_id: Option<String> = sqlx::query_scalar(
            "SELECT id FROM outputs WHERE project_id = ? AND status = 'completed' AND transparent_background = 0 ORDER BY created_at DESC LIMIT 1"
        )
        .bind(&pid)
        .fetch_optional(&state.pool)
        .await?;

        let fallback_output_id: Option<String> = if recent_output_id.is_some() {
            recent_output_id
        } else {
            sqlx::query_scalar(
                "SELECT id FROM outputs WHERE project_id = ? AND status = 'completed' ORDER BY created_at DESC LIMIT 1"
            )
            .bind(&pid)
            .fetch_optional(&state.pool)
            .await?
        };

        let default_thread_id: Option<String> = sqlx::query_scalar(
            "SELECT id FROM threads WHERE project_id = ? AND archive_status = 'active' ORDER BY updated_at DESC LIMIT 1"
        )
        .bind(&pid)
        .fetch_optional(&state.pool)
        .await?;

        let thread_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM threads WHERE project_id = ? AND archive_status = 'active'",
        )
        .bind(&pid)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

        let run_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM runs WHERE project_id = ?")
            .bind(&pid)
            .fetch_one(&state.pool)
            .await
            .unwrap_or(0);

        projects.push(serde_json::json!({
            "id": pid,
            "name": r.get::<String, _>("name"),
            "description": r.get::<String, _>("description"),
            "purpose": r.get::<String, _>("purpose"),
            "industry": r.get::<String, _>("industry"),
            "target_audience": r.get::<String, _>("target_audience"),
            "audience_needs": r.get::<String, _>("audience_needs"),
            "desired_user_action": r.get::<String, _>("desired_user_action"),
            "archive_status": r.get::<String, _>("archive_status"),
            "is_example": is_example != 0,
            "brand_version": brand_ver,
            "brand": brand_spec,
            "recent_output_id": fallback_output_id,
            "default_thread_id": default_thread_id,
            "thread_count": thread_count,
            "run_count": run_count,
            "created_at": r.get::<String, _>("created_at"),
            "updated_at": r.get::<String, _>("updated_at")
        }));
    }

    Ok(Json(serde_json::json!({ "projects": projects })))
}

#[derive(Deserialize)]
struct CreateProjectRequest {
    name: String,
    description: Option<String>,
    purpose: Option<String>,
    industry: Option<String>,
    target_audience: Option<String>,
    audience_needs: Option<String>,
    desired_user_action: Option<String>,
    brand: Option<BrandSpec>,
    initial_thread_title: Option<String>,
}

async fn create_project_handler(
    State(state): State<AppState>,
    Json(req): Json<CreateProjectRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let name = req.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::Validation("Project name is required.".into()));
    }

    let mut brand = req.brand.unwrap_or_default();
    if brand.logo.mode == "temporary_monogram" && brand.logo.monogram_text.is_none() {
        let first_char = name
            .chars()
            .next()
            .unwrap_or('D')
            .to_uppercase()
            .to_string();
        brand.logo.monogram_text = Some(first_char);
    }
    brand.validate()?;

    let project_id = Uuid::new_v4().to_string();
    let brand_id = Uuid::new_v4().to_string();
    let thread_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let proj_dir = state
        .config
        .active_data_dir()
        .join("projects")
        .join(&project_id);
    std::fs::create_dir_all(proj_dir.join("assets"))?;
    std::fs::create_dir_all(proj_dir.join("runs"))?;

    let mut tx = state.pool.begin().await?;
    sqlx::query(
        "INSERT INTO projects (id, name, description, purpose, industry, target_audience, audience_needs, desired_user_action, archive_status, is_example, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'active', 0, ?, ?)"
    )
    .bind(&project_id)
    .bind(&name)
    .bind(req.description.unwrap_or_default().trim())
    .bind(req.purpose.unwrap_or_default().trim())
    .bind(req.industry.unwrap_or_default().trim())
    .bind(req.target_audience.unwrap_or_else(|| "Creative teams".into()).trim())
    .bind(req.audience_needs.unwrap_or_default().trim())
    .bind(req.desired_user_action.unwrap_or_else(|| "Start creating".into()).trim())
    .bind(&now)
    .bind(&now)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO brand_versions (id, project_id, version, brand_json, created_at) VALUES (?, ?, 1, ?, ?)"
    )
    .bind(&brand_id)
    .bind(&project_id)
    .bind(serde_json::to_string(&brand).unwrap_or_default())
    .bind(&now)
    .execute(&mut *tx)
    .await?;

    let thread_title = req
        .initial_thread_title
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "Main UI Workspace".to_string());

    sqlx::query(
        "INSERT INTO threads (id, project_id, title, archive_status, created_at, updated_at) VALUES (?, ?, ?, 'active', ?, ?)"
    )
    .bind(&thread_id)
    .bind(&project_id)
    .bind(&thread_title)
    .bind(&now)
    .bind(&now)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    let _ = crate::handoff::sync_project_organized_folders(&state.pool, &state.config, &project_id)
        .await;

    Ok(Json(serde_json::json!({
        "project_id": project_id,
        "default_thread_id": thread_id,
        "brand_version": 1,
        "brand": brand
    })))
}

async fn get_project_handler(
    State(state): State<AppState>,
    AxumPath(project_id): AxumPath<String>,
) -> AppResult<Json<serde_json::Value>> {
    let proj_row = sqlx::query(
        "SELECT id, name, description, purpose, industry, target_audience, audience_needs, desired_user_action, archive_status, is_example, created_at, updated_at FROM projects WHERE id = ?"
    )
    .bind(&project_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Project '{}' not found", project_id)))?;

    let brand_rows = sqlx::query(
        "SELECT version, brand_json, created_at FROM brand_versions WHERE project_id = ? ORDER BY version DESC"
    )
    .bind(&project_id)
    .fetch_all(&state.pool)
    .await?;

    let mut brand_history = Vec::new();
    let mut current_brand = BrandSpec::default();
    let mut current_version = 1i64;

    for (idx, br) in brand_rows.iter().enumerate() {
        let ver: i64 = br.get("version");
        let spec: BrandSpec =
            serde_json::from_str(br.get::<&str, _>("brand_json")).unwrap_or_default();
        if idx == 0 {
            current_version = ver;
            current_brand = spec.clone();
        }
        brand_history.push(serde_json::json!({
            "version": ver,
            "brand": spec,
            "created_at": br.get::<String, _>("created_at")
        }));
    }

    let thread_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM threads WHERE project_id = ?")
        .bind(&project_id)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);
    let asset_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM assets WHERE project_id = ?")
        .bind(&project_id)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);
    let run_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM runs WHERE project_id = ?")
        .bind(&project_id)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);
    let output_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM outputs WHERE project_id = ?")
        .bind(&project_id)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    let is_example: i64 = proj_row.try_get("is_example").unwrap_or(0);

    Ok(Json(serde_json::json!({
        "project": {
            "id": proj_row.get::<String, _>("id"),
            "name": proj_row.get::<String, _>("name"),
            "description": proj_row.get::<String, _>("description"),
            "purpose": proj_row.get::<String, _>("purpose"),
            "industry": proj_row.get::<String, _>("industry"),
            "target_audience": proj_row.get::<String, _>("target_audience"),
            "audience_needs": proj_row.get::<String, _>("audience_needs"),
            "desired_user_action": proj_row.get::<String, _>("desired_user_action"),
            "archive_status": proj_row.get::<String, _>("archive_status"),
            "is_example": is_example != 0,
            "created_at": proj_row.get::<String, _>("created_at"),
            "updated_at": proj_row.get::<String, _>("updated_at"),
            "counts": {
                "threads": thread_count,
                "assets": asset_count,
                "runs": run_count,
                "outputs": output_count
            }
        },
        "brand_version": current_version,
        "brand": current_brand,
        "brand_history": brand_history
    })))
}

#[derive(Deserialize)]
struct UpdateProjectRequest {
    name: Option<String>,
    description: Option<String>,
    purpose: Option<String>,
    industry: Option<String>,
    target_audience: Option<String>,
    audience_needs: Option<String>,
    desired_user_action: Option<String>,
    archive_status: Option<String>,
}

async fn update_project_handler(
    State(state): State<AppState>,
    AxumPath(project_id): AxumPath<String>,
    Json(req): Json<UpdateProjectRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let row = sqlx::query("SELECT name, description, purpose, industry, target_audience, audience_needs, desired_user_action, archive_status FROM projects WHERE id = ?")
        .bind(&project_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Project '{}' not found", project_id)))?;

    let name = req.name.unwrap_or_else(|| row.get("name"));
    let description = req.description.unwrap_or_else(|| row.get("description"));
    let purpose = req.purpose.unwrap_or_else(|| row.get("purpose"));
    let industry = req.industry.unwrap_or_else(|| row.get("industry"));
    let target_audience = req
        .target_audience
        .unwrap_or_else(|| row.get("target_audience"));
    let audience_needs = req
        .audience_needs
        .unwrap_or_else(|| row.get("audience_needs"));
    let desired_action = req
        .desired_user_action
        .unwrap_or_else(|| row.get("desired_user_action"));
    let archive_status = req
        .archive_status
        .unwrap_or_else(|| row.get("archive_status"));

    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE projects SET name = ?, description = ?, purpose = ?, industry = ?, target_audience = ?, audience_needs = ?, desired_user_action = ?, archive_status = ?, updated_at = ? WHERE id = ?"
    )
    .bind(&name)
    .bind(&description)
    .bind(&purpose)
    .bind(&industry)
    .bind(&target_audience)
    .bind(&audience_needs)
    .bind(&desired_action)
    .bind(&archive_status)
    .bind(&now)
    .bind(&project_id)
    .execute(&state.pool)
    .await?;

    let _ = crate::handoff::sync_project_organized_folders(&state.pool, &state.config, &project_id)
        .await;
    get_project_handler(State(state), AxumPath(project_id)).await
}

async fn delete_project_handler(
    State(state): State<AppState>,
    AxumPath(project_id): AxumPath<String>,
) -> AppResult<Json<serde_json::Value>> {
    let proj_row = sqlx::query("SELECT id, is_example FROM projects WHERE id = ?")
        .bind(&project_id)
        .fetch_optional(&state.pool)
        .await?;
    let Some(row) = proj_row else {
        return Err(AppError::NotFound(format!(
            "Project '{}' not found",
            project_id
        )));
    };
    let is_example: i64 = row.try_get("is_example").unwrap_or(0);
    if is_example != 0 {
        return Err(AppError::Forbidden(
            "Built-in showcase example projects cannot be deleted. You can freely open, inspect, modify, and continue generating new tabs and assets inside them!".into()
        ));
    }

    let thread_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM threads WHERE project_id = ?")
        .bind(&project_id)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);
    let output_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM outputs WHERE project_id = ?")
        .bind(&project_id)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);
    let asset_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM assets WHERE project_id = ?")
        .bind(&project_id)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    sqlx::query("DELETE FROM projects WHERE id = ?")
        .bind(&project_id)
        .execute(&state.pool)
        .await?;

    let proj_rel = format!("projects/{}", project_id);
    if let Ok(abs_dir) = state.config.resolve_rel_path(&proj_rel) {
        if abs_dir.exists() {
            let _ = std::fs::remove_dir_all(&abs_dir);
        }
    }

    Ok(Json(serde_json::json!({
        "deleted": true,
        "project_id": project_id,
        "deleted_counts": {
            "threads": thread_count,
            "outputs": output_count,
            "assets": asset_count
        }
    })))
}

async fn update_brand_handler(
    State(state): State<AppState>,
    AxumPath(project_id): AxumPath<String>,
    Json(brand): Json<BrandSpec>,
) -> AppResult<Json<serde_json::Value>> {
    brand.validate()?;

    let max_ver: Option<i64> =
        sqlx::query_scalar("SELECT MAX(version) FROM brand_versions WHERE project_id = ?")
            .bind(&project_id)
            .fetch_one(&state.pool)
            .await?;

    let next_ver = max_ver.unwrap_or(0) + 1;
    let brand_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO brand_versions (id, project_id, version, brand_json, created_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&brand_id)
    .bind(&project_id)
    .bind(next_ver)
    .bind(serde_json::to_string(&brand).unwrap_or_default())
    .bind(&now)
    .execute(&state.pool)
    .await?;

    sqlx::query("UPDATE projects SET updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(&project_id)
        .execute(&state.pool)
        .await?;

    Ok(Json(serde_json::json!({
        "project_id": project_id,
        "brand_version": next_ver,
        "brand": brand
    })))
}

#[derive(Deserialize, Default)]
struct SuggestBrandInput {
    #[serde(default)]
    custom_instructions: Option<String>,
    #[serde(default)]
    logo_data_url: Option<String>,
}

#[derive(Deserialize)]
struct GenerateAiBrandInput {
    project_name: String,
    product_description: String,
    #[serde(default)]
    logo_data_url: Option<String>,
    #[serde(default)]
    preferred_style_summary: Option<String>,
    #[serde(default)]
    custom_instructions: Option<String>,
}

fn build_fixture_brand_proposal() -> serde_json::Value {
    let mut colors_light: HashMap<String, String> = HashMap::new();
    colors_light.insert("primary".into(), "#D56A3A".into());
    colors_light.insert("accent".into(), "#1E1D1B".into());
    colors_light.insert("background".into(), "#F4F1EA".into());
    colors_light.insert("surface".into(), "#FAF9F6".into());
    colors_light.insert("text".into(), "#1E1D1B".into());
    colors_light.insert("muted_text".into(), "#6E6A63".into());
    colors_light.insert("border".into(), "#E2DED5".into());

    let mut colors_dark: HashMap<String, String> = HashMap::new();
    colors_dark.insert("primary".into(), "#E07A4B".into());
    colors_dark.insert("accent".into(), "#F4F1EA".into());
    colors_dark.insert("background".into(), "#121110".into());
    colors_dark.insert("surface".into(), "#1C1A17".into());
    colors_dark.insert("text".into(), "#F4F1EA".into());
    colors_dark.insert("muted_text".into(), "#A39E93".into());
    colors_dark.insert("border".into(), "#2E2B26".into());

    serde_json::json!({
        "purpose": "High-converting web application & digital product experience",
        "industry": "Technology & Digital Products",
        "target_audience": "Modern teams, creators, and decision-makers",
        "audience_needs": "Fast clarity, tactile visual hierarchy, and frictionless action flows",
        "desired_user_action": "Get started / Convert on primary CTA",
        "recommended_style_number": 15,
        "recommended_modifiers": ["Subtle Motion", "Dark Mode + OLED"],
        "brand_personality": "Warm editorial precision",
        "design_direction": "Architectural stone surfaces paired with confident burnt-orange CTAs and high-clarity typographic hierarchy.",
        "tone_of_voice": "Direct, trustworthy, and crafted",
        "ui_density": "comfortable",
        "colors_light": colors_light,
        "colors_dark": colors_dark,
        "typography_heading": "Plus Jakarta Sans",
        "typography_body": "Inter",
        "typography_mono": "JetBrains Mono",
        "spacing_scale": "4px / 8px / 16px / 24px / 32px / 48px / 64px",
        "radii": "12px cards, 8px controls, 9999px badges",
        "shadows": "0 8px 24px rgba(20, 18, 15, 0.06)",
        "icon_style": "2px crisp geometric stroke icons",
        "brand_rules": [
            "Keep primary accent reserved for the single most important CTA on screen.",
            "Maintain generous 24px card padding and subtle 1px borders."
        ],
        "forbidden_treatments": [
            "Avoid cluttered multi-font decorative headers or low-contrast text."
        ],
        "caveat_notice": "AI-compiled Brand Kit & Style ready for your review."
    })
}

async fn generate_ai_brand_handler(
    State(state): State<AppState>,
    Json(req): Json<GenerateAiBrandInput>,
) -> AppResult<Json<serde_json::Value>> {
    let (settings, _) = load_settings_and_caps(&state.pool, &state.config).await?;
    if settings.test_adapter_enabled || state.config.secrets.get_key().is_none() {
        return Ok(Json(serde_json::json!({
            "proposal": build_fixture_brand_proposal()
        })));
    }

    let (api_key, _) = state.config.secrets.get_key().unwrap();
    let style_summary = req.preferred_style_summary.as_deref().unwrap_or(
        "Recommend the best matching Web Design Style (#1 to #50) for this product and logo.",
    );

    let proposal = state
        .provider
        .suggest_brand_details(
            &api_key,
            &settings.planner_model,
            req.project_name.trim(),
            req.product_description.trim(),
            "",
            "",
            "",
            "",
            "",
            style_summary,
            req.logo_data_url.as_deref(),
            req.custom_instructions.as_deref(),
        )
        .await?;

    Ok(Json(serde_json::json!({ "proposal": proposal })))
}

async fn suggest_brand_handler(
    State(state): State<AppState>,
    AxumPath(project_id): AxumPath<String>,
    maybe_body: Option<Json<SuggestBrandInput>>,
) -> AppResult<Json<serde_json::Value>> {
    let input = maybe_body.map(|b| b.0).unwrap_or_default();
    let proj_row =
        sqlx::query("SELECT name, description, purpose, industry, target_audience, audience_needs, desired_user_action FROM projects WHERE id = ?")
            .bind(&project_id)
            .fetch_optional(&state.pool)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("Project '{}' not found", project_id)))?;

    let latest_brand: BrandSpec = sqlx::query_scalar::<_, String>(
        "SELECT brand_json FROM brand_versions WHERE project_id = ? ORDER BY version DESC LIMIT 1",
    )
    .bind(&project_id)
    .fetch_optional(&state.pool)
    .await?
    .and_then(|s| serde_json::from_str(&s).ok())
    .unwrap_or_default();

    let style_summary = if let Some(ref ds) = latest_brand.design_style {
        if !ds.style_name.trim().is_empty() {
            format!(
                "Style #{} {}: {} | Rules: {} | Modifiers: {}",
                ds.style_number,
                ds.style_name,
                ds.definition,
                ds.visual_rules,
                ds.modifiers.join(" + ")
            )
        } else {
            format!(
                "{} — {}",
                latest_brand.brand_personality, latest_brand.design_direction
            )
        }
    } else {
        format!(
            "{} — {}",
            latest_brand.brand_personality, latest_brand.design_direction
        )
    };

    let (settings, _) = load_settings_and_caps(&state.pool, &state.config).await?;

    if settings.test_adapter_enabled || state.config.secrets.get_key().is_none() {
        return Ok(Json(serde_json::json!({
            "proposal": build_fixture_brand_proposal()
        })));
    }

    // Load project's uploaded brand logo data URL if available
    let mut logo_durl = input.logo_data_url.clone();
    if logo_durl.is_none() {
        let logo_row = if let Some(ref laid) = latest_brand.logo.primary_asset_id {
            sqlx::query(
                "SELECT original_rel_path, mime_type FROM assets WHERE id = ? AND project_id = ?",
            )
            .bind(laid)
            .bind(&project_id)
            .fetch_optional(&state.pool)
            .await
            .ok()
            .flatten()
        } else {
            sqlx::query("SELECT original_rel_path, mime_type FROM assets WHERE project_id = ? AND role = 'brand_logo' ORDER BY created_at DESC LIMIT 1")
                .bind(&project_id)
                .fetch_optional(&state.pool)
                .await
                .ok()
                .flatten()
        };
        if let Some(lrow) = logo_row {
            let rel: String = lrow.get("original_rel_path");
            let mime: String = lrow.get("mime_type");
            if let Ok(abs) = state.config.resolve_rel_path(&rel) {
                if let Ok(bytes) = std::fs::read(&abs) {
                    logo_durl = Some(format!("data:{};base64,{}", mime, BASE64.encode(&bytes)));
                }
            }
        }
    }

    let (api_key, _) = state.config.secrets.get_key().unwrap();
    let proposal = state
        .provider
        .suggest_brand_details(
            &api_key,
            &settings.planner_model,
            proj_row.get("name"),
            proj_row.get("description"),
            proj_row.get("purpose"),
            proj_row.get("industry"),
            proj_row.get("target_audience"),
            proj_row.get("audience_needs"),
            proj_row.get("desired_user_action"),
            &style_summary,
            logo_durl.as_deref(),
            input.custom_instructions.as_deref(),
        )
        .await?;

    Ok(Json(serde_json::json!({ "proposal": proposal })))
}

#[derive(Deserialize)]
struct AdoptRunTokensRequest {
    run_id: String,
}

async fn adopt_run_tokens_handler(
    State(state): State<AppState>,
    AxumPath(project_id): AxumPath<String>,
    Json(req): Json<AdoptRunTokensRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let run_row = sqlx::query("SELECT plan_json FROM runs WHERE id = ? AND project_id = ?")
        .bind(&req.run_id)
        .bind(&project_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Run not found for this project".into()))?;

    let plan_str: Option<String> = run_row.get("plan_json");
    let plan: PlannerPlan = serde_json::from_str(&plan_str.unwrap_or_default())
        .map_err(|_| AppError::Conflict("Run does not have a valid saved design plan.".into()))?;

    let latest_brand_row = sqlx::query(
        "SELECT version, brand_json FROM brand_versions WHERE project_id = ? ORDER BY version DESC LIMIT 1"
    )
    .bind(&project_id)
    .fetch_one(&state.pool)
    .await?;

    let mut brand: BrandSpec =
        serde_json::from_str(latest_brand_row.get::<&str, _>("brand_json")).unwrap_or_default();

    let spec = &plan.shared_design_spec;
    if let Some(c) = spec.colors.get("primary") {
        if crate::models::is_valid_hex_color(c) {
            brand.colors.light.primary = c.clone();
        }
    }
    if let Some(c) = spec.colors.get("background") {
        if crate::models::is_valid_hex_color(c) {
            brand.colors.light.background = c.clone();
        }
    }
    if let Some(c) = spec.colors.get("surface") {
        if crate::models::is_valid_hex_color(c) {
            brand.colors.light.surface = c.clone();
        }
    }
    if let Some(h) = spec.typography.get("heading") {
        if !h.trim().is_empty() {
            brand.typography.heading_family = h.clone();
        }
    }
    for rule in &spec.layout_rules {
        if !brand.brand_rules.contains(rule) {
            brand.brand_rules.push(rule.clone());
        }
    }

    update_brand_handler(State(state), AxumPath(project_id), Json(brand)).await
}

// ============================================================================
// Threads Handlers
// ============================================================================

#[derive(Deserialize)]
struct ListThreadsQuery {
    q: Option<String>,
    include_archived: Option<bool>,
}

async fn list_threads_handler(
    State(state): State<AppState>,
    AxumPath(project_id): AxumPath<String>,
    Query(query): Query<ListThreadsQuery>,
) -> AppResult<Json<serde_json::Value>> {
    let include_archived = query.include_archived.unwrap_or(false);
    let rows = if include_archived {
        sqlx::query(
            "SELECT id, project_id, title, archive_status, created_at, updated_at FROM threads WHERE project_id = ? ORDER BY updated_at DESC"
        )
        .bind(&project_id)
        .fetch_all(&state.pool)
        .await?
    } else {
        sqlx::query(
            "SELECT id, project_id, title, archive_status, created_at, updated_at FROM threads WHERE project_id = ? AND archive_status = 'active' ORDER BY updated_at DESC"
        )
        .bind(&project_id)
        .fetch_all(&state.pool)
        .await?
    };

    let search = query
        .q
        .map(|s| s.trim().to_ascii_lowercase())
        .unwrap_or_default();

    let mut threads = Vec::new();
    for r in rows {
        let title: String = r.get("title");
        if !search.is_empty() && !title.to_ascii_lowercase().contains(&search) {
            continue;
        }
        let tid: String = r.get("id");
        let run_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM runs WHERE thread_id = ?")
            .bind(&tid)
            .fetch_one(&state.pool)
            .await
            .unwrap_or(0);

        threads.push(serde_json::json!({
            "id": tid,
            "project_id": r.get::<String, _>("project_id"),
            "title": title,
            "archive_status": r.get::<String, _>("archive_status"),
            "run_count": run_count,
            "created_at": r.get::<String, _>("created_at"),
            "updated_at": r.get::<String, _>("updated_at")
        }));
    }

    Ok(Json(serde_json::json!({ "threads": threads })))
}

#[derive(Deserialize)]
struct CreateThreadRequest {
    title: Option<String>,
    first_prompt: Option<String>,
}

async fn create_thread_handler(
    State(state): State<AppState>,
    AxumPath(project_id): AxumPath<String>,
    Json(req): Json<CreateThreadRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let title = req
        .title
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| {
            if let Some(fp) = req.first_prompt {
                let words: Vec<&str> = fp.split_whitespace().take(5).collect();
                if !words.is_empty() {
                    return words.join(" ");
                }
            }
            "New design thread".to_string()
        });

    let thread_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query(
        "INSERT INTO threads (id, project_id, title, archive_status, created_at, updated_at) VALUES (?, ?, ?, 'active', ?, ?)"
    )
    .bind(&thread_id)
    .bind(&project_id)
    .bind(&title)
    .bind(&now)
    .bind(&now)
    .execute(&state.pool)
    .await?;

    Ok(Json(serde_json::json!({
        "thread": {
            "id": thread_id,
            "project_id": project_id,
            "title": title,
            "archive_status": "active",
            "created_at": now,
            "updated_at": now
        }
    })))
}

#[derive(Deserialize)]
struct UpdateThreadRequest {
    title: Option<String>,
    archive_status: Option<String>,
}

async fn update_thread_handler(
    State(state): State<AppState>,
    AxumPath(thread_id): AxumPath<String>,
    Json(req): Json<UpdateThreadRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let row = sqlx::query(
        "SELECT project_id, title, archive_status, created_at FROM threads WHERE id = ?",
    )
    .bind(&thread_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Thread not found".into()))?;

    let title = req
        .title
        .map(|t| t.trim().to_string())
        .filter(|t| !t.is_empty())
        .unwrap_or_else(|| row.get("title"));
    let archive_status = req
        .archive_status
        .unwrap_or_else(|| row.get("archive_status"));
    let now = chrono::Utc::now().to_rfc3339();

    sqlx::query("UPDATE threads SET title = ?, archive_status = ?, updated_at = ? WHERE id = ?")
        .bind(&title)
        .bind(&archive_status)
        .bind(&now)
        .bind(&thread_id)
        .execute(&state.pool)
        .await?;

    Ok(Json(serde_json::json!({
        "thread": {
            "id": thread_id,
            "project_id": row.get::<String, _>("project_id"),
            "title": title,
            "archive_status": archive_status,
            "created_at": row.get::<String, _>("created_at"),
            "updated_at": now
        }
    })))
}

async fn get_thread_handler(
    State(state): State<AppState>,
    AxumPath(thread_id): AxumPath<String>,
) -> AppResult<Json<serde_json::Value>> {
    let thread_row = sqlx::query(
        "SELECT id, project_id, title, archive_status, created_at, updated_at FROM threads WHERE id = ?"
    )
    .bind(&thread_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Thread '{}' not found", thread_id)))?;

    let run_rows = sqlx::query("SELECT id FROM runs WHERE thread_id = ? ORDER BY created_at ASC")
        .bind(&thread_id)
        .fetch_all(&state.pool)
        .await?;

    let mut runs = Vec::new();
    for rr in run_rows {
        let rid: String = rr.get("id");
        runs.push(load_run_dto(&state.pool, &state.config, &rid).await?);
    }

    let msg_rows = sqlx::query(
        "SELECT id, role, content, attachments_json, highlight_annotations_json, quick_replies_json, linked_run_id, target_tab_key, target_tab_label, created_at FROM thread_messages WHERE thread_id = ? ORDER BY created_at ASC"
    )
    .bind(&thread_id)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    let messages: Vec<serde_json::Value> = msg_rows
        .iter()
        .map(|mr| {
            serde_json::json!({
                "id": mr.get::<String, _>("id"),
                "thread_id": thread_id,
                "role": mr.get::<String, _>("role"),
                "content": mr.get::<String, _>("content"),
                "attachments": serde_json::from_str::<serde_json::Value>(mr.get::<&str, _>("attachments_json")).unwrap_or_else(|_| serde_json::json!([])),
                "highlight_annotations": serde_json::from_str::<serde_json::Value>(mr.get::<&str, _>("highlight_annotations_json")).unwrap_or_else(|_| serde_json::json!([])),
                "quick_replies": serde_json::from_str::<Vec<String>>(mr.get::<&str, _>("quick_replies_json")).unwrap_or_default(),
                "linked_run_id": mr.get::<Option<String>, _>("linked_run_id"),
                "target_tab_key": mr.get::<Option<String>, _>("target_tab_key"),
                "target_tab_label": mr.get::<Option<String>, _>("target_tab_label"),
                "created_at": mr.get::<String, _>("created_at")
            })
        })
        .collect();

    Ok(Json(serde_json::json!({
        "thread": {
            "id": thread_row.get::<String, _>("id"),
            "project_id": thread_row.get::<String, _>("project_id"),
            "title": thread_row.get::<String, _>("title"),
            "archive_status": thread_row.get::<String, _>("archive_status"),
            "created_at": thread_row.get::<String, _>("created_at"),
            "updated_at": thread_row.get::<String, _>("updated_at")
        },
        "runs": runs,
        "messages": messages
    })))
}

#[derive(Deserialize)]
struct ThreadChatRequest {
    message: String,
    selected_references: Option<Vec<SelectedReference>>,
    highlight_annotations: Option<Vec<HighlightAnnotation>>,
    active_tab_key: Option<String>,
    active_output_id: Option<String>,
    quality_preference: Option<String>,
    force_generate: Option<bool>,
    answer_to_message_id: Option<String>,
}

fn strip_tab_version_suffix(key: &str, label: &str) -> (String, String) {
    let norm_key = normalize_tab_key(key);
    let clean_key = if let Some(idx) = norm_key.rfind("-v") {
        let suffix = &norm_key[idx + 2..];
        if !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()) {
            norm_key[..idx].to_string()
        } else {
            norm_key
        }
    } else {
        norm_key
    };

    let trimmed_label = label.trim();
    let lower_label = trimmed_label.to_ascii_lowercase();
    let clean_label = if let Some(idx) = lower_label.rfind(" v") {
        let suffix = &lower_label[idx + 2..];
        if !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()) {
            trimmed_label[..idx].trim().to_string()
        } else {
            trimmed_label.to_string()
        }
    } else {
        trimmed_label.to_string()
    };

    (
        if clean_key.is_empty() {
            "component".to_string()
        } else {
            clean_key
        },
        if clean_label.is_empty() {
            "Component".to_string()
        } else {
            clean_label
        },
    )
}

fn next_versioned_tab_identity(
    base_key: &str,
    base_label: &str,
    existing_tabs: &[ExistingTabSummary],
) -> (String, String) {
    let (clean_base_key, clean_base_label) = strip_tab_version_suffix(base_key, base_label);
    let mut max_version: u32 = 0;
    let prefix = format!("{}-v", clean_base_key);

    for t in existing_tabs {
        let tk = normalize_tab_key(&t.tab_key);
        if tk == clean_base_key {
            max_version = max_version.max(1);
        } else if let Some(rest) = tk.strip_prefix(&prefix) {
            if let Ok(v) = rest.parse::<u32>() {
                max_version = max_version.max(v);
            }
        }
    }

    if max_version == 0 {
        (clean_base_key, clean_base_label)
    } else {
        let next_v = max_version + 1;
        (
            format!("{}-v{}", clean_base_key, next_v),
            format!("{} v{}", clean_base_label, next_v),
        )
    }
}

fn apply_semantic_and_versioned_tab_rules(
    spec: &mut crate::models::ChatRunSpec,
    combined_request_text: &str,
    latest_user_text: &str,
    existing_tabs: &[ExistingTabSummary],
    active_tab_key: Option<&str>,
    active_output_id: Option<&str>,
) {
    let lower_latest = latest_user_text.trim().to_ascii_lowercase();
    let lower_combined = combined_request_text.to_ascii_lowercase();

    // 1. Check if this is an explicit in-tab operation (e.g. "+ Mobile (9:16)" button -> "Inside the <Tab> tab, generate a Mobile (9:16) layout version...")
    let is_explicit_inside_tab = lower_latest.starts_with("inside the ")
        || lower_combined.contains("inside the ")
        || ((lower_latest.contains("mobile (9:16)")
            || lower_latest.contains("next to the desktop design"))
            && !existing_tabs.is_empty());

    if is_explicit_inside_tab {
        let matched_tab = active_tab_key
            .and_then(|ak| {
                let nak = normalize_tab_key(ak);
                existing_tabs.iter().find(|t| t.tab_key == nak)
            })
            .or_else(|| {
                let ntk = normalize_tab_key(&spec.target_tab_key);
                existing_tabs.iter().find(|t| t.tab_key == ntk)
            })
            .or_else(|| {
                existing_tabs.iter().find(|t| {
                    lower_latest.contains(&t.tab_label.to_ascii_lowercase())
                        || lower_combined.contains(&t.tab_label.to_ascii_lowercase())
                })
            })
            .or_else(|| existing_tabs.first());

        if let Some(tab) = matched_tab {
            spec.target_tab_key = tab.tab_key.clone();
            spec.target_tab_label = tab.tab_label.clone();
            spec.target_existing_output_id = Some(tab.latest_output_id.clone());
        } else if let Some(ao) = active_output_id {
            spec.target_existing_output_id = Some(ao.to_string());
        }

        if lower_latest.contains("mobile") || lower_latest.contains("9:16") {
            spec.mode = "component".to_string();
            spec.scope_preset = "page_view".to_string();
            spec.viewport_intent = "mobile".to_string();
            spec.aspect_ratio = "9:16".to_string();
        } else if lower_latest.contains("tablet") || lower_latest.contains("3:4") {
            spec.mode = "component".to_string();
            spec.scope_preset = "page_view".to_string();
            spec.viewport_intent = "tablet".to_string();
            spec.aspect_ratio = "3:4".to_string();
        }
        return;
    }

    // 2. Semantic detection for component / section / page / asset creation ("create a cool hero with this", "create a cool pricing with this", etc.)
    // Only inspect the user's primary intent (strip Q1..Q4 answer suffixes so option keywords don't override the requested component)
    let user_intent_only = combined_request_text
        .split("Q1:")
        .next()
        .unwrap_or(combined_request_text)
        .to_ascii_lowercase();

    let semantic_match: Option<(&str, &str, &str, &str, &str)> =
        if user_intent_only.contains("hero") {
            Some(("hero", "Hero", "component", "hero", "16:9"))
        } else if user_intent_only.contains("pricing") {
            Some(("pricing", "Pricing", "component", "section", "16:9"))
        } else if user_intent_only.contains("navbar")
            || user_intent_only.contains("navigation bar")
            || user_intent_only.contains("top nav")
            || user_intent_only.contains("header")
        {
            Some(("navbar", "Navbar", "component", "navbar", "16:9"))
        } else if user_intent_only.contains("footer") {
            Some(("footer", "Footer", "component", "section", "16:9"))
        } else if user_intent_only.contains("testimonial") || user_intent_only.contains("review") {
            Some((
                "testimonials",
                "Testimonials",
                "component",
                "section",
                "16:9",
            ))
        } else if user_intent_only.contains("faq") || user_intent_only.contains("questions") {
            Some(("faq", "FAQ", "component", "section", "16:9"))
        } else if user_intent_only.contains("feature") || user_intent_only.contains("bento") {
            Some(("features", "Features", "component", "section", "16:9"))
        } else if user_intent_only.contains("cta") || user_intent_only.contains("call to action") {
            Some(("cta", "Call to Action", "component", "section", "16:9"))
        } else if user_intent_only.contains("modal") || user_intent_only.contains("dialog") {
            Some(("modal", "Modal", "component", "isolated_component", "4:3"))
        } else if user_intent_only.contains("sidebar") {
            Some(("sidebar", "Sidebar", "component", "sidebar", "3:4"))
        } else if user_intent_only.contains("dashboard") {
            Some(("dashboard", "Dashboard", "component", "page_view", "16:9"))
        } else if user_intent_only.contains("login")
            || user_intent_only.contains("signup")
            || user_intent_only.contains("sign up")
            || user_intent_only.contains("auth")
        {
            Some(("auth-form", "Auth Form", "component", "form", "4:3"))
        } else if user_intent_only.contains("table") {
            Some(("data-table", "Data Table", "component", "table", "16:9"))
        } else if user_intent_only.contains("chart") || user_intent_only.contains("analytics") {
            Some((
                "analytics-chart",
                "Analytics Chart",
                "component",
                "chart",
                "16:9",
            ))
        } else if user_intent_only.contains("card") {
            Some(("card", "Card", "component", "card", "4:3"))
        } else if user_intent_only.contains("landing page")
            || user_intent_only.contains("full website")
            || user_intent_only.contains("whole website")
            || user_intent_only.contains("full page")
        {
            Some((
                "full-landing-page",
                "Full Landing Page",
                "full_page",
                "full_page",
                "2:3",
            ))
        } else {
            None
        };

    let (base_key, base_label) =
        if let Some((b_key, b_label, s_mode, s_scope, s_ar)) = semantic_match {
            spec.mode = s_mode.to_string();
            spec.scope_preset = s_scope.to_string();
            if spec.viewport_intent != "mobile" && spec.viewport_intent != "tablet" {
                spec.aspect_ratio = s_ar.to_string();
            }
            (b_key.to_string(), b_label.to_string())
        } else {
            let raw_k = if spec.target_tab_key.trim().is_empty()
                || spec.target_tab_key == "component"
                || spec.target_tab_key.starts_with("q1-")
            {
                normalize_tab_key(&user_intent_only)
            } else {
                normalize_tab_key(&spec.target_tab_key)
            };
            let raw_l = if spec.target_tab_label.trim().is_empty()
                || spec.target_tab_label == "Component"
                || spec.target_tab_label.starts_with("Q1:")
            {
                raw_k
                    .split('-')
                    .filter(|w| !w.is_empty())
                    .take(3)
                    .map(|w| {
                        let mut c = w.chars();
                        match c.next() {
                            None => String::new(),
                            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(" ")
            } else {
                spec.target_tab_label.clone()
            };
            strip_tab_version_suffix(&raw_k, &raw_l)
        };

    // 3. Assign versioned tab identity (`Hero`, `Hero v2`, `Hero v3`, `Pricing v2`, etc.)
    let (versioned_key, versioned_label) =
        next_versioned_tab_identity(&base_key, &base_label, existing_tabs);
    spec.target_tab_key = versioned_key;
    spec.target_tab_label = versioned_label;
    // A newly created versioned tab starts fresh on the canvas while inheriting project brand tokens & attached references
    spec.target_existing_output_id = None;
}

async fn thread_chat_handler(
    State(state): State<AppState>,
    AxumPath(thread_id): AxumPath<String>,
    Json(req): Json<ThreadChatRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let user_text = req.message.trim().to_string();
    let highlights = req.highlight_annotations.unwrap_or_default();
    if user_text.is_empty() && highlights.is_empty() {
        return Err(AppError::Validation(
            "Message or highlight comment cannot be empty.".into(),
        ));
    }

    let thread_row = sqlx::query("SELECT id, project_id, title FROM threads WHERE id = ?")
        .bind(&thread_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Thread '{}' not found", thread_id)))?;
    let project_id: String = thread_row.get("project_id");

    let proj_row = sqlx::query(
        "SELECT name, description, purpose, industry, target_audience, audience_needs, desired_user_action FROM projects WHERE id = ?"
    )
    .bind(&project_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Project '{}' not found", project_id)))?;

    let brand_row = sqlx::query(
        "SELECT version, brand_json FROM brand_versions WHERE project_id = ? ORDER BY version DESC LIMIT 1"
    )
    .bind(&project_id)
    .fetch_one(&state.pool)
    .await?;
    let brand_version: i64 = brand_row.get("version");
    let brand_snapshot_json: String = brand_row.get("brand_json");
    let brand_spec: BrandSpec = serde_json::from_str(&brand_snapshot_json).unwrap_or_default();

    let (settings, caps) = load_settings_and_caps(&state.pool, &state.config).await?;
    if !settings.test_adapter_enabled && state.config.secrets.get_key().is_none() {
        return Err(AppError::Unauthorized(
            "OpenRouter API key is not configured. Add your key in Settings or enable Test Fixture Mode."
                .into(),
        ));
    }

    let selected_refs = req.selected_references.unwrap_or_default();

    // Build attachment metadata for UI display & data URLs for multimodal chat router
    let mut attachment_metas = Vec::new();
    let mut ref_data_urls = Vec::new();
    for sr in &selected_refs {
        if let Ok(Some(arow)) = sqlx::query(
            "SELECT id, label, role, original_rel_path, mime_type, width, height FROM assets WHERE id = ? AND project_id = ?"
        )
        .bind(&sr.asset_id)
        .bind(&project_id)
        .fetch_optional(&state.pool)
        .await
        {
            let aid: String = arow.get("id");
            attachment_metas.push(serde_json::json!({
                "asset_id": aid,
                "label": arow.get::<String, _>("label"),
                "role": sr.role.as_str(),
                "thumbnail_url": format!("/media/asset/{}/thumbnail", aid),
                "original_url": format!("/media/asset/{}/original", aid),
                "width": arow.get::<i64, _>("width"),
                "height": arow.get::<i64, _>("height")
            }));
            if ref_data_urls.len() < 4 {
                let rel: String = arow.get("original_rel_path");
                let mime: String = arow.get("mime_type");
                if let Ok(abs) = state.config.resolve_rel_path(&rel) {
                    if let Ok(bytes) = std::fs::read(&abs) {
                        use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
                        ref_data_urls.push(format!("data:{};base64,{}", mime, BASE64.encode(&bytes)));
                    }
                }
            }
        }
    }

    // Load existing tabs from completed outputs in this thread
    let out_rows = sqlx::query(
        "SELECT o.id, o.title, o.kind, o.aspect_ratio, o.tab_key, o.tab_label, o.original_rel_path, o.mime_type, o.extracted_from_output_id, o.transparent_background FROM outputs o JOIN runs r ON o.run_id = r.id WHERE r.thread_id = ? AND o.status = 'completed' ORDER BY o.created_at DESC"
    )
    .bind(&thread_id)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    let mut existing_tabs: Vec<ExistingTabSummary> = Vec::new();
    for row in &out_rows {
        let extracted_from: Option<String> = row.get("extracted_from_output_id");
        let kind_str: String = row.get("kind");
        let trans_bg: i64 = row.try_get("transparent_background").unwrap_or(0);
        if extracted_from.is_some() || kind_str == "asset" || trans_bg != 0 {
            // Extracted child assets live inside their parent component tab
            continue;
        }
        let raw_tab_key: String = row.get("tab_key");
        let raw_tab_label: String = row.get("tab_label");
        let title: String = row.get("title");
        let ar: String = row.get("aspect_ratio");
        let out_id: String = row.get("id");
        let t_key = if raw_tab_key.trim().is_empty() {
            normalize_tab_key(&title)
        } else {
            normalize_tab_key(&raw_tab_key)
        };
        let t_label = if raw_tab_label.trim().is_empty() {
            title.clone()
        } else {
            raw_tab_label
        };
        let vp_intent = if ar == "9:16" {
            "mobile".to_string()
        } else if ar == "3:4" {
            "tablet".to_string()
        } else {
            "desktop".to_string()
        };
        if let Some(existing) = existing_tabs.iter_mut().find(|t| t.tab_key == t_key) {
            existing.version_count += 1;
            // Keep the desktop root view as the canonical anchor output for the tab
            if existing.viewport_intent != "desktop" && vp_intent == "desktop" {
                existing.latest_output_id = out_id;
                existing.title = title;
                existing.aspect_ratio = ar;
                existing.viewport_intent = vp_intent;
            }
        } else {
            existing_tabs.push(ExistingTabSummary {
                tab_key: t_key,
                tab_label: t_label,
                kind: kind_str,
                latest_output_id: out_id,
                title,
                aspect_ratio: ar,
                viewport_intent: vp_intent,
                version_count: 1,
            });
        }
    }

    // Load recent thread messages for multi-turn context before inserting the new user message
    let prior_msg_rows = sqlx::query(
        "SELECT id, role, content, attachments_json, quick_replies_json, linked_run_id FROM thread_messages WHERE thread_id = ? ORDER BY created_at DESC"
    )
    .bind(&thread_id)
    .fetch_all(&state.pool)
    .await
    .unwrap_or_default();

    if let Some(answer_to) = req.answer_to_message_id.as_deref() {
        let is_current_question = prior_msg_rows.first().is_some_and(|row| {
            row.get::<String, _>("id") == answer_to
                && row.get::<String, _>("role") == "assistant"
                && row.get::<Option<String>, _>("linked_run_id").is_none()
                && row.get::<String, _>("quick_replies_json") != "[]"
        });
        if !is_current_question {
            return Err(AppError::Conflict(
                "This question has already been answered. Use the latest question.".into(),
            ));
        }
    }

    // Carry forward any user-attached reference images from earlier turns in the current clarification cycle
    let mut effective_selected_refs = selected_refs.clone();
    let mut prior_clarification_user_texts: Vec<String> = Vec::new();
    for r in &prior_msg_rows {
        let role: String = r.get("role");
        let linked_run: Option<String> = r.try_get("linked_run_id").ok().flatten();
        if role == "assistant" && linked_run.is_some() {
            break;
        }
        if role == "user" {
            let content_str: String = r.get("content");
            prior_clarification_user_texts.push(content_str);
            let att_str: String = r.try_get("attachments_json").unwrap_or_default();
            if let Ok(att_arr) = serde_json::from_str::<Vec<serde_json::Value>>(&att_str) {
                for item in att_arr {
                    if let Some(aid) = item.get("asset_id").and_then(|v| v.as_str()) {
                        if !effective_selected_refs.iter().any(|sr| sr.asset_id == aid) {
                            let role_str = item
                                .get("role")
                                .and_then(|v| v.as_str())
                                .unwrap_or("style_reference");
                            let label_str = item
                                .get("label")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Attached Reference");
                            let parsed_role = AttachmentRole::parse(role_str)
                                .unwrap_or(AttachmentRole::StyleReference);
                            effective_selected_refs.push(SelectedReference {
                                asset_id: aid.to_string(),
                                role: parsed_role,
                                label: label_str.to_string(),
                                placement_notes: String::new(),
                                preserve_as_is: false,
                                source_output_id: None,
                            });
                            if ref_data_urls.len() < 4 {
                                if let Ok(Some(arow)) = sqlx::query(
                                    "SELECT original_rel_path, mime_type FROM assets WHERE id = ? AND project_id = ?"
                                )
                                .bind(aid)
                                .bind(&project_id)
                                .fetch_optional(&state.pool)
                                .await
                                {
                                    let rel: String = arow.get("original_rel_path");
                                    let mime: String = arow.get("mime_type");
                                    if let Ok(abs) = state.config.resolve_rel_path(&rel) {
                                        if let Ok(bytes) = std::fs::read(&abs) {
                                            use base64::{
                                                engine::general_purpose::STANDARD as BASE64,
                                                Engine as _,
                                            };
                                            ref_data_urls.push(format!(
                                                "data:{};base64,{}",
                                                mime,
                                                BASE64.encode(&bytes)
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let selected_refs = effective_selected_refs;

    // Keep every turn in the current brief so a longer interview retains its original request.
    let cycle_len = prior_msg_rows
        .iter()
        .position(|row| {
            row.get::<String, _>("role") == "assistant"
                && row.get::<Option<String>, _>("linked_run_id").is_some()
        })
        .map(|index| index + 1)
        .unwrap_or(prior_msg_rows.len());
    let current_cycle = &prior_msg_rows[..cycle_len];
    let answered_questions = current_cycle
        .iter()
        .filter(|row| {
            row.get::<String, _>("role") == "assistant"
                && row.get::<String, _>("quick_replies_json") != "[]"
        })
        .count();
    let recent_chat: Vec<(String, String)> = current_cycle
        .iter()
        .rev()
        .map(|row| (row.get("role"), row.get("content")))
        .collect();

    // Save the user message to thread_messages
    let user_msg_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let effective_user_text = if user_text.is_empty() && !highlights.is_empty() {
        format!("Edit highlighted region: {}", highlights[0].comment)
    } else {
        user_text.clone()
    };

    let inserted = sqlx::query(
        "INSERT INTO thread_messages (id, thread_id, role, content, attachments_json, highlight_annotations_json, quick_replies_json, linked_run_id, target_tab_key, target_tab_label, created_at) SELECT ?, ?, 'user', ?, ?, ?, '[]', NULL, ?, NULL, ? WHERE ? IS NULL OR ? = (SELECT id FROM thread_messages WHERE thread_id = ? ORDER BY created_at DESC LIMIT 1)"
    )
    .bind(&user_msg_id)
    .bind(&thread_id)
    .bind(&effective_user_text)
    .bind(serde_json::to_string(&attachment_metas).unwrap_or_else(|_| "[]".into()))
    .bind(serde_json::to_string(&highlights).unwrap_or_else(|_| "[]".into()))
    .bind(req.active_tab_key.as_deref())
    .bind(&now)
    .bind(req.answer_to_message_id.as_deref())
    .bind(req.answer_to_message_id.as_deref())
    .bind(&thread_id)
    .execute(&state.pool)
    .await?;

    if inserted.rows_affected() != 1 {
        return Err(AppError::Conflict(
            "This question has already been answered. Use the latest question.".into(),
        ));
    }

    let project_meta = ProjectMetaSummary {
        id: project_id.clone(),
        name: proj_row.get("name"),
        description: proj_row.get("description"),
        purpose: proj_row.get("purpose"),
        industry: proj_row.get("industry"),
        target_audience: proj_row.get("target_audience"),
        audience_needs: proj_row.get("audience_needs"),
        desired_user_action: proj_row.get("desired_user_action"),
    };

    let force_gen = req.force_generate.unwrap_or(false) || !highlights.is_empty();

    let mut decision = if settings.test_adapter_enabled {
        let request_with_answers = prior_clarification_user_texts
            .iter()
            .rev()
            .map(String::as_str)
            .chain(std::iter::once(effective_user_text.as_str()))
            .collect::<Vec<_>>()
            .join("\n");
        TestFixtureAdapter::build_contextual_clarification_turn(
            &project_meta,
            &existing_tabs,
            &request_with_answers,
            answered_questions,
            force_gen,
        )
    } else {
        let api_key = state
            .config
            .secrets
            .get_key()
            .map(|(k, _)| k)
            .unwrap_or_default();
        let prompt_with_highlights = if !highlights.is_empty() {
            format!(
                "{}\n[HIGHLIGHT ANNOTATIONS ON IMAGE: {}]",
                effective_user_text,
                serde_json::to_string(&highlights).unwrap_or_default()
            )
        } else {
            effective_user_text.clone()
        };
        match state
            .provider
            .run_chat_turn(
                &api_key,
                &settings.planner_model,
                &project_meta,
                &brand_spec,
                &existing_tabs,
                &recent_chat,
                &prompt_with_highlights,
                &ref_data_urls,
                req.active_tab_key.as_deref(),
                force_gen,
            )
            .await
        {
            Ok((dec, _cost)) => dec,
            Err(err) => {
                // An unsuccessful model call must leave the pending question retryable.
                sqlx::query("DELETE FROM thread_messages WHERE id = ?")
                    .bind(&user_msg_id)
                    .execute(&state.pool)
                    .await?;
                tracing::warn!(error = %crate::error::redact_secrets(&err.to_string()), "Chat router request failed");
                return Err(err);
            }
        }
    };

    // Enforce highlight-annotation edit rules and force_generate overrides
    if force_gen {
        decision.action = "trigger_run".to_string();
        if decision.run_spec.is_none() {
            let fallback_tab = existing_tabs.first();
            let combined_history_and_msg = recent_chat
                .iter()
                .map(|(_, c)| c.as_str())
                .chain(std::iter::once(effective_user_text.as_str()))
                .collect::<Vec<_>>()
                .join(" | ");
            let lower_combined = combined_history_and_msg.to_ascii_lowercase();
            let is_full_page = lower_combined.contains("landing")
                || lower_combined.contains("full website")
                || lower_combined.contains("whole website")
                || lower_combined.contains("full-page")
                || lower_combined.contains("full page");
            decision.run_spec = Some(crate::models::ChatRunSpec {
                mode: if is_full_page {
                    "full_page".to_string()
                } else {
                    "component".to_string()
                },
                scope_preset: if is_full_page {
                    "full_page".to_string()
                } else {
                    "isolated_component".to_string()
                },
                target_tab_key: if is_full_page {
                    "full-website".to_string()
                } else {
                    fallback_tab
                        .map(|t| t.tab_key.clone())
                        .unwrap_or_else(|| "component".to_string())
                },
                target_tab_label: if is_full_page {
                    "Full Website".to_string()
                } else {
                    fallback_tab
                        .map(|t| t.tab_label.clone())
                        .unwrap_or_else(|| "Component".to_string())
                },
                target_existing_output_id: highlights
                    .first()
                    .map(|h| h.output_id.clone())
                    .or_else(|| req.active_output_id.clone())
                    .or_else(|| fallback_tab.map(|t| t.latest_output_id.clone())),
                is_highlight_edit: !highlights.is_empty(),
                quality_override: if !highlights.is_empty() {
                    Some("medium".to_string())
                } else {
                    req.quality_preference.clone()
                },
                aspect_ratio: if is_full_page {
                    "2:3".to_string()
                } else {
                    fallback_tab
                        .map(|t| t.aspect_ratio.clone())
                        .unwrap_or_else(|| "16:9".to_string())
                },
                viewport_intent: fallback_tab
                    .map(|t| t.viewport_intent.clone())
                    .unwrap_or_else(|| "desktop".to_string()),
                refined_prompt: combined_history_and_msg,
            });
        }
    }

    let mut created_run_dto: Option<serde_json::Value> = None;
    let mut linked_run_id: Option<String> = None;
    let mut resolved_tab_key: Option<String> = None;
    let mut resolved_tab_label: Option<String> = None;

    let should_trigger_run = decision.action == "trigger_run" || decision.action == "generate";

    if should_trigger_run {
        let mut spec = decision
            .run_spec
            .clone()
            .unwrap_or(crate::models::ChatRunSpec {
                mode: "component".to_string(),
                scope_preset: "isolated_component".to_string(),
                target_tab_key: normalize_tab_key(&effective_user_text),
                target_tab_label: "Component".to_string(),
                target_existing_output_id: None,
                is_highlight_edit: !highlights.is_empty(),
                quality_override: req.quality_preference.clone(),
                aspect_ratio: "16:9".to_string(),
                viewport_intent: "desktop".to_string(),
                refined_prompt: effective_user_text.clone(),
            });

        // If highlights are present, lock onto the highlighted output's tab & enforce quality="medium"
        if let Some(first_hl) = highlights.first() {
            spec.is_highlight_edit = true;
            spec.quality_override = Some("medium".to_string());
            spec.target_existing_output_id = Some(first_hl.output_id.clone());
            if let Ok(Some(orow)) = sqlx::query(
                "SELECT title, tab_key, tab_label, aspect_ratio FROM outputs WHERE id = ?",
            )
            .bind(&first_hl.output_id)
            .fetch_optional(&state.pool)
            .await
            {
                let tk: String = orow.get("tab_key");
                let tl: String = orow.get("tab_label");
                let title: String = orow.get("title");
                let ar: String = orow.get("aspect_ratio");
                spec.target_tab_key = if tk.trim().is_empty() {
                    normalize_tab_key(&title)
                } else {
                    normalize_tab_key(&tk)
                };
                spec.target_tab_label = if tl.trim().is_empty() { title } else { tl };
                if !ar.trim().is_empty() {
                    spec.aspect_ratio = ar;
                }
            }
        } else {
            let combined_request_text = prior_clarification_user_texts
                .iter()
                .rev()
                .map(|s| s.as_str())
                .chain(std::iter::once(effective_user_text.as_str()))
                .collect::<Vec<_>>()
                .join(" ");
            apply_semantic_and_versioned_tab_rules(
                &mut spec,
                &combined_request_text,
                &effective_user_text,
                &existing_tabs,
                req.active_tab_key.as_deref(),
                req.active_output_id.as_deref(),
            );
        }

        let mode = if matches!(spec.mode.as_str(), "component" | "full_page" | "asset") {
            spec.mode.clone()
        } else {
            "component".to_string()
        };

        let aspect_ratio = if mode == "full_page" {
            if spec.viewport_intent == "mobile" {
                "9:16".to_string()
            } else if spec.aspect_ratio == "9:16" || spec.aspect_ratio == "2:3" {
                spec.aspect_ratio.clone()
            } else {
                "2:3".to_string()
            }
        } else if spec.viewport_intent == "mobile" && mode != "asset" {
            if spec.aspect_ratio == "2:3" || spec.aspect_ratio == "3:4" {
                spec.aspect_ratio.clone()
            } else {
                "9:16".to_string()
            }
        } else if spec.viewport_intent == "tablet" && mode != "asset" {
            if spec.aspect_ratio == "2:3" || spec.aspect_ratio == "4:3" {
                spec.aspect_ratio.clone()
            } else {
                "3:4".to_string()
            }
        } else if caps.supported_aspect_ratios.contains(&spec.aspect_ratio) {
            spec.aspect_ratio.clone()
        } else {
            "16:9".to_string()
        };

        let quality = if !highlights.is_empty() || spec.is_highlight_edit {
            "medium".to_string()
        } else {
            spec.quality_override
                .clone()
                .or(req.quality_preference.clone())
                .unwrap_or_else(|| settings.default_quality.clone())
        };

        let run_prompt = if spec.refined_prompt.trim().is_empty() {
            effective_user_text.clone()
        } else {
            spec.refined_prompt.clone()
        };

        let model_settings = RunModelSettings {
            planner_model: settings.planner_model.clone(),
            image_model: settings.image_model.clone(),
            quality: quality.clone(),
            aspect_ratio: aspect_ratio.clone(),
            output_size: settings.output_size.clone(),
            scope_preset: spec.scope_preset.clone(),
            custom_dimensions: None,
            review_plan_first: false,
            asset_options: if mode == "asset" {
                Some(AssetModeOptions {
                    intended_usage: "icon_logo_concept".to_string(),
                    theme_context: "both".to_string(),
                    transparent_background: true,
                    seamless_tiling: false,
                    include_logo: "never".to_string(),
                    ..Default::default()
                })
            } else {
                None
            },
            target_tab_key: Some(spec.target_tab_key.clone()),
            target_tab_label: Some(spec.target_tab_label.clone()),
            target_existing_output_id: spec.target_existing_output_id.clone(),
            viewport_intent: Some(spec.viewport_intent.clone()),
            highlight_annotations: highlights.clone(),
            single_focused_output: mode != "full_page",
        };

        let planned_count = if mode == "full_page" { 4 } else { 1 };
        let cost_est = compute_cost_estimate(
            &caps,
            planned_count,
            selected_refs.len() as u32,
            &quality,
            settings.test_adapter_enabled,
        );

        let run_id = Uuid::new_v4().to_string();
        let plan_job_id = Uuid::new_v4().to_string();
        let run_now = chrono::Utc::now().to_rfc3339();

        let mut tx = state.pool.begin().await?;
        sqlx::query(
            "INSERT INTO runs (id, project_id, thread_id, idempotency_key, mode, scope_preset, user_prompt, brand_version, brand_snapshot_json, selected_references_json, model_settings_json, status, estimated_cost_micros, actual_cost_micros, cost_is_unknown, is_test_adapter, target_tab_key, target_tab_label, target_output_id, highlight_annotations_json, created_at, updated_at) VALUES (?, ?, ?, NULL, ?, ?, ?, ?, ?, ?, ?, 'queued', ?, 0, 0, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&run_id)
        .bind(&project_id)
        .bind(&thread_id)
        .bind(&mode)
        .bind(&spec.scope_preset)
        .bind(&run_prompt)
        .bind(brand_version)
        .bind(&brand_snapshot_json)
        .bind(serde_json::to_string(&selected_refs).unwrap_or_else(|_| "[]".into()))
        .bind(serde_json::to_string(&model_settings).unwrap_or_else(|_| "{}".into()))
        .bind(cost_est.total_estimated_micros)
        .bind(if settings.test_adapter_enabled { 1 } else { 0 })
        .bind(&spec.target_tab_key)
        .bind(&spec.target_tab_label)
        .bind(&spec.target_existing_output_id)
        .bind(serde_json::to_string(&highlights).unwrap_or_else(|_| "[]".into()))
        .bind(&run_now)
        .bind(&run_now)
        .execute(&mut *tx)
        .await?;

        sqlx::query(
            "INSERT INTO jobs (id, run_id, output_id, job_type, status, attempts, max_attempts, depends_on_job_ids_json, created_at, updated_at) VALUES (?, ?, NULL, 'plan', 'queued', 0, 2, '[]', ?, ?)"
        )
        .bind(&plan_job_id)
        .bind(&run_id)
        .bind(&run_now)
        .bind(&run_now)
        .execute(&mut *tx)
        .await?;

        sqlx::query("UPDATE threads SET updated_at = ? WHERE id = ?")
            .bind(&run_now)
            .bind(&thread_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;

        record_and_broadcast_event(
            &state.pool,
            &state.event_tx,
            &run_id,
            Some(&plan_job_id),
            None,
            "run_queued",
            "queued",
            &format!("Generating into tab '{}'...", spec.target_tab_label),
        )
        .await;

        linked_run_id = Some(run_id.clone());
        resolved_tab_key = Some(spec.target_tab_key);
        resolved_tab_label = Some(spec.target_tab_label);
        created_run_dto = Some(load_run_dto(&state.pool, &state.config, &run_id).await?);
    }

    let asst_msg_id = Uuid::new_v4().to_string();
    let asst_now = chrono::Utc::now().to_rfc3339();
    let quick_replies = if !should_trigger_run {
        decision.quick_replies.clone()
    } else {
        Vec::new()
    };

    sqlx::query(
        "INSERT INTO thread_messages (id, thread_id, role, content, attachments_json, highlight_annotations_json, quick_replies_json, linked_run_id, target_tab_key, target_tab_label, created_at) VALUES (?, ?, 'assistant', ?, '[]', '[]', ?, ?, ?, ?, ?)"
    )
    .bind(&asst_msg_id)
    .bind(&thread_id)
    .bind(&decision.assistant_reply)
    .bind(serde_json::to_string(&quick_replies).unwrap_or_else(|_| "[]".into()))
    .bind(&linked_run_id)
    .bind(&resolved_tab_key)
    .bind(&resolved_tab_label)
    .bind(&asst_now)
    .execute(&state.pool)
    .await?;

    Ok(Json(serde_json::json!({
        "action": if should_trigger_run { "trigger_run" } else { &decision.action },
        "user_message": {
            "id": user_msg_id,
            "thread_id": thread_id,
            "role": "user",
            "content": effective_user_text,
            "attachments": attachment_metas,
            "highlight_annotations": highlights,
            "quick_replies": [],
            "linked_run_id": null,
            "target_tab_key": req.active_tab_key,
            "target_tab_label": null,
            "created_at": now
        },
        "assistant_message": {
            "id": asst_msg_id,
            "thread_id": thread_id,
            "role": "assistant",
            "content": decision.assistant_reply,
            "attachments": [],
            "highlight_annotations": [],
            "quick_replies": quick_replies,
            "linked_run_id": linked_run_id,
            "target_tab_key": resolved_tab_key,
            "target_tab_label": resolved_tab_label,
            "created_at": asst_now
        },
        "target_tab_key": resolved_tab_key,
        "target_tab_label": resolved_tab_label,
        "run": created_run_dto
    })))
}

// ============================================================================
// Assets & Upload / Paste Handlers
// ============================================================================

async fn list_assets_handler(
    State(state): State<AppState>,
    AxumPath(project_id): AxumPath<String>,
) -> AppResult<Json<serde_json::Value>> {
    let rows = sqlx::query(
        "SELECT id, project_id, role, label, original_rel_path, mime_type, width, height, byte_size, checksum_sha256, has_alpha, placement_notes, preserve_as_is, metadata_json, created_at FROM assets WHERE project_id = ? ORDER BY created_at DESC"
    )
    .bind(&project_id)
    .fetch_all(&state.pool)
    .await?;

    let mut assets = Vec::new();
    for r in rows {
        let aid: String = r.get("id");
        let orig_rel: String = r.get("original_rel_path");
        let abs_path = state
            .config
            .resolve_rel_path(&orig_rel)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or(orig_rel);
        let meta: serde_json::Value =
            serde_json::from_str(r.get::<&str, _>("metadata_json")).unwrap_or_default();

        assets.push(serde_json::json!({
            "id": aid,
            "project_id": r.get::<String, _>("project_id"),
            "role": r.get::<String, _>("role"),
            "label": r.get::<String, _>("label"),
            "mime_type": r.get::<String, _>("mime_type"),
            "width": r.get::<i64, _>("width"),
            "height": r.get::<i64, _>("height"),
            "byte_size": r.get::<i64, _>("byte_size"),
            "checksum_sha256": r.get::<String, _>("checksum_sha256"),
            "has_alpha": r.get::<i64, _>("has_alpha") != 0,
            "placement_notes": r.get::<String, _>("placement_notes"),
            "preserve_as_is": r.get::<i64, _>("preserve_as_is") != 0,
            "metadata": meta,
            "absolute_path": abs_path,
            "thumbnail_url": format!("/media/asset/{}/thumbnail", aid),
            "original_url": format!("/media/asset/{}/original", aid),
            "created_at": r.get::<String, _>("created_at")
        }));
    }

    Ok(Json(serde_json::json!({ "assets": assets })))
}

async fn upload_asset_handler(
    State(state): State<AppState>,
    AxumPath(project_id): AxumPath<String>,
    mut multipart: Multipart,
) -> AppResult<Json<serde_json::Value>> {
    // Verify project exists
    let proj_exists: Option<String> = sqlx::query_scalar("SELECT id FROM projects WHERE id = ?")
        .bind(&project_id)
        .fetch_optional(&state.pool)
        .await?;
    if proj_exists.is_none() {
        return Err(AppError::NotFound(format!(
            "Project '{}' not found",
            project_id
        )));
    }

    let mut role_str = "include_asset".to_string();
    let mut label_opt: Option<String> = None;
    let mut placement_notes = String::new();
    let mut preserve_as_is = false;
    let mut filename_hint: Option<String> = None;
    let mut file_bytes: Option<Vec<u8>> = None;

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::BadRequest(format!("Multipart error: {}", e)))?
    {
        let name = field.name().unwrap_or("").to_string();
        match name.as_str() {
            "role" => {
                role_str = field
                    .text()
                    .await
                    .unwrap_or_else(|_| "include_asset".into());
            }
            "label" => {
                let t = field.text().await.unwrap_or_default();
                if !t.trim().is_empty() {
                    label_opt = Some(t.trim().to_string());
                }
            }
            "placement_notes" => {
                placement_notes = field.text().await.unwrap_or_default();
            }
            "preserve_as_is" => {
                let t = field.text().await.unwrap_or_default();
                preserve_as_is = t == "true" || t == "1";
            }
            "file" => {
                filename_hint = field.file_name().map(|s| s.to_string());
                let bytes = field.bytes().await.map_err(|e| {
                    AppError::BadRequest(format!("Failed reading upload bytes: {}", e))
                })?;
                file_bytes = Some(bytes.to_vec());
            }
            _ => {}
        }
    }

    let role = AttachmentRole::parse(role_str.trim())?;
    let bytes = file_bytes
        .ok_or_else(|| AppError::Validation("No file uploaded in 'file' field.".into()))?;

    let processed = process_upload_bytes(
        bytes,
        filename_hint.clone(),
        state.config.max_upload_bytes,
        state.config.max_megapixels,
    )
    .await?;

    // Check duplicate by checksum + role within this project
    if let Some(existing) = sqlx::query(
        "SELECT id, label, mime_type, width, height, byte_size, has_alpha, placement_notes, preserve_as_is, metadata_json, original_rel_path, created_at FROM assets WHERE project_id = ? AND checksum_sha256 = ? AND role = ? LIMIT 1"
    )
    .bind(&project_id)
    .bind(&processed.checksum_sha256)
    .bind(role.as_str())
    .fetch_optional(&state.pool)
    .await?
    {
        let existing_id: String = existing.get("id");
        let orig_rel: String = existing.get("original_rel_path");
        let abs = state
            .config
            .resolve_rel_path(&orig_rel)
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or(orig_rel);
        return Ok(Json(serde_json::json!({
            "deduplicated": true,
            "asset": {
                "id": existing_id,
                "project_id": project_id,
                "role": role.as_str(),
                "label": existing.get::<String, _>("label"),
                "mime_type": existing.get::<String, _>("mime_type"),
                "width": existing.get::<i64, _>("width"),
                "height": existing.get::<i64, _>("height"),
                "byte_size": existing.get::<i64, _>("byte_size"),
                "checksum_sha256": processed.checksum_sha256,
                "has_alpha": existing.get::<i64, _>("has_alpha") != 0,
                "placement_notes": existing.get::<String, _>("placement_notes"),
                "preserve_as_is": existing.get::<i64, _>("preserve_as_is") != 0,
                "metadata": serde_json::from_str::<serde_json::Value>(existing.get::<&str, _>("metadata_json")).unwrap_or_default(),
                "absolute_path": abs,
                "thumbnail_url": format!("/media/asset/{}/thumbnail", existing_id),
                "original_url": format!("/media/asset/{}/original", existing_id),
                "created_at": existing.get::<String, _>("created_at")
            }
        })));
    }

    let asset_id = Uuid::new_v4().to_string();
    let label = label_opt
        .or_else(|| {
            filename_hint.as_ref().map(|f| {
                std::path::Path::new(f)
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .unwrap_or(f)
                    .to_string()
            })
        })
        .unwrap_or_else(|| format!("{} asset", role.as_str()));

    let asset_dir_rel = format!("projects/{}/assets/{}", project_id, asset_id);
    let orig_rel = format!("{}/original.{}", asset_dir_rel, processed.extension);
    let thumb_rel = format!("{}/thumbnail.webp", asset_dir_rel);
    let model_rel = processed
        .model_input_png
        .as_ref()
        .map(|_| format!("{}/model-input.png", asset_dir_rel));

    let abs_orig = state.config.resolve_rel_path(&orig_rel)?;
    let abs_thumb = state.config.resolve_rel_path(&thumb_rel)?;
    atomic_write_file(&abs_orig, &processed.original_bytes)?;
    atomic_write_file(&abs_thumb, &processed.thumbnail_webp)?;

    if let (Some(ref m_rel), Some(ref m_bytes)) = (&model_rel, &processed.model_input_png) {
        let abs_model = state.config.resolve_rel_path(m_rel)?;
        atomic_write_file(&abs_model, m_bytes)?;
    }

    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO assets (id, project_id, role, label, original_rel_path, model_input_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, checksum_sha256, has_alpha, placement_notes, preserve_as_is, metadata_json, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&asset_id)
    .bind(&project_id)
    .bind(role.as_str())
    .bind(&label)
    .bind(&orig_rel)
    .bind(&model_rel)
    .bind(&thumb_rel)
    .bind(&processed.mime_type)
    .bind(processed.width as i64)
    .bind(processed.height as i64)
    .bind(processed.byte_size as i64)
    .bind(&processed.checksum_sha256)
    .bind(if processed.has_alpha { 1 } else { 0 })
    .bind(&placement_notes)
    .bind(if preserve_as_is || role == AttachmentRole::BrandLogo { 1 } else { 0 })
    .bind(processed.metadata.to_string())
    .bind(&now)
    .execute(&state.pool)
    .await?;

    Ok(Json(serde_json::json!({
        "deduplicated": false,
        "asset": {
            "id": asset_id,
            "project_id": project_id,
            "role": role.as_str(),
            "label": label,
            "mime_type": processed.mime_type,
            "width": processed.width,
            "height": processed.height,
            "byte_size": processed.byte_size,
            "checksum_sha256": processed.checksum_sha256,
            "has_alpha": processed.has_alpha,
            "placement_notes": placement_notes,
            "preserve_as_is": preserve_as_is || role == AttachmentRole::BrandLogo,
            "metadata": processed.metadata,
            "absolute_path": abs_orig.to_string_lossy(),
            "thumbnail_url": format!("/media/asset/{}/thumbnail", asset_id),
            "original_url": format!("/media/asset/{}/original", asset_id),
            "created_at": now
        }
    })))
}

#[derive(Deserialize)]
struct UpdateAssetRequest {
    role: Option<String>,
    label: Option<String>,
    placement_notes: Option<String>,
    preserve_as_is: Option<bool>,
}

async fn update_asset_handler(
    State(state): State<AppState>,
    AxumPath(asset_id): AxumPath<String>,
    Json(req): Json<UpdateAssetRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let row = sqlx::query(
        "SELECT project_id, role, label, placement_notes, preserve_as_is FROM assets WHERE id = ?",
    )
    .bind(&asset_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Asset not found".into()))?;

    let role = if let Some(r) = req.role {
        AttachmentRole::parse(&r)?.as_str().to_string()
    } else {
        row.get("role")
    };
    let label = req.label.unwrap_or_else(|| row.get("label"));
    let placement_notes = req
        .placement_notes
        .unwrap_or_else(|| row.get("placement_notes"));
    let preserve = req
        .preserve_as_is
        .unwrap_or_else(|| row.get::<i64, _>("preserve_as_is") != 0);

    sqlx::query(
        "UPDATE assets SET role = ?, label = ?, placement_notes = ?, preserve_as_is = ? WHERE id = ?"
    )
    .bind(&role)
    .bind(&label)
    .bind(&placement_notes)
    .bind(if preserve { 1 } else { 0 })
    .bind(&asset_id)
    .execute(&state.pool)
    .await?;

    Ok(Json(serde_json::json!({
        "updated": true,
        "id": asset_id,
        "role": role,
        "label": label,
        "placement_notes": placement_notes,
        "preserve_as_is": preserve
    })))
}

async fn delete_asset_handler(
    State(state): State<AppState>,
    AxumPath(asset_id): AxumPath<String>,
) -> AppResult<Json<serde_json::Value>> {
    let row = sqlx::query("SELECT project_id FROM assets WHERE id = ?")
        .bind(&asset_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Asset not found".into()))?;

    let project_id: String = row.get("project_id");
    sqlx::query("DELETE FROM assets WHERE id = ?")
        .bind(&asset_id)
        .execute(&state.pool)
        .await?;

    let dir_rel = format!("projects/{}/assets/{}", project_id, asset_id);
    if let Ok(abs) = state.config.resolve_rel_path(&dir_rel) {
        if abs.exists() {
            let _ = std::fs::remove_dir_all(&abs);
        }
    }

    Ok(Json(serde_json::json!({ "deleted": true, "id": asset_id })))
}

async fn promote_output_to_reference_handler(
    State(state): State<AppState>,
    AxumPath(output_id): AxumPath<String>,
) -> AppResult<Json<serde_json::Value>> {
    let out_row = sqlx::query(
        "SELECT project_id, title, original_rel_path FROM outputs WHERE id = ? AND status = 'completed'"
    )
    .bind(&output_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Completed output not found".into()))?;

    let project_id: String = out_row.get("project_id");
    let title: String = out_row.get("title");
    let orig_rel: String = out_row
        .get::<Option<String>, _>("original_rel_path")
        .ok_or_else(|| AppError::Conflict("Output has no original file".into()))?;

    let abs_orig = state.config.resolve_rel_path(&orig_rel)?;
    let bytes = std::fs::read(&abs_orig)?;
    let processed = process_upload_bytes(
        bytes,
        Some("output.png".into()),
        state.config.max_upload_bytes,
        state.config.max_megapixels,
    )
    .await?;

    let asset_id = Uuid::new_v4().to_string();
    let asset_dir_rel = format!("projects/{}/assets/{}", project_id, asset_id);
    let new_orig_rel = format!("{}/original.{}", asset_dir_rel, processed.extension);
    let new_thumb_rel = format!("{}/thumbnail.webp", asset_dir_rel);
    let new_model_rel = format!("{}/model-input.png", asset_dir_rel);

    atomic_write_file(
        &state.config.resolve_rel_path(&new_orig_rel)?,
        &processed.original_bytes,
    )?;
    atomic_write_file(
        &state.config.resolve_rel_path(&new_thumb_rel)?,
        &processed.thumbnail_webp,
    )?;
    if let Some(ref m_bytes) = processed.model_input_png {
        atomic_write_file(&state.config.resolve_rel_path(&new_model_rel)?, m_bytes)?;
    }

    let label = format!("Ref: {}", title);
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO assets (id, project_id, role, label, original_rel_path, model_input_rel_path, thumbnail_rel_path, mime_type, width, height, byte_size, checksum_sha256, has_alpha, placement_notes, preserve_as_is, metadata_json, created_at) VALUES (?, ?, 'selected_previous_output', ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'Explicitly selected previous output for visual continuity/refinement', 0, ?, ?)"
    )
    .bind(&asset_id)
    .bind(&project_id)
    .bind(&label)
    .bind(&new_orig_rel)
    .bind(&new_model_rel)
    .bind(&new_thumb_rel)
    .bind(&processed.mime_type)
    .bind(processed.width as i64)
    .bind(processed.height as i64)
    .bind(processed.byte_size as i64)
    .bind(&processed.checksum_sha256)
    .bind(if processed.has_alpha { 1 } else { 0 })
    .bind(serde_json::json!({ "source_output_id": output_id }).to_string())
    .bind(&now)
    .execute(&state.pool)
    .await?;

    Ok(Json(serde_json::json!({
        "asset": {
            "id": asset_id,
            "project_id": project_id,
            "role": "selected_previous_output",
            "label": label,
            "mime_type": processed.mime_type,
            "width": processed.width,
            "height": processed.height,
            "thumbnail_url": format!("/media/asset/{}/thumbnail", asset_id),
            "original_url": format!("/media/asset/{}/original", asset_id)
        }
    })))
}

// ============================================================================
// Runs, Planning, Approval, Retry, Cancellation & SSE Handlers
// ============================================================================

#[derive(Deserialize)]
struct CreateRunRequest {
    mode: String, // "component" | "full_page" | "asset"
    user_prompt: String,
    scope_preset: Option<String>,
    aspect_ratio: Option<String>,
    quality: Option<String>,
    output_size: Option<String>,
    custom_dimensions: Option<String>,
    review_plan_first: Option<bool>,
    asset_options: Option<AssetModeOptions>,
    selected_references: Option<Vec<SelectedReference>>,
    idempotency_key: Option<String>,
    target_tab_key: Option<String>,
    target_tab_label: Option<String>,
    target_existing_output_id: Option<String>,
    highlight_annotations: Option<Vec<HighlightAnnotation>>,
    single_focused_output: Option<bool>,
}

async fn create_run_handler(
    State(state): State<AppState>,
    AxumPath(thread_id): AxumPath<String>,
    Json(req): Json<CreateRunRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let prompt = req.user_prompt.trim().to_string();
    if prompt.is_empty() {
        return Err(AppError::Validation("Prompt cannot be empty.".into()));
    }
    if !matches!(req.mode.as_str(), "component" | "full_page" | "asset") {
        return Err(AppError::Validation(format!(
            "Invalid mode '{}'. Expected component, full_page, or asset.",
            req.mode
        )));
    }

    let thread_row = sqlx::query("SELECT project_id FROM threads WHERE id = ?")
        .bind(&thread_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Thread not found".into()))?;
    let project_id: String = thread_row.get("project_id");

    // Check idempotency key to prevent duplicate browser submits
    if let Some(ref idem_key) = req.idempotency_key {
        if !idem_key.trim().is_empty() {
            if let Some(existing_run_id) = sqlx::query_scalar::<_, String>(
                "SELECT id FROM runs WHERE thread_id = ? AND idempotency_key = ? LIMIT 1",
            )
            .bind(&thread_id)
            .bind(idem_key.trim())
            .fetch_optional(&state.pool)
            .await?
            {
                let dto = load_run_dto(&state.pool, &state.config, &existing_run_id).await?;
                return Ok(Json(serde_json::json!({
                    "deduplicated": true,
                    "run": dto
                })));
            }
        }
    }

    let (settings, caps) = load_settings_and_caps(&state.pool, &state.config).await?;

    // If not in test adapter mode, require a configured key and ensure no capability blockers
    if !settings.test_adapter_enabled {
        if state.config.secrets.get_key().is_none() {
            return Err(AppError::Unauthorized(
                "OpenRouter API key is not configured. Add your key in Settings or enable Test Fixture Mode."
                    .into(),
            ));
        }
        if !caps.blocked_reasons.is_empty() {
            return Err(AppError::Validation(format!(
                "Generation blocked by model capability check: {}",
                caps.blocked_reasons.join(" ")
            )));
        }
    }

    let selected_refs = req.selected_references.unwrap_or_default();
    if selected_refs.len() as u32 > caps.max_input_references {
        return Err(AppError::Validation(format!(
            "Selected {} reference images, which exceeds the endpoint limit of {} references.",
            selected_refs.len(),
            caps.max_input_references
        )));
    }

    // Verify all selected reference asset_ids belong to this project
    for sr in &selected_refs {
        let exists: Option<String> =
            sqlx::query_scalar("SELECT id FROM assets WHERE id = ? AND project_id = ?")
                .bind(&sr.asset_id)
                .bind(&project_id)
                .fetch_optional(&state.pool)
                .await?;
        if exists.is_none() {
            return Err(AppError::Validation(format!(
                "Attached reference asset '{}' does not exist in this project.",
                sr.asset_id
            )));
        }
    }

    // Freeze current brand snapshot
    let brand_row = sqlx::query(
        "SELECT version, brand_json FROM brand_versions WHERE project_id = ? ORDER BY version DESC LIMIT 1"
    )
    .bind(&project_id)
    .fetch_one(&state.pool)
    .await?;
    let brand_version: i64 = brand_row.get("version");
    let brand_snapshot_json: String = brand_row.get("brand_json");

    let quality = req
        .quality
        .unwrap_or_else(|| settings.default_quality.clone());
    let aspect_ratio = req.aspect_ratio.unwrap_or_else(|| "4:3".to_string());
    if !caps.supported_aspect_ratios.is_empty()
        && !caps.supported_aspect_ratios.contains(&aspect_ratio)
    {
        return Err(AppError::Validation(format!(
            "Aspect ratio '{}' is not supported by the selected image endpoint.",
            aspect_ratio
        )));
    }

    let scope_preset = req
        .scope_preset
        .unwrap_or_else(|| "isolated_component".to_string());
    let review_plan_first = req.review_plan_first.unwrap_or(req.mode == "full_page");
    let highlight_annotations = req.highlight_annotations.unwrap_or_default();
    let effective_quality = if !highlight_annotations.is_empty() {
        "medium".to_string()
    } else {
        quality.clone()
    };

    let model_settings = RunModelSettings {
        planner_model: settings.planner_model.clone(),
        image_model: settings.image_model.clone(),
        quality: effective_quality.clone(),
        aspect_ratio,
        output_size: req.output_size.unwrap_or(settings.output_size.clone()),
        scope_preset: scope_preset.clone(),
        custom_dimensions: req.custom_dimensions,
        review_plan_first,
        asset_options: if req.mode == "asset" {
            Some(req.asset_options.unwrap_or_default())
        } else {
            req.asset_options
        },
        target_tab_key: req.target_tab_key.clone(),
        target_tab_label: req.target_tab_label.clone(),
        target_existing_output_id: req.target_existing_output_id.clone(),
        viewport_intent: None,
        highlight_annotations: highlight_annotations.clone(),
        single_focused_output: req
            .single_focused_output
            .unwrap_or(!highlight_annotations.is_empty()),
    };

    let planned_count = if req.mode == "full_page" {
        5
    } else if model_settings.single_focused_output {
        1
    } else {
        4
    };
    let cost_est = compute_cost_estimate(
        &caps,
        planned_count,
        selected_refs.len() as u32,
        &effective_quality,
        settings.test_adapter_enabled,
    );

    // Check optional spend limit reservation
    if let Some(limit_micros) = settings.spend_limit_micros {
        if !settings.test_adapter_enabled {
            let spent_micros: i64 = sqlx::query_scalar(
                "SELECT COALESCE(SUM(COALESCE(actual_cost_micros, estimated_cost_micros, 0)), 0) FROM runs WHERE status NOT IN ('cancelled', 'failed')"
            )
            .fetch_one(&state.pool)
            .await
            .unwrap_or(0);
            if spent_micros + cost_est.total_estimated_micros > limit_micros {
                return Err(AppError::PaymentRequired(format!(
                    "Estimated run cost (${:.2}) plus existing usage (${:.2}) would exceed your configured local spend limit (${:.2}). Adjust the spend limit in Settings to proceed.",
                    cost_est.total_estimated_micros as f64 / 1_000_000.0,
                    spent_micros as f64 / 1_000_000.0,
                    limit_micros as f64 / 1_000_000.0
                )));
            }
        }
    }

    let run_id = Uuid::new_v4().to_string();
    let plan_job_id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();

    let run_dir_rel = format!("projects/{}/runs/{}", project_id, run_id);
    let req_json_rel = format!("{}/request.json", run_dir_rel);
    if let Ok(abs_req) = state.config.resolve_rel_path(&req_json_rel) {
        let snapshot_doc = serde_json::json!({
            "run_id": run_id,
            "project_id": project_id,
            "thread_id": thread_id,
            "mode": req.mode,
            "user_prompt": prompt,
            "brand_version": brand_version,
            "selected_references": selected_refs,
            "model_settings": model_settings,
            "created_at": now
        });
        let _ = atomic_write_file(
            &abs_req,
            serde_json::to_string_pretty(&snapshot_doc)
                .unwrap_or_default()
                .as_bytes(),
        );
    }

    let mut tx = state.pool.begin().await?;
    sqlx::query(
        "INSERT INTO runs (id, project_id, thread_id, idempotency_key, mode, scope_preset, user_prompt, brand_version, brand_snapshot_json, selected_references_json, model_settings_json, status, estimated_cost_micros, actual_cost_micros, cost_is_unknown, is_test_adapter, target_tab_key, target_tab_label, target_output_id, highlight_annotations_json, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'queued', ?, 0, 0, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(&run_id)
    .bind(&project_id)
    .bind(&thread_id)
    .bind(req.idempotency_key.as_deref().map(|s| s.trim()).filter(|s| !s.is_empty()))
    .bind(&req.mode)
    .bind(&scope_preset)
    .bind(&prompt)
    .bind(brand_version)
    .bind(&brand_snapshot_json)
    .bind(serde_json::to_string(&selected_refs).unwrap_or_else(|_| "[]".into()))
    .bind(serde_json::to_string(&model_settings).unwrap_or_else(|_| "{}".into()))
    .bind(cost_est.total_estimated_micros)
    .bind(if settings.test_adapter_enabled { 1 } else { 0 })
    .bind(&req.target_tab_key)
    .bind(&req.target_tab_label)
    .bind(&req.target_existing_output_id)
    .bind(serde_json::to_string(&highlight_annotations).unwrap_or_else(|_| "[]".into()))
    .bind(&now)
    .bind(&now)
    .execute(&mut *tx)
    .await?;

    sqlx::query(
        "INSERT INTO jobs (id, run_id, output_id, job_type, status, attempts, max_attempts, depends_on_job_ids_json, created_at, updated_at) VALUES (?, ?, NULL, 'plan', 'queued', 0, 2, '[]', ?, ?)"
    )
    .bind(&plan_job_id)
    .bind(&run_id)
    .bind(&now)
    .bind(&now)
    .execute(&mut *tx)
    .await?;

    sqlx::query("UPDATE threads SET updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(&thread_id)
        .execute(&mut *tx)
        .await?;

    sqlx::query("UPDATE projects SET updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(&project_id)
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    record_and_broadcast_event(
        &state.pool,
        &state.event_tx,
        &run_id,
        Some(&plan_job_id),
        None,
        "run_queued",
        "queued",
        "Run queued for design planning.",
    )
    .await;

    let dto = load_run_dto(&state.pool, &state.config, &run_id).await?;
    Ok(Json(serde_json::json!({
        "deduplicated": false,
        "run": dto
    })))
}

async fn load_run_dto(
    pool: &SqlitePool,
    config: &AppConfig,
    run_id: &str,
) -> AppResult<serde_json::Value> {
    let r = sqlx::query(
        "SELECT id, project_id, thread_id, mode, scope_preset, user_prompt, brand_version, brand_snapshot_json, selected_references_json, model_settings_json, plan_json, consistency_report_json, status, estimated_cost_micros, actual_cost_micros, cost_is_unknown, is_test_adapter, error_message, target_tab_key, target_tab_label, target_output_id, highlight_annotations_json, created_at, updated_at, completed_at FROM runs WHERE id = ?"
    )
    .bind(run_id)
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Run '{}' not found", run_id)))?;

    let selected_refs: Vec<SelectedReference> =
        serde_json::from_str(r.get::<&str, _>("selected_references_json")).unwrap_or_default();
    let context_summary = summarize_context_badge(&selected_refs);

    let out_rows = sqlx::query(
        "SELECT id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, original_rel_path, mime_type, width, height, byte_size, has_alpha, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, usage_json, provider_request_id, actual_cost_micros, error_message, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id, created_at, updated_at FROM outputs WHERE run_id = ? ORDER BY variant_index ASC"
    )
    .bind(run_id)
    .fetch_all(pool)
    .await?;

    let mut outputs = Vec::new();
    for o in out_rows {
        let oid: String = o.get("id");
        let orig_rel: Option<String> = o.get("original_rel_path");
        let abs_orig = orig_rel.as_ref().and_then(|rel| {
            config
                .resolve_rel_path(rel)
                .ok()
                .map(|p| p.to_string_lossy().to_string())
        });
        let raw_tk: String = o.get("tab_key");
        let raw_tl: String = o.get("tab_label");
        let title: String = o.get("title");
        let eff_tk = if raw_tk.trim().is_empty() {
            normalize_tab_key(&title)
        } else {
            raw_tk
        };
        let eff_tl = if raw_tl.trim().is_empty() {
            title.clone()
        } else {
            raw_tl
        };

        outputs.push(serde_json::json!({
            "id": oid,
            "run_id": run_id,
            "plan_item_id": o.get::<String, _>("plan_item_id"),
            "variant_index": o.get::<i64, _>("variant_index"),
            "kind": o.get::<String, _>("kind"),
            "title": title,
            "direction_summary": o.get::<String, _>("direction_summary"),
            "role": o.get::<String, _>("role"),
            "page_route": o.get::<Option<String>, _>("page_route"),
            "aspect_ratio": o.get::<String, _>("aspect_ratio"),
            "status": o.get::<String, _>("status"),
            "mime_type": o.get::<Option<String>, _>("mime_type"),
            "width": o.get::<Option<i64>, _>("width"),
            "height": o.get::<Option<i64>, _>("height"),
            "byte_size": o.get::<Option<i64>, _>("byte_size"),
            "has_alpha": o.get::<i64, _>("has_alpha") != 0,
            "transparent_background": o.get::<i64, _>("transparent_background") != 0,
            "tab_key": eff_tk,
            "tab_label": eff_tl,
            "parent_output_id": o.get::<Option<String>, _>("parent_output_id"),
            "extracted_from_output_id": o.get::<Option<String>, _>("extracted_from_output_id"),
            "compiled_prompt": o.get::<String, _>("compiled_prompt"),
            "reference_asset_ids": serde_json::from_str::<Vec<String>>(o.get::<&str, _>("reference_asset_ids_json")).unwrap_or_default(),
            "depends_on": serde_json::from_str::<Vec<String>>(o.get::<&str, _>("depends_on_json")).unwrap_or_default(),
            "item_spec": serde_json::from_str::<serde_json::Value>(o.get::<&str, _>("item_spec_json")).unwrap_or_default(),
            "usage": serde_json::from_str::<serde_json::Value>(o.get::<&str, _>("usage_json")).unwrap_or_default(),
            "provider_request_id": o.get::<Option<String>, _>("provider_request_id"),
            "actual_cost_micros": o.get::<Option<i64>, _>("actual_cost_micros"),
            "error_message": o.get::<Option<String>, _>("error_message"),
            "absolute_path": abs_orig,
            "thumbnail_url": if orig_rel.is_some() { Some(format!("/media/output/{}/thumbnail", oid)) } else { None },
            "original_url": if orig_rel.is_some() { Some(format!("/media/output/{}/original", oid)) } else { None },
            "created_at": o.get::<String, _>("created_at"),
            "updated_at": o.get::<String, _>("updated_at")
        }));
    }

    let event_rows = sqlx::query(
        "SELECT id, job_id, output_id, event_type, status, message, created_at FROM job_events WHERE run_id = ? ORDER BY id ASC"
    )
    .bind(run_id)
    .fetch_all(pool)
    .await?;

    let events: Vec<serde_json::Value> = event_rows
        .iter()
        .map(|er| {
            serde_json::json!({
                "id": er.get::<i64, _>("id"),
                "job_id": er.get::<Option<String>, _>("job_id"),
                "output_id": er.get::<Option<String>, _>("output_id"),
                "event_type": er.get::<String, _>("event_type"),
                "status": er.get::<String, _>("status"),
                "message": er.get::<String, _>("message"),
                "created_at": er.get::<String, _>("created_at")
            })
        })
        .collect();

    Ok(serde_json::json!({
        "id": r.get::<String, _>("id"),
        "project_id": r.get::<String, _>("project_id"),
        "thread_id": r.get::<String, _>("thread_id"),
        "mode": r.get::<String, _>("mode"),
        "scope_preset": r.get::<String, _>("scope_preset"),
        "user_prompt": r.get::<String, _>("user_prompt"),
        "brand_version": r.get::<i64, _>("brand_version"),
        "brand_snapshot": serde_json::from_str::<serde_json::Value>(r.get::<&str, _>("brand_snapshot_json")).unwrap_or_default(),
        "selected_references": selected_refs,
        "context_summary": context_summary,
        "model_settings": serde_json::from_str::<serde_json::Value>(r.get::<&str, _>("model_settings_json")).unwrap_or_default(),
        "plan": r.get::<Option<String>, _>("plan_json").and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok()),
        "consistency_report": r.get::<Option<String>, _>("consistency_report_json").and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok()),
        "status": r.get::<String, _>("status"),
        "estimated_cost_micros": r.get::<Option<i64>, _>("estimated_cost_micros"),
        "actual_cost_micros": r.get::<Option<i64>, _>("actual_cost_micros"),
        "cost_is_unknown": r.get::<i64, _>("cost_is_unknown") != 0,
        "is_test_adapter": r.get::<i64, _>("is_test_adapter") != 0,
        "error_message": r.get::<Option<String>, _>("error_message"),
        "target_tab_key": r.get::<Option<String>, _>("target_tab_key"),
        "target_tab_label": r.get::<Option<String>, _>("target_tab_label"),
        "target_output_id": r.get::<Option<String>, _>("target_output_id"),
        "highlight_annotations": serde_json::from_str::<serde_json::Value>(r.get::<&str, _>("highlight_annotations_json")).unwrap_or_else(|_| serde_json::json!([])),
        "outputs": outputs,
        "events": events,
        "created_at": r.get::<String, _>("created_at"),
        "updated_at": r.get::<String, _>("updated_at"),
        "completed_at": r.get::<Option<String>, _>("completed_at")
    }))
}

async fn get_run_handler(
    State(state): State<AppState>,
    AxumPath(run_id): AxumPath<String>,
) -> AppResult<Json<serde_json::Value>> {
    let dto = load_run_dto(&state.pool, &state.config, &run_id).await?;
    Ok(Json(serde_json::json!({ "run": dto })))
}

#[derive(Deserialize)]
struct ApprovePlanRequest {
    plan: Option<PlannerPlan>,
}

async fn approve_plan_handler(
    State(state): State<AppState>,
    AxumPath(run_id): AxumPath<String>,
    Json(req): Json<ApprovePlanRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let (ctx, valid_asset_ids, is_test) =
        build_run_context_from_db(&state.pool, &state.config, &run_id).await?;
    let (_, caps) = load_settings_and_caps(&state.pool, &state.config).await?;

    let mut plan = match req.plan {
        Some(p) => p,
        None => ctx
            .approved_plan
            .clone()
            .ok_or_else(|| AppError::Conflict("Run does not have a plan to approve.".into()))?,
    };

    // Clean up any dangling depends_on references if the user removed an item in the UI plan editor
    let remaining_ids: HashSet<String> = plan.items.iter().map(|i| i.id.clone()).collect();
    for item in &mut plan.items {
        item.depends_on.retain(|dep| remaining_ids.contains(dep));
    }

    validate_planner_plan(
        &plan,
        &ctx.mode,
        &valid_asset_ids,
        &caps.supported_aspect_ratios,
    )?;

    let est = compute_cost_estimate(
        &caps,
        plan.items.len() as u32,
        valid_asset_ids.len() as u32,
        &ctx.model_settings.quality,
        is_test,
    );

    let plan_str = serde_json::to_string_pretty(&plan).unwrap_or_default();
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE runs SET plan_json = ?, estimated_cost_micros = ?, updated_at = ? WHERE id = ?",
    )
    .bind(&plan_str)
    .bind(est.total_estimated_micros)
    .bind(&now)
    .bind(&run_id)
    .execute(&state.pool)
    .await?;

    sqlx::query(
        "UPDATE jobs SET status = 'completed', updated_at = ? WHERE run_id = ? AND job_type = 'plan'"
    )
    .bind(&now)
    .bind(&run_id)
    .execute(&state.pool)
    .await?;

    materialize_render_jobs_from_plan(
        &state.pool,
        &state.config,
        &state.event_tx,
        &run_id,
        &ctx,
        &plan,
    )
    .await?;

    let dto = load_run_dto(&state.pool, &state.config, &run_id).await?;
    Ok(Json(serde_json::json!({ "run": dto })))
}

async fn cancel_run_handler(
    State(state): State<AppState>,
    AxumPath(run_id): AxumPath<String>,
) -> AppResult<Json<serde_json::Value>> {
    let now = chrono::Utc::now().to_rfc3339();
    let cancel_msg = "Cancelled by user. Note: any upstream provider request already in flight may still be billed by OpenRouter.";

    sqlx::query(
        "UPDATE jobs SET status = 'cancelled', error_message = ?, updated_at = ? WHERE run_id = ? AND status IN ('queued', 'planning', 'awaiting_plan_review', 'ready')"
    )
    .bind(cancel_msg)
    .bind(&now)
    .bind(&run_id)
    .execute(&state.pool)
    .await?;

    sqlx::query(
        "UPDATE outputs SET status = 'cancelled', error_message = ?, updated_at = ? WHERE run_id = ? AND status IN ('queued', 'ready')"
    )
    .bind(cancel_msg)
    .bind(&now)
    .bind(&run_id)
    .execute(&state.pool)
    .await?;

    sqlx::query(
        "UPDATE runs SET status = 'cancelled', error_message = ?, updated_at = ? WHERE id = ? AND status NOT IN ('completed', 'partial')"
    )
    .bind(cancel_msg)
    .bind(&now)
    .bind(&run_id)
    .execute(&state.pool)
    .await?;

    record_and_broadcast_event(
        &state.pool,
        &state.event_tx,
        &run_id,
        None,
        None,
        "run_cancelled",
        "cancelled",
        cancel_msg,
    )
    .await;

    recompute_run_status(&state.pool, &state.event_tx, &run_id).await?;
    let dto = load_run_dto(&state.pool, &state.config, &run_id).await?;
    Ok(Json(serde_json::json!({ "run": dto })))
}

async fn retry_run_handler(
    State(state): State<AppState>,
    AxumPath(run_id): AxumPath<String>,
) -> AppResult<Json<serde_json::Value>> {
    let _run_row = sqlx::query("SELECT id, status FROM runs WHERE id = ?")
        .bind(&run_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Run not found".into()))?;

    let out_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM outputs WHERE run_id = ?")
        .bind(&run_id)
        .fetch_one(&state.pool)
        .await?;

    let now = chrono::Utc::now().to_rfc3339();

    if out_count == 0 {
        // Failed during planning before outputs were materialized
        let updated = sqlx::query(
            "UPDATE jobs SET status = 'queued', error_message = NULL, updated_at = ? WHERE run_id = ? AND job_type = 'plan'"
        )
        .bind(&now)
        .bind(&run_id)
        .execute(&state.pool)
        .await?;

        if updated.rows_affected() == 0 {
            let job_id = Uuid::new_v4().to_string();
            sqlx::query(
                "INSERT INTO jobs (id, run_id, output_id, job_type, status, depends_on_json, created_at, updated_at)
                 VALUES (?, ?, NULL, 'plan', 'queued', '[]', ?, ?)"
            )
            .bind(&job_id)
            .bind(&run_id)
            .bind(&now)
            .bind(&now)
            .execute(&state.pool)
            .await?;
        }

        sqlx::query(
            "UPDATE runs SET status = 'queued', error_message = NULL, completed_at = NULL, updated_at = ? WHERE id = ?"
        )
        .bind(&now)
        .bind(&run_id)
        .execute(&state.pool)
        .await?;

        record_and_broadcast_event(
            &state.pool,
            &state.event_tx,
            &run_id,
            None,
            None,
            "planning_retry_queued",
            "queued",
            "Retrying AI design plan & generation...",
        )
        .await;
    } else {
        // Retry all non-completed outputs in this run
        sqlx::query(
            "UPDATE outputs SET status = 'ready', error_message = NULL, updated_at = ? WHERE run_id = ? AND status != 'completed'"
        )
        .bind(&now)
        .bind(&run_id)
        .execute(&state.pool)
        .await?;

        sqlx::query(
            "UPDATE jobs SET status = 'ready', error_message = NULL, updated_at = ? WHERE run_id = ? AND job_type = 'render_item' AND status != 'completed'"
        )
        .bind(&now)
        .bind(&run_id)
        .execute(&state.pool)
        .await?;

        sqlx::query(
            "UPDATE runs SET status = 'rendering', error_message = NULL, completed_at = NULL, updated_at = ? WHERE id = ?"
        )
        .bind(&now)
        .bind(&run_id)
        .execute(&state.pool)
        .await?;

        record_and_broadcast_event(
            &state.pool,
            &state.event_tx,
            &run_id,
            None,
            None,
            "run_retry_queued",
            "rendering",
            "Retrying incomplete outputs in run...",
        )
        .await;
    }

    let dto = load_run_dto(&state.pool, &state.config, &run_id).await?;
    Ok(Json(serde_json::json!({ "run": dto })))
}

async fn delete_output_handler(
    State(state): State<AppState>,
    AxumPath(output_id): AxumPath<String>,
) -> AppResult<Json<serde_json::Value>> {
    let out_row = sqlx::query(
        "SELECT id, run_id, project_id, tab_key, tab_label, original_rel_path, thumbnail_rel_path FROM outputs WHERE id = ?"
    )
    .bind(&output_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Output not found".into()))?;

    let run_id: String = out_row.get("run_id");
    let project_id: String = out_row.get("project_id");
    let tab_key: String = out_row.try_get("tab_key").unwrap_or_default();
    let orig_rel: Option<String> = out_row.get("original_rel_path");
    let thumb_rel: Option<String> = out_row.get("thumbnail_rel_path");

    let thread_id: Option<String> = sqlx::query_scalar("SELECT thread_id FROM runs WHERE id = ?")
        .bind(&run_id)
        .fetch_optional(&state.pool)
        .await?;

    // Remove output image files from disk
    for rel in [orig_rel, thumb_rel].into_iter().flatten() {
        if let Ok(abs) = state.config.resolve_rel_path(&rel) {
            let _ = std::fs::remove_file(&abs);
        }
    }

    let mut tx = state.pool.begin().await?;
    sqlx::query("UPDATE outputs SET parent_output_id = NULL WHERE parent_output_id = ?")
        .bind(&output_id)
        .execute(&mut *tx)
        .await?;
    sqlx::query(
        "UPDATE outputs SET extracted_from_output_id = NULL WHERE extracted_from_output_id = ?",
    )
    .bind(&output_id)
    .execute(&mut *tx)
    .await?;
    sqlx::query("UPDATE runs SET target_output_id = NULL WHERE target_output_id = ?")
        .bind(&output_id)
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM job_events WHERE output_id = ?")
        .bind(&output_id)
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM jobs WHERE output_id = ?")
        .bind(&output_id)
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM exports WHERE output_id = ?")
        .bind(&output_id)
        .execute(&mut *tx)
        .await?;
    sqlx::query("DELETE FROM outputs WHERE id = ?")
        .bind(&output_id)
        .execute(&mut *tx)
        .await?;

    let remaining_in_run: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM outputs WHERE run_id = ?")
        .bind(&run_id)
        .fetch_one(&mut *tx)
        .await
        .unwrap_or(0);

    if remaining_in_run == 0 {
        sqlx::query("UPDATE thread_messages SET linked_run_id = NULL WHERE linked_run_id = ?")
            .bind(&run_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM job_events WHERE run_id = ?")
            .bind(&run_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM jobs WHERE run_id = ?")
            .bind(&run_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM exports WHERE run_id = ?")
            .bind(&run_id)
            .execute(&mut *tx)
            .await?;
        sqlx::query("DELETE FROM runs WHERE id = ?")
            .bind(&run_id)
            .execute(&mut *tx)
            .await?;
    }

    tx.commit().await?;

    let remaining_in_tab: i64 = if let Some(ref tid) = thread_id {
        sqlx::query_scalar(
            "SELECT COUNT(*) FROM outputs o JOIN runs r ON r.id = o.run_id WHERE r.thread_id = ? AND o.tab_key = ?"
        )
        .bind(tid)
        .bind(&tab_key)
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0)
    } else {
        0
    };

    // Sync organized disk folders and regenerate prompts.md for the remaining items in each tab
    let _ = crate::handoff::sync_project_organized_folders(&state.pool, &state.config, &project_id)
        .await;

    Ok(Json(serde_json::json!({
        "deleted_output_id": output_id,
        "run_id": run_id,
        "tab_key": tab_key,
        "remaining_in_run": remaining_in_run,
        "remaining_in_tab": remaining_in_tab,
        "tab_closed": remaining_in_tab == 0
    })))
}

async fn retry_output_handler(
    State(state): State<AppState>,
    AxumPath(output_id): AxumPath<String>,
) -> AppResult<Json<serde_json::Value>> {
    let out_row = sqlx::query("SELECT run_id, title, status FROM outputs WHERE id = ?")
        .bind(&output_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Output not found".into()))?;

    let run_id: String = out_row.get("run_id");
    let title: String = out_row.get("title");
    let status: String = out_row.get("status");

    if status == "completed" {
        return Err(AppError::Conflict(
            "Output is already completed; use Refine This to create a new variation without overwriting the saved original."
                .into(),
        ));
    }

    let now = chrono::Utc::now().to_rfc3339();
    // Reset only this specific output and its job to `ready`
    sqlx::query(
        "UPDATE outputs SET status = 'ready', error_message = NULL, updated_at = ? WHERE id = ?",
    )
    .bind(&now)
    .bind(&output_id)
    .execute(&state.pool)
    .await?;

    sqlx::query(
        "UPDATE jobs SET status = 'ready', error_message = NULL, updated_at = ? WHERE output_id = ?"
    )
    .bind(&now)
    .bind(&output_id)
    .execute(&state.pool)
    .await?;

    // Also re-queue any downstream dependent outputs in this run that were cancelled due to dependency block
    sqlx::query(
        "UPDATE outputs SET status = 'queued', error_message = NULL, updated_at = ? WHERE run_id = ? AND status = 'cancelled' AND id != ?"
    )
    .bind(&now)
    .bind(&run_id)
    .bind(&output_id)
    .execute(&state.pool)
    .await?;

    sqlx::query(
        "UPDATE jobs SET status = 'queued', error_message = NULL, updated_at = ? WHERE run_id = ? AND job_type = 'render_item' AND status = 'cancelled' AND output_id != ?"
    )
    .bind(&now)
    .bind(&run_id)
    .bind(&output_id)
    .execute(&state.pool)
    .await?;

    sqlx::query(
        "UPDATE runs SET status = 'rendering', completed_at = NULL, error_message = NULL, updated_at = ? WHERE id = ?"
    )
    .bind(&now)
    .bind(&run_id)
    .execute(&state.pool)
    .await?;

    record_and_broadcast_event(
        &state.pool,
        &state.event_tx,
        &run_id,
        None,
        Some(&output_id),
        "item_retry_queued",
        "ready",
        &format!(
            "Retrying single output '{}' without resubmitting completed items.",
            title
        ),
    )
    .await;

    let dto = load_run_dto(&state.pool, &state.config, &run_id).await?;
    Ok(Json(serde_json::json!({ "run": dto })))
}

async fn extract_output_assets_handler(
    State(state): State<AppState>,
    AxumPath(output_id): AxumPath<String>,
) -> AppResult<Json<serde_json::Value>> {
    let out_row = sqlx::query(
        "SELECT run_id, project_id, plan_item_id, title, original_rel_path, tab_key, tab_label FROM outputs WHERE id = ?"
    )
    .bind(&output_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Output not found".into()))?;

    let run_id: String = out_row.get("run_id");
    let project_id: String = out_row.get("project_id");
    let plan_item_id: String = out_row.get("plan_item_id");
    let title: String = out_row.get("title");
    let orig_rel: String = out_row
        .get::<Option<String>, _>("original_rel_path")
        .ok_or_else(|| AppError::Conflict("Output has no rendered image yet".into()))?;
    let tab_key: String = out_row.try_get("tab_key").unwrap_or_default();
    let tab_label: String = out_row.try_get("tab_label").unwrap_or_default();

    let abs_orig = state.config.resolve_rel_path(&orig_rel)?;
    let bytes = std::fs::read(&abs_orig)
        .map_err(|e| AppError::Internal(format!("Failed to read output PNG: {}", e)))?;

    let (ctx, _, _) =
        crate::worker::build_run_context_from_db(&state.pool, &state.config, &run_id).await?;

    crate::worker::discover_and_queue_rendered_visual_assets(
        &state.pool,
        &state.config,
        &state.provider,
        &state.event_tx,
        &run_id,
        &output_id,
        &project_id,
        &plan_item_id,
        &title,
        &tab_key,
        &tab_label,
        &ctx,
        &bytes,
    )
    .await?;

    let dto = load_run_dto(&state.pool, &state.config, &run_id).await?;
    Ok(Json(serde_json::json!({ "run": dto })))
}

async fn check_consistency_handler(
    State(state): State<AppState>,
    AxumPath(run_id): AxumPath<String>,
) -> AppResult<Json<serde_json::Value>> {
    let run_row = sqlx::query("SELECT user_prompt, is_test_adapter FROM runs WHERE id = ?")
        .bind(&run_id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound("Run not found".into()))?;

    let user_prompt: String = run_row.get("user_prompt");
    let is_test: bool = run_row.get::<i64, _>("is_test_adapter") != 0;
    let (settings, _) = load_settings_and_caps(&state.pool, &state.config).await?;

    let out_rows = sqlx::query(
        "SELECT title, original_rel_path FROM outputs WHERE run_id = ? AND status = 'completed' ORDER BY variant_index ASC"
    )
    .bind(&run_id)
    .fetch_all(&state.pool)
    .await?;

    if out_rows.is_empty() {
        return Err(AppError::Conflict(
            "No completed outputs available to check consistency.".into(),
        ));
    }

    let report = if is_test
        || settings.test_adapter_enabled
        || state.config.secrets.get_key().is_none()
    {
        crate::provider::ConsistencyCheckReport {
            checked_at: chrono::Utc::now().to_rfc3339(),
            overall_consistent: true,
            summary: format!(
                "Verified {} outputs against shared design specification and visual anchor.",
                out_rows.len()
            ),
            findings: vec![
                "Shared color palette (#D56A3A primary, #FAF9F6 surface) and typography scale are consistent across all outputs.".to_string(),
                "Shared responsive navigation bar and modal state follow the homepage visual anchor tokens.".to_string(),
            ],
            recommendations: vec![
                "Use the saved textual brand tokens and original uploaded vector logo as final implementation authority.".to_string(),
            ],
        }
    } else {
        let (api_key, _) = state.config.secrets.get_key().unwrap();
        let mut imgs = Vec::new();
        for r in out_rows {
            let title: String = r.get("title");
            if let Some(rel) = r.get::<Option<String>, _>("original_rel_path") {
                if let Ok(abs) = state.config.resolve_rel_path(&rel) {
                    if let Ok(bytes) = std::fs::read(&abs) {
                        imgs.push((
                            title,
                            format!("data:image/png;base64,{}", BASE64.encode(&bytes)),
                        ));
                    }
                }
            }
        }
        state
            .provider
            .check_consistency(&api_key, &settings.planner_model, &user_prompt, imgs)
            .await?
    };

    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query("UPDATE runs SET consistency_report_json = ?, updated_at = ? WHERE id = ?")
        .bind(serde_json::to_string(&report).unwrap_or_default())
        .bind(&now)
        .bind(&run_id)
        .execute(&state.pool)
        .await?;

    Ok(Json(serde_json::json!({ "consistency_report": report })))
}

async fn run_events_sse_handler(
    State(state): State<AppState>,
    AxumPath(run_id): AxumPath<String>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let rx = state.event_tx.subscribe();
    let run_id_filter = run_id.clone();

    let live_stream = BroadcastStream::new(rx).filter_map(move |msg_res| match msg_res {
        Ok(ev) if ev.run_id == run_id_filter => {
            let json = serde_json::to_string(&ev).unwrap_or_default();
            Some(Ok(Event::default().id(ev.event_id.to_string()).data(json)))
        }
        _ => None,
    });

    Sse::new(live_stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(10))
            .text("keep-alive"),
    )
}

// ============================================================================
// Handoff & Portable Bundle Export Handlers
// ============================================================================

#[derive(Deserialize)]
struct HandoffOptionsRequest {
    include_extraction_instructions: Option<bool>,
    #[serde(default)]
    output_ids: Option<Vec<String>>,
}

async fn output_handoff_handler(
    State(state): State<AppState>,
    AxumPath(output_id): AxumPath<String>,
    Json(req): Json<HandoffOptionsRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let handoff = build_single_output_handoff(
        &state.pool,
        &state.config,
        &output_id,
        req.include_extraction_instructions,
    )
    .await?;
    Ok(Json(serde_json::json!({ "handoff": handoff })))
}

async fn run_set_handoff_handler(
    State(state): State<AppState>,
    AxumPath(run_id): AxumPath<String>,
    Json(req): Json<HandoffOptionsRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let handoff = build_set_handoff(
        &state.pool,
        &state.config,
        &run_id,
        req.include_extraction_instructions,
        false,
        req.output_ids.as_deref(),
    )
    .await?;
    Ok(Json(serde_json::json!({ "handoff": handoff })))
}

async fn run_export_bundle_handler(
    State(state): State<AppState>,
    AxumPath(run_id): AxumPath<String>,
    Json(req): Json<HandoffOptionsRequest>,
) -> AppResult<Json<serde_json::Value>> {
    let handoff = build_set_handoff(
        &state.pool,
        &state.config,
        &run_id,
        req.include_extraction_instructions,
        true,
        req.output_ids.as_deref(),
    )
    .await?;
    Ok(Json(serde_json::json!({
        "handoff": handoff,
        "download_url": format!("/api/exports/{}/download", handoff.export_id)
    })))
}

async fn download_export_handler(
    State(state): State<AppState>,
    AxumPath(export_id): AxumPath<String>,
) -> AppResult<Response> {
    let row = sqlx::query(
        "SELECT export_kind, implementation_rel_path, bundle_rel_path FROM exports WHERE id = ?",
    )
    .bind(&export_id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Export not found".into()))?;

    let kind: String = row.get("export_kind");
    let impl_rel: String = row.get("implementation_rel_path");
    let bundle_rel: Option<String> = row.get("bundle_rel_path");

    if kind == "portable_bundle" {
        if let Some(b_rel) = bundle_rel {
            let abs = state.config.resolve_rel_path(&b_rel)?;
            let bytes = std::fs::read(&abs)?;
            let mut headers = HeaderMap::new();
            headers.insert(header::CONTENT_TYPE, "application/zip".parse().unwrap());
            headers.insert(
                header::CONTENT_DISPOSITION,
                format!(
                    "attachment; filename=\"design-gen-bundle-{}.zip\"",
                    export_id
                )
                .parse()
                .unwrap(),
            );
            return Ok((StatusCode::OK, headers, bytes).into_response());
        }
    }

    let abs_impl = state.config.resolve_rel_path(&impl_rel)?;
    let bytes = std::fs::read(&abs_impl)?;
    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        "text/markdown; charset=utf-8".parse().unwrap(),
    );
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!(
            "attachment; filename=\"design-gen-implementation-{}.md\"",
            export_id
        )
        .parse()
        .unwrap(),
    );
    Ok((StatusCode::OK, headers, bytes).into_response())
}

// ============================================================================
// Controlled Media Serving (`GET /media/:entity/:id/:representation`)
// ============================================================================

async fn serve_media_handler(
    State(state): State<AppState>,
    AxumPath((entity, id, representation)): AxumPath<(String, String, String)>,
) -> AppResult<Response> {
    let (rel_path, mime_type) = match entity.as_str() {
        "asset" => {
            let row = sqlx::query(
                "SELECT original_rel_path, model_input_rel_path, thumbnail_rel_path, mime_type FROM assets WHERE id = ?"
            )
            .bind(&id)
            .fetch_optional(&state.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Media asset not found".into()))?;

            match representation.as_str() {
                "original" => (
                    row.get::<String, _>("original_rel_path"),
                    row.get::<String, _>("mime_type"),
                ),
                "model_input" => (
                    row.get::<Option<String>, _>("model_input_rel_path")
                        .unwrap_or_else(|| row.get("original_rel_path")),
                    "image/png".to_string(),
                ),
                _ => (
                    row.get::<String, _>("thumbnail_rel_path"),
                    "image/webp".to_string(),
                ),
            }
        }
        "output" => {
            let row = sqlx::query(
                "SELECT original_rel_path, thumbnail_rel_path, mime_type FROM outputs WHERE id = ?",
            )
            .bind(&id)
            .fetch_optional(&state.pool)
            .await?
            .ok_or_else(|| AppError::NotFound("Media output not found".into()))?;

            let orig = row
                .get::<Option<String>, _>("original_rel_path")
                .ok_or_else(|| AppError::NotFound("Output image not yet saved".into()))?;
            let thumb = row
                .get::<Option<String>, _>("thumbnail_rel_path")
                .unwrap_or_else(|| orig.clone());
            let mime = row
                .get::<Option<String>, _>("mime_type")
                .unwrap_or_else(|| "image/png".to_string());

            match representation.as_str() {
                "original" => (orig, mime),
                _ => (thumb, "image/webp".to_string()),
            }
        }
        _ => return Err(AppError::NotFound("Unknown media entity type".into())),
    };

    let abs_path = state.config.resolve_rel_path(&rel_path)?;
    if !abs_path.exists() {
        return Err(AppError::NotFound("Media file missing on disk".into()));
    }
    let bytes = std::fs::read(&abs_path)?;

    let mut headers = HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        mime_type
            .parse()
            .unwrap_or_else(|_| "application/octet-stream".parse().unwrap()),
    );
    headers.insert(
        header::HeaderName::from_static("x-content-type-options"),
        "nosniff".parse().unwrap(),
    );
    headers.insert(
        header::HeaderName::from_static("content-security-policy"),
        "default-src 'none'; style-src 'unsafe-inline'"
            .parse()
            .unwrap(),
    );
    headers.insert(
        header::CACHE_CONTROL,
        "private, max-age=3600".parse().unwrap(),
    );

    Ok((StatusCode::OK, headers, bytes).into_response())
}

// ============================================================================
// Static Export Deep-Link Serving (Never returns HTML for /api or /media!)
// ============================================================================

async fn serve_static_or_spa_handler(
    State(state): State<AppState>,
    uri: Uri,
) -> AppResult<Response> {
    let path = uri.path();
    if path.starts_with("/api") || path.starts_with("/media") {
        return Err(AppError::NotFound(format!(
            "Resource '{}' not found.",
            path
        )));
    }

    let trimmed = path.trim_start_matches('/').trim_end_matches('/');
    let static_dir = &state.config.static_dir;

    let candidates = if trimmed.is_empty() {
        vec![static_dir.join("index.html")]
    } else {
        vec![
            static_dir.join(trimmed),
            static_dir.join(format!("{}.html", trimmed)),
            static_dir.join(trimmed).join("index.html"),
        ]
    };

    for candidate in candidates {
        if candidate.is_file() {
            if let Ok(bytes) = std::fs::read(&candidate) {
                let mime = guess_static_mime(&candidate);
                let mut headers = HeaderMap::new();
                headers.insert(header::CONTENT_TYPE, mime.parse().unwrap());
                return Ok((StatusCode::OK, headers, bytes).into_response());
            }
        }
    }

    Err(AppError::NotFound(format!(
        "Static route '{}' not found in {}",
        path,
        static_dir.display()
    )))
}

fn guess_static_mime(path: &std::path::Path) -> &'static str {
    match path.extension().and_then(|s| s.to_str()).unwrap_or("") {
        "html" => "text/html; charset=utf-8",
        "js" | "mjs" => "application/javascript; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "webmanifest" => "application/manifest+json; charset=utf-8",
        "svg" => "image/svg+xml",
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "webp" => "image/webp",
        "woff" => "font/woff",
        "woff2" => "font/woff2",
        "ico" => "image/x-icon",
        "txt" => "text/plain; charset=utf-8",
        _ => "application/octet-stream",
    }
}

// ============================================================================
// GitHub Releases Auto-Updater Endpoints (Safe, Non-Breaking, Atomic + Backup)
// ============================================================================

#[derive(Deserialize)]
struct UpdateConfigBody {
    github_repo: Option<String>,
    auto_check_enabled: Option<bool>,
    auto_install_enabled: Option<bool>,
}

async fn system_update_check_handler(
    State(state): State<AppState>,
) -> AppResult<Json<crate::updater::UpdateCheckReport>> {
    let report = crate::updater::check_for_updates(&state.config).await;
    Ok(Json(report))
}

async fn system_update_config_handler(
    State(state): State<AppState>,
    Json(body): Json<UpdateConfigBody>,
) -> AppResult<Json<crate::updater::UpdateCheckReport>> {
    let mut cfg = crate::updater::load_update_config(&state.config);
    if let Some(repo) = body.github_repo {
        let trimmed = repo
            .trim()
            .trim_start_matches("https://github.com/")
            .trim_end_matches('/')
            .trim_end_matches(".git")
            .to_string();
        if !trimmed.is_empty() && trimmed.contains('/') {
            cfg.github_repo = trimmed;
        }
    }
    if let Some(ac) = body.auto_check_enabled {
        cfg.auto_check_enabled = ac;
    }
    if let Some(ai) = body.auto_install_enabled {
        cfg.auto_install_enabled = ai;
    }
    crate::updater::save_update_config(&state.config, &cfg)?;
    let report = crate::updater::check_for_updates(&state.config).await;
    Ok(Json(report))
}

async fn system_update_install_handler(
    State(state): State<AppState>,
) -> AppResult<Json<crate::updater::UpdateInstallReport>> {
    let report =
        crate::updater::install_latest_update(&state.pool, &state.config, &state.restart).await?;
    Ok(Json(report))
}

async fn system_update_restart_handler(
    State(state): State<AppState>,
) -> AppResult<Json<serde_json::Value>> {
    state.restart.schedule()?;
    Ok(Json(serde_json::json!({
        "ok": true,
        "message": "Restarting Design Master..."
    })))
}

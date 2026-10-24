use crate::{
    compiler::{compile_image_prompt, should_use_transparent_background},
    config::AppConfig,
    context_builder::{
        build_generation_context, BuiltGenerationContext, ProjectMetaSummary,
        ResolvedReferenceInput, PLANNER_SYSTEM_PROMPT_V1,
    },
    error::{AppError, AppResult},
    image_util::{atomic_write_file, validate_and_thumbnail_output_bytes},
    models::{
        normalize_tab_key, validate_planner_plan, BrandSpec, PlanItem, PlannerPlan,
        RunModelSettings, SelectedReference,
    },
    provider::{ModelCapabilitiesReport, OpenRouterClient, RenderFailureKind, TestFixtureAdapter},
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use serde::{Deserialize, Serialize};
use sqlx::{Row, SqlitePool};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Duration,
};
use tokio::sync::{broadcast, Semaphore};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunEventMessage {
    pub event_id: i64,
    pub run_id: String,
    pub job_id: Option<String>,
    pub output_id: Option<String>,
    pub event_type: String,
    pub status: String,
    pub message: String,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SaveJournalRecord {
    pub output_id: String,
    pub original_rel_path: String,
    pub thumbnail_rel_path: String,
    pub mime_type: String,
    pub width: u32,
    pub height: u32,
    pub byte_size: usize,
    pub has_alpha: bool,
    pub provider_request_id: String,
    pub usage_json: serde_json::Value,
    pub reported_cost_micros: Option<i64>,
}

#[allow(clippy::too_many_arguments)]
pub async fn record_and_broadcast_event(
    pool: &SqlitePool,
    tx: &broadcast::Sender<RunEventMessage>,
    run_id: &str,
    job_id: Option<&str>,
    output_id: Option<&str>,
    event_type: &str,
    status: &str,
    message: &str,
) {
    let now = chrono::Utc::now().to_rfc3339();
    let res = sqlx::query(
        "INSERT INTO job_events (run_id, job_id, output_id, event_type, status, message, payload_json, created_at) VALUES (?, ?, ?, ?, ?, ?, '{}', ?)"
    )
    .bind(run_id)
    .bind(job_id)
    .bind(output_id)
    .bind(event_type)
    .bind(status)
    .bind(message)
    .bind(&now)
    .execute(pool)
    .await;

    let event_id = res.map(|r| r.last_insert_rowid()).unwrap_or(0);
    let _ = tx.send(RunEventMessage {
        event_id,
        run_id: run_id.to_string(),
        job_id: job_id.map(|s| s.to_string()),
        output_id: output_id.map(|s| s.to_string()),
        event_type: event_type.to_string(),
        status: status.to_string(),
        message: message.to_string(),
        created_at: now,
    });
}

/// Startup recovery: attaches any fully saved outputs via save_journal, and transitions in-flight
/// planning/rendering jobs to `interrupted` or `outcome_unknown` without re-billing.
pub async fn recover_interrupted_jobs(
    pool: &SqlitePool,
    config: &AppConfig,
    event_tx: &broadcast::Sender<RunEventMessage>,
) -> AppResult<()> {
    let in_flight_jobs = sqlx::query(
        "SELECT id, run_id, output_id, job_type, status, upstream_request_id FROM jobs WHERE status IN ('planning', 'rendering', 'saving')"
    )
    .fetch_all(pool)
    .await?;

    let now = chrono::Utc::now().to_rfc3339();
    let mut affected_runs = HashSet::new();

    for row in in_flight_jobs {
        let job_id: String = row.get("id");
        let run_id: String = row.get("run_id");
        let output_id: Option<String> = row.get("output_id");
        let status: String = row.get("status");
        let upstream_req_id: Option<String> = row.get("upstream_request_id");
        affected_runs.insert(run_id.clone());

        // Check if a save_journal exists for this output
        if let Some(ref out_id) = output_id {
            if let Ok(Some(out_row)) =
                sqlx::query("SELECT save_journal_path FROM outputs WHERE id = ?")
                    .bind(out_id)
                    .fetch_optional(pool)
                    .await
            {
                let journal_rel: Option<String> = out_row.get("save_journal_path");
                if let Some(j_rel) = journal_rel {
                    if let Ok(abs_journal) = config.resolve_rel_path(&j_rel) {
                        if let Ok(j_bytes) = std::fs::read(&abs_journal) {
                            if let Ok(journal) =
                                serde_json::from_slice::<SaveJournalRecord>(&j_bytes)
                            {
                                if let Ok(abs_orig) =
                                    config.resolve_rel_path(&journal.original_rel_path)
                                {
                                    if abs_orig.exists() {
                                        // Attach saved file without regenerating!
                                        sqlx::query(
                                            "UPDATE outputs SET status = 'completed', original_rel_path = ?, thumbnail_rel_path = ?, mime_type = ?, width = ?, height = ?, byte_size = ?, has_alpha = ?, provider_request_id = ?, usage_json = ?, actual_cost_micros = ?, updated_at = ? WHERE id = ?"
                                        )
                                        .bind(&journal.original_rel_path)
                                        .bind(&journal.thumbnail_rel_path)
                                        .bind(&journal.mime_type)
                                        .bind(journal.width as i64)
                                        .bind(journal.height as i64)
                                        .bind(journal.byte_size as i64)
                                        .bind(if journal.has_alpha { 1 } else { 0 })
                                        .bind(&journal.provider_request_id)
                                        .bind(journal.usage_json.to_string())
                                        .bind(journal.reported_cost_micros)
                                        .bind(&now)
                                        .bind(out_id)
                                        .execute(pool)
                                        .await?;

                                        sqlx::query("UPDATE jobs SET status = 'completed', updated_at = ? WHERE id = ?")
                                            .bind(&now)
                                            .bind(&job_id)
                                            .execute(pool)
                                            .await?;

                                        record_and_broadcast_event(
                                            pool,
                                            event_tx,
                                            &run_id,
                                            Some(&job_id),
                                            Some(out_id),
                                            "recovered_from_journal",
                                            "completed",
                                            "Attached completed image from save journal after server restart.",
                                        )
                                        .await;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Otherwise, never automatically resubmit an in-flight paid request
        let new_status = if status == "rendering" || upstream_req_id.is_some() {
            "outcome_unknown"
        } else {
            "interrupted"
        };
        let msg = if new_status == "outcome_unknown" {
            "Server restarted while upstream request was in flight; outcome unknown. Use explicit Retry to regenerate if needed."
        } else {
            "Job interrupted by server restart before upstream completion."
        };

        sqlx::query("UPDATE jobs SET status = ?, error_message = ?, updated_at = ? WHERE id = ?")
            .bind(new_status)
            .bind(msg)
            .bind(&now)
            .bind(&job_id)
            .execute(pool)
            .await?;

        if let Some(ref out_id) = output_id {
            sqlx::query(
                "UPDATE outputs SET status = ?, error_message = ?, updated_at = ? WHERE id = ?",
            )
            .bind(new_status)
            .bind(msg)
            .bind(&now)
            .bind(out_id)
            .execute(pool)
            .await?;
        }

        record_and_broadcast_event(
            pool,
            event_tx,
            &run_id,
            Some(&job_id),
            output_id.as_deref(),
            "recovery",
            new_status,
            msg,
        )
        .await;
    }

    for run_id in affected_runs {
        recompute_run_status(pool, event_tx, &run_id).await?;
    }

    Ok(())
}

pub fn spawn_job_worker(
    pool: SqlitePool,
    config: AppConfig,
    provider: Arc<OpenRouterClient>,
    event_tx: broadcast::Sender<RunEventMessage>,
    mut shutdown_rx: broadcast::Receiver<()>,
) {
    tokio::spawn(async move {
        let active_semaphore = Arc::new(Semaphore::new(2));

        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    tracing::info!("Job worker shutting down gracefully.");
                    break;
                }
                _ = tokio::time::sleep(Duration::from_millis(250)) => {
                    if let Err(e) = tick_job_queue(&pool, &config, &provider, &event_tx, &active_semaphore).await {
                        tracing::error!("Job worker tick error: {}", e);
                    }
                }
            }
        }
    });
}

async fn tick_job_queue(
    pool: &SqlitePool,
    config: &AppConfig,
    provider: &Arc<OpenRouterClient>,
    event_tx: &broadcast::Sender<RunEventMessage>,
    semaphore: &Arc<Semaphore>,
) -> AppResult<()> {
    // 1. Check for queued `plan` jobs first
    if let Some(plan_job) = sqlx::query(
        "SELECT id, run_id FROM jobs WHERE job_type = 'plan' AND status = 'queued' ORDER BY created_at ASC LIMIT 1"
    )
    .fetch_optional(pool)
    .await?
    {
        let job_id: String = plan_job.get("id");
        let run_id: String = plan_job.get("run_id");
        execute_plan_job(pool, config, provider, event_tx, &job_id, &run_id).await?;
        return Ok(());
    }

    // 2. Check for `queued` or `ready` `render_item` jobs
    if semaphore.available_permits() == 0 {
        return Ok(());
    }

    let candidate_jobs = sqlx::query(
        "SELECT id, run_id, output_id, depends_on_job_ids_json FROM jobs WHERE job_type = 'render_item' AND status IN ('queued', 'ready') ORDER BY created_at ASC LIMIT 10"
    )
    .fetch_all(pool)
    .await?;

    for row in candidate_jobs {
        if semaphore.available_permits() == 0 {
            break;
        }
        let job_id: String = row.get("id");
        let run_id: String = row.get("run_id");
        let output_id: String = row
            .get::<Option<String>, _>("output_id")
            .unwrap_or_default();
        let deps_str: String = row.get("depends_on_job_ids_json");
        let dep_job_ids: Vec<String> = serde_json::from_str(&deps_str).unwrap_or_default();

        // Check prerequisite job statuses
        let mut all_deps_completed = true;
        let mut failed_dep: Option<String> = None;

        for dep_id in &dep_job_ids {
            if let Some(dep_row) = sqlx::query("SELECT status FROM jobs WHERE id = ?")
                .bind(dep_id)
                .fetch_optional(pool)
                .await?
            {
                let dep_status: String = dep_row.get("status");
                match dep_status.as_str() {
                    "completed" => {}
                    "failed" | "cancelled" | "interrupted" | "outcome_unknown" => {
                        failed_dep = Some(format!(
                            "Prerequisite job {} ended with status '{}'",
                            dep_id, dep_status
                        ));
                        break;
                    }
                    _ => {
                        all_deps_completed = false;
                    }
                }
            }
        }

        if let Some(reason) = failed_dep {
            // Fail safely when a prerequisite anchor or asset failed!
            let now = chrono::Utc::now().to_rfc3339();
            let msg = format!(
                "Blocked safely: prerequisite anchor or asset did not complete ({}). No paid request made.",
                reason
            );
            sqlx::query("UPDATE jobs SET status = 'cancelled', error_message = ?, updated_at = ? WHERE id = ?")
                .bind(&msg)
                .bind(&now)
                .bind(&job_id)
                .execute(pool)
                .await?;
            sqlx::query("UPDATE outputs SET status = 'cancelled', error_message = ?, updated_at = ? WHERE id = ?")
                .bind(&msg)
                .bind(&now)
                .bind(&output_id)
                .execute(pool)
                .await?;
            record_and_broadcast_event(
                pool,
                event_tx,
                &run_id,
                Some(&job_id),
                Some(&output_id),
                "dependency_blocked",
                "cancelled",
                &msg,
            )
            .await;
            recompute_run_status(pool, event_tx, &run_id).await?;
            continue;
        }

        if !all_deps_completed {
            continue;
        }

        // Claim the job atomically
        let now = chrono::Utc::now().to_rfc3339();
        let claimed = sqlx::query(
            "UPDATE jobs SET status = 'rendering', attempts = attempts + 1, leased_at = ?, updated_at = ? WHERE id = ? AND status IN ('queued', 'ready')"
        )
        .bind(&now)
        .bind(&now)
        .bind(&job_id)
        .execute(pool)
        .await?;

        if claimed.rows_affected() == 0 {
            continue;
        }

        let permit = match semaphore.clone().try_acquire_owned() {
            Ok(p) => p,
            Err(_) => {
                // Revert to ready if permit wasn't available
                sqlx::query("UPDATE jobs SET status = 'ready', updated_at = ? WHERE id = ?")
                    .bind(&now)
                    .bind(&job_id)
                    .execute(pool)
                    .await?;
                break;
            }
        };

        let pool_c = pool.clone();
        let config_c = config.clone();
        let provider_c = provider.clone();
        let event_tx_c = event_tx.clone();

        tokio::spawn(async move {
            let _permit = permit;
            if let Err(e) = execute_render_item_job(
                &pool_c,
                &config_c,
                &provider_c,
                &event_tx_c,
                &job_id,
                &run_id,
                &output_id,
            )
            .await
            {
                tracing::error!(job_id = %job_id, "Render item job error: {}", e);
            }
        });
    }

    Ok(())
}

pub async fn build_run_context_from_db(
    pool: &SqlitePool,
    config: &AppConfig,
    run_id: &str,
) -> AppResult<(BuiltGenerationContext, HashSet<String>, bool)> {
    let run_row = sqlx::query(
        "SELECT project_id, thread_id, mode, user_prompt, brand_version, brand_snapshot_json, selected_references_json, model_settings_json, plan_json, is_test_adapter FROM runs WHERE id = ?"
    )
    .bind(run_id)
    .fetch_one(pool)
    .await?;

    let project_id: String = run_row.get("project_id");
    let thread_id: String = run_row.get("thread_id");
    let mode: String = run_row.get("mode");
    let user_prompt: String = run_row.get("user_prompt");
    let brand_version: i64 = run_row.get("brand_version");
    let brand_snapshot: BrandSpec =
        serde_json::from_str(run_row.get::<&str, _>("brand_snapshot_json")).unwrap_or_default();
    let selected_refs: Vec<SelectedReference> =
        serde_json::from_str(run_row.get::<&str, _>("selected_references_json"))
            .unwrap_or_default();
    let mut model_settings: RunModelSettings =
        serde_json::from_str(run_row.get::<&str, _>("model_settings_json"))?;
    let approved_plan: Option<PlannerPlan> = run_row
        .get::<Option<String>, _>("plan_json")
        .and_then(|s| serde_json::from_str(&s).ok());
    let is_test_adapter: bool = run_row.get::<i64, _>("is_test_adapter") != 0;

    let proj_row = sqlx::query(
        "SELECT id, name, description, purpose, industry, target_audience, audience_needs, desired_user_action FROM projects WHERE id = ?"
    )
    .bind(&project_id)
    .fetch_one(pool)
    .await?;

    let proj_meta = ProjectMetaSummary {
        id: proj_row.get("id"),
        name: proj_row.get("name"),
        description: proj_row.get("description"),
        purpose: proj_row.get("purpose"),
        industry: proj_row.get("industry"),
        target_audience: proj_row.get("target_audience"),
        audience_needs: proj_row.get("audience_needs"),
        desired_user_action: proj_row.get("desired_user_action"),
    };

    let mut resolved_refs = Vec::new();
    let mut valid_asset_ids = HashSet::new();

    // Also include primary brand logo if configured and not already in selected_refs
    let mut refs_to_load = selected_refs.clone();
    if let Some(ref logo_id) = brand_snapshot.logo.primary_asset_id {
        if !refs_to_load.iter().any(|r| &r.asset_id == logo_id)
            && brand_snapshot.logo.inclusion_policy != "never"
        {
            refs_to_load.push(SelectedReference {
                asset_id: logo_id.clone(),
                role: crate::models::AttachmentRole::BrandLogo,
                label: "Primary Brand Logo".to_string(),
                placement_notes: brand_snapshot.logo.preferred_usage.clone(),
                preserve_as_is: true,
                source_output_id: None,
            });
        }
    }

    for sr in refs_to_load {
        if let Some(asset_row) = sqlx::query(
            "SELECT id, label, original_rel_path, model_input_rel_path, mime_type, width, height FROM assets WHERE id = ? AND project_id = ?"
        )
        .bind(&sr.asset_id)
        .bind(&project_id)
        .fetch_optional(pool)
        .await?
        {
            let aid: String = asset_row.get("id");
            valid_asset_ids.insert(aid.clone());
            let model_rel: Option<String> = asset_row.get("model_input_rel_path");
            let orig_rel: String = asset_row.get("original_rel_path");
            let use_rel = model_rel.unwrap_or(orig_rel);
            let mut data_url = String::new();
            if let Ok(abs) = config.resolve_rel_path(&use_rel) {
                if let Ok(bytes) = std::fs::read(&abs) {
                    data_url = format!("data:image/png;base64,{}", BASE64.encode(&bytes));
                }
            }
            resolved_refs.push(ResolvedReferenceInput {
                asset_id: aid,
                role: sr.role,
                label: sr.label,
                placement_notes: sr.placement_notes,
                preserve_as_is: sr.preserve_as_is,
                mime_type: asset_row.get("mime_type"),
                width: asset_row.get::<i64, _>("width") as u32,
                height: asset_row.get::<i64, _>("height") as u32,
                data_url,
            });
        }
    }

    let out_rows = sqlx::query(
        "SELECT o.id, o.title, o.kind, o.aspect_ratio, o.tab_key, o.tab_label, o.extracted_from_output_id, o.transparent_background FROM outputs o JOIN runs r ON o.run_id = r.id WHERE r.thread_id = ? AND o.status = 'completed' ORDER BY o.created_at DESC"
    )
    .bind(&thread_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    let mut existing_tabs: Vec<crate::models::ExistingTabSummary> = Vec::new();
    for row in &out_rows {
        let extracted_from: Option<String> = row.get("extracted_from_output_id");
        let kind_str: String = row.get("kind");
        let trans_bg: i64 = row.try_get("transparent_background").unwrap_or(0);
        if extracted_from.is_some() || kind_str == "asset" || trans_bg != 0 {
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
            // When adapting a tab to mobile/tablet, prefer anchoring to the Desktop root UI output
            if (model_settings.viewport_intent.as_deref() == Some("mobile")
                || model_settings.viewport_intent.as_deref() == Some("tablet"))
                && existing.viewport_intent != "desktop"
                && vp_intent == "desktop"
            {
                existing.latest_output_id = out_id;
                existing.title = title;
                existing.aspect_ratio = ar;
                existing.viewport_intent = vp_intent;
            }
        } else {
            existing_tabs.push(crate::models::ExistingTabSummary {
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

    if model_settings
        .target_existing_output_id
        .as_ref()
        .map(|s| s.trim().is_empty())
        .unwrap_or(true)
    {
        if let Some(ref tk) = model_settings.target_tab_key {
            let norm_tk = normalize_tab_key(tk);
            if let Some(matching_tab) = existing_tabs.iter().find(|t| t.tab_key == norm_tk) {
                model_settings.target_existing_output_id =
                    Some(matching_tab.latest_output_id.clone());
            }
        }
    }

    // Ensure `target_existing_output_id` is loaded into `resolved_refs` as `SelectedPreviousOutput`
    // so BOTH the multimodal planner and image-to-image renderer receive the existing tab UI image!
    if let Some(ref target_out_id) = model_settings.target_existing_output_id {
        let already_loaded = resolved_refs.iter().any(|r| {
            r.role == crate::models::AttachmentRole::SelectedPreviousOutput
                || r.asset_id == *target_out_id
        });
        if !already_loaded {
            if let Ok(Some(prev_row)) = sqlx::query(
                "SELECT id, title, original_rel_path, mime_type, width, height FROM outputs WHERE id = ? AND project_id = ?",
            )
            .bind(target_out_id)
            .bind(&project_id)
            .fetch_optional(pool)
            .await
            {
                if let Some(rel_path) = prev_row.get::<Option<String>, _>("original_rel_path") {
                    if let Ok(abs_path) = config.resolve_rel_path(&rel_path) {
                        if let Ok(bytes) = std::fs::read(&abs_path) {
                            let mime: String = prev_row
                                .get::<Option<String>, _>("mime_type")
                                .unwrap_or_else(|| "image/png".to_string());
                            let data_url =
                                format!("data:{};base64,{}", mime, BASE64.encode(&bytes));
                            valid_asset_ids.insert(target_out_id.clone());
                            resolved_refs.push(ResolvedReferenceInput {
                                asset_id: target_out_id.clone(),
                                role: crate::models::AttachmentRole::SelectedPreviousOutput,
                                label: prev_row.get("title"),
                                placement_notes: String::new(),
                                preserve_as_is: false,
                                mime_type: mime,
                                width: prev_row.get::<Option<i64>, _>("width").unwrap_or(1024)
                                    as u32,
                                height: prev_row.get::<Option<i64>, _>("height").unwrap_or(1024)
                                    as u32,
                                data_url,
                            });
                        }
                    }
                }
            }
        }
    }

    let ctx = build_generation_context(
        PLANNER_SYSTEM_PROMPT_V1,
        proj_meta,
        brand_version,
        brand_snapshot,
        &mode,
        &user_prompt,
        model_settings,
        resolved_refs,
        approved_plan,
        vec![],
    );

    Ok((ctx, valid_asset_ids, is_test_adapter))
}

async fn execute_plan_job(
    pool: &SqlitePool,
    config: &AppConfig,
    provider: &Arc<OpenRouterClient>,
    event_tx: &broadcast::Sender<RunEventMessage>,
    job_id: &str,
    run_id: &str,
) -> AppResult<()> {
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE jobs SET status = 'planning', attempts = attempts + 1, updated_at = ? WHERE id = ?",
    )
    .bind(&now)
    .bind(job_id)
    .execute(pool)
    .await?;
    sqlx::query("UPDATE runs SET status = 'planning', updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(run_id)
        .execute(pool)
        .await?;

    record_and_broadcast_event(
        pool,
        event_tx,
        run_id,
        Some(job_id),
        None,
        "planning_started",
        "planning",
        "Analyzing brand kit, references, and request to build design plan...",
    )
    .await;

    let (ctx, valid_asset_ids, is_test_adapter) =
        build_run_context_from_db(pool, config, run_id).await?;

    let settings_row = sqlx::query("SELECT capability_cache_json FROM settings WHERE id = 1")
        .fetch_one(pool)
        .await?;
    let caps: ModelCapabilitiesReport = settings_row
        .get::<Option<String>, _>("capability_cache_json")
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    let plan_result: AppResult<(PlannerPlan, Option<i64>)> = if is_test_adapter {
        let mut plan = TestFixtureAdapter::build_fixture_plan(&ctx);
        crate::models::sanitize_planner_plan(
            &mut plan,
            &ctx.mode,
            &valid_asset_ids,
            &caps.supported_aspect_ratios,
        );
        validate_planner_plan(
            &plan,
            &ctx.mode,
            &valid_asset_ids,
            &caps.supported_aspect_ratios,
        )?;
        Ok((plan, Some(0)))
    } else {
        match config.secrets.get_key() {
            Some((key, _)) => {
                let pool_hb = pool.clone();
                let tx_hb = event_tx.clone();
                let run_id_hb = run_id.to_string();
                let job_id_hb = job_id.to_string();
                let hb_handle = tokio::spawn(async move {
                    let mut elapsed = 0_u64;
                    loop {
                        tokio::time::sleep(Duration::from_secs(4)).await;
                        elapsed += 4;
                        record_and_broadcast_event(
                            &pool_hb,
                            &tx_hb,
                            &run_id_hb,
                            Some(&job_id_hb),
                            None,
                            "planning_progress",
                            "planning",
                            &format!(
                                "GLM 5.3 Flash is designing layout architecture & component specs ({}s elapsed)...",
                                elapsed
                            ),
                        )
                        .await;
                    }
                });
                let res = provider
                    .run_planner(&key, &ctx, &valid_asset_ids, &caps.supported_aspect_ratios)
                    .await;
                hb_handle.abort();
                res
            }
            None => Err(AppError::Unauthorized(
                "No OpenRouter API key configured. Configure an API key in Settings or enable Test Fixture Mode."
                    .into(),
            )),
        }
    };

    let (plan, planner_cost_micros) = match plan_result {
        Ok(v) => v,
        Err(e) => {
            let err_msg = e.to_string();
            let now2 = chrono::Utc::now().to_rfc3339();
            sqlx::query(
                "UPDATE jobs SET status = 'failed', error_message = ?, updated_at = ? WHERE id = ?",
            )
            .bind(&err_msg)
            .bind(&now2)
            .bind(job_id)
            .execute(pool)
            .await?;
            sqlx::query(
                "UPDATE runs SET status = 'failed', error_message = ?, updated_at = ? WHERE id = ?",
            )
            .bind(&err_msg)
            .bind(&now2)
            .bind(run_id)
            .execute(pool)
            .await?;
            if let Ok(Some(t_id)) =
                sqlx::query_scalar::<_, String>("SELECT thread_id FROM runs WHERE id = ?")
                    .bind(run_id)
                    .fetch_optional(pool)
                    .await
            {
                let msg_id = Uuid::new_v4().to_string();
                let _ = sqlx::query(
                    "INSERT INTO thread_messages (id, thread_id, project_id, role, content, attachments_json, quick_replies_json, highlight_annotations_json, run_id, target_tab_key, target_tab_label, created_at) VALUES (?, ?, ?, 'assistant', ?, '[]', '[]', '[]', ?, ?, ?, ?)"
                )
                .bind(&msg_id)
                .bind(&t_id)
                .bind(&ctx.project_meta.id)
                .bind(format!("⚠️ Generation stopped during planning: {}\n\nYou can click Retry Generation on the canvas tab or send a new prompt.", err_msg))
                .bind(run_id)
                .bind(&ctx.model_settings.target_tab_key)
                .bind(&ctx.model_settings.target_tab_label)
                .bind(&now2)
                .execute(pool)
                .await;
            }
            record_and_broadcast_event(
                pool,
                event_tx,
                run_id,
                Some(job_id),
                None,
                "planning_failed",
                "failed",
                &format!("Planning failed: {}", err_msg),
            )
            .await;
            return Ok(());
        }
    };

    // Save plan.json to disk and runs table
    let plan_json_str = serde_json::to_string_pretty(&plan).unwrap_or_default();
    let plan_rel = format!("projects/{}/runs/{}/plan.json", ctx.project_meta.id, run_id);
    if let Ok(abs_plan) = config.resolve_rel_path(&plan_rel) {
        let _ = atomic_write_file(&abs_plan, plan_json_str.as_bytes());
    }

    let now2 = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE runs SET plan_json = ?, actual_cost_micros = COALESCE(actual_cost_micros, 0) + COALESCE(?, 0), updated_at = ? WHERE id = ?"
    )
    .bind(&plan_json_str)
    .bind(planner_cost_micros)
    .bind(&now2)
    .bind(run_id)
    .execute(pool)
    .await?;

    // If full_page mode and review_plan_first is enabled, pause at `awaiting_plan_review`!
    if ctx.mode == "full_page" && ctx.model_settings.review_plan_first {
        sqlx::query("UPDATE jobs SET status = 'awaiting_plan_review', updated_at = ? WHERE id = ?")
            .bind(&now2)
            .bind(job_id)
            .execute(pool)
            .await?;
        sqlx::query("UPDATE runs SET status = 'awaiting_plan_review', updated_at = ? WHERE id = ?")
            .bind(&now2)
            .bind(run_id)
            .execute(pool)
            .await?;
        record_and_broadcast_event(
            pool,
            event_tx,
            run_id,
            Some(job_id),
            None,
            "awaiting_plan_review",
            "awaiting_plan_review",
            &format!(
                "Plan ready for review ({} planned outputs). Review or edit items and click Generate Set.",
                plan.items.len()
            ),
        )
        .await;
        return Ok(());
    }

    sqlx::query("UPDATE jobs SET status = 'completed', updated_at = ? WHERE id = ?")
        .bind(&now2)
        .bind(job_id)
        .execute(pool)
        .await?;

    materialize_render_jobs_from_plan(pool, config, event_tx, run_id, &ctx, &plan).await?;
    Ok(())
}

pub async fn materialize_render_jobs_from_plan(
    pool: &SqlitePool,
    config: &AppConfig,
    event_tx: &broadcast::Sender<RunEventMessage>,
    run_id: &str,
    ctx: &BuiltGenerationContext,
    plan: &PlannerPlan,
) -> AppResult<()> {
    let settings_row = sqlx::query("SELECT capability_cache_json FROM settings WHERE id = 1")
        .fetch_one(pool)
        .await?;
    let caps: ModelCapabilitiesReport = settings_row
        .get::<Option<String>, _>("capability_cache_json")
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();

    let mut valid_asset_ids = HashSet::new();
    if let Some(ref cs) = ctx.current_screenshot {
        valid_asset_ids.insert(cs.asset_id.clone());
    }
    if let Some(ref prev) = ctx.selected_previous_output {
        valid_asset_ids.insert(prev.asset_id.clone());
    }
    for att in &ctx.explicit_attachments {
        valid_asset_ids.insert(att.asset_id.clone());
    }

    let mut sanitized_plan = plan.clone();
    if (ctx.model_settings.viewport_intent.as_deref() == Some("mobile")
        || ctx.model_settings.viewport_intent.as_deref() == Some("tablet"))
        && ctx.model_settings.target_existing_output_id.is_some()
        && sanitized_plan
            .items
            .iter()
            .any(|it| it.kind != "asset" && !it.transparent_background)
    {
        sanitized_plan
            .items
            .retain(|it| it.kind != "asset" && !it.transparent_background);
        let kept_ids: HashSet<String> = sanitized_plan
            .items
            .iter()
            .map(|it| it.id.clone())
            .collect();
        for it in &mut sanitized_plan.items {
            it.depends_on.retain(|d| kept_ids.contains(d));
        }
    }
    crate::models::sanitize_planner_plan(
        &mut sanitized_plan,
        &ctx.mode,
        &valid_asset_ids,
        &caps.supported_aspect_ratios,
    );
    let plan = &sanitized_plan;

    let topo_order = validate_planner_plan(
        plan,
        &ctx.mode,
        &valid_asset_ids,
        &caps.supported_aspect_ratios,
    )?;

    let item_by_id: HashMap<String, &PlanItem> =
        plan.items.iter().map(|i| (i.id.clone(), i)).collect();

    let now = chrono::Utc::now().to_rfc3339();
    let mut item_id_to_job_id: HashMap<String, String> = HashMap::new();
    let mut item_id_to_output_id: HashMap<String, String> = HashMap::new();

    for (idx, item_id) in topo_order.iter().enumerate() {
        let item = item_by_id
            .get(item_id)
            .ok_or_else(|| AppError::Internal("Missing topo item".into()))?;
        let output_id = Uuid::new_v4().to_string();
        let render_job_id = Uuid::new_v4().to_string();
        let compiled_prompt = compile_image_prompt(ctx, plan, item);

        let dep_job_ids: Vec<String> = item
            .depends_on
            .iter()
            .filter_map(|dep_item_id| item_id_to_job_id.get(dep_item_id).cloned())
            .collect();

        let initial_status = if dep_job_ids.is_empty() {
            "ready"
        } else {
            "queued"
        };

        let journal_rel = format!(
            "projects/{}/runs/{}/outputs/{}/save_journal.json",
            ctx.project_meta.id, run_id, item.id
        );

        let tab_key = if let Some(tk) = ctx
            .model_settings
            .target_tab_key
            .as_ref()
            .filter(|s| !s.trim().is_empty())
        {
            normalize_tab_key(tk)
        } else if ctx.mode == "full_page" {
            "full-website".to_string()
        } else if !item.tab_key.trim().is_empty() {
            normalize_tab_key(&item.tab_key)
        } else {
            normalize_tab_key(&item.name)
        };

        let tab_label = if let Some(tl) = ctx
            .model_settings
            .target_tab_label
            .as_ref()
            .filter(|s| !s.trim().is_empty())
        {
            tl.trim().to_string()
        } else if ctx.mode == "full_page" {
            "Full Website".to_string()
        } else if !item.tab_label.trim().is_empty() {
            item.tab_label.trim().to_string()
        } else {
            item.name.clone()
        };

        let extracted_from_output_id: Option<String> = item
            .extracted_from_item_id
            .as_ref()
            .and_then(|src_id| item_id_to_output_id.get(src_id).cloned())
            .or_else(|| {
                if item.kind == "asset" && !item.depends_on.is_empty() {
                    item.depends_on
                        .first()
                        .and_then(|dep_id| item_id_to_output_id.get(dep_id).cloned())
                } else {
                    None
                }
            });

        let parent_output_id: Option<String> = if extracted_from_output_id.is_some() {
            extracted_from_output_id.clone()
        } else {
            ctx.model_settings
                .target_existing_output_id
                .as_ref()
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .or_else(|| {
                    item.target_existing_output_id
                        .as_ref()
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                })
        };

        let transparent_bg_flag = if should_use_transparent_background(ctx, item) {
            1_i64
        } else {
            0_i64
        };

        sqlx::query(
            "INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, save_journal_path, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&output_id)
        .bind(run_id)
        .bind(&ctx.project_meta.id)
        .bind(&item.id)
        .bind((idx + 1) as i64)
        .bind(&item.kind)
        .bind(&item.name)
        .bind(&item.direction)
        .bind(if item.is_visual_anchor { "visual_anchor" } else { &item.kind })
        .bind(&item.page_route)
        .bind(&item.aspect_ratio)
        .bind(initial_status)
        .bind(&compiled_prompt)
        .bind(serde_json::to_string(&item.reference_asset_ids).unwrap_or_else(|_| "[]".into()))
        .bind(serde_json::to_string(&item.depends_on).unwrap_or_else(|_| "[]".into()))
        .bind(serde_json::to_string(item).unwrap_or_else(|_| "{}".into()))
        .bind(&journal_rel)
        .bind(&tab_key)
        .bind(&tab_label)
        .bind(&parent_output_id)
        .bind(transparent_bg_flag)
        .bind(&extracted_from_output_id)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query(
            "INSERT INTO jobs (id, run_id, output_id, job_type, status, attempts, max_attempts, depends_on_job_ids_json, created_at, updated_at) VALUES (?, ?, ?, 'render_item', ?, 0, 2, ?, ?, ?)"
        )
        .bind(&render_job_id)
        .bind(run_id)
        .bind(&output_id)
        .bind(initial_status)
        .bind(serde_json::to_string(&dep_job_ids).unwrap_or_else(|_| "[]".into()))
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        item_id_to_job_id.insert(item.id.clone(), render_job_id);
        item_id_to_output_id.insert(item.id.clone(), output_id);
    }

    let _ = config;
    sqlx::query("UPDATE runs SET status = 'rendering', updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(run_id)
        .execute(pool)
        .await?;

    record_and_broadcast_event(
        pool,
        event_tx,
        run_id,
        None,
        None,
        "rendering_queued",
        "rendering",
        &format!(
            "Scheduled {} independent image generation jobs.",
            topo_order.len()
        ),
    )
    .await;

    Ok(())
}

async fn execute_render_item_job(
    pool: &SqlitePool,
    config: &AppConfig,
    provider: &Arc<OpenRouterClient>,
    event_tx: &broadcast::Sender<RunEventMessage>,
    job_id: &str,
    run_id: &str,
    output_id: &str,
) -> AppResult<()> {
    // Check if run was cancelled
    let run_status_row = sqlx::query("SELECT status, is_test_adapter FROM runs WHERE id = ?")
        .bind(run_id)
        .fetch_one(pool)
        .await?;
    let run_status: String = run_status_row.get("status");
    let is_test_adapter: bool = run_status_row.get::<i64, _>("is_test_adapter") != 0;

    if run_status == "cancelled" {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query("UPDATE jobs SET status = 'cancelled', updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(job_id)
            .execute(pool)
            .await?;
        sqlx::query("UPDATE outputs SET status = 'cancelled', updated_at = ? WHERE id = ?")
            .bind(&now)
            .bind(output_id)
            .execute(pool)
            .await?;
        return Ok(());
    }

    let out_row = sqlx::query(
        "SELECT project_id, plan_item_id, variant_index, kind, title, aspect_ratio, compiled_prompt, reference_asset_ids_json, depends_on_json, save_journal_path, parent_output_id, transparent_background, extracted_from_output_id, item_spec_json, tab_key, tab_label FROM outputs WHERE id = ?"
    )
    .bind(output_id)
    .fetch_one(pool)
    .await?;

    let project_id: String = out_row.get("project_id");
    let plan_item_id: String = out_row.get("plan_item_id");
    let variant_index: u32 = out_row.get::<i64, _>("variant_index") as u32;
    let out_kind: String = out_row
        .try_get("kind")
        .unwrap_or_else(|_| "component".to_string());
    let out_tab_key: String = out_row.try_get("tab_key").unwrap_or_default();
    let out_tab_label: String = out_row.try_get("tab_label").unwrap_or_default();
    let title: String = out_row.get("title");
    let aspect_ratio: String = out_row.get("aspect_ratio");
    let compiled_prompt: String = out_row.get("compiled_prompt");
    let ref_ids: Vec<String> =
        serde_json::from_str(out_row.get::<&str, _>("reference_asset_ids_json"))
            .unwrap_or_default();
    let dep_item_ids: Vec<String> =
        serde_json::from_str(out_row.get::<&str, _>("depends_on_json")).unwrap_or_default();
    let parent_output_id: Option<String> = out_row.get("parent_output_id");
    let transparent_bg_col: i64 = out_row
        .try_get::<i64, _>("transparent_background")
        .unwrap_or(0);
    let extracted_from_output_id: Option<String> = out_row.get("extracted_from_output_id");
    let item_spec_str: String = out_row
        .try_get::<String, _>("item_spec_json")
        .unwrap_or_default();
    let parsed_item: Option<PlanItem> = serde_json::from_str(&item_spec_str).ok();
    let save_journal_rel: String = out_row
        .get::<Option<String>, _>("save_journal_path")
        .unwrap_or_else(|| {
            format!(
                "projects/{}/runs/{}/outputs/{}/save_journal.json",
                project_id, run_id, plan_item_id
            )
        });

    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query("UPDATE outputs SET status = 'rendering', updated_at = ? WHERE id = ?")
        .bind(&now)
        .bind(output_id)
        .execute(pool)
        .await?;

    record_and_broadcast_event(
        pool,
        event_tx,
        run_id,
        Some(job_id),
        Some(output_id),
        "item_rendering",
        "rendering",
        &format!("Rendering '{}'...", title),
    )
    .await;

    let (ctx, _, _) = build_run_context_from_db(pool, config, run_id).await?;

    // Gather input_references data URLs:
    // 1. Existing tab output or parent composite output (`extracted_from_output_id` / `parent_output_id`)
    // 2. Prerequisite completed outputs (Visual Anchor Page `item_1` for multi-page Image-to-Image consistency, or Hero composite for extracted transparent assets)
    // 3. Explicitly referenced project assets & current screenshot
    let mut input_data_urls: Vec<String> = Vec::new();

    for ref_out_id in [extracted_from_output_id.as_ref(), parent_output_id.as_ref()]
        .into_iter()
        .flatten()
    {
        if let Some(p_row) = sqlx::query(
            "SELECT original_rel_path FROM outputs WHERE id = ? AND status = 'completed'",
        )
        .bind(ref_out_id)
        .fetch_optional(pool)
        .await?
        {
            if let Some(rel) = p_row.get::<Option<String>, _>("original_rel_path") {
                if let Ok(abs) = config.resolve_rel_path(&rel) {
                    if let Ok(bytes) = std::fs::read(&abs) {
                        let durl = format!("data:image/png;base64,{}", BASE64.encode(&bytes));
                        if !input_data_urls.contains(&durl) {
                            input_data_urls.push(durl);
                        }
                    }
                }
            }
        }
    }

    // Attach prerequisite completed outputs FIRST (e.g. Visual Anchor Page `item_1` for Image-to-Image cross-page navbar/style consistency, or Hero composite for transparent asset extraction)
    for dep_item_id in &dep_item_ids {
        if let Some(dep_out) = sqlx::query(
            "SELECT original_rel_path FROM outputs WHERE run_id = ? AND plan_item_id = ? AND status = 'completed'"
        )
        .bind(run_id)
        .bind(dep_item_id)
        .fetch_optional(pool)
        .await?
        {
            if let Some(rel) = dep_out.get::<Option<String>, _>("original_rel_path") {
                if let Ok(abs) = config.resolve_rel_path(&rel) {
                    if let Ok(bytes) = std::fs::read(&abs) {
                        let durl = format!("data:image/png;base64,{}", BASE64.encode(&bytes));
                        if !input_data_urls.contains(&durl) {
                            input_data_urls.push(durl);
                        }
                    }
                }
            }
        }
    }

    if let Some(ref cs) = ctx.current_screenshot {
        if (ref_ids.contains(&cs.asset_id) || extracted_from_output_id.is_none())
            && !cs.data_url.is_empty()
            && !input_data_urls.contains(&cs.data_url)
        {
            input_data_urls.push(cs.data_url.clone());
        }
    }
    if let Some(ref prev) = ctx.selected_previous_output {
        if (ref_ids.contains(&prev.asset_id) || extracted_from_output_id.is_none())
            && !prev.data_url.is_empty()
            && !input_data_urls.contains(&prev.data_url)
        {
            input_data_urls.push(prev.data_url.clone());
        }
    }
    for att in &ctx.explicit_attachments {
        if ref_ids.contains(&att.asset_id)
            && !att.data_url.is_empty()
            && !input_data_urls.contains(&att.data_url)
        {
            input_data_urls.push(att.data_url.clone());
        }
    }

    let request_provider_alpha = transparent_bg_col != 0
        || ctx
            .model_settings
            .asset_options
            .as_ref()
            .map(|o| o.transparent_background)
            .unwrap_or(false);
    let seamless_tiling = ctx
        .model_settings
        .asset_options
        .as_ref()
        .map(|o| o.seamless_tiling)
        .unwrap_or(false);

    let effective_quality = if !ctx.model_settings.highlight_annotations.is_empty() {
        "medium".to_string()
    } else if let Some(ref q_over) = parsed_item
        .as_ref()
        .and_then(|it| it.quality_override.clone())
    {
        q_over.clone()
    } else {
        ctx.model_settings.quality.clone()
    };

    let rendered = if is_test_adapter {
        TestFixtureAdapter::render_fixture_item(
            &ctx.brand_snapshot,
            &aspect_ratio,
            variant_index,
            request_provider_alpha,
            seamless_tiling,
        )
        .map_err(|e| crate::provider::RenderJobError {
            kind: RenderFailureKind::DefiniteUpstreamFailure,
            message: e.to_string(),
            upstream_request_id: None,
        })
    } else {
        let (api_key, _) = config.secrets.get_key().ok_or_else(|| {
            AppError::Unauthorized("OpenRouter API key missing during render job.".into())
        })?;

        let hb_pool = pool.clone();
        let hb_tx = event_tx.clone();
        let hb_run_id = run_id.to_string();
        let hb_job_id = job_id.to_string();
        let hb_output_id = output_id.to_string();
        let hb_title = title.clone();
        let hb_model = ctx.model_settings.image_model.clone();
        let hb_handle = tokio::spawn(async move {
            let mut elapsed = 0u64;
            loop {
                tokio::time::sleep(Duration::from_secs(4)).await;
                elapsed += 4;
                record_and_broadcast_event(
                    &hb_pool,
                    &hb_tx,
                    &hb_run_id,
                    Some(&hb_job_id),
                    Some(&hb_output_id),
                    "item_progress",
                    "rendering",
                    &format!(
                        "Rendering '{}' with {} ({}s elapsed)...",
                        hb_title, hb_model, elapsed
                    ),
                )
                .await;
            }
        });

        // Support up to 1 bounded retry ONLY on 429 RateLimited
        let first_attempt = provider
            .render_single_image(
                &api_key,
                &ctx.model_settings.image_model,
                &compiled_prompt,
                &effective_quality,
                &aspect_ratio,
                &input_data_urls,
                request_provider_alpha,
            )
            .await;

        let res = match first_attempt {
            Err(err) => {
                if let RenderFailureKind::RateLimited { retry_after_secs } = err.kind {
                    tokio::time::sleep(Duration::from_secs(retry_after_secs)).await;
                    provider
                        .render_single_image(
                            &api_key,
                            &ctx.model_settings.image_model,
                            &compiled_prompt,
                            &effective_quality,
                            &aspect_ratio,
                            &input_data_urls,
                            request_provider_alpha,
                        )
                        .await
                } else {
                    Err(err)
                }
            }
            Ok(out) => Ok(out),
        };
        hb_handle.abort();
        res
    };

    let rendered = match rendered {
        Ok(r) => r,
        Err(err) => {
            let status_str = match err.kind {
                RenderFailureKind::OutcomeUnknown => "outcome_unknown",
                _ => "failed",
            };
            let now2 = chrono::Utc::now().to_rfc3339();
            sqlx::query(
                "UPDATE jobs SET status = ?, upstream_request_id = ?, error_message = ?, updated_at = ? WHERE id = ?"
            )
            .bind(status_str)
            .bind(&err.upstream_request_id)
            .bind(&err.message)
            .bind(&now2)
            .bind(job_id)
            .execute(pool)
            .await?;

            sqlx::query(
                "UPDATE outputs SET status = ?, provider_request_id = ?, error_message = ?, updated_at = ? WHERE id = ?"
            )
            .bind(status_str)
            .bind(&err.upstream_request_id)
            .bind(&err.message)
            .bind(&now2)
            .bind(output_id)
            .execute(pool)
            .await?;

            record_and_broadcast_event(
                pool,
                event_tx,
                run_id,
                Some(job_id),
                Some(output_id),
                "item_failed",
                status_str,
                &format!("'{}' {}: {}", title, status_str, err.message),
            )
            .await;
            recompute_run_status(pool, event_tx, run_id).await?;
            return Ok(());
        }
    };

    // Transition to `saving` before disk writes
    let now_saving = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE jobs SET status = 'saving', upstream_request_id = ?, updated_at = ? WHERE id = ?",
    )
    .bind(&rendered.provider_request_id)
    .bind(&now_saving)
    .bind(job_id)
    .execute(pool)
    .await?;
    sqlx::query("UPDATE outputs SET status = 'saving', provider_request_id = ?, updated_at = ? WHERE id = ?")
        .bind(&rendered.provider_request_id)
        .bind(&now_saving)
        .bind(output_id)
        .execute(pool)
        .await?;

    record_and_broadcast_event(
        pool,
        event_tx,
        run_id,
        Some(job_id),
        Some(output_id),
        "item_saving",
        "saving",
        &format!("Validating and saving '{}' original & thumbnail...", title),
    )
    .await;

    // Decode, verify signature/dimensions/alpha, and generate thumbnail outside DB transaction
    let processed = match validate_and_thumbnail_output_bytes(rendered.bytes.clone()).await {
        Ok(p) => p,
        Err(e) => {
            let msg = e.to_string();
            let now_err = chrono::Utc::now().to_rfc3339();
            sqlx::query(
                "UPDATE jobs SET status = 'failed', error_message = ?, updated_at = ? WHERE id = ?",
            )
            .bind(&msg)
            .bind(&now_err)
            .bind(job_id)
            .execute(pool)
            .await?;
            sqlx::query("UPDATE outputs SET status = 'failed', error_message = ?, updated_at = ? WHERE id = ?")
                .bind(&msg)
                .bind(&now_err)
                .bind(output_id)
                .execute(pool)
                .await?;
            recompute_run_status(pool, event_tx, run_id).await?;
            return Ok(());
        }
    };

    // If transparent_background was requested, verify actual alpha channel pixels
    let mut usage_meta = rendered.usage_json.clone();
    if let Some(obj) = usage_meta.as_object_mut() {
        obj.insert(
            "duration_ms".to_string(),
            serde_json::json!(rendered.duration_ms),
        );
        if let Some(ref prov) = rendered.provider_name {
            obj.insert("provider".to_string(), serde_json::json!(prov));
        }
        if request_provider_alpha {
            obj.insert(
                "transparency_verified".to_string(),
                serde_json::json!(processed.has_alpha),
            );
            if !processed.has_alpha {
                obj.insert(
                    "transparency_warning".to_string(),
                    serde_json::json!("Transparent background was requested, but the returned image is opaque (no alpha < 250 pixels detected)."),
                );
            }
        }
    }

    let orig_rel = format!(
        "projects/{}/runs/{}/outputs/{}/original.{}",
        project_id, run_id, plan_item_id, processed.extension
    );
    let thumb_rel = format!(
        "projects/{}/runs/{}/outputs/{}/thumbnail.webp",
        project_id, run_id, plan_item_id
    );

    let abs_orig = config.resolve_rel_path(&orig_rel)?;
    let abs_thumb = config.resolve_rel_path(&thumb_rel)?;
    let abs_journal = config.resolve_rel_path(&save_journal_rel)?;

    // Save untouched original bytes, thumbnail, and save_journal atomically BEFORE DB commit
    atomic_write_file(&abs_orig, &rendered.bytes)?;
    atomic_write_file(&abs_thumb, &processed.thumbnail_webp)?;

    let journal = SaveJournalRecord {
        output_id: output_id.to_string(),
        original_rel_path: orig_rel.clone(),
        thumbnail_rel_path: thumb_rel.clone(),
        mime_type: processed.mime_type.clone(),
        width: processed.width,
        height: processed.height,
        byte_size: processed.byte_size,
        has_alpha: processed.has_alpha,
        provider_request_id: rendered.provider_request_id.clone(),
        usage_json: usage_meta.clone(),
        reported_cost_micros: rendered.reported_cost_micros,
    };
    atomic_write_file(
        &abs_journal,
        serde_json::to_string_pretty(&journal)
            .unwrap_or_default()
            .as_bytes(),
    )?;

    let now_done = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "UPDATE outputs SET status = 'completed', original_rel_path = ?, thumbnail_rel_path = ?, mime_type = ?, width = ?, height = ?, byte_size = ?, has_alpha = ?, usage_json = ?, provider_request_id = ?, actual_cost_micros = ?, error_message = NULL, updated_at = ? WHERE id = ?"
    )
    .bind(&orig_rel)
    .bind(&thumb_rel)
    .bind(&processed.mime_type)
    .bind(processed.width as i64)
    .bind(processed.height as i64)
    .bind(processed.byte_size as i64)
    .bind(if processed.has_alpha { 1 } else { 0 })
    .bind(usage_meta.to_string())
    .bind(&rendered.provider_request_id)
    .bind(rendered.reported_cost_micros)
    .bind(&now_done)
    .bind(output_id)
    .execute(pool)
    .await?;

    sqlx::query(
        "UPDATE jobs SET status = 'completed', error_message = NULL, updated_at = ? WHERE id = ?",
    )
    .bind(&now_done)
    .bind(job_id)
    .execute(pool)
    .await?;

    if let Some(cost) = rendered.reported_cost_micros {
        sqlx::query(
            "UPDATE runs SET actual_cost_micros = COALESCE(actual_cost_micros, 0) + ?, updated_at = ? WHERE id = ?"
        )
        .bind(cost)
        .bind(&now_done)
        .bind(run_id)
        .execute(pool)
        .await?;
    } else if !is_test_adapter {
        sqlx::query("UPDATE runs SET cost_is_unknown = 1, updated_at = ? WHERE id = ?")
            .bind(&now_done)
            .bind(run_id)
            .execute(pool)
            .await?;
    }

    record_and_broadcast_event(
        pool,
        event_tx,
        run_id,
        Some(job_id),
        Some(output_id),
        "item_completed",
        "completed",
        &format!(
            "Completed '{}' ({}x{}px).",
            title, processed.width, processed.height
        ),
    )
    .await;

    // Post-Render Vision Asset Audit & Auto-Extraction:
    // Inspect the newly rendered UI image (component, hero, or primary visual anchor page) so ANY visual asset,
    // 3D prop (e.g. potted plant, 3D tile), brand logo / spark mark, or custom illustration
    // visible in the rendered UI that wasn't already extracted gets automatically queued as a
    // separate transparent-background PNG in the exact same tab!
    let is_primary_or_anchor_ui = ctx.mode != "full_page"
        || plan_item_id == "item_1"
        || parsed_item
            .as_ref()
            .map(|it| it.is_visual_anchor)
            .unwrap_or(false)
        || dep_item_ids.is_empty();
    if !is_test_adapter
        && transparent_bg_col == 0
        && extracted_from_output_id.is_none()
        && out_kind != "asset"
        && is_primary_or_anchor_ui
    {
        if let Err(e) = discover_and_queue_rendered_visual_assets(
            pool,
            config,
            provider,
            event_tx,
            run_id,
            output_id,
            &project_id,
            &plan_item_id,
            &title,
            &out_tab_key,
            &out_tab_label,
            &ctx,
            &rendered.bytes,
        )
        .await
        {
            tracing::warn!(
                run_id = %run_id,
                output_id = %output_id,
                "Post-render visual asset discovery warning (non-fatal): {}",
                e
            );
        }
    }

    let _ = crate::handoff::sync_project_organized_folders(pool, config, &project_id).await;
    recompute_run_status(pool, event_tx, run_id).await?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub async fn discover_and_queue_rendered_visual_assets(
    pool: &SqlitePool,
    config: &AppConfig,
    provider: &Arc<OpenRouterClient>,
    event_tx: &broadcast::Sender<RunEventMessage>,
    run_id: &str,
    parent_output_id: &str,
    project_id: &str,
    parent_plan_item_id: &str,
    parent_title: &str,
    tab_key: &str,
    tab_label: &str,
    ctx: &BuiltGenerationContext,
    rendered_png_bytes: &[u8],
) -> AppResult<()> {
    let Some((api_key, _)) = config.secrets.get_key() else {
        return Ok(());
    };

    record_and_broadcast_event(
        pool,
        event_tx,
        run_id,
        None,
        Some(parent_output_id),
        "item_progress",
        "rendering",
        &format!(
            "Auditing rendered '{}' to extract all visual assets, 3D props & logos onto transparent backgrounds...",
            parent_title
        ),
    )
    .await;

    let existing_rows = sqlx::query(
        "SELECT title, kind, transparent_background FROM outputs WHERE project_id = ? AND (tab_key = ? OR run_id = ?) AND id != ? AND status = 'completed'"
    )
    .bind(project_id)
    .bind(tab_key)
    .bind(run_id)
    .bind(parent_output_id)
    .fetch_all(pool)
    .await
    .unwrap_or_default();

    let tab_already_has_extracted_assets = existing_rows.iter().any(|r| {
        let k: String = r.try_get("kind").unwrap_or_default();
        let tb: i64 = r.try_get("transparent_background").unwrap_or(0);
        k == "asset" || tb != 0
    });
    let is_in_tab_viewport_adaptation = (ctx.model_settings.viewport_intent.as_deref()
        == Some("mobile")
        || ctx.model_settings.viewport_intent.as_deref() == Some("tablet"))
        && ctx.model_settings.target_existing_output_id.is_some();

    let existing_titles: Vec<String> = existing_rows
        .iter()
        .map(|r| r.get::<String, _>("title"))
        .collect();

    let rendered_data_url = format!(
        "data:image/png;base64,{}",
        BASE64.encode(rendered_png_bytes)
    );
    let audit_res = provider
        .discover_missing_visual_assets(
            &api_key,
            &ctx.model_settings.planner_model,
            parent_title,
            &ctx.project_meta.name,
            &rendered_data_url,
            &existing_titles,
        )
        .await?;

    let discovered = if is_in_tab_viewport_adaptation && tab_already_has_extracted_assets {
        Vec::new()
    } else {
        audit_res.missing_assets
    };
    let coded_separators_and_backgrounds = audit_res.coded_separators_and_backgrounds;

    if discovered.is_empty() && coded_separators_and_backgrounds.is_empty() {
        return Ok(());
    }

    let current_plan_str: Option<String> =
        sqlx::query_scalar("SELECT plan_json FROM runs WHERE id = ?")
            .bind(run_id)
            .fetch_optional(pool)
            .await?
            .flatten();

    let mut plan: PlannerPlan = current_plan_str
        .as_deref()
        .and_then(|s| crate::models::parse_planner_plan_tolerant(s).ok())
        .or_else(|| ctx.approved_plan.clone())
        .unwrap_or_else(|| PlannerPlan {
            schema_version: 1,
            mode: ctx.mode.clone(),
            request_summary: ctx.user_request.clone(),
            requirements: vec![],
            assumptions: vec![],
            shared_design_spec: Default::default(),
            items: vec![],
            shared_components: vec![],
            page_composition: vec![],
        });

    let now = chrono::Utc::now().to_rfc3339();

    // Persist the image-specific separator, divider, background & 3D rotation rules into the parent item & plan
    if !coded_separators_and_backgrounds.is_empty() {
        for note in &coded_separators_and_backgrounds {
            let formatted = format!(
                "CODED SEPARATOR / BACKGROUND / 3D SPEC FROM IMAGE ({}): {}",
                parent_title, note
            );
            if !plan.shared_design_spec.asset_rules.contains(&formatted) {
                plan.shared_design_spec.asset_rules.push(formatted);
            }
        }
        if let Some(parent_item) = plan
            .items
            .iter_mut()
            .find(|it| it.id == parent_plan_item_id)
        {
            for note in &coded_separators_and_backgrounds {
                let formatted = format!("CODED SEPARATOR / BACKGROUND SPEC: {}", note);
                if !parent_item.implementation_notes.contains(&formatted) {
                    parent_item.implementation_notes.push(formatted.clone());
                }
                if !parent_item.asset_instructions.contains(&formatted) {
                    parent_item.asset_instructions.push(formatted);
                }
            }
            if let Ok(updated_item_json) = serde_json::to_string(parent_item) {
                let _ = sqlx::query(
                    "UPDATE outputs SET item_spec_json = ?, updated_at = ? WHERE id = ?",
                )
                .bind(&updated_item_json)
                .bind(&now)
                .bind(parent_output_id)
                .execute(pool)
                .await;
            }
        }
    }

    let base_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM outputs WHERE run_id = ?")
        .bind(run_id)
        .fetch_one(pool)
        .await
        .unwrap_or(1);

    let mut queued_any_new_assets = false;

    let mut existing_slugs: HashSet<String> = existing_titles
        .iter()
        .map(|t| normalize_tab_key(t))
        .collect();
    let mut has_existing_logo = existing_titles.iter().any(|t| {
        let tl = t.to_ascii_lowercase();
        tl.contains("logo") || tl.contains("wordmark")
    });
    let mut has_existing_icon_sheet = existing_titles.iter().any(|t| {
        let tl = t.to_ascii_lowercase();
        tl.contains("icon sheet") || tl.contains("icon pack") || tl.contains("icon set")
    });

    for (idx, disc) in discovered.into_iter().enumerate() {
        let slug = normalize_tab_key(&disc.name);
        let name_lower = disc.name.to_ascii_lowercase();
        let is_logo = name_lower.contains("logo") || name_lower.contains("wordmark");
        let is_icon_sheet = name_lower.contains("icon sheet")
            || name_lower.contains("icon pack")
            || name_lower.contains("icon set");
        if existing_slugs.contains(&slug)
            || (is_logo && has_existing_logo)
            || (is_icon_sheet && has_existing_icon_sheet)
        {
            continue;
        }
        existing_slugs.insert(slug);
        if is_logo {
            has_existing_logo = true;
        }
        if is_icon_sheet {
            has_existing_icon_sheet = true;
        }
        queued_any_new_assets = true;

        let new_item_id = format!("{}_auto_asset_{}", parent_plan_item_id, idx + 1);
        let variant_idx = (base_count as u32) + (idx as u32) + 1;
        let new_item = PlanItem {
            id: new_item_id.clone(),
            kind: "asset".to_string(),
            name: disc.name.clone(),
            tab_key: tab_key.to_string(),
            tab_label: tab_label.to_string(),
            page_route: None,
            variant_index: variant_idx,
            viewport: crate::models::ViewportSpec {
                intent: "isolated".to_string(),
                css_width: 1024,
            },
            aspect_ratio: disc.aspect_ratio.clone(),
            transparent_background: true,
            extracted_from_item_id: Some(parent_plan_item_id.to_string()),
            target_existing_output_id: None,
            quality_override: None,
            requirements_covered: vec!["req_1".to_string()],
            direction: disc.direction.clone(),
            exact_copy: vec![],
            reference_asset_ids: vec![],
            depends_on: vec![parent_plan_item_id.to_string()],
            image_prompt: disc.image_prompt.clone(),
            implementation_notes: vec![
                "Render on a 100% transparent PNG background and layer in the component/page layout"
                    .to_string(),
            ],
            behaviors: vec![],
            responsive_rules: vec!["Scale proportionally across breakpoints".to_string()],
            asset_instructions: vec![disc.asset_instructions.clone()],
            is_visual_anchor: false,
        };

        let compiled_prompt = compile_image_prompt(ctx, &plan, &new_item);
        let new_output_id = Uuid::new_v4().to_string();
        let new_job_id = Uuid::new_v4().to_string();
        let journal_rel = format!(
            "projects/{}/runs/{}/outputs/{}/save_journal.json",
            project_id, run_id, new_item_id
        );

        sqlx::query(
            "INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, direction_summary, role, page_route, aspect_ratio, status, compiled_prompt, reference_asset_ids_json, depends_on_json, item_spec_json, save_journal_path, tab_key, tab_label, parent_output_id, transparent_background, extracted_from_output_id, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 'asset', ?, ?, 'asset', NULL, ?, 'ready', ?, '[]', ?, ?, ?, ?, ?, ?, 1, ?, ?, ?)"
        )
        .bind(&new_output_id)
        .bind(run_id)
        .bind(project_id)
        .bind(&new_item_id)
        .bind(variant_idx as i64)
        .bind(&new_item.name)
        .bind(&new_item.direction)
        .bind(&new_item.aspect_ratio)
        .bind(&compiled_prompt)
        .bind(serde_json::to_string(&new_item.depends_on).unwrap_or_else(|_| "[]".into()))
        .bind(serde_json::to_string(&new_item).unwrap_or_else(|_| "{}".into()))
        .bind(&journal_rel)
        .bind(tab_key)
        .bind(tab_label)
        .bind(parent_output_id)
        .bind(parent_output_id)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        sqlx::query(
            "INSERT INTO jobs (id, run_id, output_id, job_type, status, attempts, max_attempts, depends_on_job_ids_json, created_at, updated_at) VALUES (?, ?, ?, 'render_item', 'ready', 0, 2, '[]', ?, ?)"
        )
        .bind(&new_job_id)
        .bind(run_id)
        .bind(&new_output_id)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;

        record_and_broadcast_event(
            pool,
            event_tx,
            run_id,
            Some(&new_job_id),
            Some(&new_output_id),
            "item_progress",
            "rendering",
            &format!(
                "Queued transparent PNG extraction for '{}' in '{}'...",
                new_item.name, tab_label
            ),
        )
        .await;

        plan.items.push(new_item);
    }

    let updated_plan_str = serde_json::to_string_pretty(&plan).unwrap_or_default();
    let plan_rel = format!("projects/{}/runs/{}/plan.json", project_id, run_id);
    if let Ok(abs_plan) = config.resolve_rel_path(&plan_rel) {
        let _ = atomic_write_file(&abs_plan, updated_plan_str.as_bytes());
    }
    if queued_any_new_assets {
        sqlx::query("UPDATE runs SET status = 'rendering', completed_at = NULL, plan_json = ?, updated_at = ? WHERE id = ?")
            .bind(&updated_plan_str)
            .bind(&now)
            .bind(run_id)
            .execute(pool)
            .await?;
    } else {
        sqlx::query("UPDATE runs SET plan_json = ?, updated_at = ? WHERE id = ?")
            .bind(&updated_plan_str)
            .bind(&now)
            .bind(run_id)
            .execute(pool)
            .await?;
    }

    Ok(())
}

pub async fn recompute_run_status(
    pool: &SqlitePool,
    event_tx: &broadcast::Sender<RunEventMessage>,
    run_id: &str,
) -> AppResult<()> {
    let out_rows = sqlx::query("SELECT status, title, error_message FROM outputs WHERE run_id = ?")
        .bind(run_id)
        .fetch_all(pool)
        .await?;

    if out_rows.is_empty() {
        return Ok(());
    }

    let mut completed = 0;
    let mut active = 0;
    let mut failed = 0;
    let mut cancelled = 0;
    let mut outcome_unknown = 0;
    let mut interrupted = 0;
    let mut failed_reasons: Vec<String> = Vec::new();

    for r in &out_rows {
        let s: String = r.get("status");
        let title: String = r.try_get("title").unwrap_or_default();
        let err_msg: Option<String> = r.try_get("error_message").ok().flatten();
        match s.as_str() {
            "completed" => completed += 1,
            "queued" | "ready" | "rendering" | "saving" => active += 1,
            "failed" => {
                failed += 1;
                if let Some(em) = err_msg {
                    failed_reasons.push(format!("{}: {}", title, em));
                }
            }
            "cancelled" => cancelled += 1,
            "outcome_unknown" => {
                outcome_unknown += 1;
                if let Some(em) = err_msg {
                    failed_reasons.push(format!("{} (outcome unknown): {}", title, em));
                }
            }
            "interrupted" => interrupted += 1,
            _ => {}
        }
    }

    if active > 0 {
        return Ok(());
    }

    let total = out_rows.len();
    let final_status = if completed == total {
        "completed"
    } else if completed > 0 {
        "partial"
    } else if outcome_unknown > 0 {
        "outcome_unknown"
    } else if interrupted > 0 {
        "interrupted"
    } else if cancelled == total {
        "cancelled"
    } else {
        "failed"
    };

    let now = chrono::Utc::now().to_rfc3339();
    let combined_error = if failed_reasons.is_empty() {
        None
    } else {
        Some(failed_reasons.join(" | "))
    };
    sqlx::query("UPDATE runs SET status = ?, error_message = COALESCE(?, error_message), completed_at = ?, updated_at = ? WHERE id = ?")
        .bind(final_status)
        .bind(&combined_error)
        .bind(&now)
        .bind(&now)
        .bind(run_id)
        .execute(pool)
        .await?;

    if let Some(ref err_summary) = combined_error {
        if let Ok(Some(run_row)) =
            sqlx::query("SELECT project_id, thread_id FROM runs WHERE id = ?")
                .bind(run_id)
                .fetch_optional(pool)
                .await
        {
            let project_id: String = run_row.get("project_id");
            let thread_id: String = run_row.get("thread_id");
            let _ = sqlx::query(
                "INSERT INTO thread_messages (id, project_id, thread_id, role, content, attachments_json, run_id, created_at)
                 VALUES (?, ?, ?, 'assistant', ?, '[]', ?, ?)"
            )
            .bind(Uuid::new_v4().to_string())
            .bind(&project_id)
            .bind(&thread_id)
            .bind(format!(
                "⚠️ **Image generation encountered an issue ({}/{} completed)**:\n`{}`\n\nClick **Retry Failed** on the tab or in the banner below to resume.",
                completed, total, err_summary
            ))
            .bind(run_id)
            .bind(&now)
            .execute(pool)
            .await;
        }
    }

    let msg = format!(
        "Run finished with status '{}' ({}/{} outputs completed, {} failed).",
        final_status, completed, total, failed
    );
    record_and_broadcast_event(
        pool,
        event_tx,
        run_id,
        None,
        None,
        "run_finished",
        final_status,
        &msg,
    )
    .await;

    Ok(())
}

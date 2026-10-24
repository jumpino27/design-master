use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use design_gen_server::{
    config::AppConfig,
    db::init_db_pool,
    handoff::{build_set_handoff, build_single_output_handoff, format_host_path},
    image_util::{generate_deterministic_fixture_png, process_upload_bytes, sanitize_svg},
    models::BrandSpec,
    provider::OpenRouterClient,
    routes::{build_router, AppState},
    worker::{recover_interrupted_jobs, spawn_job_worker},
};
use http_body_util::BodyExt;
use std::{io::Read, path::Path, sync::Arc, time::Duration};
use tempfile::TempDir;
use tokio::sync::broadcast;
use tower::ServiceExt;

async fn setup_test_state() -> (TempDir, AppState, broadcast::Sender<()>) {
    let temp_dir = TempDir::new().expect("create temp dir");
    let data_dir = temp_dir.path().join("data");
    let static_dir = temp_dir.path().join("static");
    std::fs::create_dir_all(&data_dir).unwrap();
    std::fs::create_dir_all(&static_dir).unwrap();
    std::fs::write(
        static_dir.join("index.html"),
        b"<!doctype html><html><body>Index</body></html>",
    )
    .unwrap();
    std::fs::write(
        static_dir.join("workspace.html"),
        b"<!doctype html><html><body>Workspace</body></html>",
    )
    .unwrap();

    let mut config = AppConfig::from_env().unwrap();
    config.data_dir = data_dir.clone();
    config.static_dir = static_dir;
    config.csrf_token = "test-csrf-token".to_string();
    config.env_test_adapter = true;
    config.secrets = design_gen_server::config::SecretManager::new(true);

    let pool = init_db_pool(&data_dir).await.unwrap();
    // Enable test adapter in DB
    sqlx::query("UPDATE settings SET test_adapter_enabled = 1 WHERE id = 1")
        .execute(&pool)
        .await
        .unwrap();

    let provider = Arc::new(OpenRouterClient::new());
    let (event_tx, _) = broadcast::channel(256);
    let (shutdown_tx, shutdown_rx) = broadcast::channel(1);

    spawn_job_worker(
        pool.clone(),
        config.clone(),
        provider.clone(),
        event_tx.clone(),
        shutdown_rx,
    );

    let state = AppState {
        pool,
        config,
        provider,
        event_tx,
    };

    (temp_dir, state, shutdown_tx)
}

async fn json_request(
    app: &axum::Router,
    method: &str,
    uri: &str,
    body: Option<serde_json::Value>,
) -> (StatusCode, serde_json::Value) {
    let req_builder = Request::builder()
        .method(method)
        .uri(uri)
        .header("Host", "127.0.0.1:4318")
        .header("Origin", "http://127.0.0.1:4318")
        .header("X-CSRF-Token", "test-csrf-token")
        .header("Content-Type", "application/json");

    let body_bytes = body
        .map(|b| serde_json::to_vec(&b).unwrap())
        .unwrap_or_default();

    let resp = app
        .clone()
        .oneshot(req_builder.body(Body::from(body_bytes)).unwrap())
        .await
        .unwrap();

    let status = resp.status();
    let collected = resp.into_body().collect().await.unwrap().to_bytes();
    let val = serde_json::from_slice(&collected).unwrap_or_else(
        |_| serde_json::json!({ "raw": String::from_utf8_lossy(&collected).to_string() }),
    );
    (status, val)
}

#[tokio::test]
async fn test_bundled_showcases_are_complete_and_preserve_existing_edits() {
    let (_tmp, state, _shutdown) = setup_test_state().await;
    design_gen_server::db::ensure_seed_example_projects(&state.pool, &state.config)
        .await
        .unwrap();
    let projects: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM projects WHERE is_example = 1")
        .fetch_one(&state.pool)
        .await
        .unwrap();
    assert_eq!(projects, 4);
    let outputs: Vec<(String, String)> =
        sqlx::query_as("SELECT original_rel_path, thumbnail_rel_path FROM outputs")
            .fetch_all(&state.pool)
            .await
            .unwrap();
    assert_eq!(outputs.len(), 23);
    for (original, thumbnail) in outputs {
        assert!(state.config.resolve_rel_path(&original).unwrap().is_file());
        assert!(state.config.resolve_rel_path(&thumbnail).unwrap().is_file());
    }
    let cost: i64 = sqlx::query_scalar("SELECT SUM(actual_cost_micros) FROM runs")
        .fetch_one(&state.pool)
        .await
        .unwrap();
    assert_eq!(cost, 0);
    sqlx::query("UPDATE projects SET name = 'My edited Nexora' WHERE id = 'example-nexora'")
        .execute(&state.pool)
        .await
        .unwrap();
    design_gen_server::db::ensure_seed_example_projects(&state.pool, &state.config)
        .await
        .unwrap();
    let name: String = sqlx::query_scalar("SELECT name FROM projects WHERE id = 'example-nexora'")
        .fetch_one(&state.pool)
        .await
        .unwrap();
    assert_eq!(name, "My edited Nexora");
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM outputs")
        .fetch_one(&state.pool)
        .await
        .unwrap();
    assert_eq!(count, 23);
}

#[tokio::test]
async fn test_clarification_answers_advance_one_question_and_preserve_the_brief() {
    let (_tmp, state, _shutdown) = setup_test_state().await;
    let app = build_router(state.clone());
    let (_, project) = json_request(
        &app,
        "POST",
        "/api/projects",
        Some(serde_json::json!({
            "name": "Interview test", "description": "Team planning", "industry": "SaaS"
        })),
    )
    .await;
    let thread = project["default_thread_id"].as_str().unwrap();
    let endpoint = format!("/api/threads/{thread}/chat");
    let original = "Create a complete landing page for our planning product";
    let (status, mut turn) = json_request(
        &app,
        "POST",
        &endpoint,
        Some(serde_json::json!({"message": original})),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let answers = [
        "Q1: Complete landing page",
        "Inside the hero, use a calm editorial layout without 3D",
        "Desktop",
        "No separate assets",
    ];
    for (index, answer) in answers.iter().enumerate() {
        assert_eq!(turn["action"], "ask_clarification");
        assert!(turn["run"].is_null());
        let question = &turn["assistant_message"];
        assert_eq!(question["quick_replies"].as_array().unwrap().len(), 3);
        assert_eq!(
            question["content"].as_str().unwrap().matches('?').count(),
            1
        );
        let previous_question = question["content"].clone();
        let (status, next) = json_request(
            &app,
            "POST",
            &endpoint,
            Some(serde_json::json!({
                "message": answer, "answer_to_message_id": question["id"]
            })),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        if index < answers.len() - 1 {
            assert_ne!(next["assistant_message"]["content"], previous_question);
        }
        turn = next;
    }
    assert_eq!(turn["action"], "trigger_run");
    assert!(turn["assistant_message"]["quick_replies"]
        .as_array()
        .unwrap()
        .is_empty());
    let prompt: String = sqlx::query_scalar("SELECT user_prompt FROM runs WHERE id = ?")
        .bind(turn["run"]["id"].as_str().unwrap())
        .fetch_one(&state.pool)
        .await
        .unwrap();
    assert!(prompt.contains(original));
    assert!(prompt.contains("Inside the hero, use a calm editorial layout without 3D"));
    assert!(prompt.contains("No separate assets"));
}

#[tokio::test]
async fn test_a_question_accepts_only_one_answer_even_with_concurrent_requests() {
    let (_tmp, state, _shutdown) = setup_test_state().await;
    let app = build_router(state.clone());
    let (_, project) = json_request(
        &app,
        "POST",
        "/api/projects",
        Some(serde_json::json!({"name": "Click test"})),
    )
    .await;
    let thread = project["default_thread_id"].as_str().unwrap();
    let endpoint = format!("/api/threads/{thread}/chat");
    let (_, turn) = json_request(
        &app,
        "POST",
        &endpoint,
        Some(serde_json::json!({"message": "Design my landing page"})),
    )
    .await;
    let body = serde_json::json!({"message": "Complete landing page", "answer_to_message_id": turn["assistant_message"]["id"]});
    let (a, b) = tokio::join!(
        json_request(&app, "POST", &endpoint, Some(body.clone())),
        json_request(&app, "POST", &endpoint, Some(body.clone())),
    );
    let statuses = [a.0, b.0];
    assert!(statuses.contains(&StatusCode::OK));
    assert!(statuses.contains(&StatusCode::CONFLICT));
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM thread_messages WHERE thread_id = ? AND role = 'user'",
    )
    .bind(thread)
    .fetch_one(&state.pool)
    .await
    .unwrap();
    assert_eq!(count, 2);
}

#[tokio::test]
async fn test_setup_key_masking_and_static_vs_api_404() {
    let (_tmp, state, _shutdown) = setup_test_state().await;
    let app = build_router(state);

    // Save a session API key and verify the raw key is NEVER returned in status
    let secret_key = "sk-or-v1-supersecretkey9876543210abcd";
    let (status, res) = json_request(
        &app,
        "POST",
        "/api/setup/key",
        Some(serde_json::json!({
            "api_key": secret_key,
            "prefer_session_only": true,
            "validate_online": false
        })),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let dump = res.to_string();
    assert!(
        !dump.contains(secret_key),
        "Raw key must never leak in API response"
    );
    assert_eq!(res["key_status"]["configured"], true);
    assert_eq!(res["key_status"]["masked_hint"], "sk-or-••••abcd");

    // Verify GET /api/setup/status also masks the key
    let (status2, res2) = json_request(&app, "GET", "/api/setup/status", None).await;
    assert_eq!(status2, StatusCode::OK);
    assert!(!res2.to_string().contains(secret_key));

    // Verify deep link static export route returns HTML
    let resp_html = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/workspace?project=123")
                .header("Host", "127.0.0.1:4318")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resp_html.status(), StatusCode::OK);
    assert_eq!(
        resp_html.headers().get("content-type").unwrap(),
        "text/html; charset=utf-8"
    );

    // Verify missing /api and /media routes NEVER return application HTML
    let (api_404_status, api_404_body) =
        json_request(&app, "GET", "/api/nonexistent-route", None).await;
    assert_eq!(api_404_status, StatusCode::NOT_FOUND);
    assert_eq!(api_404_body["error"]["code"], "NOT_FOUND");

    let (media_404_status, media_404_body) =
        json_request(&app, "GET", "/media/output/missing-id/original", None).await;
    assert_eq!(media_404_status, StatusCode::NOT_FOUND);
    assert_eq!(media_404_body["error"]["code"], "NOT_FOUND");
}

#[tokio::test]
async fn test_svg_sanitizer_and_upload_processing() {
    // Reject malicious SVG with <script> or onload
    let bad_svg =
        b"<svg xmlns='http://www.w3.org/2000/svg' onload='alert(1)'><script>evil()</script></svg>";
    assert!(sanitize_svg(bad_svg).is_err());

    // Accept and rasterize clean SVG logo
    let good_svg =
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="120" height="60" viewBox="0 0 120 60">
        <rect x="4" y="4" width="52" height="52" rx="10" fill="#D56A3A"/>
        <circle cx="30" cy="30" r="12" fill="#FAF9F6"/>
    </svg>"##;
    let processed = process_upload_bytes(
        good_svg.to_vec(),
        Some("forma-logo.svg".into()),
        20 * 1024 * 1024,
        60_000_000,
    )
    .await
    .expect("valid SVG should sanitize and rasterize");
    assert_eq!(processed.mime_type, "image/svg+xml");
    assert_eq!(processed.extension, "svg");
    assert!(processed.model_input_png.is_some());
    assert!(!processed.thumbnail_webp.is_empty());
}

#[tokio::test]
async fn test_component_run_brand_versioning_retry_and_handoff() {
    let (_tmp, state, _shutdown) = setup_test_state().await;
    let app = build_router(state.clone());

    // 1. Create Project "Forma"
    let (st, created) = json_request(
        &app,
        "POST",
        "/api/projects",
        Some(serde_json::json!({
            "name": "Forma",
            "description": "Design collaboration platform",
            "purpose": "Convert visitors to Pro subscribers",
            "industry": "Creative SaaS",
            "target_audience": "Creative teams",
            "desired_user_action": "Start creating"
        })),
    )
    .await;
    assert_eq!(st, StatusCode::OK);
    let project_id = created["project_id"].as_str().unwrap().to_string();
    let thread_id = created["default_thread_id"].as_str().unwrap().to_string();

    // 2. Submit Component Mode run (4 directions)
    let (st_run, run_res) = json_request(
        &app,
        "POST",
        &format!("/api/threads/{}/runs", thread_id),
        Some(serde_json::json!({
            "mode": "component",
            "user_prompt": "Polish this pricing card. Add an annual billing toggle, emphasize the Pro plan, and create a stronger CTA.",
            "scope_preset": "isolated_component",
            "aspect_ratio": "4:3",
            "quality": "medium",
            "idempotency_key": "idem-comp-1"
        })),
    )
    .await;
    assert_eq!(st_run, StatusCode::OK);
    let run_id = run_res["run"]["id"].as_str().unwrap().to_string();

    // Verify duplicate browser submit with same idempotency_key is deduplicated
    let (_, dup_res) = json_request(
        &app,
        "POST",
        &format!("/api/threads/{}/runs", thread_id),
        Some(serde_json::json!({
            "mode": "component",
            "user_prompt": "Polish this pricing card.",
            "idempotency_key": "idem-comp-1"
        })),
    )
    .await;
    assert_eq!(dup_res["deduplicated"], true);
    assert_eq!(dup_res["run"]["id"], run_id);

    // Wait for worker to finish all 4 component jobs
    let mut final_run = serde_json::Value::Null;
    for _ in 0..40 {
        tokio::time::sleep(Duration::from_millis(150)).await;
        let (_, r) = json_request(&app, "GET", &format!("/api/runs/{}", run_id), None).await;
        if r["run"]["status"] == "completed" {
            final_run = r["run"].clone();
            break;
        }
    }
    assert_eq!(final_run["status"], "completed");
    let outputs = final_run["outputs"].as_array().unwrap();
    assert_eq!(outputs.len(), 4, "Component mode must produce 4 outputs");

    let first_output_id = outputs[0]["id"].as_str().unwrap().to_string();

    // 3. Edit the project's brand kit (change primary color to #123456) -> creates version 2
    let mut updated_brand = BrandSpec::default();
    updated_brand.colors.light.primary = "#123456".to_string();
    let (st_b, b_res) = json_request(
        &app,
        "PUT",
        &format!("/api/projects/{}/brand", project_id),
        Some(serde_json::to_value(&updated_brand).unwrap()),
    )
    .await;
    assert_eq!(st_b, StatusCode::OK);
    assert_eq!(b_res["brand_version"], 2);

    // 4. Generate handoff for the old run's output -> must still use brand version 1 (#D56A3A), NOT version 2 (#123456)!
    let handoff =
        build_single_output_handoff(&state.pool, &state.config, &first_output_id, Some(true))
            .await
            .expect("build single handoff");
    assert!(handoff.clipboard_text.contains("#D56A3A"));
    assert!(
        !handoff.clipboard_text.contains("#123456"),
        "Editing brand kit later must not rewrite an old run's handoff snapshot"
    );
    assert!(handoff
        .clipboard_text
        .contains("Implement the design described below"));
    assert!(handoff
        .clipboard_text
        .contains("extract or crop it into its own asset"));

    // 5. Simulate partial failure & retry of a single item:
    // Mark output[3] as failed and verify retrying it only re-runs output[3] without erasing outputs[0..3]
    let fourth_output_id = outputs[3]["id"].as_str().unwrap().to_string();
    sqlx::query(
        "UPDATE outputs SET status = 'failed', error_message = 'Simulated failure' WHERE id = ?",
    )
    .bind(&fourth_output_id)
    .execute(&state.pool)
    .await
    .unwrap();
    sqlx::query("UPDATE jobs SET status = 'failed' WHERE output_id = ?")
        .bind(&fourth_output_id)
        .execute(&state.pool)
        .await
        .unwrap();

    let (st_retry, _) = json_request(
        &app,
        "POST",
        &format!("/api/outputs/{}/retry", fourth_output_id),
        Some(serde_json::json!({})),
    )
    .await;
    assert_eq!(st_retry, StatusCode::OK);

    // Wait for retry to complete
    for _ in 0..30 {
        tokio::time::sleep(Duration::from_millis(150)).await;
        let (_, r) = json_request(&app, "GET", &format!("/api/runs/{}", run_id), None).await;
        if r["run"]["status"] == "completed" {
            break;
        }
    }
    let (_, after_retry) = json_request(&app, "GET", &format!("/api/runs/{}", run_id), None).await;
    assert_eq!(after_retry["run"]["status"], "completed");
}

#[tokio::test]
async fn test_full_page_plan_review_dag_and_portable_bundle() {
    let (_tmp, state, _shutdown) = setup_test_state().await;
    let app = build_router(state.clone());

    let (_, created) = json_request(
        &app,
        "POST",
        "/api/projects",
        Some(serde_json::json!({
            "name": "Forma Full Site",
            "purpose": "Marketing & Pricing flow",
            "target_audience": "Product designers"
        })),
    )
    .await;
    let thread_id = created["default_thread_id"].as_str().unwrap().to_string();

    // Start Full-page run with review_plan_first = true
    let (_, run_res) = json_request(
        &app,
        "POST",
        &format!("/api/threads/{}/runs", thread_id),
        Some(serde_json::json!({
            "mode": "full_page",
            "user_prompt": "A homepage and pricing page for Forma with shared navbar, hero background asset, and signup modal.",
            "review_plan_first": true
        })),
    )
    .await;
    let run_id = run_res["run"]["id"].as_str().unwrap().to_string();

    // Wait for run to reach `awaiting_plan_review`
    let mut plan_ready = false;
    for _ in 0..30 {
        tokio::time::sleep(Duration::from_millis(100)).await;
        let (_, r) = json_request(&app, "GET", &format!("/api/runs/{}", run_id), None).await;
        if r["run"]["status"] == "awaiting_plan_review" {
            plan_ready = true;
            break;
        }
    }
    assert!(
        plan_ready,
        "Full-page mode should pause at awaiting_plan_review"
    );

    // Approve the plan (`POST /api/runs/:id/approve-plan`)
    let (st_app, _) = json_request(
        &app,
        "POST",
        &format!("/api/runs/{}/approve-plan", run_id),
        Some(serde_json::json!({})),
    )
    .await;
    assert_eq!(st_app, StatusCode::OK);

    // Wait for all 5 dependency-ordered outputs in the set to finish
    let mut completed = false;
    for _ in 0..50 {
        tokio::time::sleep(Duration::from_millis(150)).await;
        let (_, r) = json_request(&app, "GET", &format!("/api/runs/{}", run_id), None).await;
        if r["run"]["status"] == "completed" {
            completed = true;
            break;
        }
    }
    assert!(
        completed,
        "Coordinated full-page set should complete all DAG items"
    );

    // Build portable ZIP export bundle and verify its contents
    let bundle_handoff =
        build_set_handoff(&state.pool, &state.config, &run_id, Some(true), true, None)
            .await
            .expect("build portable set bundle");
    assert!(bundle_handoff.is_complete_set);
    assert!(bundle_handoff.missing_items.is_empty());
    let zip_path = bundle_handoff.bundle_path.expect("bundle zip path");
    let zip_file = std::fs::File::open(&zip_path).expect("open zip file");
    let mut archive = zip::ZipArchive::new(zip_file).expect("read zip archive");

    let mut has_impl_md = false;
    let mut has_manifest = false;
    let mut has_brand = false;
    let mut output_count = 0;

    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).unwrap();
        let name = entry.name().to_string();
        if name == "implementation.md" {
            has_impl_md = true;
            let mut text = String::new();
            entry.read_to_string(&mut text).unwrap();
            assert!(
                text.contains("outputs/"),
                "Portable implementation.md must use bundle-relative paths"
            );
            assert!(
                !text.contains("sk-or-"),
                "Bundle must never contain API keys"
            );
        } else if name == "manifest.json" {
            has_manifest = true;
        } else if name == "brand.json" {
            has_brand = true;
        } else if name.starts_with("outputs/") {
            output_count += 1;
        }
    }

    assert!(has_impl_md);
    assert!(has_manifest);
    assert!(has_brand);
    assert_eq!(output_count, 5);
}

#[tokio::test]
async fn test_restart_recovery_does_not_duplicate_in_flight_paid_job() {
    let (_tmp, state, _shutdown) = setup_test_state().await;

    // Create a dummy project, thread, run, output, and a job stuck in `rendering` with an upstream_request_id
    let now = chrono::Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO projects (id, name, created_at, updated_at) VALUES ('p1', 'Test', ?, ?)",
    )
    .bind(&now)
    .bind(&now)
    .execute(&state.pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO threads (id, project_id, title, created_at, updated_at) VALUES ('t1', 'p1', 'T', ?, ?)")
        .bind(&now)
        .bind(&now)
        .execute(&state.pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO runs (id, project_id, thread_id, mode, user_prompt, brand_version, brand_snapshot_json, status, created_at, updated_at) VALUES ('r1', 'p1', 't1', 'component', 'Prompt', 1, '{}', 'rendering', ?, ?)")
        .bind(&now)
        .bind(&now)
        .execute(&state.pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO outputs (id, run_id, project_id, plan_item_id, variant_index, kind, title, role, status, created_at, updated_at) VALUES ('o1', 'r1', 'p1', 'd1', 1, 'component', 'Dir 1', 'component', 'rendering', ?, ?)")
        .bind(&now)
        .bind(&now)
        .execute(&state.pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO jobs (id, run_id, output_id, job_type, status, attempts, max_attempts, upstream_request_id, created_at, updated_at) VALUES ('j1', 'r1', 'o1', 'render_item', 'rendering', 1, 2, 'or-req-123', ?, ?)")
        .bind(&now)
        .bind(&now)
        .execute(&state.pool)
        .await
        .unwrap();

    // Run startup recovery
    recover_interrupted_jobs(&state.pool, &state.config, &state.event_tx)
        .await
        .unwrap();

    let job_status: String = sqlx::query_scalar("SELECT status FROM jobs WHERE id = 'j1'")
        .fetch_one(&state.pool)
        .await
        .unwrap();
    let out_status: String = sqlx::query_scalar("SELECT status FROM outputs WHERE id = 'o1'")
        .fetch_one(&state.pool)
        .await
        .unwrap();
    assert_eq!(job_status, "outcome_unknown");
    assert_eq!(out_status, "outcome_unknown");
}

#[test]
fn test_windows_and_unicode_path_formatting() {
    let win_space =
        Path::new(r"C:\Users\René Designer\AppData\Local\design-gen\projects\p1\original.png");
    let formatted = format_host_path(win_space);
    assert_eq!(
        formatted,
        r#""C:\Users\René Designer\AppData\Local\design-gen\projects\p1\original.png""#
    );

    let transparent_png =
        generate_deterministic_fixture_png("1:1", 1, "#D56A3A", "#F4F1EA", "#FAF9F6", true, false)
            .unwrap();
    let decoded = image::load_from_memory(&transparent_png).unwrap();
    assert!(design_gen_server::image_util::verify_actual_alpha(&decoded));
}

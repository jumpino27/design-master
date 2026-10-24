-- Design Gen initial schema (SQLite WAL with foreign keys enforced)

CREATE TABLE IF NOT EXISTS settings (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    planner_model TEXT NOT NULL DEFAULT 'z-ai/glm-5.3-flash',
    image_model TEXT NOT NULL DEFAULT 'openai/gpt-image-2.5-sunburst',
    default_quality TEXT NOT NULL DEFAULT 'medium',
    output_size TEXT NOT NULL DEFAULT 'provider_default',
    include_extraction_instructions INTEGER NOT NULL DEFAULT 1,
    max_concurrent_jobs INTEGER NOT NULL DEFAULT 2,
    spend_limit_micros INTEGER,
    test_adapter_enabled INTEGER NOT NULL DEFAULT 0,
    capability_cache_json TEXT,
    capability_cached_at TEXT,
    setup_completed_at TEXT,
    updated_at TEXT NOT NULL
);

INSERT OR IGNORE INTO settings (
    id,
    planner_model,
    image_model,
    default_quality,
    output_size,
    include_extraction_instructions,
    max_concurrent_jobs,
    test_adapter_enabled,
    updated_at
) VALUES (
    1,
    'z-ai/glm-5.3-flash',
    'openai/gpt-image-2.5-sunburst',
    'medium',
    'provider_default',
    1,
    2,
    0,
    strftime('%Y-%m-%dT%H:%M:%SZ', 'now')
);

CREATE TABLE IF NOT EXISTS projects (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    purpose TEXT NOT NULL DEFAULT '',
    industry TEXT NOT NULL DEFAULT '',
    target_audience TEXT NOT NULL DEFAULT '',
    audience_needs TEXT NOT NULL DEFAULT '',
    desired_user_action TEXT NOT NULL DEFAULT '',
    archive_status TEXT NOT NULL DEFAULT 'active' CHECK (archive_status IN ('active', 'archived')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_projects_archive_updated ON projects(archive_status, updated_at DESC);

CREATE TABLE IF NOT EXISTS brand_versions (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    version INTEGER NOT NULL,
    brand_json TEXT NOT NULL,
    created_at TEXT NOT NULL,
    UNIQUE(project_id, version)
);

CREATE INDEX IF NOT EXISTS idx_brand_versions_project ON brand_versions(project_id, version DESC);

CREATE TABLE IF NOT EXISTS threads (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    title TEXT NOT NULL,
    archive_status TEXT NOT NULL DEFAULT 'active' CHECK (archive_status IN ('active', 'archived')),
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_threads_project_updated ON threads(project_id, archive_status, updated_at DESC);

CREATE TABLE IF NOT EXISTS assets (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
    label TEXT NOT NULL,
    original_rel_path TEXT NOT NULL,
    model_input_rel_path TEXT,
    thumbnail_rel_path TEXT NOT NULL,
    mime_type TEXT NOT NULL,
    width INTEGER NOT NULL DEFAULT 0,
    height INTEGER NOT NULL DEFAULT 0,
    byte_size INTEGER NOT NULL DEFAULT 0,
    checksum_sha256 TEXT NOT NULL,
    has_alpha INTEGER NOT NULL DEFAULT 0,
    placement_notes TEXT NOT NULL DEFAULT '',
    preserve_as_is INTEGER NOT NULL DEFAULT 0,
    metadata_json TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_assets_project_created ON assets(project_id, created_at DESC);

CREATE TABLE IF NOT EXISTS runs (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    thread_id TEXT NOT NULL REFERENCES threads(id) ON DELETE CASCADE,
    idempotency_key TEXT,
    mode TEXT NOT NULL CHECK (mode IN ('component', 'full_page', 'asset')),
    scope_preset TEXT NOT NULL DEFAULT 'isolated_component',
    user_prompt TEXT NOT NULL,
    brand_version INTEGER NOT NULL,
    brand_snapshot_json TEXT NOT NULL,
    selected_references_json TEXT NOT NULL DEFAULT '[]',
    model_settings_json TEXT NOT NULL DEFAULT '{}',
    plan_json TEXT,
    consistency_report_json TEXT,
    status TEXT NOT NULL CHECK (status IN (
        'queued',
        'planning',
        'awaiting_plan_review',
        'ready',
        'rendering',
        'saving',
        'completed',
        'partial',
        'failed',
        'cancelled',
        'interrupted',
        'outcome_unknown'
    )),
    estimated_cost_micros INTEGER,
    actual_cost_micros INTEGER,
    cost_is_unknown INTEGER NOT NULL DEFAULT 0,
    is_test_adapter INTEGER NOT NULL DEFAULT 0,
    error_message TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    completed_at TEXT,
    UNIQUE(thread_id, idempotency_key)
);

CREATE INDEX IF NOT EXISTS idx_runs_thread_created ON runs(thread_id, created_at ASC);
CREATE INDEX IF NOT EXISTS idx_runs_project_created ON runs(project_id, created_at DESC);

CREATE TABLE IF NOT EXISTS outputs (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL REFERENCES runs(id) ON DELETE CASCADE,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    plan_item_id TEXT NOT NULL,
    variant_index INTEGER NOT NULL,
    kind TEXT NOT NULL,
    title TEXT NOT NULL,
    direction_summary TEXT NOT NULL DEFAULT '',
    role TEXT NOT NULL,
    page_route TEXT,
    aspect_ratio TEXT NOT NULL DEFAULT '4:3',
    status TEXT NOT NULL CHECK (status IN (
        'queued',
        'ready',
        'rendering',
        'saving',
        'completed',
        'failed',
        'cancelled',
        'interrupted',
        'outcome_unknown'
    )),
    original_rel_path TEXT,
    thumbnail_rel_path TEXT,
    mime_type TEXT,
    width INTEGER,
    height INTEGER,
    byte_size INTEGER,
    has_alpha INTEGER NOT NULL DEFAULT 0,
    compiled_prompt TEXT NOT NULL DEFAULT '',
    reference_asset_ids_json TEXT NOT NULL DEFAULT '[]',
    depends_on_json TEXT NOT NULL DEFAULT '[]',
    item_spec_json TEXT NOT NULL DEFAULT '{}',
    usage_json TEXT NOT NULL DEFAULT '{}',
    provider_request_id TEXT,
    estimated_cost_micros INTEGER,
    actual_cost_micros INTEGER,
    error_message TEXT,
    save_journal_path TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_outputs_run_variant ON outputs(run_id, variant_index ASC);

CREATE TABLE IF NOT EXISTS jobs (
    id TEXT PRIMARY KEY,
    run_id TEXT NOT NULL REFERENCES runs(id) ON DELETE CASCADE,
    output_id TEXT REFERENCES outputs(id) ON DELETE CASCADE,
    job_type TEXT NOT NULL CHECK (job_type IN ('plan', 'render_item')),
    status TEXT NOT NULL CHECK (status IN (
        'queued',
        'planning',
        'awaiting_plan_review',
        'ready',
        'rendering',
        'saving',
        'completed',
        'failed',
        'cancelled',
        'interrupted',
        'outcome_unknown'
    )),
    attempts INTEGER NOT NULL DEFAULT 0,
    max_attempts INTEGER NOT NULL DEFAULT 2,
    depends_on_job_ids_json TEXT NOT NULL DEFAULT '[]',
    upstream_request_id TEXT,
    lease_owner TEXT,
    leased_at TEXT,
    error_code TEXT,
    error_message TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_jobs_status_created ON jobs(status, created_at ASC);
CREATE INDEX IF NOT EXISTS idx_jobs_run ON jobs(run_id);

CREATE TABLE IF NOT EXISTS job_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    run_id TEXT NOT NULL REFERENCES runs(id) ON DELETE CASCADE,
    job_id TEXT REFERENCES jobs(id) ON DELETE CASCADE,
    output_id TEXT REFERENCES outputs(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    status TEXT NOT NULL,
    message TEXT NOT NULL,
    payload_json TEXT NOT NULL DEFAULT '{}',
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_job_events_run_id ON job_events(run_id, id ASC);

CREATE TABLE IF NOT EXISTS exports (
    id TEXT PRIMARY KEY,
    project_id TEXT NOT NULL REFERENCES projects(id) ON DELETE CASCADE,
    run_id TEXT NOT NULL REFERENCES runs(id) ON DELETE CASCADE,
    output_id TEXT REFERENCES outputs(id) ON DELETE CASCADE,
    export_kind TEXT NOT NULL CHECK (export_kind IN ('single_handoff', 'set_handoff', 'portable_bundle')),
    manifest_version INTEGER NOT NULL DEFAULT 1,
    manifest_rel_path TEXT NOT NULL,
    implementation_rel_path TEXT NOT NULL,
    bundle_rel_path TEXT,
    include_extraction INTEGER NOT NULL DEFAULT 1,
    handoff_text TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_exports_run_output ON exports(run_id, output_id);

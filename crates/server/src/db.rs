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

/// Adds the approved showcases without changing projects that are already present.
pub async fn ensure_seed_example_projects(
    pool: &SqlitePool,
    config: &crate::config::AppConfig,
) -> AppResult<()> {
    crate::showcases::seed(pool, config).await
}

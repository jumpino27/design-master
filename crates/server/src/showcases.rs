use crate::{
    config::AppConfig,
    error::AppResult,
    image_util::{atomic_write_file, validate_and_thumbnail_output_bytes},
};
use sqlx::{Row, SqlitePool};

struct Showcase {
    id: &'static str,
    sql: &'static str,
    images: &'static [(&'static str, &'static [u8])],
}

include!("showcase_catalog.rs");

pub async fn seed(pool: &SqlitePool, config: &AppConfig) -> AppResult<()> {
    for showcase in SHOWCASES {
        let mut transaction = pool.begin().await?;
        let exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM projects WHERE id = ?)")
            .bind(showcase.id)
            .fetch_one(&mut *transaction)
            .await?;
        if exists {
            continue;
        }

        sqlx::raw_sql(showcase.sql)
            .execute(&mut *transaction)
            .await?;
        for (output_id, bytes) in showcase.images {
            let output = sqlx::query(
                "SELECT original_rel_path, thumbnail_rel_path FROM outputs WHERE id = ?",
            )
            .bind(output_id)
            .fetch_one(&mut *transaction)
            .await?;
            let original = config.resolve_rel_path(output.get("original_rel_path"))?;
            let thumbnail = config.resolve_rel_path(output.get("thumbnail_rel_path"))?;
            let processed = validate_and_thumbnail_output_bytes(bytes.to_vec()).await?;
            atomic_write_file(&original, bytes)?;
            atomic_write_file(&thumbnail, &processed.thumbnail_webp)?;
        }
        transaction.commit().await?;
    }
    crate::handoff::sync_all_projects_organized_folders(pool, config).await?;
    Ok(())
}

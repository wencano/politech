use chrono::Utc;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::models::{IngestJobDto, SourceConfigDto};

pub async fn list_sources(db: &PgPool) -> Result<Vec<SourceConfigDto>, sqlx::Error> {
    let rows = sqlx::query(
        "select id, name, ingest_mode, base_url, poll_interval_seconds, status, created_at
         from source_config
         order by created_at desc
         limit 100",
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| SourceConfigDto {
            id: row.get("id"),
            name: row.get("name"),
            ingest_mode: row.get("ingest_mode"),
            base_url: row.get("base_url"),
            poll_interval_seconds: row.get("poll_interval_seconds"),
            status: row.get("status"),
            created_at: row.get("created_at"),
        })
        .collect())
}

pub async fn insert_source(
    db: &PgPool,
    id: Uuid,
    name: &str,
    ingest_mode: &str,
    base_url: &str,
    poll_interval_seconds: i32,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        "insert into source_config
         (id, name, ingest_mode, base_url, poll_interval_seconds, status, created_at, updated_at)
         values ($1, $2, $3, $4, $5, 'active', $6, $6)",
    )
    .bind(id)
    .bind(name)
    .bind(ingest_mode)
    .bind(base_url)
    .bind(poll_interval_seconds)
    .bind(now)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn list_ingest_jobs(db: &PgPool) -> Result<Vec<IngestJobDto>, sqlx::Error> {
    let rows = sqlx::query(
        "select id, source_id, mode, status, started_at, ended_at, error_message, created_at
         from ingest_job
         order by created_at desc
         limit 100",
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| IngestJobDto {
            id: row.get("id"),
            source_id: row.get("source_id"),
            mode: row.get("mode"),
            status: row.get("status"),
            started_at: row.get("started_at"),
            ended_at: row.get("ended_at"),
            error_message: row.get("error_message"),
            created_at: row.get("created_at"),
        })
        .collect())
}

/// Returns the ingest_mode of an active source, or None if not found/inactive.
pub async fn find_active_source_mode(
    db: &PgPool,
    source_id: Uuid,
) -> Result<Option<String>, sqlx::Error> {
    let row =
        sqlx::query("select ingest_mode from source_config where id = $1 and status = 'active'")
            .bind(source_id)
            .fetch_optional(db)
            .await?;
    Ok(row.map(|r| r.get("ingest_mode")))
}

pub async fn insert_ingest_job(
    db: &PgPool,
    job_id: Uuid,
    source_id: Uuid,
    mode: &str,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        "insert into ingest_job (id, source_id, mode, status, created_at)
         values ($1, $2, $3, 'pending', $4)",
    )
    .bind(job_id)
    .bind(source_id)
    .bind(mode)
    .bind(now)
    .execute(db)
    .await?;
    Ok(())
}

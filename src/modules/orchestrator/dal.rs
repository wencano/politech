use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::models::OrchestratorRunDto;

pub async fn list_runs(db: &PgPool) -> Result<Vec<OrchestratorRunDto>, sqlx::Error> {
    let rows = sqlx::query(
        "select id, request_type, provider, credential_id, status, attempt_count,
                started_at, ended_at, error_code, error_message, created_at
         from orchestration_run
         order by created_at desc
         limit 50",
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| OrchestratorRunDto {
            id: row.get("id"),
            request_type: row.get("request_type"),
            provider: row.get("provider"),
            credential_id: row.get("credential_id"),
            status: row.get("status"),
            attempt_count: row.get("attempt_count"),
            started_at: row.get("started_at"),
            ended_at: row.get("ended_at"),
            error_code: row.get("error_code"),
            error_message: row.get("error_message"),
            created_at: row.get("created_at"),
        })
        .collect())
}

/// Select the best available credential for a provider (health-aware weighted routing).
pub async fn select_best_credential(
    db: &PgPool,
    provider: &str,
) -> Result<Option<(Uuid, String)>, sqlx::Error> {
    let row = sqlx::query(
        "select c.id, c.encrypted_secret
         from credential c
         left join credential_health h on h.credential_id = c.id
         where c.provider = $1
           and c.status = 'active'
           and (h.cooldown_until is null or h.cooldown_until < now())
           and (h.retry_after   is null or h.retry_after   < now())
         order by c.priority_weight desc,
                  coalesce(h.recent_error_rate, 0) asc
         limit 1",
    )
    .bind(provider)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|r| (r.get("id"), r.get("encrypted_secret"))))
}

pub async fn update_health_success(
    db: &PgPool,
    credential_id: Uuid,
    latency_ms: Option<i32>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "update credential_health
         set last_success_at       = now(),
             failure_count_window  = 0,
             cooldown_until        = null,
             retry_after           = null,
             recent_latency_ms_p95 = coalesce($2, recent_latency_ms_p95)
         where credential_id = $1",
    )
    .bind(credential_id)
    .bind(latency_ms)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn update_health_failure(
    db: &PgPool,
    credential_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "update credential_health
         set last_failure_at      = now(),
             failure_count_window = failure_count_window + 1,
             cooldown_until       = case
                 when failure_count_window + 1 >= 5
                 then now() + interval '5 minutes'
                 else cooldown_until
             end
         where credential_id = $1",
    )
    .bind(credential_id)
    .execute(db)
    .await?;
    Ok(())
}

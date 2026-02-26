use chrono::Utc;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::models::{CredentialDto, CredentialHealthDto};

pub async fn list_credentials(db: &PgPool) -> Result<Vec<CredentialDto>, sqlx::Error> {
    let rows = sqlx::query(
        "select c.id, c.provider, c.label, c.status, c.capabilities, c.priority_weight,
                c.secret_version, c.created_at,
                h.last_success_at, h.last_failure_at,
                h.failure_count_window, h.cooldown_until
         from credential c
         left join credential_health h on h.credential_id = c.id
         order by c.created_at desc
         limit 100",
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| CredentialDto {
            id: row.get("id"),
            provider: row.get("provider"),
            label: row.get("label"),
            status: row.get("status"),
            capabilities: row.get::<serde_json::Value, _>("capabilities"),
            priority_weight: row.get("priority_weight"),
            secret_version: row.get("secret_version"),
            created_at: row.get("created_at"),
            health: CredentialHealthDto {
                last_success_at: row.get("last_success_at"),
                last_failure_at: row.get("last_failure_at"),
                failure_count_window: row
                    .get::<Option<i32>, _>("failure_count_window")
                    .unwrap_or(0),
                cooldown_until: row.get("cooldown_until"),
            },
        })
        .collect())
}

pub async fn insert_credential(
    db: &PgPool,
    id: Uuid,
    provider: &str,
    label: &str,
    encrypted_secret: &str,
    capabilities: &serde_json::Value,
    priority_weight: i32,
    created_by_user_id: Uuid,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        "insert into credential
         (id, provider, label, status, encrypted_secret, secret_version,
          capabilities, priority_weight, created_by_user_id, created_at, updated_at)
         values ($1, $2, $3, 'active', $4, 1, $5, $6, $7, $8, $8)",
    )
    .bind(id)
    .bind(provider)
    .bind(label)
    .bind(encrypted_secret)
    .bind(capabilities)
    .bind(priority_weight)
    .bind(created_by_user_id)
    .bind(now)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn insert_credential_health(db: &PgPool, credential_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query(
        "insert into credential_health (credential_id, failure_count_window) values ($1, 0)",
    )
    .bind(credential_id)
    .execute(db)
    .await?;
    Ok(())
}

/// Returns rows affected (0 = not found).
pub async fn update_credential_meta(
    db: &PgPool,
    id: Uuid,
    label: Option<String>,
    status: Option<String>,
    priority_weight: Option<i32>,
    capabilities: Option<serde_json::Value>,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "update credential
         set label           = coalesce($2, label),
             status          = coalesce($3, status),
             priority_weight = coalesce($4, priority_weight),
             capabilities    = coalesce($5, capabilities),
             updated_at      = now()
         where id = $1",
    )
    .bind(id)
    .bind(label)
    .bind(status)
    .bind(priority_weight)
    .bind(capabilities)
    .execute(db)
    .await?;
    Ok(result.rows_affected())
}

pub async fn set_credential_status(
    db: &PgPool,
    id: Uuid,
    status: &str,
) -> Result<u64, sqlx::Error> {
    let result =
        sqlx::query("update credential set status = $2, updated_at = now() where id = $1")
            .bind(id)
            .bind(status)
            .execute(db)
            .await?;
    Ok(result.rows_affected())
}

pub async fn rotate_credential_secret(
    db: &PgPool,
    id: Uuid,
    encrypted_secret: &str,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        "update credential
         set encrypted_secret = $2,
             secret_version   = secret_version + 1,
             updated_at       = now()
         where id = $1",
    )
    .bind(id)
    .bind(encrypted_secret)
    .execute(db)
    .await?;
    Ok(result.rows_affected())
}

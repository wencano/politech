use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::models::SessionDto;

pub async fn list_sessions_for_user(
    db: &PgPool,
    user_id: Uuid,
) -> Result<Vec<SessionDto>, sqlx::Error> {
    let rows = sqlx::query(
        "select id, user_id, status, created_at, expires_at
         from user_session
         where user_id = $1
         order by created_at desc
         limit 20",
    )
    .bind(user_id)
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| SessionDto {
            id: row.get("id"),
            user_id: row.get("user_id"),
            status: row.get("status"),
            created_at: row.get("created_at"),
            expires_at: row.get("expires_at"),
        })
        .collect())
}

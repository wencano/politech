use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct UserRow {
    pub id: Uuid,
    pub role: String,
}

pub struct ActiveSessionRow {
    pub user_id: Uuid,
    pub role: String,
}

pub async fn find_active_user_by_email(
    db: &PgPool,
    email: &str,
) -> Result<Option<UserRow>, sqlx::Error> {
    let row =
        sqlx::query("select id, role from app_user where email = $1 and status = 'active'")
            .bind(email)
            .fetch_optional(db)
            .await?;
    Ok(row.map(|r| UserRow { id: r.get("id"), role: r.get("role") }))
}

pub async fn insert_user(
    db: &PgPool,
    id: Uuid,
    email: &str,
    password_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "insert into app_user (id, email, password_hash, role, status)
         values ($1, $2, $3, 'user', 'active')
         on conflict (email) do nothing",
    )
    .bind(id)
    .bind(email)
    .bind(password_hash)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn insert_session(
    db: &PgPool,
    session_id: Uuid,
    user_id: Uuid,
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "insert into user_session (id, user_id, status, created_at, expires_at)
         values ($1, $2, 'active', $3, $4)",
    )
    .bind(session_id)
    .bind(user_id)
    .bind(created_at)
    .bind(expires_at)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn update_session_status(
    db: &PgPool,
    session_id: Uuid,
    status: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query("update user_session set status = $2 where id = $1")
        .bind(session_id)
        .bind(status)
        .execute(db)
        .await?;
    Ok(())
}

pub async fn find_active_session(
    db: &PgPool,
    session_id: Uuid,
) -> Result<Option<ActiveSessionRow>, sqlx::Error> {
    let row = sqlx::query(
        "select s.user_id, u.role
         from user_session s
         join app_user u on u.id = s.user_id
         where s.id = $1 and s.status = 'active'",
    )
    .bind(session_id)
    .fetch_optional(db)
    .await?;
    Ok(row.map(|r| ActiveSessionRow { user_id: r.get("user_id"), role: r.get("role") }))
}

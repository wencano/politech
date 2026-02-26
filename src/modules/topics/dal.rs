use chrono::Utc;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::models::TopicDto;

pub async fn list_topics(db: &PgPool) -> Result<Vec<TopicDto>, sqlx::Error> {
    let rows = sqlx::query(
        "select id, title, traits, created_at from topic order by created_at desc limit 100",
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| TopicDto {
            id: row.get("id"),
            title: row.get("title"),
            traits: row.get("traits"),
            created_at: row.get("created_at"),
        })
        .collect())
}

pub async fn get_topic_by_id(db: &PgPool, id: Uuid) -> Result<Option<TopicDto>, sqlx::Error> {
    let row = sqlx::query(
        "select id, title, traits, created_at from topic where id = $1",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| TopicDto {
        id: row.get("id"),
        title: row.get("title"),
        traits: row.get("traits"),
        created_at: row.get("created_at"),
    }))
}

pub async fn insert_topic(
    db: &PgPool,
    id: Uuid,
    title: &str,
    traits: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    sqlx::query(
        "insert into topic (id, title, traits, created_at) values ($1, $2, $3, $4)",
    )
    .bind(id)
    .bind(title)
    .bind(traits)
    .bind(now)
    .execute(db)
    .await?;
    Ok(())
}

pub async fn update_topic(
    db: &PgPool,
    id: Uuid,
    title: &str,
    traits: &serde_json::Value,
) -> Result<bool, sqlx::Error> {
    let rows = sqlx::query("update topic set title = $2, traits = $3 where id = $1")
        .bind(id)
        .bind(title)
        .bind(traits)
        .execute(db)
        .await?
        .rows_affected();
    Ok(rows > 0)
}

pub async fn delete_topic(db: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let rows = sqlx::query("delete from topic where id = $1")
        .bind(id)
        .execute(db)
        .await?
        .rows_affected();
    Ok(rows > 0)
}

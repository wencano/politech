use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::models::LocalityDto;

pub async fn list_localities(db: &PgPool) -> Result<Vec<LocalityDto>, sqlx::Error> {
    let rows = sqlx::query(
        "select id, code, name, locality_type, parent_id
         from locality
         order by locality_type, name
         limit 200",
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| LocalityDto {
            id: row.get::<Uuid, _>("id"),
            code: row.get("code"),
            name: row.get("name"),
            locality_type: row.get("locality_type"),
            parent_id: row.get("parent_id"),
        })
        .collect())
}

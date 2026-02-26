use sqlx::{PgPool, Row};
use uuid::Uuid;

use super::models::LocationDto;

pub async fn list_locations(db: &PgPool) -> Result<Vec<LocationDto>, sqlx::Error> {
    let rows = sqlx::query(
        "select id, code, name, level, geojson_key, traits from location order by code",
    )
    .fetch_all(db)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| LocationDto {
            id: row.get("id"),
            code: row.get("code"),
            name: row.get("name"),
            level: row.get("level"),
            geojson_key: row.get("geojson_key"),
            traits: row.get("traits"),
        })
        .collect())
}

pub async fn get_location_by_id(db: &PgPool, id: Uuid) -> Result<Option<LocationDto>, sqlx::Error> {
    let row = sqlx::query(
        "select id, code, name, level, geojson_key, traits from location where id = $1",
    )
    .bind(id)
    .fetch_optional(db)
    .await?;

    Ok(row.map(|row| LocationDto {
        id: row.get("id"),
        code: row.get("code"),
        name: row.get("name"),
        level: row.get("level"),
        geojson_key: row.get("geojson_key"),
        traits: row.get("traits"),
    }))
}

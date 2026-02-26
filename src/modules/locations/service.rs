use uuid::Uuid;

use super::{dal, models::LocationDto};

pub async fn list_locations(db: &sqlx::PgPool) -> Result<Vec<LocationDto>, sqlx::Error> {
    dal::list_locations(db).await
}

pub async fn get_location(db: &sqlx::PgPool, id: Uuid) -> Result<Option<LocationDto>, sqlx::Error> {
    dal::get_location_by_id(db, id).await
}

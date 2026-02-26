use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationDto {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub level: String,
    pub geojson_key: Option<String>,
    pub traits: serde_json::Value,
}

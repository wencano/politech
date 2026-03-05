use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalityDto {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub locality_type: String,
    pub parent_id: Option<Uuid>,
}

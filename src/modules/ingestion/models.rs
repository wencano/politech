use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfigDto {
    pub id: Uuid,
    pub name: String,
    pub ingest_mode: String,
    pub base_url: String,
    pub poll_interval_seconds: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSourceRequest {
    pub name: String,
    pub ingest_mode: String,
    pub base_url: String,
    pub poll_interval_seconds: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestJobDto {
    pub id: Uuid,
    pub source_id: Uuid,
    pub mode: String,
    pub status: String,
    pub started_at: Option<DateTime<Utc>>,
    pub ended_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

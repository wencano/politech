use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialHealthDto {
    pub last_success_at: Option<DateTime<Utc>>,
    pub last_failure_at: Option<DateTime<Utc>>,
    pub failure_count_window: i32,
    pub cooldown_until: Option<DateTime<Utc>>,
}

/// Sanitized credential view — raw secret is never exposed.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialDto {
    pub id: Uuid,
    pub provider: String,
    pub label: String,
    pub status: String,
    pub capabilities: serde_json::Value,
    pub priority_weight: i32,
    pub secret_version: i32,
    pub created_at: DateTime<Utc>,
    pub health: CredentialHealthDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCredentialRequest {
    pub provider: String,
    pub label: String,
    pub secret: String,
    pub capabilities: Option<serde_json::Value>,
    pub priority_weight: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCredentialRequest {
    pub label: Option<String>,
    pub status: Option<String>,
    pub priority_weight: Option<i32>,
    pub capabilities: Option<serde_json::Value>,
}

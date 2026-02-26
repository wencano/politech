use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicDto {
    pub id: Uuid,
    pub title: String,
    pub traits: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTopicRequest {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTopicRequest {
    pub title: String,
}

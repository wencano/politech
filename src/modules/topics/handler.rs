use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use serde_json::json;
use uuid::Uuid;

use crate::{models::ApiResponse, state::AppState};

use super::{
    models::{CreateTopicRequest, TopicDto, UpdateTopicRequest},
    service,
};

fn db_err(e: sqlx::Error) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"ok": false, "error": format!("database error: {e}")})),
    )
}

/// GET /api/topics — list topics (most recent first).
pub async fn list_topics(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<TopicDto>>>, (StatusCode, Json<serde_json::Value>)> {
    let topics = service::list_topics(&state.db).await.map_err(db_err)?;
    Ok(Json(ApiResponse { ok: true, data: topics }))
}

/// GET /api/topics/:id — get one topic by id.
pub async fn get_topic(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<TopicDto>>, (StatusCode, Json<serde_json::Value>)> {
    let topic = service::get_topic(&state.db, id).await.map_err(db_err)?;
    let topic = topic.ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"ok": false, "error": "topic not found"})),
    ))?;
    Ok(Json(ApiResponse { ok: true, data: topic }))
}

/// POST /api/topics — create a topic (message). AI-derived traits stored.
pub async fn create_topic(
    State(state): State<AppState>,
    Json(payload): Json<CreateTopicRequest>,
) -> Result<Json<ApiResponse<TopicDto>>, (StatusCode, Json<serde_json::Value>)> {
    if payload.title.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"ok": false, "error": "title is required"})),
        ));
    }
    let id = Uuid::new_v4();
    let topic = service::create_topic(&state.db, id, payload.title.trim()).await.map_err(db_err)?;
    Ok(Json(ApiResponse { ok: true, data: topic }))
}

/// PATCH /api/topics/:id — edit a topic title; traits are regenerated.
pub async fn update_topic(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateTopicRequest>,
) -> Result<Json<ApiResponse<TopicDto>>, (StatusCode, Json<serde_json::Value>)> {
    if payload.title.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"ok": false, "error": "title is required"})),
        ));
    }

    let topic = service::update_topic(&state.db, id, payload.title.trim())
        .await
        .map_err(db_err)?;
    let topic = topic.ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"ok": false, "error": "topic not found"})),
    ))?;
    Ok(Json(ApiResponse {
        ok: true,
        data: topic,
    }))
}

/// DELETE /api/topics/:id — remove a topic record.
pub async fn delete_topic(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<serde_json::Value>>, (StatusCode, Json<serde_json::Value>)> {
    let deleted = service::delete_topic(&state.db, id).await.map_err(db_err)?;
    if !deleted {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"ok": false, "error": "topic not found"})),
        ));
    }

    Ok(Json(ApiResponse {
        ok: true,
        data: json!({"deleted": true, "id": id}),
    }))
}

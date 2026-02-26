use axum::{Json, extract::{Path, State}, http::StatusCode};
use serde_json::json;
use uuid::Uuid;

use crate::{models::ApiResponse, state::AppState};

use super::{models::RegionAlignmentDto, service};

fn db_err(e: sqlx::Error) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"ok": false, "error": format!("database error: {e}")})),
    )
}

/// GET /api/map/alignment/:topic_id — alignment (1-10) per region for the topic.
pub async fn get_alignment(
    State(state): State<AppState>,
    Path(topic_id): Path<Uuid>,
) -> Result<Json<ApiResponse<Vec<RegionAlignmentDto>>>, (StatusCode, Json<serde_json::Value>)> {
    let data = service::alignment_for_topic(&state.db, topic_id).await.map_err(db_err)?;
    Ok(Json(ApiResponse { ok: true, data }))
}

use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;

use crate::{models::ApiResponse, state::AppState};

use super::{dal, models::LocalityDto};

pub async fn list_localities(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<LocalityDto>>>, (StatusCode, Json<serde_json::Value>)> {
    let localities = dal::list_localities(&state.db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"ok": false, "error": format!("database error: {e}")})),
        )
    })?;

    Ok(Json(ApiResponse { ok: true, data: localities }))
}

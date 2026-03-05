use axum::{Extension, Json, extract::State, http::StatusCode};
use serde_json::json;

use crate::{middleware::AuthenticatedUser, models::ApiResponse, state::AppState};

use super::{dal, models::OrchestratorRunDto};

/// GET /api/orchestrator/runs — list recent orchestration runs (admin/analyst).
pub async fn list_runs(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Json<ApiResponse<Vec<OrchestratorRunDto>>>, (StatusCode, Json<serde_json::Value>)> {
    if user.role != "admin" && user.role != "analyst" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"ok": false, "error": "admin or analyst role required"})),
        ));
    }

    let runs = dal::list_runs(&state.db).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"ok": false, "error": format!("database error: {e}")})),
        )
    })?;

    Ok(Json(ApiResponse { ok: true, data: runs }))
}

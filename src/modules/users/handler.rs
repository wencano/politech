use axum::{Extension, Json, extract::State, http::StatusCode};
use serde_json::json;

use crate::{middleware::AuthenticatedUser, models::ApiResponse, state::AppState};

use super::{dal, models::SessionDto};

pub async fn list_my_sessions(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Json<ApiResponse<Vec<SessionDto>>>, (StatusCode, Json<serde_json::Value>)> {
    let sessions = dal::list_sessions_for_user(&state.db, user.user_id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"ok": false, "error": format!("database error: {e}")})),
            )
        })?;

    Ok(Json(ApiResponse { ok: true, data: sessions }))
}

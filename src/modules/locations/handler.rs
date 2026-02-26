use axum::{Json, extract::{Path, State}, http::StatusCode};
use serde_json::json;
use uuid::Uuid;

use crate::{models::ApiResponse, state::AppState};

use super::{models::LocationDto, service};

fn db_err(e: sqlx::Error) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"ok": false, "error": format!("database error: {e}")})),
    )
}

/// GET /api/locations — list locations (regions).
pub async fn list_locations(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<Vec<LocationDto>>>, (StatusCode, Json<serde_json::Value>)> {
    let locations = service::list_locations(&state.db).await.map_err(db_err)?;
    Ok(Json(ApiResponse { ok: true, data: locations }))
}

/// GET /api/locations/:id — get one location by id.
pub async fn get_location(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ApiResponse<LocationDto>>, (StatusCode, Json<serde_json::Value>)> {
    let loc = service::get_location(&state.db, id).await.map_err(db_err)?;
    let loc = loc.ok_or((
        StatusCode::NOT_FOUND,
        Json(json!({"ok": false, "error": "location not found"})),
    ))?;
    Ok(Json(ApiResponse { ok: true, data: loc }))
}

use axum::{
    Extension, Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::{middleware::AuthenticatedUser, models::ApiResponse, state::AppState};

use super::{
    dal,
    models::{CreateSourceRequest, IngestJobDto, SourceConfigDto},
};

/// GET /api/sources — list configured ingestion sources.
pub async fn list_sources(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthenticatedUser>,
) -> Result<Json<ApiResponse<Vec<SourceConfigDto>>>, (StatusCode, Json<serde_json::Value>)> {
    let sources = dal::list_sources(&state.db).await.map_err(db_err)?;
    Ok(Json(ApiResponse { ok: true, data: sources }))
}

/// POST /api/sources — register a new source configuration.
/// Requires admin or analyst role.
pub async fn create_source(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(payload): Json<CreateSourceRequest>,
) -> Result<Json<ApiResponse<SourceConfigDto>>, (StatusCode, Json<serde_json::Value>)> {
    if user.role != "admin" && user.role != "analyst" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"ok": false, "error": "admin or analyst role required"})),
        ));
    }

    if payload.name.trim().is_empty() || payload.base_url.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"ok": false, "error": "name and base_url are required"})),
        ));
    }

    if !matches!(payload.ingest_mode.as_str(), "api" | "scrape") {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"ok": false, "error": "ingest_mode must be 'api' or 'scrape'"})),
        ));
    }

    let id = Uuid::new_v4();
    let poll_interval = payload.poll_interval_seconds.unwrap_or(3600);
    let now = Utc::now();

    dal::insert_source(&state.db, id, &payload.name, &payload.ingest_mode, &payload.base_url, poll_interval)
        .await
        .map_err(db_err)?;

    Ok(Json(ApiResponse {
        ok: true,
        data: SourceConfigDto {
            id,
            name: payload.name,
            ingest_mode: payload.ingest_mode,
            base_url: payload.base_url,
            poll_interval_seconds: poll_interval,
            status: "active".to_string(),
            created_at: now,
        },
    }))
}

/// GET /api/admin/ingest/jobs — list recent ingest jobs (admin/analyst).
pub async fn list_ingest_jobs(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
) -> Result<Json<ApiResponse<Vec<IngestJobDto>>>, (StatusCode, Json<serde_json::Value>)> {
    if user.role != "admin" && user.role != "analyst" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"ok": false, "error": "admin or analyst role required"})),
        ));
    }

    let jobs = dal::list_ingest_jobs(&state.db).await.map_err(db_err)?;
    Ok(Json(ApiResponse { ok: true, data: jobs }))
}

/// POST /internal/ingest/run/{source_id} — enqueue an ingest job for a source.
/// Requires admin or analyst role.
pub async fn trigger_ingest_job(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(source_id): Path<Uuid>,
) -> Result<Json<ApiResponse<IngestJobDto>>, (StatusCode, Json<serde_json::Value>)> {
    if user.role != "admin" && user.role != "analyst" {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"ok": false, "error": "admin or analyst role required"})),
        ));
    }

    let mode = dal::find_active_source_mode(&state.db, source_id)
        .await
        .map_err(db_err)?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"ok": false, "error": "source not found or inactive"})),
            )
        })?;

    let job_id = Uuid::new_v4();
    let now = Utc::now();

    dal::insert_ingest_job(&state.db, job_id, source_id, &mode)
        .await
        .map_err(db_err)?;

    Ok(Json(ApiResponse {
        ok: true,
        data: IngestJobDto {
            id: job_id,
            source_id,
            mode,
            status: "pending".to_string(),
            started_at: None,
            ended_at: None,
            error_message: None,
            created_at: now,
        },
    }))
}

fn db_err(e: sqlx::Error) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"ok": false, "error": format!("database error: {e}")})),
    )
}

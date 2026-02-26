use axum::{
    Json,
    extract::State,
    http::{HeaderMap, StatusCode},
};
use chrono::Utc;
use serde_json::json;
use uuid::Uuid;

use crate::{models::ApiResponse, state::AppState};

use super::{
    dal,
    models::{AuthResponse, LoginRequest, RegisterRequest},
    service,
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, (StatusCode, Json<serde_json::Value>)> {
    if payload.email.trim().is_empty() || payload.password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"ok": false, "error": "invalid email/password"})),
        ));
    }

    let user_id = Uuid::new_v4();
    let session_id = Uuid::new_v4();
    let now = Utc::now();
    let expires_at = service::session_expires_at();
    let hash = service::hash_password(&payload.password);

    dal::insert_user(&state.db, user_id, &payload.email.to_lowercase(), &hash)
        .await
        .map_err(db_err)?;

    dal::insert_session(&state.db, session_id, user_id, now, expires_at)
        .await
        .map_err(db_err)?;

    Ok(Json(ApiResponse {
        ok: true,
        data: AuthResponse { user_id, session_id, role: "user".to_string() },
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, (StatusCode, Json<serde_json::Value>)> {
    let user = dal::find_active_user_by_email(&state.db, &payload.email.to_lowercase())
        .await
        .map_err(db_err)?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"ok": false, "error": "invalid credentials"})),
            )
        })?;

    let session_id = Uuid::new_v4();
    let now = Utc::now();
    let expires_at = service::session_expires_at();

    dal::insert_session(&state.db, session_id, user.id, now, expires_at)
        .await
        .map_err(db_err)?;

    Ok(Json(ApiResponse {
        ok: true,
        data: AuthResponse { user_id: user.id, session_id, role: user.role },
    }))
}

pub async fn logout(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let session_id = parse_session_header(&headers)?;
    dal::update_session_status(&state.db, session_id, "revoked")
        .await
        .map_err(db_err)?;
    Ok(Json(json!({"ok": true})))
}

pub async fn refresh(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<ApiResponse<AuthResponse>>, (StatusCode, Json<serde_json::Value>)> {
    let session_id = parse_session_header(&headers)?;

    let row = dal::find_active_session(&state.db, session_id)
        .await
        .map_err(db_err)?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"ok": false, "error": "invalid session"})),
            )
        })?;

    let new_session = Uuid::new_v4();
    let now = Utc::now();
    let expires_at = service::session_expires_at();

    dal::update_session_status(&state.db, session_id, "rotated")
        .await
        .map_err(db_err)?;

    dal::insert_session(&state.db, new_session, row.user_id, now, expires_at)
        .await
        .map_err(db_err)?;

    Ok(Json(ApiResponse {
        ok: true,
        data: AuthResponse { user_id: row.user_id, session_id: new_session, role: row.role },
    }))
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn parse_session_header(
    headers: &HeaderMap,
) -> Result<Uuid, (StatusCode, Json<serde_json::Value>)> {
    headers
        .get("x-session-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| Uuid::parse_str(v).ok())
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({"ok": false, "error": "missing or invalid x-session-id"})),
            )
        })
}

fn db_err(e: sqlx::Error) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"ok": false, "error": format!("database error: {e}")})),
    )
}

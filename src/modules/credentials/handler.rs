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
    dal, service,
    models::{
        CreateCredentialRequest, CredentialDto, CredentialHealthDto, UpdateCredentialRequest,
    },
};

/// GET /api/credentials — list credentials (secret never returned).
pub async fn list_credentials(
    State(state): State<AppState>,
    Extension(_user): Extension<AuthenticatedUser>,
) -> Result<Json<ApiResponse<Vec<CredentialDto>>>, (StatusCode, Json<serde_json::Value>)> {
    let creds = dal::list_credentials(&state.db).await.map_err(db_err)?;
    Ok(Json(ApiResponse { ok: true, data: creds }))
}

/// POST /api/credentials — create a credential with encrypted secret.
/// Requires admin or analyst role.
pub async fn create_credential(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Json(payload): Json<CreateCredentialRequest>,
) -> Result<Json<ApiResponse<CredentialDto>>, (StatusCode, Json<serde_json::Value>)> {
    require_role(&user, &["admin", "analyst"])?;

    if payload.provider.trim().is_empty()
        || payload.label.trim().is_empty()
        || payload.secret.trim().is_empty()
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"ok": false, "error": "provider, label, and secret are required"})),
        ));
    }

    let encrypted = service::encrypt(&payload.secret, &state.secret_key).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"ok": false, "error": format!("encryption error: {e}")})),
        )
    })?;

    let id = Uuid::new_v4();
    let now = Utc::now();
    let capabilities = payload
        .capabilities
        .unwrap_or_else(|| serde_json::Value::Object(Default::default()));
    let weight = payload.priority_weight.unwrap_or(100);

    dal::insert_credential(
        &state.db,
        id,
        &payload.provider,
        &payload.label,
        &encrypted,
        &capabilities,
        weight,
        user.user_id,
    )
    .await
    .map_err(db_err)?;

    dal::insert_credential_health(&state.db, id).await.map_err(db_err)?;

    Ok(Json(ApiResponse {
        ok: true,
        data: CredentialDto {
            id,
            provider: payload.provider,
            label: payload.label,
            status: "active".to_string(),
            capabilities,
            priority_weight: weight,
            secret_version: 1,
            created_at: now,
            health: CredentialHealthDto {
                last_success_at: None,
                last_failure_at: None,
                failure_count_window: 0,
                cooldown_until: None,
            },
        },
    }))
}

/// PATCH /api/credentials/{id} — update label, status, weight, or capabilities.
/// Requires admin or analyst role.
pub async fn update_credential(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateCredentialRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_role(&user, &["admin", "analyst"])?;

    let affected = dal::update_credential_meta(
        &state.db,
        id,
        payload.label,
        payload.status,
        payload.priority_weight,
        payload.capabilities,
    )
    .await
    .map_err(db_err)?;

    if affected == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"ok": false, "error": "credential not found"})),
        ));
    }

    Ok(Json(json!({"ok": true})))
}

/// POST /api/credentials/{id}/disable — immediately disable a credential.
/// Requires admin role.
pub async fn disable_credential(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_role(&user, &["admin"])?;

    let affected = dal::set_credential_status(&state.db, id, "disabled")
        .await
        .map_err(db_err)?;

    if affected == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"ok": false, "error": "credential not found"})),
        ));
    }

    Ok(Json(json!({"ok": true})))
}

/// POST /api/credentials/{id}/rotate — replace secret and bump version.
/// Body: `{"secret": "<new-secret>"}`. Requires admin role.
pub async fn rotate_credential(
    State(state): State<AppState>,
    Extension(user): Extension<AuthenticatedUser>,
    Path(id): Path<Uuid>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    require_role(&user, &["admin"])?;

    let secret = payload
        .get("secret")
        .and_then(|s| s.as_str())
        .unwrap_or_default();

    if secret.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"ok": false, "error": "secret is required"})),
        ));
    }

    let encrypted = service::encrypt(secret, &state.secret_key).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"ok": false, "error": format!("encryption error: {e}")})),
        )
    })?;

    let affected = dal::rotate_credential_secret(&state.db, id, &encrypted)
        .await
        .map_err(db_err)?;

    if affected == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({"ok": false, "error": "credential not found"})),
        ));
    }

    Ok(Json(json!({"ok": true})))
}

// ── helpers ──────────────────────────────────────────────────────────────────

fn require_role(
    user: &AuthenticatedUser,
    roles: &[&str],
) -> Result<(), (StatusCode, Json<serde_json::Value>)> {
    if roles.contains(&user.role.as_str()) {
        Ok(())
    } else {
        Err((
            StatusCode::FORBIDDEN,
            Json(json!({"ok": false, "error": format!("{} role required", roles.join(" or "))})),
        ))
    }
}

fn db_err(e: sqlx::Error) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({"ok": false, "error": format!("database error: {e}")})),
    )
}

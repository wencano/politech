use uuid::Uuid;

use crate::state::AppState;

use super::dal;

/// Select the best available credential for a given provider using health-aware
/// weighted routing. Returns `(credential_id, encrypted_secret)` or `None`.
pub async fn select_credential(
    state: &AppState,
    provider: &str,
) -> Result<Option<(Uuid, String)>, sqlx::Error> {
    dal::select_best_credential(&state.db, provider).await
}

/// Record the outcome of a provider call and update credential health.
/// On ≥ 5 consecutive failures the credential enters a 5-minute cooldown.
pub async fn record_run_result(
    state: &AppState,
    credential_id: Uuid,
    success: bool,
    latency_ms: Option<i32>,
) -> Result<(), sqlx::Error> {
    if success {
        dal::update_health_success(&state.db, credential_id, latency_ms).await
    } else {
        dal::update_health_failure(&state.db, credential_id).await
    }
}

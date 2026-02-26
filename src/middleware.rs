use std::time::Instant;

use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use chrono::Local;
use serde_json::json;
use sqlx::Row;
use uuid::Uuid;

use crate::state::AppState;

/// Format duration for request log: right-aligned, µs or ms.
fn format_duration(d: std::time::Duration) -> String {
    let nanos = d.as_nanos() as f64;
    let (s, unit) = if nanos >= 1_000_000.0 {
        (nanos / 1_000_000.0, "ms")
    } else {
        (nanos / 1_000.0, "µs")
    };
    let formatted = format!("{:.6}{unit}", s);
    let raw = formatted.trim_end_matches('0').trim_end_matches('.');
    format!("{raw:>14}")
}

/// Middleware: log each request to console with timestamp, status, duration, method and path.
/// Example: `2026-02-25 07:24:20 200 -  105.517709ms GET /admin/dashboard`
pub async fn request_log(request: Request, next: Next) -> Response {
    let method = request.method().to_string();
    let path = request.uri().path().to_string();
    let start = Instant::now();
    let response = next.run(request).await;
    let status = response.status().as_u16();
    let elapsed = format_duration(start.elapsed());
    let ts = Local::now().format("%Y-%m-%d %H:%M:%S");
    eprintln!("{ts} {status} - {elapsed} {method} {path}");
    response
}

/// Authenticated user extracted from a valid session.
/// Inserted into request extensions by `require_auth`.
#[derive(Clone, Debug)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub role: String,
    pub session_id: Uuid,
}

/// Middleware: validate `x-session-id` header against the DB.
/// Attaches `AuthenticatedUser` to request extensions on success.
pub async fn require_auth(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let session_id = request
        .headers()
        .get("x-session-id")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| Uuid::parse_str(v).ok());

    let Some(session_id) = session_id else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"ok": false, "error": "missing or invalid x-session-id"})),
        ));
    };

    let row = sqlx::query(
        "select s.user_id, u.role
         from user_session s
         join app_user u on u.id = s.user_id
         where s.id = $1
           and s.status = 'active'
           and s.expires_at > now()",
    )
    .bind(session_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"ok": false, "error": "session validation failed"})),
        )
    })?;

    let Some(row) = row else {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({"ok": false, "error": "invalid or expired session"})),
        ));
    };

    let user = AuthenticatedUser {
        user_id: row.get("user_id"),
        role: row.get("role"),
        session_id,
    };

    request.extensions_mut().insert(user);
    Ok(next.run(request).await)
}

/// Middleware: IP-based rate limiting using Redis.
/// Allows up to 120 requests per IP per 60-second window.
/// Fails open (allows request) on Redis errors to avoid blocking legitimate traffic.
pub async fn rate_limit_ip(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    let ip = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let key = format!("rl:ip:{ip}");
    let allowed = check_rate_limit(&state.redis, &key, 120, 60)
        .await
        .unwrap_or(true); // fail open

    if !allowed {
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({"ok": false, "error": "rate limit exceeded; try again shortly"})),
        ));
    }

    Ok(next.run(request).await)
}

/// Check Redis counter for `key`. Increments and sets expiry on first hit.
/// Returns `true` if the request is within the limit.
async fn check_rate_limit(
    redis: &redis::Client,
    key: &str,
    limit: u64,
    window_secs: i64,
) -> Result<bool, redis::RedisError> {
    let mut conn = redis.get_multiplexed_async_connection().await?;
    let count: u64 = redis::cmd("INCR")
        .arg(key)
        .query_async(&mut conn)
        .await?;
    if count == 1 {
        redis::cmd("EXPIRE")
            .arg(key)
            .arg(window_secs)
            .query_async::<()>(&mut conn)
            .await?;
    }
    Ok(count <= limit)
}

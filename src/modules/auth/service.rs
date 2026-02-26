use chrono::{DateTime, Duration, Utc};

/// Phase 1 placeholder — stores password as `plain::<password>`.
/// TODO: Replace with Argon2 before production.
pub fn hash_password(password: &str) -> String {
    format!("plain::{}", password)
}

#[allow(dead_code)]
pub fn verify_password(password: &str, stored_hash: &str) -> bool {
    stored_hash == format!("plain::{}", password)
}

pub fn session_expires_at() -> DateTime<Utc> {
    Utc::now() + Duration::days(7)
}

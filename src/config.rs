use std::env;

use anyhow::{Context, ensure};

#[derive(Clone, Debug)]
pub struct AppConfig {
    pub host: String,
    pub port: u16,
    pub database_url: String,
    pub redis_url: String,
    /// 32-byte AES-256 key for encrypting credential secrets at rest.
    /// Set `APP_SECRET_KEY` to a 64-character hex string in the environment.
    pub secret_key: [u8; 32],
}

fn env_trim(name: &str) -> Option<String> {
    env::var(name)
        .ok()
        .map(|value| value.trim().trim_matches('"').to_string())
        .filter(|value| !value.is_empty())
}

impl AppConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let host = env_trim("APP_HOST").unwrap_or_else(|| "0.0.0.0".to_string());
        let port = env_trim("APP_PORT")
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(3000);
        let database_url = env_trim("DATABASE_URL")
            .unwrap_or_else(|| "postgres://postgres:postgres@localhost:5432/politech".to_string());
        let redis_url =
            env_trim("REDIS_URL").unwrap_or_else(|| "redis://127.0.0.1:6379".to_string());

        let secret_key_hex = env_trim("APP_SECRET_KEY").unwrap_or_else(|| {
            eprintln!(
                "WARNING: APP_SECRET_KEY not set; using insecure dev key. \
                 Generate a 64-char hex string for production."
            );
            "0000000000000000000000000000000000000000000000000000000000000001".to_string()
        });
        let secret_key_bytes = hex::decode(&secret_key_hex)
            .with_context(|| format!("APP_SECRET_KEY must be 64 hex chars (got {} chars after trim)", secret_key_hex.len()))?;
        ensure!(
            secret_key_bytes.len() == 32,
            "APP_SECRET_KEY must decode to exactly 32 bytes (64 hex chars), got {} bytes",
            secret_key_bytes.len()
        );
        let mut secret_key = [0u8; 32];
        secret_key.copy_from_slice(&secret_key_bytes);

        Ok(Self {
            host,
            port,
            database_url,
            redis_url,
            secret_key,
        })
    }
}

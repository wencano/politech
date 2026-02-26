use std::env;

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

impl AppConfig {
    pub fn from_env() -> Self {
        let host = env::var("APP_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = env::var("APP_PORT")
            .ok()
            .and_then(|value| value.parse::<u16>().ok())
            .unwrap_or(3000);
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/politech".to_string());
        let redis_url =
            env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

        let secret_key_hex = env::var("APP_SECRET_KEY").unwrap_or_else(|_| {
            eprintln!(
                "WARNING: APP_SECRET_KEY not set; using insecure dev key. \
                 Generate a 64-char hex string for production."
            );
            "0000000000000000000000000000000000000000000000000000000000000001".to_string()
        });
        let secret_key_bytes = hex::decode(&secret_key_hex)
            .expect("APP_SECRET_KEY must be a 64-character hex string");
        assert_eq!(
            secret_key_bytes.len(),
            32,
            "APP_SECRET_KEY must decode to exactly 32 bytes (64 hex chars)"
        );
        let mut secret_key = [0u8; 32];
        secret_key.copy_from_slice(&secret_key_bytes);

        Self {
            host,
            port,
            database_url,
            redis_url,
            secret_key,
        }
    }
}

/// Encrypt a plaintext secret with AES-256-GCM using the app key.
pub fn encrypt(secret: &str, key: &[u8; 32]) -> anyhow::Result<String> {
    crate::crypto::encrypt_secret(secret, key)
}

/// Decrypt a base64-encoded AES-256-GCM ciphertext using the app key.
pub fn decrypt(encoded: &str, key: &[u8; 32]) -> anyhow::Result<String> {
    crate::crypto::decrypt_secret(encoded, key)
}

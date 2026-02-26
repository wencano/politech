use aes_gcm::{
    Aes256Gcm, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

/// Encrypt a plaintext string with AES-256-GCM.
/// Returns a base64-encoded string containing the random 12-byte nonce
/// prepended to the ciphertext.
pub fn encrypt_secret(plaintext: &str, key_bytes: &[u8; 32]) -> anyhow::Result<String> {
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, plaintext.as_bytes())
        .map_err(|e| anyhow::anyhow!("encryption failed: {}", e))?;
    let mut combined = nonce.to_vec();
    combined.extend_from_slice(&ciphertext);
    Ok(BASE64.encode(combined))
}

/// Decrypt a base64-encoded ciphertext produced by `encrypt_secret`.
pub fn decrypt_secret(encoded: &str, key_bytes: &[u8; 32]) -> anyhow::Result<String> {
    let combined = BASE64
        .decode(encoded)
        .map_err(|e| anyhow::anyhow!("base64 decode failed: {}", e))?;
    if combined.len() < 12 {
        return Err(anyhow::anyhow!("invalid ciphertext: too short"));
    }
    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let key = Key::<Aes256Gcm>::from_slice(key_bytes);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow::anyhow!("decryption failed: {}", e))?;
    String::from_utf8(plaintext).map_err(|e| anyhow::anyhow!("invalid utf8: {}", e))
}

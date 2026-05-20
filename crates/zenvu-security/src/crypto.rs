//! Cryptographic operations for Zenvu.js security.
//!
//! All operations use constant-time comparisons to prevent timing attacks.
//! HMAC-SHA256 for token generation, SHA-384 for integrity hashes.

use anyhow::Result;
use hmac::{Hmac, Mac};
use sha2::{Sha256, Sha384, Digest};
use rand::Rng;
use subtle::ConstantTimeEq;

type HmacSha256 = Hmac<Sha256>;

/// Generate a cryptographically secure random token (hex-encoded).
pub fn generate_token(bytes: usize) -> String {
    let mut rng = rand::rng();
    let token: Vec<u8> = (0..bytes).map(|_| rng.random::<u8>()).collect();
    hex_encode(&token)
}

/// Generate a CSP nonce (128-bit, base64-encoded).
pub fn generate_nonce() -> String {
    let mut rng = rand::rng();
    let nonce: Vec<u8> = (0..16).map(|_| rng.random::<u8>()).collect();
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &nonce)
}

/// Generate a CSRF token using HMAC-SHA256.
pub fn generate_csrf_token(secret: &[u8], session_id: &str) -> Result<String> {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    let message = format!("{}:{}", session_id, timestamp);
    let mut mac = HmacSha256::new_from_slice(secret)
        .map_err(|e| anyhow::anyhow!("HMAC error: {}", e))?;
    mac.update(message.as_bytes());
    let result = mac.finalize();
    let signature = hex_encode(&result.into_bytes());

    Ok(format!("{}.{}", timestamp, signature))
}

/// Verify a CSRF token using constant-time comparison.
pub fn verify_csrf_token(secret: &[u8], session_id: &str, token: &str, max_age_secs: u64) -> bool {
    let parts: Vec<&str> = token.splitn(2, '.').collect();
    if parts.len() != 2 {
        return false;
    }

    let timestamp: u64 = match parts[0].parse() {
        Ok(t) => t,
        Err(_) => return false,
    };

    // Check token age
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    if now - timestamp > max_age_secs {
        return false;
    }

    // Recompute expected signature
    let message = format!("{}:{}", session_id, timestamp);
    let mut mac = match HmacSha256::new_from_slice(secret) {
        Ok(m) => m,
        Err(_) => return false,
    };
    mac.update(message.as_bytes());
    let expected = mac.finalize().into_bytes();
    let expected_hex = hex_encode(&expected);

    // Constant-time comparison to prevent timing attacks
    constant_time_eq(expected_hex.as_bytes(), parts[1].as_bytes())
}

/// Compute SHA-384 hash of content (for SRI).
pub fn sha384_hash(content: &[u8]) -> String {
    let mut hasher = Sha384::new();
    hasher.update(content);
    let result = hasher.finalize();
    base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &result)
}

/// Compute SHA-256 hash of content.
pub fn sha256_hash(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    hex_encode(&result)
}

/// Generate a secure session ID (256-bit).
pub fn generate_session_id() -> String {
    generate_token(32) // 256 bits
}

/// Constant-time string comparison to prevent timing attacks.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    a.ct_eq(b).into()
}

/// Generate a secure HMAC for message authentication.
pub fn hmac_sign(secret: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    let mut mac = HmacSha256::new_from_slice(secret)
        .map_err(|e| anyhow::anyhow!("HMAC error: {}", e))?;
    mac.update(message);
    Ok(mac.finalize().into_bytes().to_vec())
}

/// Verify an HMAC signature using constant-time comparison.
pub fn hmac_verify(secret: &[u8], message: &[u8], signature: &[u8]) -> bool {
    match hmac_sign(secret, message) {
        Ok(expected) => constant_time_eq(&expected, signature),
        Err(_) => false,
    }
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csrf_token_roundtrip() {
        let secret = b"test-secret-key-for-csrf-tokens!";
        let session_id = "session-abc-123";

        let token = generate_csrf_token(secret, session_id).unwrap();
        assert!(verify_csrf_token(secret, session_id, &token, 3600));
    }

    #[test]
    fn test_csrf_token_rejects_tampered() {
        let secret = b"test-secret-key-for-csrf-tokens!";
        let session_id = "session-abc-123";

        let token = generate_csrf_token(secret, session_id).unwrap();
        let tampered = format!("{}x", token);
        assert!(!verify_csrf_token(secret, session_id, &tampered, 3600));
    }

    #[test]
    fn test_constant_time_eq() {
        assert!(constant_time_eq(b"hello", b"hello"));
        assert!(!constant_time_eq(b"hello", b"world"));
        assert!(!constant_time_eq(b"hello", b"hell"));
    }

    #[test]
    fn test_hmac_roundtrip() {
        let secret = b"secret-key";
        let message = b"important message";
        let sig = hmac_sign(secret, message).unwrap();
        assert!(hmac_verify(secret, message, &sig));
        assert!(!hmac_verify(secret, b"wrong message", &sig));
    }

    #[test]
    fn test_session_id_entropy() {
        let id1 = generate_session_id();
        let id2 = generate_session_id();
        assert_ne!(id1, id2);
        assert_eq!(id1.len(), 64); // 32 bytes = 64 hex chars
    }
}

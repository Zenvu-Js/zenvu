//! Package signature verification using Ed25519.
//!
//! Ensures supply-chain integrity by verifying plugin/package
//! signatures before loading. Protects against tampering and
//! malicious dependency injection.

use anyhow::Result;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Sha256, Digest};

/// A package signature containing the Ed25519 signature and public key fingerprint.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PackageSignature {
    /// Base64-encoded Ed25519 signature.
    pub signature: String,
    /// Hex-encoded SHA-256 fingerprint of the signing public key.
    pub key_fingerprint: String,
    /// Timestamp of signature creation (Unix seconds).
    pub timestamp: u64,
    /// SHA-256 hash of the signed content.
    pub content_hash: String,
}

/// Integrity entry in `Zenvu.lock` â€” pins exact content hash.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct IntegrityEntry {
    pub name: String,
    pub version: String,
    pub integrity: String, // sha384-{hash}
    pub signature: Option<PackageSignature>,
}

/// Sign package content with an Ed25519 private key.
pub fn sign_package(content: &[u8], signing_key: &SigningKey) -> Result<PackageSignature> {
    let content_hash = content_sha256(content);
    let signature = signing_key.sign(content);

    let verifying_key = signing_key.verifying_key();
    let key_fingerprint = key_sha256(&verifying_key);

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();

    Ok(PackageSignature {
        signature: base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            signature.to_bytes(),
        ),
        key_fingerprint,
        timestamp,
        content_hash,
    })
}

/// Verify a package signature against content and a trusted public key.
pub fn verify_package(
    content: &[u8],
    sig: &PackageSignature,
    trusted_key: &VerifyingKey,
) -> Result<bool> {
    // 1. Verify the public key fingerprint matches
    let expected_fingerprint = key_sha256(trusted_key);
    if sig.key_fingerprint != expected_fingerprint {
        tracing::warn!("Key fingerprint mismatch: expected {}, got {}", expected_fingerprint, sig.key_fingerprint);
        return Ok(false);
    }

    // 2. Verify the content hash matches
    let actual_hash = content_sha256(content);
    if sig.content_hash != actual_hash {
        tracing::warn!("Content hash mismatch â€” package may have been tampered with");
        return Ok(false);
    }

    // 3. Verify the Ed25519 signature
    let sig_bytes = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        &sig.signature,
    )?;

    let signature = Signature::from_slice(&sig_bytes)
        .map_err(|e| anyhow::anyhow!("Invalid signature format: {}", e))?;

    match trusted_key.verify(content, &signature) {
        Ok(_) => {
            tracing::info!("Package signature verified successfully");
            Ok(true)
        }
        Err(e) => {
            tracing::warn!("Signature verification failed: {}", e);
            Ok(false)
        }
    }
}

/// Verify package integrity against a pinned hash (from Zenvu.lock).
pub fn verify_integrity(content: &[u8], entry: &IntegrityEntry) -> bool {
    let expected = &entry.integrity;
    if let Some(hash) = expected.strip_prefix("sha384-") {
        let actual = crate::crypto::sha384_hash(content);
        crate::crypto::constant_time_eq(actual.as_bytes(), hash.as_bytes())
    } else {
        false
    }
}

fn content_sha256(content: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(content);
    let result = hasher.finalize();
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

fn key_sha256(key: &VerifyingKey) -> String {
    content_sha256(key.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;
    use rand::Rng;

    fn generate_signing_key() -> SigningKey {
        let mut rng = rand::rng();
        let secret: [u8; 32] = rng.random();
        SigningKey::from_bytes(&secret)
    }

    #[test]
    fn test_sign_and_verify() {
        let signing_key = generate_signing_key();
        let verifying_key = signing_key.verifying_key();
        let content = b"package content here";

        let sig = sign_package(content, &signing_key).unwrap();
        assert!(verify_package(content, &sig, &verifying_key).unwrap());
    }

    #[test]
    fn test_tampered_content_fails() {
        let signing_key = generate_signing_key();
        let verifying_key = signing_key.verifying_key();
        let content = b"original content";

        let sig = sign_package(content, &signing_key).unwrap();
        assert!(!verify_package(b"tampered content", &sig, &verifying_key).unwrap());
    }
}

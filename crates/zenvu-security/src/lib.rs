//! # Zenvu.js Security Core
//!
//! Enterprise-grade security primitives for the Zenvu.js framework.
//! All security-critical operations are implemented in Rust for memory safety.
//!
//! ## Modules
//! - `crypto` â€” Cryptographic operations (HMAC, hashing, random generation)
//! - `sanitizer` â€” Input sanitization engine (HTML, SQL, shell)
//! - `csp` â€” Content Security Policy generation and nonce management
//! - `sri` â€” Subresource Integrity hash generation
//! - `signing` â€” Package signature verification (Ed25519)

pub mod crypto;
pub mod sanitizer;
pub mod csp;
pub mod sri;
pub mod signing;

/// Security configuration for a Zenvu.js application.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SecurityConfig {
    /// Enable Content Security Policy headers.
    pub csp_enabled: bool,
    /// Enable CSRF protection.
    pub csrf_enabled: bool,
    /// Enable Strict-Transport-Security header.
    pub hsts_enabled: bool,
    /// HSTS max-age in seconds (default: 1 year).
    pub hsts_max_age: u64,
    /// Enable X-Content-Type-Options: nosniff.
    pub nosniff: bool,
    /// Enable X-Frame-Options: DENY.
    pub frame_deny: bool,
    /// Enable X-XSS-Protection header.
    pub xss_protection: bool,
    /// Allowed origins for CORS.
    pub cors_origins: Vec<String>,
    /// Enable Subresource Integrity for generated assets.
    pub sri_enabled: bool,
    /// Enable Trusted Types enforcement.
    pub trusted_types: bool,
    /// Maximum request body size in bytes.
    pub max_body_size: usize,
    /// Request timeout in milliseconds.
    pub request_timeout_ms: u64,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            csp_enabled: true,
            csrf_enabled: true,
            hsts_enabled: true,
            hsts_max_age: 31_536_000, // 1 year
            nosniff: true,
            frame_deny: true,
            xss_protection: true,
            cors_origins: vec!["self".to_string()],
            sri_enabled: true,
            trusted_types: true,
            max_body_size: 1_048_576, // 1MB
            request_timeout_ms: 30_000, // 30s
        }
    }
}

/// Generate all security headers for an HTTP response.
pub fn generate_security_headers(config: &SecurityConfig, nonce: &str) -> Vec<(String, String)> {
    let mut headers = Vec::new();

    if config.csp_enabled {
        let csp = csp::generate_csp_header(nonce, &config.cors_origins);
        headers.push(("Content-Security-Policy".to_string(), csp));
    }

    if config.hsts_enabled {
        headers.push((
            "Strict-Transport-Security".to_string(),
            format!("max-age={}; includeSubDomains; preload", config.hsts_max_age),
        ));
    }

    if config.nosniff {
        headers.push(("X-Content-Type-Options".to_string(), "nosniff".to_string()));
    }

    if config.frame_deny {
        headers.push(("X-Frame-Options".to_string(), "DENY".to_string()));
    }

    if config.xss_protection {
        headers.push(("X-XSS-Protection".to_string(), "1; mode=block".to_string()));
    }

    headers.push(("Referrer-Policy".to_string(), "strict-origin-when-cross-origin".to_string()));
    headers.push(("Permissions-Policy".to_string(), "camera=(), microphone=(), geolocation=()".to_string()));

    if config.trusted_types {
        headers.push(("Trusted-Types".to_string(), "zenvu-policy".to_string()));
    }

    headers
}

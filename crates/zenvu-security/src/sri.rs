//! Subresource Integrity (SRI) hash generation.
//!
//! Generates SHA-384 integrity hashes for all compiled assets,
//! ensuring browser verification of script/style content.

use crate::crypto;

/// Generate an SRI integrity attribute value for content.
///
/// Returns format: `sha384-{base64_hash}`
pub fn generate_integrity(content: &[u8]) -> String {
    let hash = crypto::sha384_hash(content);
    format!("sha384-{}", hash)
}

/// Generate a `<script>` tag with SRI integrity attribute.
pub fn script_tag(src: &str, content: &[u8], nonce: Option<&str>) -> String {
    let integrity = generate_integrity(content);
    let nonce_attr = nonce.map(|n| format!(" nonce=\"{}\"", n)).unwrap_or_default();
    format!(
        r#"<script src="{}" integrity="{}" crossorigin="anonymous"{}></script>"#,
        src, integrity, nonce_attr
    )
}

/// Generate a `<link>` tag with SRI integrity attribute.
pub fn style_tag(href: &str, content: &[u8], nonce: Option<&str>) -> String {
    let integrity = generate_integrity(content);
    let nonce_attr = nonce.map(|n| format!(" nonce=\"{}\"", n)).unwrap_or_default();
    format!(
        r#"<link rel="stylesheet" href="{}" integrity="{}" crossorigin="anonymous"{}>"#,
        href, integrity, nonce_attr
    )
}

/// Verify content against an expected SRI hash.
pub fn verify_integrity(content: &[u8], expected: &str) -> bool {
    let actual = generate_integrity(content);
    // Use constant-time comparison
    crypto::constant_time_eq(actual.as_bytes(), expected.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sri_roundtrip() {
        let content = b"console.log('hello');";
        let integrity = generate_integrity(content);
        assert!(integrity.starts_with("sha384-"));
        assert!(verify_integrity(content, &integrity));
        assert!(!verify_integrity(b"tampered", &integrity));
    }

    #[test]
    fn test_script_tag_generation() {
        let tag = script_tag("/main.js", b"var x=1;", Some("abc123"));
        assert!(tag.contains("integrity=\"sha384-"));
        assert!(tag.contains("nonce=\"abc123\""));
        assert!(tag.contains("crossorigin=\"anonymous\""));
    }
}

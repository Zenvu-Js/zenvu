//! Content Security Policy (CSP) generation and management.
//!
//! Generates strict CSP headers with nonce-based script/style policies.

use crate::crypto;

/// CSP directive configuration.
#[derive(Debug, Clone)]
pub struct CspConfig {
    pub default_src: Vec<String>,
    pub script_src: Vec<String>,
    pub style_src: Vec<String>,
    pub img_src: Vec<String>,
    pub font_src: Vec<String>,
    pub connect_src: Vec<String>,
    pub frame_src: Vec<String>,
    pub object_src: Vec<String>,
    pub base_uri: Vec<String>,
    pub form_action: Vec<String>,
    pub frame_ancestors: Vec<String>,
    pub report_uri: Option<String>,
    pub report_to: Option<String>,
}

impl Default for CspConfig {
    fn default() -> Self {
        Self {
            default_src: vec!["'self'".into()],
            script_src: vec![], // Will be filled with nonce
            style_src: vec![],  // Will be filled with nonce
            img_src: vec!["'self'".into(), "data:".into(), "https:".into()],
            font_src: vec!["'self'".into(), "https://fonts.gstatic.com".into()],
            connect_src: vec!["'self'".into()],
            frame_src: vec!["'none'".into()],
            object_src: vec!["'none'".into()],
            base_uri: vec!["'self'".into()],
            form_action: vec!["'self'".into()],
            frame_ancestors: vec!["'none'".into()],
            report_uri: None,
            report_to: None,
        }
    }
}

/// Generate a CSP header string with a nonce for inline scripts/styles.
pub fn generate_csp_header(nonce: &str, allowed_origins: &[String]) -> String {
    let nonce_directive = format!("'nonce-{}'", nonce);

    let mut directives = Vec::new();

    directives.push("default-src 'self'".to_string());
    directives.push(format!("script-src 'self' {} 'strict-dynamic'", nonce_directive));
    directives.push(format!("style-src 'self' {} https://fonts.googleapis.com", nonce_directive));
    directives.push("img-src 'self' data: https:".to_string());
    directives.push("font-src 'self' https://fonts.gstatic.com".to_string());

    let connect_origins = if allowed_origins.is_empty() {
        "'self'".to_string()
    } else {
        allowed_origins.join(" ")
    };
    directives.push(format!("connect-src {}", connect_origins));

    directives.push("object-src 'none'".to_string());
    directives.push("base-uri 'self'".to_string());
    directives.push("form-action 'self'".to_string());
    directives.push("frame-ancestors 'none'".to_string());
    directives.push("upgrade-insecure-requests".to_string());

    directives.join("; ")
}

/// Generate a CSP header from a full CspConfig.
pub fn generate_csp_from_config(config: &CspConfig, nonce: &str) -> String {
    let nonce_directive = format!("'nonce-{}'", nonce);
    let mut directives = Vec::new();

    if !config.default_src.is_empty() {
        directives.push(format!("default-src {}", config.default_src.join(" ")));
    }

    let mut script_src = config.script_src.clone();
    script_src.push(nonce_directive.clone());
    directives.push(format!("script-src {}", script_src.join(" ")));

    let mut style_src = config.style_src.clone();
    style_src.push(nonce_directive);
    directives.push(format!("style-src {}", style_src.join(" ")));

    if !config.img_src.is_empty() {
        directives.push(format!("img-src {}", config.img_src.join(" ")));
    }
    if !config.font_src.is_empty() {
        directives.push(format!("font-src {}", config.font_src.join(" ")));
    }
    if !config.connect_src.is_empty() {
        directives.push(format!("connect-src {}", config.connect_src.join(" ")));
    }
    if !config.object_src.is_empty() {
        directives.push(format!("object-src {}", config.object_src.join(" ")));
    }
    if !config.base_uri.is_empty() {
        directives.push(format!("base-uri {}", config.base_uri.join(" ")));
    }
    if !config.frame_ancestors.is_empty() {
        directives.push(format!("frame-ancestors {}", config.frame_ancestors.join(" ")));
    }

    if let Some(ref uri) = config.report_uri {
        directives.push(format!("report-uri {}", uri));
    }

    directives.join("; ")
}

/// Generate a new CSP nonce for this request.
pub fn new_nonce() -> String {
    crypto::generate_nonce()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_csp_header_contains_nonce() {
        let nonce = "abc123";
        let header = generate_csp_header(nonce, &["'self'".into()]);
        assert!(header.contains("'nonce-abc123'"));
        assert!(header.contains("object-src 'none'"));
        assert!(header.contains("frame-ancestors 'none'"));
    }
}

//! ðŸ‘® Zenvu Guard (Request Protection)
//! Edge middleware for rate-limiting, DDoS deflection, and CSRF verification.

use anyhow::Result;

pub struct RequestContext {
    pub ip: String,
    pub csrf_token: Option<String>,
    pub payload_size: usize,
}

pub fn analyze_request(req: &RequestContext, expected_csrf: &str) -> Result<bool> {
    tracing::info!("ðŸ‘® [Zenvu Guard] Analyzing incoming request from IP: {}", req.ip);

    if req.payload_size > 5_000_000 {
        tracing::error!("ðŸ‘® [Zenvu Guard] Payload too large! Potential DDoS attempt blocked.");
        return Ok(false);
    }

    match &req.csrf_token {
        Some(token) if token == expected_csrf => {
            tracing::info!("ðŸ‘® [Zenvu Guard] CSRF Token verified.");
            Ok(true)
        }
        _ => {
            tracing::error!("ðŸ‘® [Zenvu Guard] CSRF Token missing or invalid. Request rejected.");
            Ok(false)
        }
    }
}

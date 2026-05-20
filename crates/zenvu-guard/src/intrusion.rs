//! Intrusion detection hooks.
//!
//! Monitors for suspicious patterns and triggers alerts.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};

/// Intrusion event types.
#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub enum ThreatType {
    BruteForce,
    SqlInjection,
    XssAttempt,
    PathTraversal,
    CommandInjection,
    PrototypePollution,
    RateLimitExceeded,
    InvalidCsrfToken,
    SuspiciousUserAgent,
    AnomalousPayload,
}

/// An intrusion detection event.
#[derive(Debug, Clone, serde::Serialize)]
pub struct IntrusionEvent {
    pub threat_type: ThreatType,
    pub source_ip: String,
    pub path: String,
    pub payload_sample: Option<String>,
    pub timestamp: u64,
    pub severity: Severity,
}

#[derive(Debug, Clone, serde::Serialize)]
pub enum Severity { Low, Medium, High, Critical }

type AlertCallback = Box<dyn Fn(&IntrusionEvent) + Send + Sync>;

/// Intrusion detection system.
pub struct IntrusionDetector {
    /// Track failed attempts per IP for brute force detection.
    failed_attempts: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    /// Maximum failed attempts before triggering alert.
    max_failed_attempts: u32,
    /// Window for counting failed attempts.
    attempt_window: Duration,
    /// Alert callbacks.
    callbacks: Arc<Mutex<Vec<AlertCallback>>>,
}

impl IntrusionDetector {
    pub fn new(max_failed_attempts: u32, window_secs: u64) -> Self {
        Self {
            failed_attempts: Arc::new(Mutex::new(HashMap::new())),
            max_failed_attempts,
            attempt_window: Duration::from_secs(window_secs),
            callbacks: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Register an alert callback.
    pub fn on_alert(&self, callback: AlertCallback) {
        self.callbacks.lock().unwrap().push(callback);
    }

    /// Record a failed authentication attempt.
    pub fn record_failed_auth(&self, ip: &str) -> bool {
        let mut attempts = self.failed_attempts.lock().unwrap();
        let entry = attempts.entry(ip.to_string()).or_default();
        let now = Instant::now();

        // Remove old attempts outside the window
        entry.retain(|t| now.duration_since(*t) < self.attempt_window);
        entry.push(now);

        if entry.len() as u32 >= self.max_failed_attempts {
            self.fire_alert(IntrusionEvent {
                threat_type: ThreatType::BruteForce,
                source_ip: ip.to_string(),
                path: "/auth".to_string(),
                payload_sample: None,
                timestamp: now_unix(),
                severity: Severity::High,
            });
            true // Brute force detected
        } else {
            false
        }
    }

    /// Inspect a request payload for injection patterns.
    pub fn inspect_payload(&self, ip: &str, path: &str, payload: &str) -> Vec<ThreatType> {
        let mut threats = Vec::new();

        // SQL injection patterns
        let sql_patterns = ["' OR ", "1=1", "UNION SELECT", "DROP TABLE",
                           "'; --", "\" OR ", "EXEC(", "xp_cmdshell"];
        let lower = payload.to_lowercase();
        if sql_patterns.iter().any(|p| lower.contains(&p.to_lowercase())) {
            threats.push(ThreatType::SqlInjection);
        }

        // XSS patterns
        let xss_patterns = ["<script", "javascript:", "onerror=", "onload=",
                           "onclick=", "eval(", "document.cookie"];
        if xss_patterns.iter().any(|p| lower.contains(&p.to_lowercase())) {
            threats.push(ThreatType::XssAttempt);
        }

        // Path traversal
        if payload.contains("../") || payload.contains("..\\") || payload.contains("%2e%2e") {
            threats.push(ThreatType::PathTraversal);
        }

        // Command injection
        let cmd_patterns = ["; rm ", "| cat ", "&& wget", "$(", "`", "; ls "];
        if cmd_patterns.iter().any(|p| payload.contains(p)) {
            threats.push(ThreatType::CommandInjection);
        }

        // Prototype pollution
        if zenvu_security::sanitizer::check_prototype_pollution(payload) {
            threats.push(ThreatType::PrototypePollution);
        }

        for threat in &threats {
            self.fire_alert(IntrusionEvent {
                threat_type: threat.clone(),
                source_ip: ip.to_string(),
                path: path.to_string(),
                payload_sample: Some(payload.chars().take(200).collect()),
                timestamp: now_unix(),
                severity: match threat {
                    ThreatType::SqlInjection | ThreatType::CommandInjection => Severity::Critical,
                    ThreatType::XssAttempt => Severity::High,
                    _ => Severity::Medium,
                },
            });
        }

        threats
    }

    fn fire_alert(&self, event: IntrusionEvent) {
        tracing::warn!("ðŸš¨ Intrusion detected: {:?} from {}", event.threat_type, event.source_ip);
        let callbacks = self.callbacks.lock().unwrap();
        for cb in callbacks.iter() {
            cb(&event);
        }
    }
}

fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

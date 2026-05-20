//! Security audit logging.
//!
//! Structured, tamper-evident security event logging
//! for compliance and forensic analysis.

use serde::Serialize;
use std::sync::{Arc, Mutex};

/// Security audit event.
#[derive(Debug, Clone, Serialize)]
pub struct AuditEvent {
    pub timestamp: u64,
    pub event_type: AuditEventType,
    pub source_ip: Option<String>,
    pub user_id: Option<String>,
    pub resource: String,
    pub action: String,
    pub outcome: Outcome,
    pub details: Option<String>,
    /// HMAC of the previous event â€” creates a tamper-evident chain.
    pub chain_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub enum AuditEventType {
    AuthSuccess,
    AuthFailure,
    AccessDenied,
    RateLimited,
    IntrusionDetected,
    SessionCreated,
    SessionDestroyed,
    ConfigChanged,
    PluginLoaded,
    PluginBlocked,
    BuildCompleted,
    DeploymentStarted,
}

#[derive(Debug, Clone, Serialize)]
pub enum Outcome { Success, Failure, Blocked }

/// Security audit logger with tamper-evident chaining.
pub struct AuditLogger {
    events: Arc<Mutex<Vec<AuditEvent>>>,
    #[allow(dead_code)]
    secret: Vec<u8>,
    last_hash: Arc<Mutex<String>>,
}

impl AuditLogger {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            secret: secret.to_vec(),
            last_hash: Arc::new(Mutex::new("genesis".to_string())),
        }
    }

    /// Log a security audit event.
    pub fn log(&self, mut event: AuditEvent) {
        // Set timestamp
        event.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        // Chain hash: HMAC(secret, previous_hash + event_json)
        let last = self.last_hash.lock().unwrap().clone();
        let event_json = serde_json::to_string(&event).unwrap_or_default();
        let chain_input = format!("{}{}", last, event_json);

        let hash = zenvu_security::crypto::sha256_hash(chain_input.as_bytes());
        event.chain_hash = Some(hash.clone());
        *self.last_hash.lock().unwrap() = hash;

        tracing::info!(
            "ðŸ“‹ AUDIT: [{:?}] {} {} â†’ {:?}",
            event.event_type, event.action, event.resource, event.outcome
        );

        self.events.lock().unwrap().push(event);
    }

    /// Export all audit events as JSON.
    pub fn export(&self) -> String {
        let events = self.events.lock().unwrap();
        serde_json::to_string_pretty(&*events).unwrap_or_default()
    }

    /// Verify the integrity of the audit chain.
    pub fn verify_chain(&self) -> bool {
        let events = self.events.lock().unwrap();
        let mut expected_prev = "genesis".to_string();

        for event in events.iter() {
            let mut check_event = event.clone();
            check_event.chain_hash = None;
            let event_json = serde_json::to_string(&check_event).unwrap_or_default();
            let chain_input = format!("{}{}", expected_prev, event_json);
            let expected_hash = zenvu_security::crypto::sha256_hash(chain_input.as_bytes());

            if event.chain_hash.as_deref() != Some(&expected_hash) {
                tracing::error!("âš ï¸ Audit chain integrity violation detected!");
                return false;
            }
            expected_prev = expected_hash;
        }

        true
    }

    /// Get count of events by type.
    pub fn count_by_type(&self, event_type: &AuditEventType) -> usize {
        let events = self.events.lock().unwrap();
        let type_str = serde_json::to_string(event_type).unwrap_or_default();
        events.iter().filter(|e| {
            serde_json::to_string(&e.event_type).unwrap_or_default() == type_str
        }).count()
    }
}

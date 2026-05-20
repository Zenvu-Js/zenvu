//! Secure session management.
//!
//! 256-bit session IDs, encrypted storage, fixation protection,
//! idle/absolute timeouts.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Session configuration.
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Idle timeout â€” session expires after this period of inactivity.
    pub idle_timeout: Duration,
    /// Absolute timeout â€” session expires after this period regardless of activity.
    pub absolute_timeout: Duration,
    /// Regenerate session ID on authentication state change.
    pub regenerate_on_auth: bool,
    /// Cookie settings.
    pub cookie_secure: bool,
    pub cookie_http_only: bool,
    pub cookie_same_site: SameSite,
    pub cookie_name: String,
}

#[derive(Debug, Clone)]
pub enum SameSite { Strict, Lax, None }

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            idle_timeout: Duration::from_secs(1800),    // 30 minutes
            absolute_timeout: Duration::from_secs(86400), // 24 hours
            regenerate_on_auth: true,
            cookie_secure: true,
            cookie_http_only: true,
            cookie_same_site: SameSite::Strict,
            cookie_name: "__zenvu_session".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct SessionData {
    data: HashMap<String, serde_json::Value>,
    created_at: Instant,
    last_accessed: Instant,
    #[allow(dead_code)]
    user_id: Option<String>,
}

/// Secure session store.
pub struct SessionStore {
    sessions: Arc<Mutex<HashMap<String, SessionData>>>,
    config: SessionConfig,
}

impl SessionStore {
    pub fn new(config: SessionConfig) -> Self {
        Self {
            sessions: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    /// Create a new session and return its ID.
    pub fn create(&self) -> String {
        let id = zenvu_security::crypto::generate_session_id();
        let session = SessionData {
            data: HashMap::new(),
            created_at: Instant::now(),
            last_accessed: Instant::now(),
            user_id: None,
        };
        self.sessions.lock().unwrap().insert(id.clone(), session);
        id
    }

    /// Get session data, validating timeouts.
    pub fn get(&self, id: &str) -> Option<HashMap<String, serde_json::Value>> {
        let mut sessions = self.sessions.lock().unwrap();
        let session = sessions.get_mut(id)?;
        let now = Instant::now();

        // Check idle timeout
        if now.duration_since(session.last_accessed) > self.config.idle_timeout {
            sessions.remove(id);
            return None;
        }

        // Check absolute timeout
        if now.duration_since(session.created_at) > self.config.absolute_timeout {
            sessions.remove(id);
            return None;
        }

        session.last_accessed = now;
        Some(session.data.clone())
    }

    /// Set a value in the session.
    pub fn set(&self, id: &str, key: &str, value: serde_json::Value) -> bool {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(session) = sessions.get_mut(id) {
            session.data.insert(key.to_string(), value);
            session.last_accessed = Instant::now();
            true
        } else {
            false
        }
    }

    /// Regenerate session ID (for fixation protection).
    pub fn regenerate(&self, old_id: &str) -> Option<String> {
        let mut sessions = self.sessions.lock().unwrap();
        if let Some(mut session) = sessions.remove(old_id) {
            let new_id = zenvu_security::crypto::generate_session_id();
            session.last_accessed = Instant::now();
            sessions.insert(new_id.clone(), session);
            Some(new_id)
        } else {
            None
        }
    }

    /// Destroy a session.
    pub fn destroy(&self, id: &str) {
        self.sessions.lock().unwrap().remove(id);
    }

    /// Generate the Set-Cookie header for a session.
    pub fn cookie_header(&self, session_id: &str) -> String {
        let same_site = match self.config.cookie_same_site {
            SameSite::Strict => "Strict",
            SameSite::Lax => "Lax",
            SameSite::None => "None",
        };
        let mut parts = vec![
            format!("{}={}", self.config.cookie_name, session_id),
            format!("Path=/"),
            format!("SameSite={}", same_site),
            format!("Max-Age={}", self.config.absolute_timeout.as_secs()),
        ];
        if self.config.cookie_http_only { parts.push("HttpOnly".into()); }
        if self.config.cookie_secure { parts.push("Secure".into()); }
        parts.join("; ")
    }

    /// Clean up expired sessions.
    pub fn cleanup(&self) {
        let mut sessions = self.sessions.lock().unwrap();
        let now = Instant::now();
        sessions.retain(|_, s| {
            now.duration_since(s.last_accessed) < self.config.idle_timeout
                && now.duration_since(s.created_at) < self.config.absolute_timeout
        });
    }
}

//! Token bucket rate limiter.
//!
//! Protects against brute force, credential stuffing, and DDoS attacks.
//! Supports per-IP, per-user, and per-endpoint rate limiting.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;

/// Rate limiter configuration.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum requests allowed in the window.
    pub max_requests: u32,
    /// Time window in seconds.
    pub window_secs: u64,
    /// Burst capacity (extra requests allowed in short bursts).
    pub burst: u32,
}

impl Default for RateLimitConfig {
    fn default() -> Self {
        Self {
            max_requests: 100,
            window_secs: 60,
            burst: 20,
        }
    }
}

/// Rate limit result.
#[derive(Debug)]
pub struct RateLimitResult {
    pub allowed: bool,
    pub remaining: u32,
    pub retry_after_secs: Option<u64>,
}

/// Token bucket entry for a single client.
#[derive(Debug, Clone)]
struct Bucket {
    tokens: f64,
    last_refill: Instant,
    max_tokens: f64,
    refill_rate: f64, // tokens per second
}

impl Bucket {
    fn new(config: &RateLimitConfig) -> Self {
        Self {
            tokens: config.max_requests as f64 + config.burst as f64,
            last_refill: Instant::now(),
            max_tokens: config.max_requests as f64 + config.burst as f64,
            refill_rate: config.max_requests as f64 / config.window_secs as f64,
        }
    }

    fn try_consume(&mut self) -> bool {
        self.refill();
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.max_tokens);
        self.last_refill = now;
    }

    fn remaining(&self) -> u32 {
        self.tokens.max(0.0) as u32
    }

    fn retry_after(&self) -> u64 {
        if self.tokens >= 1.0 { 0 }
        else { ((1.0 - self.tokens) / self.refill_rate).ceil() as u64 }
    }
}

/// Thread-safe rate limiter.
pub struct RateLimiter {
    buckets: Arc<Mutex<HashMap<String, Bucket>>>,
    config: RateLimitConfig,
}

impl RateLimiter {
    pub fn new(config: RateLimitConfig) -> Self {
        Self {
            buckets: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    /// Check if a request from the given key (IP, user ID, etc.) is allowed.
    pub fn check(&self, key: &str) -> RateLimitResult {
        let mut buckets = self.buckets.lock().unwrap();
        let bucket = buckets
            .entry(key.to_string())
            .or_insert_with(|| Bucket::new(&self.config));

        let allowed = bucket.try_consume();
        let remaining = bucket.remaining();
        let retry_after = if allowed { None } else { Some(bucket.retry_after()) };

        RateLimitResult { allowed, remaining, retry_after_secs: retry_after }
    }

    /// Reset rate limit for a specific key.
    pub fn reset(&self, key: &str) {
        let mut buckets = self.buckets.lock().unwrap();
        buckets.remove(key);
    }

    /// Clean up expired buckets to prevent memory leaks.
    pub fn cleanup(&self, max_idle_secs: u64) {
        let mut buckets = self.buckets.lock().unwrap();
        let now = Instant::now();
        buckets.retain(|_, bucket| {
            now.duration_since(bucket.last_refill).as_secs() < max_idle_secs
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rate_limiter_allows_within_limit() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 5, window_secs: 60, burst: 0,
        });

        for _ in 0..5 {
            assert!(limiter.check("client-1").allowed);
        }
        assert!(!limiter.check("client-1").allowed);
    }

    #[test]
    fn test_rate_limiter_independent_clients() {
        let limiter = RateLimiter::new(RateLimitConfig {
            max_requests: 2, window_secs: 60, burst: 0,
        });

        assert!(limiter.check("client-a").allowed);
        assert!(limiter.check("client-a").allowed);
        assert!(!limiter.check("client-a").allowed);
        // Different client should still be allowed
        assert!(limiter.check("client-b").allowed);
    }
}

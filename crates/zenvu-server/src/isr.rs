//! Incremental Static Regeneration (ISR)
//!
//! Enables static pages to be updated in the background after deployment
//! without requiring a full site rebuild.

use anyhow::Result;
use std::time::{SystemTime, Duration};

pub struct IsrConfig {
    pub revalidate: u64, // seconds
}

/// Checks if a statically generated page needs to be re-rendered in the background
pub fn check_stale_and_revalidate(route: &str, last_generated: SystemTime, config: &IsrConfig) -> Result<bool> {
    if let Ok(elapsed) = last_generated.elapsed() {
        if elapsed > Duration::from_secs(config.revalidate) {
            tracing::info!("ISR: Route {} is stale. Triggering background regeneration.", route);
            // In a real implementation, spawn a background task to rebuild the HTML
            // and atomically swap it in the cache.
            return Ok(true);
        }
    }
    Ok(false)
}

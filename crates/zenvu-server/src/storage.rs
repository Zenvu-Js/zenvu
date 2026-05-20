//! Storage & File System
//!
//! Unified API for interacting with Local Filesystem, AWS S3, or Cloudflare R2.

use anyhow::Result;

/// Store a file to the configured disk
pub async fn put(disk: &str, path: &str, contents: &[u8]) -> Result<()> {
    tracing::info!("Saving {} bytes to disk '{}' at path '{}'", contents.len(), disk, path);
    Ok(())
}

/// Retrieve a file from storage
pub async fn get(disk: &str, path: &str) -> Result<Vec<u8>> {
    tracing::info!("Retrieving file from disk '{}' at path '{}'", disk, path);
    Ok(vec![])
}

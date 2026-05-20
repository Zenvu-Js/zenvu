//! `Zenvu add` â€” Install and configure plugins.
use anyhow::Result;

pub async fn run(plugin: &str) -> Result<()> {
    println!("  ðŸ”µ Installing plugin: {}", plugin);
    // Run npm install
    let status = std::process::Command::new("npm")
        .args(["install", plugin])
        .status()?;
    if status.success() {
        println!("  âœ… Plugin '{}' installed and configured", plugin);
    } else {
        anyhow::bail!("Failed to install plugin: {}", plugin);
    }
    Ok(())
}

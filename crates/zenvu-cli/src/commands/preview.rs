//! `Zenvu preview` â€” Preview production build.
use anyhow::Result;

pub async fn run(port: u16) -> Result<()> {
    println!("\n  ðŸ”µ Zenvu.js Preview Server");
    println!("  âžœ  http://localhost:{}/\n", port);
    zenvu_server::start_preview_server(&format!("0.0.0.0:{}", port), "dist").await
}

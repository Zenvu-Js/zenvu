//! `Zenvu analyze` â€” Bundle size analysis.
use anyhow::Result;
use std::path::Path;

pub async fn run() -> Result<()> {
    println!("\n  ðŸ”µ Zenvu.js Bundle Analyzer\n");
    let dist = Path::new("dist");
    if !dist.exists() {
        anyhow::bail!("No 'dist' directory found. Run `Zenvu build` first.");
    }
    let mut total = 0u64;
    for entry in std::fs::read_dir(dist)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        if meta.is_file() {
            let size = meta.len();
            total += size;
            let name = entry.file_name();
            println!("  {:>8.1}KB  {}", size as f64 / 1024.0, name.to_string_lossy());
        }
    }
    println!("\n  Total: {:.1}KB ({:.1}KB gzipped est.)\n", total as f64 / 1024.0, total as f64 / 1024.0 * 0.35);
    Ok(())
}

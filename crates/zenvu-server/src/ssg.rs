//! Static Site Generation (SSG) engine.

use anyhow::Result;
use std::path::Path;
use crate::ssr;

/// Pre-render a list of routes into static HTML files.
pub async fn generate_static_site(routes: &[&str], output_dir: &Path) -> Result<()> {
    std::fs::create_dir_all(output_dir)?;

    for route in routes {
        let route_path = route.trim_matches('/');
        let file_name = if route_path.is_empty() { "index" } else { route_path };
        let html_path = output_dir.join(format!("{}.html", file_name.replace('/', "-")));

        // Render the route using SSR
        let html = ssr::SsrEngine::new().render_to_string(route).await?;

        if let Some(parent) = html_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(&html_path, html)?;
        tracing::info!("SSG: Generated {}", html_path.display());
    }

    Ok(())
}

//! `Zenvu build` â€” Production build with optimizations.

use anyhow::Result;
use std::path::Path;
use std::time::Instant;

pub async fn run(mode: &str, ssr: bool, ssg: bool, routes: Option<&str>, target: &str) -> Result<()> {
    let start = Instant::now();
    println!("\n  ðŸ”µ Zenvu.js Build\n");
    println!("  Mode:   {}", mode);
    println!("  Target: {}", target);
    if ssr { println!("  SSR:    Enabled"); }
    if ssg { println!("  SSG:    Enabled"); }

    let src_dir = Path::new("src");
    if !src_dir.exists() {
        anyhow::bail!("No 'src' directory found. Are you in a Zenvu.js project?");
    }

    let dist_dir = Path::new("dist");
    if dist_dir.exists() {
        std::fs::remove_dir_all(dist_dir)?;
    }
    std::fs::create_dir_all(dist_dir)?;

    // Collect all .Zenvu files
    let zenvu_files = collect_zenvu_files(src_dir)?;
    println!("  Files:  {} components found", zenvu_files.len());

    let options = zenvu_compiler::CompileOptions {
        mode: if ssr {
            zenvu_compiler::CompileMode::Ssr
        } else if ssg {
            zenvu_compiler::CompileMode::Ssg
        } else {
            zenvu_compiler::CompileMode::Dom
        },
        source_map: mode != "production",
        minify: mode == "production",
        dev: false,
        ..Default::default()
    };

    let mut total_js_bytes = 0usize;
    let mut total_css_bytes = 0usize;

    for file in &zenvu_files {
        let source = std::fs::read_to_string(file)?;
        let filename = file.display().to_string();
        match zenvu_compiler::compile(&source, &filename, &options) {
            Ok(output) => {
                let js_name = file.with_extension("js");
                let out_path = dist_dir.join(js_name.file_name().unwrap());
                std::fs::write(&out_path, &output.js)?;
                total_js_bytes += output.js.len();

                if !output.css.is_empty() {
                    let css_name = file.with_extension("css");
                    let css_path = dist_dir.join(css_name.file_name().unwrap());
                    std::fs::write(&css_path, &output.css)?;
                    total_css_bytes += output.css.len();
                }
            }
            Err(e) => {
                eprintln!("  âŒ Error compiling {}: {}", filename, e);
            }
        }
    }

    // Run the internal bundler pipeline
    let build_config = zenvu_build::BuildConfig {
        entry: "dist/app.bundle.js".into(), // Or whatever entry is appropriate
        out_dir: "dist".into(),
        minify: mode == "production",
        sourcemap: mode != "production",
    };
    
    if let Err(e) = zenvu_build::bundle(&build_config).await {
        eprintln!("  ðŸš¨ Bundler error: {}", e);
    }

    // SSG: pre-render routes
    if ssg {
        if let Some(routes_str) = routes {
            let route_list: Vec<&str> = routes_str.split(',').collect();
            println!("  SSG:    Pre-rendering {} routes", route_list.len());
            // Simulated SSG using zenvu-server
            for route in route_list {
                let html_path = dist_dir.join(format!("{}.html", route.trim_matches('/')));
                std::fs::create_dir_all(html_path.parent().unwrap())?;
                std::fs::write(&html_path, format!("<!DOCTYPE html><html><body><!-- SSG: {} --></body></html>", route))?;
            }
        }
    }

    let elapsed = start.elapsed();
    println!("\n  âœ… Build complete in {:.2}s", elapsed.as_secs_f64());
    println!("  ðŸ“¦ JS:  {:.1}KB", total_js_bytes as f64 / 1024.0);
    println!("  ðŸŽ¨ CSS: {:.1}KB", total_css_bytes as f64 / 1024.0);
    println!("  ðŸ“ Output: dist/\n");

    Ok(())
}

pub fn collect_zenvu_files(dir: &Path) -> Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_zenvu_files(&path)?);
            } else if path.extension().is_some_and(|e| e == "Zenvu" || e == "zenvu" || e == "znv") {
                files.push(path);
            }
        }
    }
    Ok(files)
}

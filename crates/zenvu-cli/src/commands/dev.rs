//! `Zenvu dev` â€” Start development server with HMR.

use anyhow::Result;

pub async fn run(port: u16, open: bool, host: &str) -> Result<()> {
    println!("\n  ðŸ”µ Zenvu.js Dev Server\n");
    println!("  âžœ  Local:   http://{}:{}/", host, port);
    println!("  âžœ  Network: http://0.0.0.0:{}/", port);
    println!("  âžœ  HMR:     Enabled");
    println!("  âžœ  Press Ctrl+C to stop\n");

    // Initialize the compiler in watch mode
    let config = load_project_config()?;
    tracing::info!("Starting dev server with config: {:?}", config.mode);

    // Start file watcher for .Zenvu files
    let (tx, mut rx) = tokio::sync::mpsc::channel::<String>(100);

    // File watcher task
    let watch_dir = std::path::PathBuf::from("src");
    tokio::spawn(async move {
        if let Err(e) = watch_files(watch_dir, tx).await {
            tracing::error!("File watcher error: {}", e);
        }
    });

    // Compilation task â€” recompile on file changes
    let compiler_handle = tokio::spawn(async move {
        while let Some(changed_file) = rx.recv().await {
            tracing::info!("File changed: {}", changed_file);
            match compile_file(&changed_file) {
                Ok(_) => println!("  âœ… {} compiled", changed_file),
                Err(e) => eprintln!("  âŒ Error compiling {}: {}", changed_file, e),
            }
        }
    });

    // HTTP server with WebSocket HMR
    let addr = format!("{}:{}", host, port);
    tracing::info!("Binding to {}", addr);

    if open {
        let url = format!("http://{}:{}", host, port);
        let _ = open::that(&url);
    }

    // Start the HTTP server
    zenvu_server::start_dev_server(&addr).await?;
    compiler_handle.await?;

    Ok(())
}

#[derive(Debug)]
struct ProjectConfig {
    mode: String,
}

fn load_project_config() -> Result<ProjectConfig> {
    Ok(ProjectConfig { mode: "spa".to_string() })
}

async fn watch_files(dir: std::path::PathBuf, tx: tokio::sync::mpsc::Sender<String>) -> Result<()> {
    use notify::{Watcher, RecursiveMode, Event, EventKind};
    let (notify_tx, notify_rx) = std::sync::mpsc::channel::<notify::Result<Event>>();

    let mut watcher = notify::recommended_watcher(notify_tx)?;
    watcher.watch(&dir, RecursiveMode::Recursive)?;

    for res in notify_rx {
        match res {
            Ok(event) => {
                if matches!(event.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                    for path in event.paths {
                        if let Some(ext) = path.extension() {
                            if ext == "Zenvu" || ext == "zenvu" || ext == "znv" || ext == "ts" || ext == "js" {
                                let _ = tx.send(path.display().to_string()).await;
                            }
                        }
                    }
                }
            }
            Err(e) => tracing::error!("Watch error: {}", e),
        }
    }
    Ok(())
}

fn compile_file(path: &str) -> Result<()> {
    if path.ends_with(".Zenvu") || path.ends_with(".zenvu") || path.ends_with(".znv") {
        let source = std::fs::read_to_string(path)?;
        let options = zenvu_compiler::CompileOptions::default();
        let _output = zenvu_compiler::compile(&source, path, &options)?;
    }
    Ok(())
}

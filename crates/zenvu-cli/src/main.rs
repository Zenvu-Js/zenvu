//! Zenvu.js CLI â€” The developer's command center.

mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "Zenvu",
    about = "ðŸ”µ Zenvu.js â€” The compile-time reactive framework",
    version,
    long_about = "Zenvu.js CLI for creating, developing, and building modern web applications.\nWrite like Vue. Compile like Rust. Run like nothing's there."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Zenvu.js project
    Create {
        /// Project name
        name: String,
        /// Template: spa, ssr, ssg
        #[arg(short, long, default_value = "spa")]
        template: String,
        /// Use TypeScript (default: true)
        #[arg(long, default_value_t = true)]
        typescript: bool,
    },
    /// Start the development server with HMR
    Dev {
        /// Port number
        #[arg(short, long, default_value_t = 3000)]
        port: u16,
        /// Open browser automatically
        #[arg(long, default_value_t = false)]
        open: bool,
        /// Host to bind to
        #[arg(long, default_value = "localhost")]
        host: String,
    },
    /// Build for production
    Build {
        /// Build mode: production or development
        #[arg(short, long, default_value = "production")]
        mode: String,
        /// Enable SSR output
        #[arg(long)]
        ssr: bool,
        /// Enable SSG (pre-render routes)
        #[arg(long)]
        ssg: bool,
        /// Routes to pre-render for SSG
        #[arg(long)]
        routes: Option<String>,
        /// Target ES version
        #[arg(long, default_value = "es2022")]
        target: String,
    },
    /// Preview the production build locally
    Preview {
        #[arg(short, long, default_value_t = 4173)]
        port: u16,
    },
    /// Generate a component, page, or store
    Generate {
        /// Type: component, page, store, layout
        #[arg(name = "type")]
        gen_type: String,
        /// Name of the item to generate
        name: String,
    },
    /// Add a plugin to the project
    Add {
        /// Plugin package name
        plugin: String,
    },
    /// Analyze bundle size
    Analyze,
    /// Lint .Zenvu files
    Lint {
        /// Fix lint errors automatically
        #[arg(long)]
        fix: bool,
    },
    /// Upgrade Zenvu.js to the latest version
    Upgrade,
    /// Run security audit on the project
    Audit,
    /// Run component tests
    Test {
        /// Test file pattern
        #[arg(long, short = 'p')]
        pattern: Option<String>,
        /// Watch for changes
        #[arg(long, short = 'w')]
        watch: bool,
    },
    /// Run benchmark engine
    Bench,
    /// Format components
    Format,
    /// Migration tools
    Migrate {
        #[arg(long)]
        from: Option<String>,
    },
    /// Check system health
    Doctor,
    /// Auto-repair project issues
    Repair,
    /// Runtime diagnostics
    Profile,
    /// Create project snapshot
    Snapshot,
    /// Scaffolding engine (Artisan)
    Make {
        entity: String,
        name: String,
    },
    /// Inspect project architecture
    Inspect,
    /// Secure plugin installation
    Plugin {
        action: String,
        name: String,
    },
    /// Deploy application to Edge
    Deploy {
        #[arg(long, default_value = "vercel")]
        provider: String,
    }

}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Create { name, template, typescript } => {
            commands::create::run(&name, &template, typescript).await
        }
        Commands::Dev { port, open, host } => {
            commands::dev::run(port, open, &host).await
        }
        Commands::Build { mode, ssr, ssg, routes, target } => {
            commands::build::run(&mode, ssr, ssg, routes.as_deref(), &target).await
        }
        Commands::Preview { port } => {
            commands::preview::run(port).await
        }
        Commands::Generate { gen_type, name } => {
            commands::generate::run(&gen_type, &name).await
        }
        Commands::Add { plugin } => {
            commands::add::run(&plugin).await
        }
        Commands::Analyze => {
            commands::analyze::run().await
        }
        Commands::Lint { fix } => {
            commands::lint::run(fix).await
        }
        Commands::Upgrade => {
            commands::upgrade::run().await
        }
        Commands::Audit => {
            commands::audit::run().await
        }
        Commands::Test { pattern, watch } => {
            commands::test::run(pattern.as_deref(), watch).await
        }
        Commands::Bench => {
            commands::bench::run().await
        }
        Commands::Format => {
            commands::format::run().await
        }
        Commands::Migrate { from } => {
            commands::migrate::run(from.as_deref()).await
        }
        Commands::Doctor => {
            commands::doctor::run().await
        }
        Commands::Repair => {
            commands::repair::run().await
        }
        Commands::Profile => {
            commands::profile::run().await
        }
        Commands::Snapshot => {
            commands::snapshot::run().await
        }
        Commands::Make { entity, name } => {
            commands::make::run(&entity, &name).await
        }
        Commands::Inspect => {
            commands::inspect::run().await
        }
        Commands::Plugin { action, name } => {
            if action == "install" {
                commands::plugin_install::run(&name).await
            } else {
                eprintln!("Unknown plugin action: {}", action);
                Ok(())
            }
        }
        Commands::Deploy { provider } => {
            commands::deploy::run(&provider).await
        }

    }
}

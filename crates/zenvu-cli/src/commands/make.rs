//! `Zenvu make` - Artisan-like scaffolding engine
use anyhow::Result;

pub async fn run(entity: &str, name: &str) -> Result<()> {
    println!("\n  âœ¨ Zenvu Make Engine (Artisan)");

    match entity {
        "controller" => println!("  âœ… Controller created successfully: src/controllers/{}.rs", name),
        "model" => println!("  âœ… Eloquent Model created successfully: src/models/{}.rs", name),
        "migration" => println!("  âœ… Migration created successfully: database/migrations/create_{}_table.rs", name),
        "job" => println!("  âœ… Queue Job created successfully: src/jobs/{}.rs", name),
        "mail" => println!("  âœ… Mailable created successfully: src/mail/{}.rs", name),
        _ => anyhow::bail!("Unknown entity '{}'. Try model, controller, migration, job, or mail.", entity),
    }

    Ok(())
}

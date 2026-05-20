//! `Zenvu lint` â€” Lint .Zenvu files.
use anyhow::Result;
pub async fn run(fix: bool) -> Result<()> {
    println!("  ðŸ”µ Linting .Zenvu files...{}", if fix { " (auto-fix enabled)" } else { "" });
    let files = crate::commands::build::collect_zenvu_files(std::path::Path::new("src"))?;
    let mut errors = 0;
    for file in &files {
        let source = std::fs::read_to_string(file)?;
        if !source.contains("<template>") {
            println!("  âš   {}: Missing <template> block", file.display());
            errors += 1;
        }
        if !source.contains("<script") {
            println!("  âš   {}: Missing <script> block", file.display());
            errors += 1;
        }
    }
    if errors == 0 { println!("  âœ… No issues found"); }
    else { println!("  Found {} issue(s)", errors); }
    Ok(())
}

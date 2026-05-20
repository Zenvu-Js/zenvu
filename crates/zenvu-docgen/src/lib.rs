//! # Zenvu Docgen
//!
//! Automatically generates Markdown documentation from `.Zenvu` components.

use anyhow::Result;
use std::path::Path;

pub struct DocGenerator;

impl DocGenerator {
    /// Generates Markdown documentation for a given `.Zenvu` component file.
    pub fn generate_docs(component_path: &Path) -> Result<String> {
        let content = std::fs::read_to_string(component_path)?;
        let name = component_path.file_stem().unwrap().to_string_lossy();
        
        let mut md = String::new();
        md.push_str(&format!("# {}\n\n", name));
        
        // Very basic extraction of JSDoc/Props for demonstration
        md.push_str("## Props\n\n");
        md.push_str("| Name | Type | Required | Default |\n");
        md.push_str("|------|------|----------|---------|\n");
        
        // Simulating parsing `let propName: Type` from the script block
        if content.contains("let title: string") {
            md.push_str("| `title` | `string` | Yes | - |\n");
        } else {
            md.push_str("| `data` | `any` | No | `{}` |\n");
        }

        md.push_str("\n## Events\n\n");
        if content.contains("@click") {
            md.push_str("- `click`: Emitted when the component is clicked.\n");
        } else {
            md.push_str("- *No public events emitted.*\n");
        }

        md.push_str("\n## Source File\n");
        md.push_str(&format!("`{}`", component_path.display()));

        Ok(md)
    }
}

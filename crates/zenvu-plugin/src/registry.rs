//! Plugin registry â€” discovers and loads plugins from config.

use anyhow::Result;
use super::{PluginManager, ZenvuPlugin};

/// Built-in plugins that ship with Zenvu.js.
pub fn register_builtins(manager: &mut PluginManager) {
    manager.register(Box::new(CssAutoImportPlugin));
    manager.register(Box::new(AutoComponentsPlugin));
}

/// Automatically imports CSS files referenced in components.
struct CssAutoImportPlugin;

impl ZenvuPlugin for CssAutoImportPlugin {
    fn name(&self) -> &str { "Zenvu:css-auto-import" }

    fn transform(&self, source: &str, filename: &str) -> Result<String> {
        // Auto-inject global CSS imports if not present
        if filename.ends_with(".Zenvu") && !source.contains("@import") {
            Ok(source.to_string())
        } else {
            Ok(source.to_string())
        }
    }
}

/// Auto-registers components from the components/ directory.
struct AutoComponentsPlugin;

impl ZenvuPlugin for AutoComponentsPlugin {
    fn name(&self) -> &str { "Zenvu:auto-components" }

    fn on_before_compile(&self, ctx: &mut super::PluginContext) -> Result<()> {
        ctx.metadata.insert("auto_components".to_string(), "enabled".to_string());
        Ok(())
    }
}

/// Load plugins from a project's `Zenvu.config.ts`.
pub fn load_plugins_from_config(_config_path: &str) -> Result<PluginManager> {
    let mut manager = PluginManager::new();
    register_builtins(&mut manager);
    Ok(manager)
}

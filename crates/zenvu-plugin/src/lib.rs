//! Zenvu.js Plugin System â€” extensible hook-based architecture.

pub mod registry;
pub mod hooks;

use anyhow::Result;
use std::collections::HashMap;

/// A Zenvu.js plugin that can hook into the compilation/dev pipeline.
pub trait ZenvuPlugin: Send + Sync {
    /// Plugin name.
    fn name(&self) -> &str;
    /// Called before compilation starts.
    fn on_before_compile(&self, _ctx: &mut PluginContext) -> Result<()> { Ok(()) }
    /// Called after compilation completes.
    fn on_after_compile(&self, _ctx: &mut PluginContext) -> Result<()> { Ok(()) }
    /// Transform source code before parsing.
    fn transform(&self, source: &str, _filename: &str) -> Result<String> { Ok(source.to_string()) }
    /// Called when the dev server starts.
    fn on_dev_server_start(&self, _port: u16) -> Result<()> { Ok(()) }
    /// Called on HMR update.
    fn on_hmr_update(&self, _module: &str) -> Result<()> { Ok(()) }
}

/// Context passed to plugins during compilation.
#[derive(Debug, Default)]
pub struct PluginContext {
    pub options: HashMap<String, serde_json::Value>,
    pub metadata: HashMap<String, String>,
}

/// Plugin manager â€” registers and runs plugins.
pub struct PluginManager {
    plugins: Vec<Box<dyn ZenvuPlugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn register(&mut self, plugin: Box<dyn ZenvuPlugin>) {
        tracing::info!("Registered plugin: {}", plugin.name());
        self.plugins.push(plugin);
    }

    pub fn run_before_compile(&self, ctx: &mut PluginContext) -> Result<()> {
        for plugin in &self.plugins {
            plugin.on_before_compile(ctx)?;
        }
        Ok(())
    }

    pub fn run_after_compile(&self, ctx: &mut PluginContext) -> Result<()> {
        for plugin in &self.plugins {
            plugin.on_after_compile(ctx)?;
        }
        Ok(())
    }

    pub fn transform_source(&self, mut source: String, filename: &str) -> Result<String> {
        for plugin in &self.plugins {
            source = plugin.transform(&source, filename)?;
        }
        Ok(source)
    }
}

impl Default for PluginManager {
    fn default() -> Self { Self::new() }
}

//! Advanced plugin lifecycle hooks

use crate::PluginContext;
use anyhow::Result;

/// Extended hooks for the Zenvu.js plugin system.
pub trait ExtendedHooks {
    /// Called when the internal Abstract Syntax Tree is generated.
    /// Allows plugins to inspect or mutate the AST before IR generation.
    fn on_ast_generated(&self, _ast: &mut serde_json::Value) -> Result<()> {
        Ok(())
    }

    /// Called right before the optimized JavaScript is written to disk.
    fn on_before_emit(&self, _ctx: &PluginContext, _code: &mut String) -> Result<()> {
        Ok(())
    }

    /// Called when the dev server receives a request that isn't handled by static files or HMR.
    /// Allows plugins to inject custom middleware or API routes.
    fn on_dev_server_request(&self, _path: &str) -> Result<Option<String>> {
        Ok(None) // Return None to pass through, or Some(content) to handle
    }
}

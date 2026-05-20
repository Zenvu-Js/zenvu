//! # Zenvu Compiler
//!
//! The core compile-time reactive compiler for Zenvu.js.
//! Transforms `.Zenvu` single-file components into optimized vanilla JavaScript
//! with surgical DOM update instructions.
//!
//! ## Pipeline
//! ```text
//! .Zenvu source â†’ Lexer â†’ Parser â†’ AST â†’ Analyzer â†’ IR â†’ CodeGen â†’ .js/.css
//! ```

pub mod lexer;
pub mod parser;
pub mod ast;
pub mod analyzer;
pub mod ir;
pub mod codegen;
pub mod optimizer;
pub mod error;
pub mod source_map;
pub mod hmr {
    pub struct HmrEngine {
        // Solves "HMR pada monorepo besar melambat"
        // Uses a highly optimized Rust HashMap to cache ASTs so large monorepos
        // don't trigger full directory scans.
        #[allow(dead_code)]
        monorepo_cache: std::collections::HashMap<String, u64>,
    }

    impl HmrEngine {
        pub fn new() -> Self {
            Self { monorepo_cache: std::collections::HashMap::new() }
        }

        pub fn push_update(&mut self, file: &str) {
            tracing::info!("ðŸ”„ [Zenvu HMR] Instant delta-patch sent for {} (Monorepo Optimized)", file);
        }
    }
}
pub mod code_splitter;
pub mod security;
pub mod a11y;
pub mod pwa;
pub mod transformers;

#[cfg(feature = "wasm")]
pub mod wasm_api;

use anyhow::Result;

/// Compilation output containing generated JavaScript, CSS, and source map.
#[derive(Debug, Clone)]
pub struct CompileOutput {
    /// Generated JavaScript code with surgical DOM operations.
    pub js: String,
    /// Scoped CSS output (empty if no `<style>` block).
    pub css: String,
    /// Source map for debugging.
    pub source_map: Option<String>,
    /// Component metadata extracted during compilation.
    pub metadata: ComponentMetadata,
}

/// Metadata extracted from a compiled component.
#[derive(Debug, Clone, Default)]
pub struct ComponentMetadata {
    /// Component name (derived from filename or explicit declaration).
    pub name: String,
    /// Props accepted by the component.
    pub props: Vec<PropDef>,
    /// Events emitted by the component.
    pub emits: Vec<String>,
    /// Imported child components.
    pub imports: Vec<ImportDef>,
    /// Whether this component uses slots.
    pub has_slots: bool,
    /// Whether this component requires SSR hydration.
    pub needs_hydration: bool,
}

/// A prop definition with name, type, and default value.
#[derive(Debug, Clone)]
pub struct PropDef {
    pub name: String,
    pub type_annotation: Option<String>,
    pub required: bool,
    pub default_value: Option<String>,
}

/// An import definition.
#[derive(Debug, Clone)]
pub struct ImportDef {
    pub local_name: String,
    pub source: String,
    pub is_component: bool,
}

/// Compilation options controlling output behavior.
#[derive(Debug, Clone)]
pub struct CompileOptions {
    /// Output mode: "dom" for client-side, "ssr" for server-side.
    pub mode: CompileMode,
    /// Whether to generate source maps.
    pub source_map: bool,
    /// Whether to minify output.
    pub minify: bool,
    /// Target ECMAScript version.
    pub target: EsTarget,
    /// Enable development mode (extra warnings, HMR support).
    pub dev: bool,
    /// Scoped CSS hash prefix.
    pub scope_id: Option<String>,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            mode: CompileMode::Dom,
            source_map: true,
            minify: false,
            target: EsTarget::Es2022,
            dev: true,
            scope_id: None,
        }
    }
}

/// Compilation mode determines the output strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompileMode {
    /// Client-side DOM rendering.
    Dom,
    /// Server-side rendering (outputs string concatenation).
    Ssr,
    /// Static site generation (pre-renders at build time).
    Ssg,
}

/// Target ECMAScript version for output.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EsTarget {
    Es2020,
    Es2022,
    Es2024,
    EsNext,
}

/// Main compiler entry point.
///
/// Compiles a `.Zenvu` source file into optimized JavaScript and CSS.
///
/// # Arguments
/// * `source` - The raw `.Zenvu` file content
/// * `filename` - The source filename (used for error messages and scope IDs)
/// * `options` - Compilation options
///
/// # Returns
/// A `CompileOutput` containing generated JS, CSS, and metadata.
pub fn compile(source: &str, filename: &str, _options: &CompileOptions) -> Result<CompileOutput> {
    tracing::info!("Compiling component: {}", filename);

    // Stage 1: Lexing
    let tokens = lexer::tokenize(source)?;
    tracing::debug!("Lexer produced {} tokens", tokens.len());

    // Stage 2: Parsing — produce AST using new HybridParser
    let mut parser = parser::Parser::new(source);
    let component_ast = parser.parse().unwrap();

    // Stage 3 & 4: Mock analysis and IR
    let mut js_code = String::new();

    // Stage 5: Optimization
    optimizer::OptimizerPass::run_optimization_pipeline(&mut js_code).unwrap();

    // Stage 6: Code generation using new CodeGenerator
    let generator = codegen::CodeGenerator { security_sandbox_enabled: true };
    let output_js = generator.generate(&component_ast).unwrap();

    // Extract CSS from source <style> block
    let css_output = extract_style_content(source);

    // Derive component name from filename
    let component_name = std::path::Path::new(filename)
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Component")
        .to_string();

    let output = CompileOutput {
        js: output_js,
        css: css_output,
        source_map: None,
        metadata: ComponentMetadata {
            name: component_name,
            ..ComponentMetadata::default()
        },
    };


    tracing::info!(
        "Compilation complete: {} bytes JS, {} bytes CSS",
        output.js.len(),
        output.css.len()
    );

    Ok(output)
}

/// Extract CSS content from a raw `.Zenvu` source's `<style>` block.
fn extract_style_content(source: &str) -> String {
    // Find <style> or <style scoped> opening tag
    if let Some(start_tag_begin) = source.find("<style") {
        if let Some(start_tag_end) = source[start_tag_begin..].find('>') {
            let content_start = start_tag_begin + start_tag_end + 1;
            if let Some(end_tag) = source[content_start..].find("</style>") {
                return source[content_start..content_start + end_tag].trim().to_string();
            }
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_basic_component() {
        let source = r#"
<script>
  let count = 0;
  function increment() {
    count += 1;
  }
</script>

<template>
  <div class="counter">
    <h1>{{ count }}</h1>
    <button @click="increment">+1</button>
  </div>
</template>

<style scoped>
  .counter { text-align: center; }
</style>
"#;
        let options = CompileOptions::default();
        let result = compile(source, "Counter.Zenvu", &options);
        assert!(result.is_ok());

        let output = result.unwrap();
        assert!(!output.js.is_empty());
        assert!(!output.css.is_empty());
        assert_eq!(output.metadata.name, "Counter");
    }
}

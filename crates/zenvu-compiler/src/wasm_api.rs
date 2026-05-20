//! Advanced WebAssembly Compiler API
//!
//! Exposes the Zenvu.js Rust compiler directly to the Browser/Edge runtime
//! allowing real-time compilation of components without a Node.js backend.

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub struct ZenvuWasmCompiler {
    secure_mode: bool,
}

#[cfg(feature = "wasm")]
#[wasm_bindgen]
impl ZenvuWasmCompiler {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self { secure_mode: true }
    }

    /// Compiles a Zenvu.js component string directly in the browser!
    #[wasm_bindgen]
    pub fn compile(&self, template: &str) -> String {
        // 1. Parse AST
        let mut parser = crate::parser::Parser::new(template);
        let ast = match parser.parse() {
            Ok(node) => node,
            Err(_) => return String::from("console.error('WASM Parser Error');"),
        };

        // 2. Generate Zero-VDOM JS
        let generator = crate::codegen::CodeGenerator {
            security_sandbox_enabled: self.secure_mode,
        };

        match generator.generate(&ast) {
            Ok(js) => js,
            Err(_) => String::from("console.error('WASM CodeGen Error');"),
        }
    }
}

//! âš™ï¸ Zenvu Secure Compiler
//! Prevents Prototype Pollution and locks Global Objects in the output bundle.

pub fn wrap_safe_output(compiled_js: &str) -> String {
    tracing::info!("âš™ï¸ [Zenvu Secure Compiler] Freezing Object prototypes in JS bundle...");
    
    let secure_wrapper = format!(
        r#"
// Zenvu.js Secure Compiler Wrapper
(function() {{
    Object.freeze(Object.prototype);
    Object.freeze(Array.prototype);
    Object.freeze(Function.prototype);
    
    // --- Application Code ---
    {}
}})();
"#,
        compiled_js
    );

    secure_wrapper
}

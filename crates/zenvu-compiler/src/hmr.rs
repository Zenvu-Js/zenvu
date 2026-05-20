//! HMR (Hot Module Replacement) Engine
//!
//! Replaces modules in the browser instantly without a full page reload.

pub fn generate_hmr_injection(module_id: &str) -> String {
    format!(
        r#"
        if (import.meta.hot) {{
            import.meta.hot.accept('{}', (newModule) => {{
                console.log('[Zenvu HMR] Hot replacing {}');
                window.__zenvu_HMR_RUNTIME__.patch(newModule);
            }});
        }}
        "#,
        module_id, module_id
    )
}

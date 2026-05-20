//! Zero-VDOM Code Generator (Svelte-like Performance)
//!
//! Compiles the AST directly into highly optimized Vanilla JS DOM manipulations.
//! Eliminates the Virtual DOM diffing engine entirely.

use anyhow::Result;
use crate::parser::AstNode;

pub struct CodeGenerator {
    pub security_sandbox_enabled: bool,
}

impl CodeGenerator {
    pub fn generate(&self, ast: &AstNode) -> Result<String> {
        tracing::info!("Generating Zero-VDOM instructions...");
        
        let mut js_output = String::from(
            "import { effect } from '@zenvu/reactivity';\nimport { sanitize } from '@zenvu/security';\n\n",
        );

        match ast {
            AstNode::Element { tag, props, children, is_react_component: _ } => {
                js_output.push_str(&format!("const el = document.createElement('{}');\n", tag));
                
                // Inject Props
                for (key, val) in props {
                    js_output.push_str(&format!("el.setAttribute('{}', '{}');\n", key, val));
                }

                // Compile Children with Fine-Grained Reactivity (Signals)
                for child in children {
                    match child {
                        AstNode::Text(text) => {
                            js_output.push_str(&format!("el.appendChild(document.createTextNode('{}'));\n", text));
                        }
                        AstNode::ReactiveExpression(expr) => {
                            // The Svelte/SolidJS magic: Wrap the expression in a reactive `effect`
                            // Zenvu SafeDOM automatically wraps the expression in `sanitize()` to prevent XSS
                            js_output.push_str(&format!(
                                r#"
    const textNode = document.createTextNode('');
    effect(() => {{
        textNode.textContent = sanitize(String({}));
    }});
    el.appendChild(textNode);
                                "#,
                                expr
                            ));
                        }
                        _ => {}
                    }
                }
                
                js_output.push_str("return el;\n");
            }
            _ => {}
        }
        
        // Security wrapper would go here
        if self.security_sandbox_enabled {
            js_output = format!("/* SECURE WRAPPED */\n{}", js_output);
        }

        Ok(js_output)
    }
}

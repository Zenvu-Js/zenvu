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
            "import { effect, ref, reactive, computed, zModel, zIf, zFor } from '@zenvu/core';\nimport { sanitize } from '@zenvu/security';\n\n",
        );

        let mut var_counter = 0;
        let mut body = String::new();
        let root_var = self.compile_node(ast, &mut body, &mut var_counter)?;

        js_output.push_str("export default function render(state) {\n");
        js_output.push_str(&body);
        js_output.push_str(&format!("return {};\n", root_var));
        js_output.push_str("}\n");

        if self.security_sandbox_enabled {
            js_output = format!("/* SECURE WRAPPED */\n{}", js_output);
        }

        Ok(js_output)
    }

    fn compile_node(&self, node: &AstNode, body: &mut String, var_counter: &mut usize) -> Result<String> {
        let var_name = format!("el{}", var_counter);
        *var_counter += 1;

        match node {
            AstNode::Element { tag, props, directives, children, is_react_component: _ } => {
                // 1. Check for structural directives like z-if first!
                if let Some(if_dir) = directives.iter().find(|d| d.kind == "if") {
                    body.push_str(&format!("const {} = zIf(() => {}, () => {{\n", var_name, if_dir.expression));
                    let mut sub_directives = directives.clone();
                    sub_directives.retain(|d| d.kind != "if");
                    let sub_node = AstNode::Element {
                        tag: tag.clone(),
                        props: props.clone(),
                        directives: sub_directives,
                        children: children.clone(),
                        is_react_component: false,
                    };
                    let mut sub_body = String::new();
                    let sub_var = self.compile_node(&sub_node, &mut sub_body, var_counter)?;
                    body.push_str(&sub_body);
                    body.push_str(&format!("return {};\n", sub_var));
                    body.push_str("});\n");
                    return Ok(var_name);
                }

                // 2. Check for structural directives like z-for next!
                if let Some(for_dir) = directives.iter().find(|d| d.kind == "for") {
                    let parts: Vec<&str> = for_dir.expression.split(" in ").collect();
                    if parts.len() == 2 {
                        let item_def = parts[0].trim();
                        let list_expr = parts[1].trim();
                        
                        body.push_str(&format!("const {} = zFor(() => {}, ({}) => {{\n", var_name, list_expr, item_def));
                        let mut sub_directives = directives.clone();
                        sub_directives.retain(|d| d.kind != "for");
                        let sub_node = AstNode::Element {
                            tag: tag.clone(),
                            props: props.clone(),
                            directives: sub_directives,
                            children: children.clone(),
                            is_react_component: false,
                        };
                        let mut sub_body = String::new();
                        let sub_var = self.compile_node(&sub_node, &mut sub_body, var_counter)?;
                        body.push_str(&sub_body);
                        body.push_str(&format!("return {};\n", sub_var));
                        body.push_str("});\n");
                        return Ok(var_name);
                    }
                }

                // 3. Normal Element compilation
                body.push_str(&format!("const {} = document.createElement('{}');\n", var_name, tag));

                // Attributes
                for (key, val) in props {
                    body.push_str(&format!("{}.setAttribute('{}', '{}');\n", var_name, key, val));
                }

                // Directives (z-model, @click, etc.)
                for dir in directives {
                    match dir.kind.as_str() {
                        "model" => {
                            body.push_str(&format!("zModel({}, state, '{}');\n", var_name, dir.expression));
                        }
                        "on" => {
                            if let Some(event_name) = &dir.argument {
                                body.push_str(&format!("{}.addEventListener('{}', (e) => {});\n", var_name, event_name, dir.expression));
                            }
                        }
                        "bind" => {
                            if let Some(attr_name) = &dir.argument {
                                body.push_str(&format!(
                                    r#"
    effect(() => {{
        {}.setAttribute('{}', String({}));
    }});
                                    "#,
                                    var_name, attr_name, dir.expression
                                ));
                            }
                        }
                        _ => {}
                    }
                }

                // Compile Children
                for child in children {
                    let child_var = self.compile_node(child, body, var_counter)?;
                    body.push_str(&format!("{}.appendChild({});\n", var_name, child_var));
                }
            }
            AstNode::Text(text) => {
                body.push_str(&format!("const {} = document.createTextNode('{}');\n", var_name, text));
            }
            AstNode::ReactiveExpression(expr) => {
                body.push_str(&format!(
                    r#"
    const {} = document.createTextNode('');
    effect(() => {{
        {}.textContent = sanitize(String({}));
    }});
                    "#,
                    var_name, var_name, expr
                ));
            }
        }

        Ok(var_name)
    }
}

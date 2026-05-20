//! Reactive analysis for Zenvu.js components.
//!
//! Builds dependency graphs between reactive variables and DOM nodes,
//! enabling surgical compile-time update generation.

use anyhow::Result;
use std::collections::{HashMap, HashSet};
use crate::ast::*;
use crate::CompileOptions;

/// Analysis result containing all reactive binding information.
#[derive(Debug, Clone)]
pub struct AnalysisResult {
    /// Map of variable name â†’ set of variables it depends on.
    pub dependency_graph: HashMap<String, HashSet<String>>,
    /// Map of variable name â†’ DOM node IDs that read it.
    pub reactive_bindings: HashMap<String, Vec<DomBinding>>,
    /// All reactive variable names (declared with `let`).
    pub reactive_vars: HashSet<String>,
    /// All derived variable names (declared with `const` that reference reactive vars).
    pub derived_vars: HashMap<String, Vec<String>>,
    /// Event handler mappings.
    pub event_handlers: Vec<EventBinding>,
    /// Two-way bindings (b-model).
    pub model_bindings: Vec<ModelBinding>,
    /// Component scope ID for CSS scoping.
    pub scope_id: String,
}

/// A binding between a reactive variable and a DOM location.
#[derive(Debug, Clone)]
pub struct DomBinding {
    /// Unique ID for the DOM node.
    pub node_id: String,
    /// The type of binding.
    pub kind: BindingKind,
    /// The expression to evaluate.
    pub expression: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BindingKind {
    TextContent,
    Attribute(String),
    InnerHtml,
    Visibility,
    ClassToggle(String),
    StyleProp(String),
}

/// An event handler binding.
#[derive(Debug, Clone)]
pub struct EventBinding {
    pub node_id: String,
    pub event: String,
    pub handler: String,
    pub modifiers: Vec<String>,
}

/// A two-way model binding.
#[derive(Debug, Clone)]
pub struct ModelBinding {
    pub node_id: String,
    pub variable: String,
    pub event: String,
    pub prop: String,
}

/// Analyze a component AST to extract reactive dependencies.
pub fn analyze(ast: &ComponentAst, options: &CompileOptions) -> Result<AnalysisResult> {
    let scope_id = options.scope_id.clone().unwrap_or_else(|| {
        format!("zenvu-{}", hash_name(&ast.name))
    });

    let mut result = AnalysisResult {
        dependency_graph: HashMap::new(),
        reactive_bindings: HashMap::new(),
        reactive_vars: HashSet::new(),
        derived_vars: HashMap::new(),
        event_handlers: Vec::new(),
        model_bindings: Vec::new(),
        scope_id,
    };

    // Phase 1: Collect reactive and derived variables from script
    if let Some(script) = &ast.script {
        for decl in &script.declarations {
            match decl {
                Declaration::ReactiveVar { name, .. } => {
                    result.reactive_vars.insert(name.clone());
                }
                Declaration::DerivedVar { name, dependencies, .. } => {
                    let reactive_deps: Vec<String> = dependencies.iter()
                        .filter(|d| result.reactive_vars.contains(*d))
                        .cloned()
                        .collect();
                    if !reactive_deps.is_empty() {
                        result.derived_vars.insert(name.clone(), reactive_deps.clone());
                        result.dependency_graph.insert(name.clone(), reactive_deps.into_iter().collect());
                    }
                }
                _ => {}
            }
        }
    }

    // Phase 2: Walk template to find reactive bindings
    if let Some(template) = &ast.template {
        let mut node_counter = 0;
        for child in &template.children {
            analyze_template_node(child, &mut result, &mut node_counter);
        }
    }

    Ok(result)
}

fn analyze_template_node(node: &TemplateNode, result: &mut AnalysisResult, counter: &mut usize) {
    match node {
        TemplateNode::Interpolation(interp) => {
            let node_id = format!("_txt{}", counter);
            *counter += 1;
            for dep in &interp.dependencies {
                if result.reactive_vars.contains(dep) || result.derived_vars.contains_key(dep) {
                    result.reactive_bindings
                        .entry(dep.clone())
                        .or_default()
                        .push(DomBinding {
                            node_id: node_id.clone(),
                            kind: BindingKind::TextContent,
                            expression: interp.expression.clone(),
                        });
                }
            }
        }
        TemplateNode::Element(el) => {
            let node_id = format!("_el{}", counter);
            *counter += 1;

            // Analyze directives
            for directive in &el.directives {
                match &directive.kind {
                    DirectiveKind::Bind => {
                        if let (Some(attr), Some(expr)) = (&directive.argument, &directive.expression) {
                            let deps = extract_reactive_deps(expr, result);
                            for dep in deps {
                                result.reactive_bindings
                                    .entry(dep)
                                    .or_default()
                                    .push(DomBinding {
                                        node_id: node_id.clone(),
                                        kind: BindingKind::Attribute(attr.clone()),
                                        expression: expr.clone(),
                                    });
                            }
                        }
                    }
                    DirectiveKind::On => {
                        if let (Some(event), Some(handler)) = (&directive.argument, &directive.expression) {
                            result.event_handlers.push(EventBinding {
                                node_id: node_id.clone(),
                                event: event.clone(),
                                handler: handler.clone(),
                                modifiers: directive.modifiers.clone(),
                            });
                        }
                    }
                    DirectiveKind::Model => {
                        if let Some(var) = &directive.expression {
                            result.model_bindings.push(ModelBinding {
                                node_id: node_id.clone(),
                                variable: var.clone(),
                                event: "input".to_string(),
                                prop: "value".to_string(),
                            });
                        }
                    }
                    DirectiveKind::Show => {
                        if let Some(expr) = &directive.expression {
                            let deps = extract_reactive_deps(expr, result);
                            for dep in deps {
                                result.reactive_bindings
                                    .entry(dep)
                                    .or_default()
                                    .push(DomBinding {
                                        node_id: node_id.clone(),
                                        kind: BindingKind::Visibility,
                                        expression: expr.clone(),
                                    });
                            }
                        }
                    }
                    _ => {}
                }
            }

            // Recurse into children
            for child in &el.children {
                analyze_template_node(child, result, counter);
            }
        }
        _ => {}
    }
}

fn extract_reactive_deps(expr: &str, result: &AnalysisResult) -> Vec<String> {
    let mut deps = Vec::new();
    let mut current = String::new();
    for ch in expr.chars() {
        if ch.is_alphanumeric() || ch == '_' {
            current.push(ch);
        } else {
            if !current.is_empty() {
                if result.reactive_vars.contains(&current) || result.derived_vars.contains_key(&current) {
                    deps.push(current.clone());
                }
            }
            current.clear();
        }
    }
    if !current.is_empty() && (result.reactive_vars.contains(&current) || result.derived_vars.contains_key(&current)) {
        deps.push(current);
    }
    deps
}

fn hash_name(name: &str) -> String {
    let mut hash: u32 = 5381;
    for byte in name.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u32);
    }
    format!("{:x}", hash & 0xFFFFFF)
}

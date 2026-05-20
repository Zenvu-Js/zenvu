//! Intermediate Representation (IR) for Zenvu.js code generation.
//!
//! The IR bridges the gap between the high-level AST and the final
//! JavaScript output, providing a target-agnostic optimization layer.

use anyhow::Result;
use crate::ast::*;
use crate::analyzer::AnalysisResult;
use crate::CompileOptions;

/// The intermediate representation of a compiled component.
#[derive(Debug, Clone)]
pub struct ComponentIR {
    pub name: String,
    pub scope_id: String,
    pub create_ops: Vec<IRNode>,
    pub mount_ops: Vec<MountOp>,
    pub update_fns: Vec<UpdateFn>,
    pub event_setups: Vec<EventSetup>,
    pub destroy_ops: Vec<DestroyOp>,
    pub css: Option<String>,
    pub imports: Vec<IRImport>,
    pub props: Vec<IRProp>,
    pub lifecycle: Vec<LifecycleOp>,
}

#[derive(Debug, Clone)]
pub enum IRNode {
    CreateElement { id: String, tag: String },
    CreateText { id: String, content: IRValue },
    CreateComponent { id: String, name: String, props: Vec<(String, IRValue)> },
    SetAttribute { target: String, attr: String, value: IRValue },
    AddClass { target: String, scope_id: String },
    AppendChild { parent: String, child: String },
}

#[derive(Debug, Clone)]
pub enum IRValue {
    Static(String),
    Dynamic(String),       // Expression referencing reactive state
    Interpolated(Vec<IRValuePart>),
}

#[derive(Debug, Clone)]
pub enum IRValuePart {
    Literal(String),
    Expression(String),
}

#[derive(Debug, Clone)]
pub struct MountOp {
    pub node_id: String,
    pub target: String,   // "target" for root, parent id for children
}

#[derive(Debug, Clone)]
pub struct UpdateFn {
    pub id: String,
    pub triggers: Vec<String>,  // Reactive vars that trigger this update
    pub ops: Vec<UpdateOp>,
}

#[derive(Debug, Clone)]
pub enum UpdateOp {
    SetText { node_id: String, expression: String },
    SetAttribute { node_id: String, attr: String, expression: String },
    SetVisibility { node_id: String, expression: String },
    SetProp { component_id: String, prop: String, expression: String },
    UpdateDerived { name: String, expression: String },
}

#[derive(Debug, Clone)]
pub struct EventSetup {
    pub node_id: String,
    pub event: String,
    pub handler: String,
    pub modifiers: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum DestroyOp {
    RemoveNode(String),
    RemoveListener { node_id: String, event: String },
    DestroyComponent(String),
}

#[derive(Debug, Clone)]
pub struct IRImport {
    pub source: String,
    pub specifiers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct IRProp {
    pub name: String,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LifecycleOp {
    pub kind: LifecycleKind,
    pub body: String,
}

/// Lower a ComponentAst + AnalysisResult into IR.
pub fn lower(ast: &ComponentAst, analysis: &AnalysisResult, _options: &CompileOptions) -> Result<ComponentIR> {
    let mut ir = ComponentIR {
        name: ast.name.clone(),
        scope_id: analysis.scope_id.clone(),
        create_ops: Vec::new(),
        mount_ops: Vec::new(),
        update_fns: Vec::new(),
        event_setups: Vec::new(),
        destroy_ops: Vec::new(),
        css: ast.style.as_ref().map(|s| s.content.clone()),
        imports: Vec::new(),
        props: Vec::new(),
        lifecycle: Vec::new(),
    };

    // Lower imports
    if let Some(script) = &ast.script {
        for import in &script.imports {
            ir.imports.push(IRImport {
                source: import.source.clone(),
                specifiers: import.specifiers.iter().map(|s| match s {
                    ImportSpecifier::Default(n) => n.clone(),
                    ImportSpecifier::Named { local, .. } => local.clone(),
                    ImportSpecifier::Namespace(n) => n.clone(),
                }).collect(),
            });
        }

        for prop in &script.props {
            ir.props.push(IRProp {
                name: prop.name.clone(),
                default_value: prop.default_value.clone(),
            });
        }

        // Lower lifecycle hooks
        for decl in &script.declarations {
            if let Declaration::LifecycleHook { hook, body, .. } = decl {
                ir.lifecycle.push(LifecycleOp { kind: hook.clone(), body: body.clone() });
            }
        }
    }

    // Lower template nodes into create/mount operations
    if let Some(template) = &ast.template {
        let mut counter = 0;
        for child in &template.children {
            lower_template_node(child, &mut ir, &mut counter, None, analysis);
        }
    }

    // Build update functions from reactive bindings
    for (var, bindings) in &analysis.reactive_bindings {
        let ops: Vec<UpdateOp> = bindings.iter().map(|b| {
            match &b.kind {
                crate::analyzer::BindingKind::TextContent => {
                    UpdateOp::SetText { node_id: b.node_id.clone(), expression: b.expression.clone() }
                }
                crate::analyzer::BindingKind::Attribute(attr) => {
                    UpdateOp::SetAttribute { node_id: b.node_id.clone(), attr: attr.clone(), expression: b.expression.clone() }
                }
                crate::analyzer::BindingKind::Visibility => {
                    UpdateOp::SetVisibility { node_id: b.node_id.clone(), expression: b.expression.clone() }
                }
                _ => UpdateOp::SetText { node_id: b.node_id.clone(), expression: b.expression.clone() }
            }
        }).collect();

        // Also update derived variables that depend on this var
        let mut all_ops = Vec::new();
        for (derived_name, deps) in &analysis.derived_vars {
            if deps.contains(var) {
                if let Some(script) = &ast.script {
                    for decl in &script.declarations {
                        if let Declaration::DerivedVar { name, expression, .. } = decl {
                            if name == derived_name {
                                all_ops.push(UpdateOp::UpdateDerived {
                                    name: name.clone(),
                                    expression: expression.clone(),
                                });
                            }
                        }
                    }
                }
            }
        }
        all_ops.extend(ops);

        if !all_ops.is_empty() {
            ir.update_fns.push(UpdateFn {
                id: format!("_update_{}", var),
                triggers: vec![var.clone()],
                ops: all_ops,
            });
        }
    }

    // Lower event handlers
    for eh in &analysis.event_handlers {
        ir.event_setups.push(EventSetup {
            node_id: eh.node_id.clone(),
            event: eh.event.clone(),
            handler: eh.handler.clone(),
            modifiers: eh.modifiers.clone(),
        });
    }

    Ok(ir)
}

fn lower_template_node(
    node: &TemplateNode, ir: &mut ComponentIR, counter: &mut usize,
    parent_id: Option<&str>, _analysis: &AnalysisResult,
) {
    match node {
        TemplateNode::Element(el) => {
            let id = format!("_el{}", counter);
            *counter += 1;

            if el.is_component {
                ir.create_ops.push(IRNode::CreateComponent {
                    id: id.clone(), name: el.tag.clone(), props: vec![],
                });
            } else {
                ir.create_ops.push(IRNode::CreateElement { id: id.clone(), tag: el.tag.clone() });
                ir.create_ops.push(IRNode::AddClass { target: id.clone(), scope_id: ir.scope_id.clone() });
            }

            for attr in &el.attributes {
                ir.create_ops.push(IRNode::SetAttribute {
                    target: id.clone(), attr: attr.name.clone(),
                    value: IRValue::Static(attr.value.clone().unwrap_or_default()),
                });
            }

            if let Some(pid) = parent_id {
                ir.mount_ops.push(MountOp { node_id: id.clone(), target: pid.to_string() });
            } else {
                ir.mount_ops.push(MountOp { node_id: id.clone(), target: "target".to_string() });
            }
            ir.destroy_ops.push(DestroyOp::RemoveNode(id.clone()));

            for child in &el.children {
                lower_template_node(child, ir, counter, Some(&id), _analysis);
            }
        }
        TemplateNode::Text(text) => {
            let id = format!("_txt{}", counter);
            *counter += 1;
            ir.create_ops.push(IRNode::CreateText { id: id.clone(), content: IRValue::Static(text.content.clone()) });
            if let Some(pid) = parent_id {
                ir.mount_ops.push(MountOp { node_id: id.clone(), target: pid.to_string() });
            }
        }
        TemplateNode::Interpolation(interp) => {
            let id = format!("_txt{}", counter);
            *counter += 1;
            ir.create_ops.push(IRNode::CreateText { id: id.clone(), content: IRValue::Dynamic(interp.expression.clone()) });
            if let Some(pid) = parent_id {
                ir.mount_ops.push(MountOp { node_id: id.clone(), target: pid.to_string() });
            }
        }
        _ => {}
    }
}

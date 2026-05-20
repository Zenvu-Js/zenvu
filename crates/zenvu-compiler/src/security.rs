//! Compiler security analysis module.
//!
//! Performs static security checks during compilation to catch
//! vulnerabilities before they reach production.

use crate::ast::*;
use crate::error::SourceLocation;

/// Security analysis result.
#[derive(Debug, Default)]
pub struct SecurityReport {
    pub warnings: Vec<SecurityWarning>,
    pub errors: Vec<SecurityError>,
}

#[derive(Debug)]
pub struct SecurityWarning {
    pub code: &'static str,
    pub message: String,
    pub location: Option<SourceLocation>,
    pub suggestion: String,
}

#[derive(Debug)]
pub struct SecurityError {
    pub code: &'static str,
    pub message: String,
    pub location: Option<SourceLocation>,
}

/// Run security analysis on a component AST.
pub fn analyze_security(ast: &ComponentAst) -> SecurityReport {
    let mut report = SecurityReport::default();

    // Analyze script block
    if let Some(script) = &ast.script {
        check_script_security(script, &mut report);
    }

    // Analyze template
    if let Some(template) = &ast.template {
        for child in &template.children {
            check_template_security(child, &mut report);
        }
    }

    // Analyze style
    if let Some(style) = &ast.style {
        check_style_security(style, &mut report);
    }

    report
}

fn check_script_security(script: &ScriptBlock, report: &mut SecurityReport) {
    let content = &script.content;

    // S001: eval() usage
    if content.contains("eval(") {
        report.errors.push(SecurityError {
            code: "S001",
            message: "eval() is blocked — it enables code injection attacks".into(),
            location: find_location(content, "eval("),
        });
    }

    // S002: new Function()
    if content.contains("new Function(") || content.contains("new Function (") {
        report.errors.push(SecurityError {
            code: "S002",
            message: "new Function() is blocked — use static expressions instead".into(),
            location: find_location(content, "new Function"),
        });
    }

    // S003: document.write
    if content.contains("document.write") {
        report.warnings.push(SecurityWarning {
            code: "S003",
            message: "document.write() is unsafe and blocked by CSP".into(),
            location: find_location(content, "document.write"),
            suggestion: "Use DOM APIs instead".into(),
        });
    }

    // S004: innerHTML assignment
    if content.contains(".innerHTML") {
        report.warnings.push(SecurityWarning {
            code: "S004",
            message: "innerHTML assignment can lead to XSS".into(),
            location: find_location(content, ".innerHTML"),
            suggestion: "Use textContent or b-html with sanitizeHtml()".into(),
        });
    }

    // S005: Prototype access
    if content.contains("__proto__") || content.contains(".__defineGetter__") {
        report.errors.push(SecurityError {
            code: "S005",
            message: "__proto__ access detected — prototype pollution risk".into(),
            location: find_location(content, "__proto__"),
        });
    }

    // S006: Dynamic import with variable
    for line in content.lines() {
        if line.contains("import(") && !line.contains("import('") && !line.contains("import(\"") {
            report.warnings.push(SecurityWarning {
                code: "S006",
                message: "Dynamic import with non-literal path".into(),
                location: None,
                suggestion: "Use static import paths to prevent code injection".into(),
            });
        }
    }
}

fn check_template_security(node: &TemplateNode, report: &mut SecurityReport) {
    match node {
        TemplateNode::Element(el) => {
            // S010: Check for dangerous elements
            let dangerous_tags = ["iframe", "object", "embed", "form"];
            if dangerous_tags.contains(&el.tag.to_lowercase().as_str()) {
                report.warnings.push(SecurityWarning {
                    code: "S010",
                    message: format!("<{}> element may introduce security risks", el.tag),
                    location: el.span.as_ref().map(|s| s.start.clone()),
                    suggestion: format!("Ensure <{}> is necessary and properly sandboxed", el.tag),
                });
            }

            // S011: Check for b-html without sanitization
            if let Some(directive) = el.get_directive(&DirectiveKind::Html) {
                if let Some(expr) = &directive.expression {
                    if !expr.contains("sanitize") && !expr.contains("escape") {
                        report.warnings.push(SecurityWarning {
                            code: "S011",
                            message: "b-html used without sanitization — XSS risk".into(),
                            location: directive.span.as_ref().map(|s| s.start.clone()),
                            suggestion: "Wrap with sanitizeHtml(): b-html=\"sanitizeHtml(data)\"".into(),
                        });
                    }
                }
            }

            // S012: Check for javascript: URLs in bindings
            for directive in &el.directives {
                if directive.kind == DirectiveKind::Bind {
                    if let Some(expr) = &directive.expression {
                        if expr.to_lowercase().contains("javascript:") {
                            report.errors.push(SecurityError {
                                code: "S012",
                                message: "javascript: URL in binding — blocked".into(),
                                location: directive.span.as_ref().map(|s| s.start.clone()),
                            });
                        }
                    }
                }
            }

            // S013: Check for event handlers with string expressions
            for directive in &el.directives {
                if directive.kind == DirectiveKind::On {
                    if let Some(expr) = &directive.expression {
                        // Warn if handler looks like it could be injection
                        if expr.contains("eval") || expr.contains("Function") {
                            report.errors.push(SecurityError {
                                code: "S013",
                                message: "Unsafe expression in event handler".into(),
                                location: directive.span.as_ref().map(|s| s.start.clone()),
                            });
                        }
                    }
                }
            }

            // Recurse into children
            for child in &el.children {
                check_template_security(child, report);
            }
        }
        _ => {}
    }
}

fn check_style_security(style: &StyleBlock, report: &mut SecurityReport) {
    let content = &style.content.to_lowercase();

    // S020: CSS expression injection
    if content.contains("expression(") {
        report.errors.push(SecurityError {
            code: "S020",
            message: "CSS expression() detected — code execution risk".into(),
            location: None,
        });
    }

    // S021: External URL in CSS
    if content.contains("url(") && (content.contains("javascript:") || content.contains("data:text/html")) {
        report.errors.push(SecurityError {
            code: "S021",
            message: "Dangerous URL scheme in CSS url()".into(),
            location: None,
        });
    }

    // S022: @import from external domain
    if content.contains("@import") && content.contains("http") {
        report.warnings.push(SecurityWarning {
            code: "S022",
            message: "External @import may be blocked by CSP".into(),
            location: None,
            suggestion: "Use local CSS imports or add the domain to CSP style-src".into(),
        });
    }
}

fn find_location(content: &str, pattern: &str) -> Option<SourceLocation> {
    if let Some(offset) = content.find(pattern) {
        let line = content[..offset].matches('\n').count() + 1;
        let last_newline = content[..offset].rfind('\n').map(|i| i + 1).unwrap_or(0);
        let column = offset - last_newline + 1;
        Some(SourceLocation { line, column, offset })
    } else {
        None
    }
}

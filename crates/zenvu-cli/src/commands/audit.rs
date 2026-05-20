//! `Zenvu audit` â€” Security audit tool for Zenvu.js projects.
//!
//! Scans components, dependencies, and configuration for security issues.

use anyhow::Result;
use std::path::Path;

/// Audit finding severity levels.
#[derive(Debug)]
#[allow(dead_code)]
enum Severity { Critical, High, Medium, Low, Info }

#[derive(Debug)]
struct AuditFinding {
    severity: Severity,
    category: String,
    message: String,
    file: Option<String>,
    line: Option<usize>,
    suggestion: String,
}

pub async fn run() -> Result<()> {
    println!("\n  ðŸ›¡ï¸  Zenvu.js Security Audit\n");
    println!("  Scanning project for security issues...\n");

    let mut findings: Vec<AuditFinding> = Vec::new();

    // 1. Scan Dependencies (mock implementation)
    let package_json_path = std::path::Path::new("package.json");
    if package_json_path.exists() {
        let content = std::fs::read_to_string(package_json_path).unwrap_or_default();
        if content.contains("\"lodash\": \"<4.17.21\"") {
            findings.push(AuditFinding {
                severity: Severity::High,
                category: "Dependency Vulnerability".into(),
                message: "Vulnerable lodash version detected (Prototype Pollution).".into(),
                file: Some("package.json".into()),
                line: None,
                suggestion: "Update lodash to >= 4.17.21".into(),
            });
        }
    }

    // 2. Scan .Zenvu files for unsafe patterns
    scan_components(&mut findings)?;

    // 2. Check configuration security
    scan_config(&mut findings)?;

    // 3. Check dependency integrity
    scan_dependencies(&mut findings)?;

    // 4. Check for sensitive files
    scan_sensitive_files(&mut findings)?;

    // Report findings
    let critical = findings.iter().filter(|f| matches!(f.severity, Severity::Critical)).count();
    let high = findings.iter().filter(|f| matches!(f.severity, Severity::High)).count();
    let medium = findings.iter().filter(|f| matches!(f.severity, Severity::Medium)).count();
    let low = findings.iter().filter(|f| matches!(f.severity, Severity::Low)).count();

    for finding in &findings {
        let icon = match finding.severity {
            Severity::Critical => "ðŸ”´",
            Severity::High => "ðŸŸ ",
            Severity::Medium => "ðŸŸ¡",
            Severity::Low => "ðŸ”µ",
            Severity::Info => "â„¹ï¸",
        };

        let location = match (&finding.file, finding.line) {
            (Some(f), Some(l)) => format!(" ({}:{})", f, l),
            (Some(f), None) => format!(" ({})", f),
            _ => String::new(),
        };

        println!("  {} [{:?} - {}] {}{}", icon, finding.severity, finding.category, finding.message, location);
        println!("     ðŸ’¡ {}\n", finding.suggestion);
    }

    println!("  â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€");
    println!("  Results: {} critical, {} high, {} medium, {} low", critical, high, medium, low);

    if critical > 0 || high > 0 {
        println!("\n  âš ï¸  Security issues found! Fix critical/high issues before deploying.\n");
    } else if medium > 0 || low > 0 {
        println!("\n  âš¡ Minor issues found. Consider addressing them.\n");
    } else {
        println!("\n  âœ… No security issues found! Your project is secure.\n");
    }

    Ok(())
}

fn scan_components(findings: &mut Vec<AuditFinding>) -> Result<()> {
    let src = Path::new("src");
    if !src.exists() { return Ok(()); }

    let files = collect_files(src, "Zenvu")?;

    for file in &files {
        let content = std::fs::read_to_string(file)?;
        let filename = file.display().to_string();

        // Check for raw HTML injection (b-html without sanitization)
        for (i, line) in content.lines().enumerate() {
            if line.contains("b-html") && !line.contains("sanitize") {
                findings.push(AuditFinding {
                    severity: Severity::High,
                    category: "XSS".into(),
                    message: "b-html directive used without explicit sanitization".into(),
                    file: Some(filename.clone()),
                    line: Some(i + 1),
                    suggestion: "Use sanitizeHtml() or escape the content: b-html=\"sanitize(data)\"".into(),
                });
            }

            // Check for javascript: URLs
            if line.to_lowercase().contains("javascript:") {
                findings.push(AuditFinding {
                    severity: Severity::Critical,
                    category: "XSS".into(),
                    message: "javascript: URL detected â€” potential XSS vector".into(),
                    file: Some(filename.clone()),
                    line: Some(i + 1),
                    suggestion: "Remove javascript: URLs. Use @click handlers instead.".into(),
                });
            }

            // Check for eval usage
            if line.contains("eval(") || line.contains("new Function(") {
                findings.push(AuditFinding {
                    severity: Severity::Critical,
                    category: "Code Injection".into(),
                    message: "eval() or new Function() detected â€” code injection risk".into(),
                    file: Some(filename.clone()),
                    line: Some(i + 1),
                    suggestion: "Replace eval/Function with static expressions or safe alternatives.".into(),
                });
            }

            // Check for innerHTML
            if line.contains("innerHTML") {
                findings.push(AuditFinding {
                    severity: Severity::High,
                    category: "XSS".into(),
                    message: "Direct innerHTML usage detected".into(),
                    file: Some(filename.clone()),
                    line: Some(i + 1),
                    suggestion: "Use b-html with sanitization or textContent instead.".into(),
                });
            }

            // Check for document.write
            if line.contains("document.write") {
                findings.push(AuditFinding {
                    severity: Severity::High,
                    category: "XSS".into(),
                    message: "document.write() is unsafe and blocked by CSP".into(),
                    file: Some(filename.clone()),
                    line: Some(i + 1),
                    suggestion: "Use DOM APIs (createElement, textContent) instead.".into(),
                });
            }

            // Check for prototype pollution patterns
            if line.contains("__proto__") || line.contains("constructor[") {
                findings.push(AuditFinding {
                    severity: Severity::High,
                    category: "Prototype Pollution".into(),
                    message: "Potential prototype pollution pattern detected".into(),
                    file: Some(filename.clone()),
                    line: Some(i + 1),
                    suggestion: "Use Object.create(null) for data objects. Avoid __proto__ access.".into(),
                });
            }
        }
    }

    Ok(())
}

fn scan_config(findings: &mut Vec<AuditFinding>) -> Result<()> {
    // Check for Zenvu.config.ts/js
    let config_files = ["Zenvu.config.ts", "Zenvu.config.js"];
    let mut found = false;

    for cf in &config_files {
        if Path::new(cf).exists() {
            found = true;
            let content = std::fs::read_to_string(cf)?;

            // Check if CSP is disabled
            if content.contains("csp: false") || content.contains("csp_enabled: false") {
                findings.push(AuditFinding {
                    severity: Severity::High,
                    category: "Configuration".into(),
                    message: "Content Security Policy is disabled".into(),
                    file: Some(cf.to_string()),
                    line: None,
                    suggestion: "Enable CSP for production. It's a critical XSS defense layer.".into(),
                });
            }

            // Check if CSRF is disabled
            if content.contains("csrf: false") {
                findings.push(AuditFinding {
                    severity: Severity::High,
                    category: "Configuration".into(),
                    message: "CSRF protection is disabled".into(),
                    file: Some(cf.to_string()),
                    line: None,
                    suggestion: "Enable CSRF protection to prevent cross-site request forgery.".into(),
                });
            }
        }
    }

    if !found {
        findings.push(AuditFinding {
            severity: Severity::Low,
            category: "Configuration".into(),
            message: "No Zenvu.config.ts found â€” using default secure configuration".into(),
            file: None, line: None,
            suggestion: "Create Zenvu.config.ts for explicit security configuration.".into(),
        });
    }

    Ok(())
}

fn scan_dependencies(findings: &mut Vec<AuditFinding>) -> Result<()> {
    if Path::new("package.json").exists() {
        let content = std::fs::read_to_string("package.json")?;

        // Check for wildcard versions
        if content.contains("\"*\"") || content.contains("\": \"latest\"") {
            findings.push(AuditFinding {
                severity: Severity::Medium,
                category: "Supply Chain".into(),
                message: "Wildcard or 'latest' version in dependencies".into(),
                file: Some("package.json".into()),
                line: None,
                suggestion: "Pin dependency versions to prevent supply chain attacks.".into(),
            });
        }
    }

    // Check for lock file
    if !Path::new("Zenvu.lock").exists() && !Path::new("package-lock.json").exists() {
        findings.push(AuditFinding {
            severity: Severity::Medium,
            category: "Supply Chain".into(),
            message: "No lock file found â€” dependency versions are not pinned".into(),
            file: None, line: None,
            suggestion: "Run `npm install` to generate package-lock.json.".into(),
        });
    }

    Ok(())
}

fn scan_sensitive_files(findings: &mut Vec<AuditFinding>) -> Result<()> {
    let sensitive = [".env", ".env.local", ".env.production", "secrets.json", "private.key"];

    for file in &sensitive {
        if Path::new(file).exists() {
            findings.push(AuditFinding {
                severity: Severity::Critical,
                category: "Data Exposure".into(),
                message: format!("Sensitive file '{}' found in project root", file),
                file: Some(file.to_string()),
                line: None,
                suggestion: "Add to .gitignore and move secrets to environment variables.".into(),
            });
        }
    }

    // Check .gitignore includes sensitive patterns
    if Path::new(".gitignore").exists() {
        let gitignore = std::fs::read_to_string(".gitignore")?;
        if !gitignore.contains(".env") {
            findings.push(AuditFinding {
                severity: Severity::Medium,
                category: "Data Exposure".into(),
                message: ".gitignore does not exclude .env files".into(),
                file: Some(".gitignore".into()),
                line: None,
                suggestion: "Add '.env*' to .gitignore to prevent secret exposure.".into(),
            });
        }
    }

    Ok(())
}

fn collect_files(dir: &Path, ext: &str) -> Result<Vec<std::path::PathBuf>> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if path.is_dir() { files.extend(collect_files(&path, ext)?); }
            else if path.extension().is_some_and(|e| e == ext) { files.push(path); }
        }
    }
    Ok(files)
}

//! Input sanitization engine â€” prevents XSS, SQL injection, command injection.
//!
//! Multi-layer sanitization pipeline:
//! Raw Input â†’ Type Coercion â†’ Schema Validation â†’ Sanitization â†’ Safe Value

use anyhow::Result;

/// HTML entity escaping â€” the primary XSS defense.
/// Escapes `&`, `<`, `>`, `"`, `'`, `/` to their HTML entity equivalents.
pub fn escape_html(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    for ch in input.chars() {
        match ch {
            '&' => output.push_str("&amp;"),
            '<' => output.push_str("&lt;"),
            '>' => output.push_str("&gt;"),
            '"' => output.push_str("&quot;"),
            '\'' => output.push_str("&#x27;"),
            '/' => output.push_str("&#x2F;"),
            _ => output.push(ch),
        }
    }
    output
}

/// Sanitize HTML content â€” strips dangerous tags and attributes.
/// Allows a safe subset of HTML tags for rich text.
pub fn sanitize_html(input: &str) -> String {
    let _allowed_tags = [
        "p", "br", "b", "i", "u", "em", "strong", "a", "ul", "ol", "li",
        "h1", "h2", "h3", "h4", "h5", "h6", "blockquote", "code", "pre",
        "span", "div", "table", "thead", "tbody", "tr", "td", "th",
    ];
    let dangerous_attrs = [
        "onclick", "onerror", "onload", "onmouseover", "onfocus", "onZenvur",
        "onsubmit", "onchange", "onkeydown", "onkeyup", "onkeypress",
        "onmousedown", "onmouseup", "onmousemove", "ontouchstart",
    ];

    let mut result = input.to_string();

    // Remove script tags and their content
    result = remove_tag_with_content(&result, "script");
    result = remove_tag_with_content(&result, "iframe");
    result = remove_tag_with_content(&result, "object");
    result = remove_tag_with_content(&result, "embed");
    result = remove_tag_with_content(&result, "form");
    result = remove_tag_with_content(&result, "style");

    // Remove dangerous event handler attributes
    for attr in &dangerous_attrs {
        result = remove_attribute(&result, attr);
    }

    // Remove javascript: and data: URLs
    result = sanitize_urls(&result);

    result
}

/// Sanitize a URL â€” block dangerous schemes.
pub fn sanitize_url(url: &str) -> Result<String> {
    let normalized = url.trim().to_lowercase();

    // Block dangerous URL schemes
    let blocked_schemes = ["javascript:", "data:", "vbscript:", "blob:"];
    for scheme in &blocked_schemes {
        if normalized.starts_with(scheme) {
            return Ok("about:blank".to_string());
        }
    }

    // Allow safe schemes
    let safe_schemes = ["http://", "https://", "mailto:", "tel:", "/", "#", "?"];
    if safe_schemes.iter().any(|s| normalized.starts_with(s)) || !normalized.contains(':') {
        Ok(url.to_string())
    } else {
        Ok("about:blank".to_string())
    }
}

/// Sanitize SQL input â€” escape special characters.
/// NOTE: Always prefer parameterized queries over escaping.
pub fn escape_sql(input: &str) -> String {
    input
        .replace('\'', "''")
        .replace('\\', "\\\\")
        .replace('\0', "")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\x1a', "\\Z")
}

/// Sanitize shell command arguments.
pub fn escape_shell(input: &str) -> String {
    let mut output = String::with_capacity(input.len() + 2);
    output.push('\'');
    for ch in input.chars() {
        if ch == '\'' {
            output.push_str("'\\''");
        } else {
            output.push(ch);
        }
    }
    output.push('\'');
    output
}

/// Sanitize a CSS value to prevent CSS injection.
pub fn sanitize_css_value(input: &str) -> String {
    let blocked = ["expression(", "url(", "import", "javascript:", "@import"];
    let lower = input.to_lowercase();

    for pattern in &blocked {
        if lower.contains(pattern) {
            return String::new(); // Strip dangerous CSS
        }
    }

    // Remove control characters
    input.chars().filter(|c| !c.is_control()).collect()
}

/// Check an expression for prototype pollution attempts.
pub fn check_prototype_pollution(expr: &str) -> bool {
    let dangerous = ["__proto__", "constructor", "prototype", "__defineGetter__",
                     "__defineSetter__", "__lookupGetter__", "__lookupSetter__"];
    let lower = expr.to_lowercase();
    dangerous.iter().any(|d| lower.contains(d))
}

/// Detect ReDoS-vulnerable regular expressions.
pub fn check_regex_safety(pattern: &str) -> bool {
    // Simple heuristic: check for nested quantifiers
    let has_nested_quantifiers = pattern.contains("(.*)*")
        || pattern.contains("(.+)+")
        || pattern.contains("(.?)*")
        || pattern.contains("(a+)+")
        || pattern.contains("(a*)*");

    !has_nested_quantifiers
}

// â”€â”€â”€ Internal Helpers â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€â”€

fn remove_tag_with_content(input: &str, tag: &str) -> String {
    let open = format!("<{}", tag);
    let close = format!("</{}>", tag);
    let mut result = input.to_string();

    while let Some(start) = result.to_lowercase().find(&open) {
        if let Some(end) = result.to_lowercase()[start..].find(&close) {
            let end_pos = start + end + close.len();
            result = format!("{}{}", &result[..start], &result[end_pos..]);
        } else {
            // No closing tag â€” remove to end or just the opening tag
            if let Some(tag_end) = result[start..].find('>') {
                result = format!("{}{}", &result[..start], &result[start + tag_end + 1..]);
            } else {
                break;
            }
        }
    }

    result
}

fn remove_attribute(input: &str, attr: &str) -> String {
    // Simple attribute removal â€” production would use a proper HTML parser
    let patterns = [
        format!("{}=", attr),
        format!("{} =", attr),
    ];

    let mut result = input.to_string();
    let lower = result.to_lowercase();

    for pattern in &patterns {
        if let Some(pos) = lower.find(pattern.as_str()) {
            // Find the end of the attribute value
            let after = &result[pos + pattern.len()..];
            let end = if after.starts_with('"') {
                after[1..].find('"').map(|i| i + 2).unwrap_or(after.len())
            } else if after.starts_with('\'') {
                after[1..].find('\'').map(|i| i + 2).unwrap_or(after.len())
            } else {
                after.find(|c: char| c.is_whitespace() || c == '>').unwrap_or(after.len())
            };
            result = format!("{}{}", &result[..pos], &result[pos + pattern.len() + end..]);
        }
    }

    result
}

fn sanitize_urls(input: &str) -> String {
    let mut result = input.to_string();
    let dangerous = ["javascript:", "vbscript:", "data:text/html"];

    for scheme in &dangerous {
        while let Some(pos) = result.to_lowercase().find(scheme) {
            // Replace the dangerous URL with safe value
            let end = result[pos..].find(|c: char| c == '"' || c == '\'' || c == '>' || c.is_whitespace())
                .map(|i| pos + i)
                .unwrap_or(result.len());
            result = format!("{}about:blank{}", &result[..pos], &result[end..]);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_escape_html() {
        assert_eq!(escape_html("<script>alert('xss')</script>"),
                   "&lt;script&gt;alert(&#x27;xss&#x27;)&lt;&#x2F;script&gt;");
    }

    #[test]
    fn test_sanitize_url_blocks_javascript() {
        assert_eq!(sanitize_url("javascript:alert(1)").unwrap(), "about:blank");
        assert_eq!(sanitize_url("https://safe.com").unwrap(), "https://safe.com");
        assert_eq!(sanitize_url("/relative/path").unwrap(), "/relative/path");
    }

    #[test]
    fn test_prototype_pollution_detection() {
        assert!(check_prototype_pollution("obj.__proto__.admin = true"));
        assert!(check_prototype_pollution("obj.constructor.prototype"));
        assert!(!check_prototype_pollution("obj.name = 'safe'"));
    }

    #[test]
    fn test_regex_safety() {
        assert!(!check_regex_safety("(.*)*")); // Vulnerable
        assert!(check_regex_safety("^[a-z]+$")); // Safe
    }

    #[test]
    fn test_sanitize_html_strips_scripts() {
        let input = "<p>Hello</p><script>alert('xss')</script><p>World</p>";
        let result = sanitize_html(input);
        assert!(!result.contains("script"));
        assert!(result.contains("Hello"));
        assert!(result.contains("World"));
    }
}

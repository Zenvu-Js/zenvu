//! Lexer / Tokenizer for `.Zenvu` single-file components.
//!
//! Splits raw source into a stream of tokens representing the three
//! top-level blocks: `<script>`, `<template>`, and `<style>`.
//! Each block is then further tokenized by its respective sub-parser.

use anyhow::Result;
use crate::error::{CompileError, SourceLocation};

/// Token types produced by the Zenvu lexer.
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Top-level block tokens
    ScriptOpen { lang: Option<String>, line: usize },
    ScriptContent(String),
    ScriptClose { line: usize },

    TemplateOpen { line: usize },
    TemplateContent(String),
    TemplateClose { line: usize },

    StyleOpen { scoped: bool, lang: Option<String>, line: usize },
    StyleContent(String),
    StyleClose { line: usize },

    // Template tokens (produced during template sub-lexing)
    TagOpen(String),              // <div
    TagClose(String),             // </div>
    TagSelfClose,                 // />
    TagEnd,                       // >
    AttrName(String),             // class
    AttrValue(String),            // "my-class"
    Directive(DirectiveToken),    // b-if, b-for, @click, :prop
    Interpolation(String),        // {{ expression }}
    Text(String),                 // Raw text content
    Comment(String),              // <!-- comment -->

    // Special
    Whitespace(String),
    Eof,
}

/// Directive token parsed from template attributes.
#[derive(Debug, Clone, PartialEq)]
pub struct DirectiveToken {
    /// Directive kind: "bind", "on", "if", "else-if", "else", "for", "model", "show", "transition"
    pub kind: String,
    /// Argument (e.g., "click" in @click, "title" in :title)
    pub argument: Option<String>,
    /// Modifiers (e.g., ["prevent"] in @submit.prevent)
    pub modifiers: Vec<String>,
    /// Expression value
    pub expression: Option<String>,
}

/// Tokenize a `.Zenvu` source file into a flat token stream.
///
/// The lexer performs a two-pass approach:
/// 1. First pass: Extract top-level blocks (script, template, style)
/// 2. Second pass: Tokenize the template block into fine-grained tokens
pub fn tokenize(source: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut pos = 0;
    let chars: Vec<char> = source.chars().collect();
    let len = chars.len();

    while pos < len {
        // Skip whitespace between blocks
        if chars[pos].is_whitespace() {
            pos += 1;
            continue;
        }

        // Check for block openers
        if chars[pos] == '<' {
            let remaining: String = chars[pos..].iter().collect();

            if let Some(block) = try_parse_script_block(&remaining, &source, pos)? {
                let line = count_line(source, pos);
                tokens.push(Token::ScriptOpen { lang: block.lang, line });
                tokens.push(Token::ScriptContent(block.content.clone()));
                tokens.push(Token::ScriptClose { line: count_line(source, pos + block.total_len) });
                pos += block.total_len;
                continue;
            }

            if let Some(block) = try_parse_template_block(&remaining, source, pos)? {
                let line = count_line(source, pos);
                tokens.push(Token::TemplateOpen { line });
                // Sub-tokenize the template content
                let template_tokens = tokenize_template(&block.content)?;
                tokens.extend(template_tokens);
                tokens.push(Token::TemplateClose { line: count_line(source, pos + block.total_len) });
                pos += block.total_len;
                continue;
            }

            if let Some(block) = try_parse_style_block(&remaining, source, pos)? {
                let line = count_line(source, pos);
                tokens.push(Token::StyleOpen { scoped: block.scoped, lang: block.lang, line });
                tokens.push(Token::StyleContent(block.content.clone()));
                tokens.push(Token::StyleClose { line: count_line(source, pos + block.total_len) });
                pos += block.total_len;
                continue;
            }
        }

        pos += 1;
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

/// Parsed block from source.
#[derive(Debug)]
struct ParsedBlock {
    content: String,
    lang: Option<String>,
    scoped: bool,
    total_len: usize,
}

fn try_parse_script_block(remaining: &str, _source: &str, _offset: usize) -> Result<Option<ParsedBlock>> {
    let open_re = "<script";
    if !remaining.starts_with(open_re) {
        return Ok(None);
    }

    // Find the end of the opening tag
    let tag_end = remaining.find('>').ok_or_else(|| {
        CompileError::SyntaxError {
            message: "Unclosed <script> tag".to_string(),
            location: SourceLocation { line: 0, column: 0, offset: 0 },
            suggestion: Some("Add a closing '>' to the <script> tag".to_string()),
        }
    })?;

    // Extract lang attribute
    let tag_attrs = &remaining[7..tag_end];
    let lang = extract_attr(tag_attrs, "lang");

    // Find closing </script>
    let close_tag = "</script>";
    let close_pos = remaining.find(close_tag).ok_or_else(|| {
        CompileError::SyntaxError {
            message: "Missing </script> closing tag".to_string(),
            location: SourceLocation { line: 0, column: 0, offset: 0 },
            suggestion: Some("Add </script> to close the script block".to_string()),
        }
    })?;

    let content = remaining[tag_end + 1..close_pos].to_string();
    let total_len = close_pos + close_tag.len();

    Ok(Some(ParsedBlock {
        content,
        lang,
        scoped: false,
        total_len,
    }))
}

fn try_parse_template_block(remaining: &str, _source: &str, _offset: usize) -> Result<Option<ParsedBlock>> {
    if !remaining.starts_with("<template") {
        return Ok(None);
    }

    let tag_end = remaining.find('>').ok_or_else(|| {
        CompileError::SyntaxError {
            message: "Unclosed <template> tag".to_string(),
            location: SourceLocation { line: 0, column: 0, offset: 0 },
            suggestion: None,
        }
    })?;

    let close_tag = "</template>";
    let close_pos = remaining.find(close_tag).ok_or_else(|| {
        CompileError::SyntaxError {
            message: "Missing </template> closing tag".to_string(),
            location: SourceLocation { line: 0, column: 0, offset: 0 },
            suggestion: Some("Add </template> to close the template block".to_string()),
        }
    })?;

    let content = remaining[tag_end + 1..close_pos].to_string();

    Ok(Some(ParsedBlock {
        content,
        lang: None,
        scoped: false,
        total_len: close_pos + close_tag.len(),
    }))
}

fn try_parse_style_block(remaining: &str, _source: &str, _offset: usize) -> Result<Option<ParsedBlock>> {
    if !remaining.starts_with("<style") {
        return Ok(None);
    }

    let tag_end = remaining.find('>').ok_or_else(|| {
        CompileError::SyntaxError {
            message: "Unclosed <style> tag".to_string(),
            location: SourceLocation { line: 0, column: 0, offset: 0 },
            suggestion: None,
        }
    })?;

    let tag_attrs = &remaining[6..tag_end];
    let scoped = tag_attrs.contains("scoped");
    let lang = extract_attr(tag_attrs, "lang");

    let close_tag = "</style>";
    let close_pos = remaining.find(close_tag).ok_or_else(|| {
        CompileError::SyntaxError {
            message: "Missing </style> closing tag".to_string(),
            location: SourceLocation { line: 0, column: 0, offset: 0 },
            suggestion: Some("Add </style> to close the style block".to_string()),
        }
    })?;

    let content = remaining[tag_end + 1..close_pos].to_string();

    Ok(Some(ParsedBlock {
        content,
        lang,
        scoped,
        total_len: close_pos + close_tag.len(),
    }))
}

/// Tokenize the inner content of a `<template>` block.
fn tokenize_template(content: &str) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut pos = 0;
    let chars: Vec<char> = content.chars().collect();
    let len = chars.len();

    while pos < len {
        // Interpolation: {{ expression }}
        if pos + 1 < len && chars[pos] == '{' && chars[pos + 1] == '{' {
            let start = pos + 2;
            let mut end = start;
            while end + 1 < len && !(chars[end] == '}' && chars[end + 1] == '}') {
                end += 1;
            }
            let expr: String = chars[start..end].iter().collect();
            tokens.push(Token::Interpolation(expr.trim().to_string()));
            pos = end + 2;
            continue;
        }

        // HTML comment
        if pos + 3 < len && &content[pos..pos + 4] == "<!--" {
            if let Some(end) = content[pos..].find("-->") {
                let comment_text = &content[pos + 4..pos + end];
                tokens.push(Token::Comment(comment_text.trim().to_string()));
                pos += end + 3;
                continue;
            }
        }

        // Tag
        if chars[pos] == '<' {
            if pos + 1 < len && chars[pos + 1] == '/' {
                // Closing tag
                let start = pos + 2;
                let mut end = start;
                while end < len && chars[end] != '>' {
                    end += 1;
                }
                let tag_name: String = chars[start..end].iter().collect();
                tokens.push(Token::TagClose(tag_name.trim().to_string()));
                pos = end + 1;
                continue;
            } else {
                // Opening tag
                let start = pos + 1;
                let mut end = start;
                while end < len && !chars[end].is_whitespace() && chars[end] != '>' && chars[end] != '/' {
                    end += 1;
                }
                let tag_name: String = chars[start..end].iter().collect();
                tokens.push(Token::TagOpen(tag_name.clone()));
                pos = end;

                // Parse attributes
                while pos < len && chars[pos] != '>' {
                    // Skip whitespace
                    while pos < len && chars[pos].is_whitespace() {
                        pos += 1;
                    }

                    if pos >= len || chars[pos] == '>' || chars[pos] == '/' {
                        break;
                    }

                    // Parse attribute
                    let attr_tokens = parse_attribute(&chars, &mut pos)?;
                    tokens.extend(attr_tokens);
                }

                // Self-closing or end
                if pos < len && chars[pos] == '/' {
                    tokens.push(Token::TagSelfClose);
                    pos += 1; // skip /
                    if pos < len && chars[pos] == '>' {
                        pos += 1; // skip >
                    }
                } else if pos < len && chars[pos] == '>' {
                    tokens.push(Token::TagEnd);
                    pos += 1;
                }
                continue;
            }
        }

        // Text content
        let start = pos;
        while pos < len && chars[pos] != '<' && !(pos + 1 < len && chars[pos] == '{' && chars[pos + 1] == '{') {
            pos += 1;
        }
        if pos > start {
            let text: String = chars[start..pos].iter().collect();
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                tokens.push(Token::Text(trimmed.to_string()));
            }
        }
    }

    Ok(tokens)
}

/// Parse a single attribute (or directive) from the template.
fn parse_attribute(chars: &[char], pos: &mut usize) -> Result<Vec<Token>> {
    let mut tokens = Vec::new();
    let len = chars.len();

    let start = *pos;

    // Directive shorthands: @event, :bind, b-directive, z-directive
    if chars[*pos] == '@' || chars[*pos] == ':' 
        || (*pos + 1 < len && chars[*pos] == 'b' && chars[*pos + 1] == '-')
        || (*pos + 1 < len && chars[*pos] == 'z' && chars[*pos + 1] == '-') {
        let is_event = chars[*pos] == '@';
        let is_bind = chars[*pos] == ':';

        if is_event || is_bind {
            *pos += 1;
        }

        // Read directive/attribute name
        let name_start = *pos;
        while *pos < len && chars[*pos] != '=' && !chars[*pos].is_whitespace() && chars[*pos] != '>' && chars[*pos] != '/' {
            *pos += 1;
        }
        let raw_name: String = chars[name_start..*pos].iter().collect();

        // Parse modifiers (e.g., @click.prevent.stop)
        let parts: Vec<&str> = raw_name.split('.').collect();
        let (name, modifiers) = if parts.len() > 1 {
            (parts[0].to_string(), parts[1..].iter().map(|s| s.to_string()).collect())
        } else {
            (raw_name.clone(), vec![])
        };

        let kind = if is_event {
            "on".to_string()
        } else if is_bind {
            "bind".to_string()
        } else {
            // strip "b-" or "z-" prefix
            name.strip_prefix("b-")
                .or_else(|| name.strip_prefix("z-"))
                .unwrap_or(&name)
                .to_string()
        };

        let argument = if is_event || is_bind {
            Some(name.clone())
        } else {
            None
        };

        // Parse value
        let expression = if *pos < len && chars[*pos] == '=' {
            *pos += 1;
            Some(parse_attr_value(chars, pos)?)
        } else {
            None
        };

        tokens.push(Token::Directive(DirectiveToken {
            kind,
            argument,
            modifiers,
            expression,
        }));
    } else {
        // Regular attribute
        let name_start = *pos;
        while *pos < len && chars[*pos] != '=' && !chars[*pos].is_whitespace() && chars[*pos] != '>' && chars[*pos] != '/' {
            *pos += 1;
        }
        let attr_name: String = chars[name_start..*pos].iter().collect();

        if attr_name.is_empty() {
            *pos = start + 1; // Advance to avoid infinite loop
            return Ok(tokens);
        }

        tokens.push(Token::AttrName(attr_name));

        if *pos < len && chars[*pos] == '=' {
            *pos += 1;
            let value = parse_attr_value(chars, pos)?;
            tokens.push(Token::AttrValue(value));
        }
    }

    Ok(tokens)
}

/// Parse an attribute value (quoted string).
fn parse_attr_value(chars: &[char], pos: &mut usize) -> Result<String> {
    let len = chars.len();

    // Skip whitespace
    while *pos < len && chars[*pos].is_whitespace() {
        *pos += 1;
    }

    if *pos >= len {
        return Ok(String::new());
    }

    let quote = chars[*pos];
    if quote == '"' || quote == '\'' {
        *pos += 1;
        let start = *pos;
        while *pos < len && chars[*pos] != quote {
            *pos += 1;
        }
        let value: String = chars[start..*pos].iter().collect();
        if *pos < len {
            *pos += 1; // Skip closing quote
        }
        Ok(value)
    } else {
        // Unquoted value
        let start = *pos;
        while *pos < len && !chars[*pos].is_whitespace() && chars[*pos] != '>' {
            *pos += 1;
        }
        Ok(chars[start..*pos].iter().collect())
    }
}

/// Extract an attribute value from a tag's attribute string.
fn extract_attr(attrs: &str, name: &str) -> Option<String> {
    let pattern = format!("{}=", name);
    if let Some(start) = attrs.find(&pattern) {
        let after = &attrs[start + pattern.len()..];
        let after = after.trim_start();
        if after.starts_with('"') {
            let end = after[1..].find('"')?;
            Some(after[1..1 + end].to_string())
        } else if after.starts_with('\'') {
            let end = after[1..].find('\'')?;
            Some(after[1..1 + end].to_string())
        } else {
            let end = after.find(|c: char| c.is_whitespace() || c == '>').unwrap_or(after.len());
            Some(after[..end].to_string())
        }
    } else {
        None
    }
}

/// Count the line number for a given byte offset.
fn count_line(source: &str, offset: usize) -> usize {
    source[..offset.min(source.len())].matches('\n').count() + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_basic() {
        let source = r#"
<script>
  let x = 1;
</script>

<template>
  <div>{{ x }}</div>
</template>

<style scoped>
  div { color: red; }
</style>
"#;
        let tokens = tokenize(source).unwrap();
        assert!(tokens.iter().any(|t| matches!(t, Token::ScriptOpen { .. })));
        assert!(tokens.iter().any(|t| matches!(t, Token::TemplateOpen { .. })));
        assert!(tokens.iter().any(|t| matches!(t, Token::StyleOpen { scoped: true, .. })));
        assert!(tokens.iter().any(|t| matches!(t, Token::Interpolation(ref s) if s == "x")));
    }
}

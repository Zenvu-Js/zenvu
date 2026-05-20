//! Compiler error types for Zenvu.js.
//!
//! Provides structured error reporting with source locations,
//! helpful error messages, and suggestions for fixes.

use thiserror::Error;

/// Source location within a `.Zenvu` file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    /// 1-based line number.
    pub line: usize,
    /// 1-based column number.
    pub column: usize,
    /// Byte offset from start of file.
    pub offset: usize,
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

/// A span within source code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Span {
    pub start: SourceLocation,
    pub end: SourceLocation,
}

/// All possible compiler errors in Zenvu.js.
#[derive(Debug, Error)]
pub enum CompileError {
    #[error("Syntax error at {location}: {message}")]
    SyntaxError {
        message: String,
        location: SourceLocation,
        suggestion: Option<String>,
    },

    #[error("Template error at {location}: {message}")]
    TemplateError {
        message: String,
        location: SourceLocation,
        suggestion: Option<String>,
    },

    #[error("Reactive analysis error: {message}")]
    ReactivityError {
        message: String,
        location: Option<SourceLocation>,
    },

    #[error("Unknown directive: {directive} at {location}")]
    UnknownDirective {
        directive: String,
        location: SourceLocation,
    },

    #[error("Component '{name}' not found â€” did you forget to import it?")]
    ComponentNotFound {
        name: String,
        location: SourceLocation,
    },

    #[error("Prop '{prop}' is required but not provided on <{component}>")]
    MissingRequiredProp {
        prop: String,
        component: String,
        location: SourceLocation,
    },

    #[error("Type error: {message}")]
    TypeError {
        message: String,
        location: SourceLocation,
    },

    #[error("CSS error at {location}: {message}")]
    CssError {
        message: String,
        location: SourceLocation,
    },

    #[error("Code generation error: {0}")]
    CodegenError(String),

    #[error("Internal compiler error: {0}")]
    InternalError(String),
}

impl CompileError {
    /// Returns a user-friendly error message with source context.
    pub fn format_with_source(&self, source: &str, filename: &str) -> String {
        let location = self.location();
        let mut output = String::new();

        output.push_str(&format!("\n  error: {}\n", self));
        output.push_str(&format!("   --> {}:{}\n", filename, location.map_or("?".to_string(), |l| l.to_string())));

        if let Some(loc) = location {
            let lines: Vec<&str> = source.lines().collect();
            if loc.line > 0 && loc.line <= lines.len() {
                let line_content = lines[loc.line - 1];
                output.push_str(&format!("    |\n"));
                output.push_str(&format!("{:>3} | {}\n", loc.line, line_content));
                output.push_str(&format!("    | {}{}\n", " ".repeat(loc.column.saturating_sub(1)), "^".repeat(1)));
            }
        }

        if let Some(suggestion) = self.suggestion() {
            output.push_str(&format!("   help: {}\n", suggestion));
        }

        output
    }

    /// Extract the source location from the error, if available.
    fn location(&self) -> Option<&SourceLocation> {
        match self {
            Self::SyntaxError { location, .. } => Some(location),
            Self::TemplateError { location, .. } => Some(location),
            Self::ReactivityError { location, .. } => location.as_ref(),
            Self::UnknownDirective { location, .. } => Some(location),
            Self::ComponentNotFound { location, .. } => Some(location),
            Self::MissingRequiredProp { location, .. } => Some(location),
            Self::TypeError { location, .. } => Some(location),
            Self::CssError { location, .. } => Some(location),
            _ => None,
        }
    }

    /// Extract a suggestion from the error, if available.
    fn suggestion(&self) -> Option<&str> {
        match self {
            Self::SyntaxError { suggestion, .. } => suggestion.as_deref(),
            Self::TemplateError { suggestion, .. } => suggestion.as_deref(),
            _ => None,
        }
    }
}

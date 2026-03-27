//! Error types and source span handling for the Newt compiler.

use thiserror::Error;

/// A span of source code (byte range + line/column for display).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: u32,
    pub column: u32,
}

impl Span {
    pub fn new(start: usize, end: usize, line: u32, column: u32) -> Self {
        Self { start, end, line, column }
    }

    pub fn merge(self, other: Span) -> Span {
        Span {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
            line: self.line,
            column: self.column,
        }
    }
}

/// Holds the full source and can slice by span.
#[derive(Clone)]
pub struct Source {
    pub code: String,
    pub path: Option<String>,
}

impl Source {
    pub fn new(code: String, path: Option<String>) -> Self {
        Self { code, path }
    }

    pub fn slice(&self, span: Span) -> &str {
        &self.code[span.start..span.end.min(self.code.len())]
    }

    pub fn line_at(&self, line: u32) -> &str {
        self.code
            .lines()
            .nth((line as usize).saturating_sub(1))
            .unwrap_or("")
    }
}

/// Compiler / runtime errors with source location.
#[derive(Error, Debug)]
pub enum NewtError {
    #[error("lexer: {message}")]
    Lexer {
        span: Span,
        message: String,
        suggestion: Option<String>,
    },

    #[error("parse: {message}")]
    Parse {
        span: Span,
        message: String,
        suggestion: Option<String>,
    },

    #[error("semantic: {message}")]
    Semantic {
        span: Span,
        message: String,
        suggestion: Option<String>,
    },

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

impl NewtError {
    pub fn lexer(span: Span, message: impl Into<String>) -> Self {
        Self::Lexer {
            span,
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn lexer_with_suggestion(span: Span, message: impl Into<String>, suggestion: impl Into<String>) -> Self {
        Self::Lexer {
            span,
            message: message.into(),
            suggestion: Some(suggestion.into()),
        }
    }

    pub fn parse(span: Span, message: impl Into<String>) -> Self {
        Self::Parse {
            span,
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn parse_with_suggestion(span: Span, message: impl Into<String>, suggestion: impl Into<String>) -> Self {
        Self::Parse {
            span,
            message: message.into(),
            suggestion: Some(suggestion.into()),
        }
    }

    pub fn semantic(span: Span, message: impl Into<String>) -> Self {
        Self::Semantic {
            span,
            message: message.into(),
            suggestion: None,
        }
    }

    pub fn span(&self) -> Option<Span> {
        match self {
            Self::Lexer { span, .. } | Self::Parse { span, .. } | Self::Semantic { span, .. } => Some(*span),
            _ => None,
        }
    }

    pub fn suggestion(&self) -> Option<&str> {
        match self {
            Self::Lexer { suggestion: s, .. } | Self::Parse { suggestion: s, .. } | Self::Semantic { suggestion: s, .. } => s.as_deref(),
            _ => None,
        }
    }
}

/// Pretty-print an error with source context.
pub fn format_error(source: &Source, err: &NewtError) -> String {
    let mut out = String::new();
    if let Some(span) = err.span() {
        let path = source.path.as_deref().unwrap_or("<input>");
        let line_content = source.line_at(span.line);
        out.push_str(&format!("  --> {}:{}:{}\n", path, span.line, span.column));
        out.push_str("   |\n");
        out.push_str(&format!("{:>4} | {}\n", span.line, line_content));
        let pad = span.column.saturating_sub(1) as usize;
        out.push_str(&format!("   | {}^\n", " ".repeat(pad)));
    }
    out.push_str(&format!("error: {}\n", err));
    if let Some(s) = err.suggestion() {
        out.push_str(&format!("  hint: {}\n", s));
    }
    out
}

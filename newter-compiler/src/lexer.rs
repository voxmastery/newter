//! Lexer for the Newt UI language.

use crate::error::{NewtError, Span};
use std::fmt;

/// A segment of an interpolated string at the token level.
#[derive(Clone, Debug, PartialEq)]
pub enum InterpPart {
    /// Literal text between interpolation braces.
    Literal(String),
    /// Raw source text of an expression inside `{…}`.
    ExprSource(String),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TokenKind {
    // Literals
    Number(f64),
    String(String),
    /// String containing `{expr}` interpolation segments.
    InterpolatedString(Vec<InterpPart>),
    Ident(String),
    True,
    False,
    HexColor(u8, u8, u8, u8),

    // Keywords (language)
    Let,
    State,
    Component,
    Screen,
    If,
    Else,
    For,
    In,
    Theme,
    Use,
    Import,

    // Element types (UI) — sections and layout
    Header,
    Footer,
    Container,
    Sidebar,
    Section,
    Box,
    Text,
    Row,
    Column,
    Grid,
    Stack,
    Center,
    Spacer,
    Image,
    Button,
    Input,
    Card,
    Widget,
    // Navigational
    Accordion,
    Bento,
    Breadcrumb,
    Hamburger,
    Kebab,
    Meatballs,
    Doner,
    Tabs,
    Pagination,
    LinkList,
    Nav,
    // Input and forms
    Password,
    Search,
    Checkbox,
    Radio,
    Dropdown,
    Combobox,
    Multiselect,
    DatePicker,
    Picker,
    Slider,
    Stepper,
    Toggle,
    Form,
    // Feedback and overlay
    Modal,
    ConfirmDialog,
    Toast,
    Notification,
    Alert,
    MessageBox,
    Tooltip,
    Loader,
    ProgressBar,
    Badge,
    // Display and content
    Icon,
    Tag,
    Comment,
    Feed,
    Carousel,
    Chart,

    // Property names (common)
    Width,
    Height,
    Fill,
    Stroke,
    Radius,
    Padding,
    Gap,
    Grow,
    Shrink,
    Align,
    Justify,
    Direction,
    FontSize,
    FontWeight,
    Shadow,

    // Delimiters
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Semicolon,
    Colon,
    Arrow, // ->

    // Operators
    Eq,       // =
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqEq,
    NotEq,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
    Not,

    Eof,
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Number(n) => write!(f, "{}", n),
            TokenKind::String(s) => write!(f, "\"{}\"", s),
            TokenKind::InterpolatedString(_) => write!(f, "\"…{{…}}…\""),
            TokenKind::Ident(s) => write!(f, "{}", s),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::HexColor(r, g, b, a) => write!(f, "#{:02x}{:02x}{:02x}{:02x}", r, g, b, a),
            TokenKind::Let => write!(f, "let"),
            TokenKind::State => write!(f, "state"),
            TokenKind::Component => write!(f, "component"),
            TokenKind::Screen => write!(f, "screen"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::For => write!(f, "for"),
            TokenKind::In => write!(f, "in"),
            TokenKind::Theme => write!(f, "theme"),
            TokenKind::Use => write!(f, "use"),
            TokenKind::Import => write!(f, "import"),
            TokenKind::Header => write!(f, "header"),
            TokenKind::Footer => write!(f, "footer"),
            TokenKind::Container => write!(f, "container"),
            TokenKind::Sidebar => write!(f, "sidebar"),
            TokenKind::Box => write!(f, "box"),
            TokenKind::Text => write!(f, "text"),
            TokenKind::Row => write!(f, "row"),
            TokenKind::Column => write!(f, "column"),
            TokenKind::Grid => write!(f, "grid"),
            TokenKind::Stack => write!(f, "stack"),
            TokenKind::Center => write!(f, "center"),
            TokenKind::Spacer => write!(f, "spacer"),
            TokenKind::Image => write!(f, "image"),
            TokenKind::Button => write!(f, "button"),
            TokenKind::Input => write!(f, "input"),
            TokenKind::Card => write!(f, "card"),
            TokenKind::Section => write!(f, "section"),
            TokenKind::Widget => write!(f, "widget"),
            TokenKind::Accordion => write!(f, "accordion"),
            TokenKind::Bento => write!(f, "bento"),
            TokenKind::Breadcrumb => write!(f, "breadcrumb"),
            TokenKind::Hamburger => write!(f, "hamburger"),
            TokenKind::Kebab => write!(f, "kebab"),
            TokenKind::Meatballs => write!(f, "meatballs"),
            TokenKind::Doner => write!(f, "doner"),
            TokenKind::Tabs => write!(f, "tabs"),
            TokenKind::Pagination => write!(f, "pagination"),
            TokenKind::LinkList => write!(f, "linkList"),
            TokenKind::Nav => write!(f, "nav"),
            TokenKind::Password => write!(f, "password"),
            TokenKind::Search => write!(f, "search"),
            TokenKind::Checkbox => write!(f, "checkbox"),
            TokenKind::Radio => write!(f, "radio"),
            TokenKind::Dropdown => write!(f, "dropdown"),
            TokenKind::Combobox => write!(f, "combobox"),
            TokenKind::Multiselect => write!(f, "multiselect"),
            TokenKind::DatePicker => write!(f, "datePicker"),
            TokenKind::Picker => write!(f, "picker"),
            TokenKind::Slider => write!(f, "slider"),
            TokenKind::Stepper => write!(f, "stepper"),
            TokenKind::Toggle => write!(f, "toggle"),
            TokenKind::Form => write!(f, "form"),
            TokenKind::Modal => write!(f, "modal"),
            TokenKind::ConfirmDialog => write!(f, "confirmDialog"),
            TokenKind::Toast => write!(f, "toast"),
            TokenKind::Notification => write!(f, "notification"),
            TokenKind::Alert => write!(f, "alert"),
            TokenKind::MessageBox => write!(f, "messageBox"),
            TokenKind::Tooltip => write!(f, "tooltip"),
            TokenKind::Loader => write!(f, "loader"),
            TokenKind::ProgressBar => write!(f, "progressBar"),
            TokenKind::Badge => write!(f, "badge"),
            TokenKind::Icon => write!(f, "icon"),
            TokenKind::Tag => write!(f, "tag"),
            TokenKind::Comment => write!(f, "comment"),
            TokenKind::Feed => write!(f, "feed"),
            TokenKind::Carousel => write!(f, "carousel"),
            TokenKind::Chart => write!(f, "chart"),
            TokenKind::Width => write!(f, "width"),
            TokenKind::Height => write!(f, "height"),
            TokenKind::Fill => write!(f, "fill"),
            TokenKind::Stroke => write!(f, "stroke"),
            TokenKind::Radius => write!(f, "radius"),
            TokenKind::Padding => write!(f, "padding"),
            TokenKind::Gap => write!(f, "gap"),
            TokenKind::Grow => write!(f, "grow"),
            TokenKind::Shrink => write!(f, "shrink"),
            TokenKind::Align => write!(f, "align"),
            TokenKind::Justify => write!(f, "justify"),
            TokenKind::Direction => write!(f, "direction"),
            TokenKind::FontSize => write!(f, "fontSize"),
            TokenKind::FontWeight => write!(f, "fontWeight"),
            TokenKind::Shadow => write!(f, "shadow"),
            TokenKind::LeftBrace => write!(f, "{{"),
            TokenKind::RightBrace => write!(f, "}}"),
            TokenKind::LeftParen => write!(f, "("),
            TokenKind::RightParen => write!(f, ")"),
            TokenKind::LeftBracket => write!(f, "["),
            TokenKind::RightBracket => write!(f, "]"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::Eq => write!(f, "="),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::EqEq => write!(f, "=="),
            TokenKind::NotEq => write!(f, "!="),
            TokenKind::Lt => write!(f, "<"),
            TokenKind::Le => write!(f, "<="),
            TokenKind::Gt => write!(f, ">"),
            TokenKind::Ge => write!(f, ">="),
            TokenKind::And => write!(f, "&&"),
            TokenKind::Or => write!(f, "||"),
            TokenKind::Not => write!(f, "!"),
            TokenKind::Eof => write!(f, "<eof>"),
        }
    }
}

/// Token category for syntax highlighting (keyword, string, number, etc.).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TokenCategory {
    Keyword,
    String,
    Number,
    Color,
    Ident,
    Comment,
    Delimiter,
    Eof,
}

impl TokenKind {
    pub fn category(&self) -> TokenCategory {
        match self {
            TokenKind::Number(_) => TokenCategory::Number,
            TokenKind::String(_) | TokenKind::InterpolatedString(_) => TokenCategory::String,
            TokenKind::Ident(_) => TokenCategory::Ident,
            TokenKind::True | TokenKind::False => TokenCategory::Ident,
            TokenKind::HexColor(_, _, _, _) => TokenCategory::Color,
            TokenKind::Let | TokenKind::State | TokenKind::Component | TokenKind::Screen | TokenKind::If
            | TokenKind::Else | TokenKind::For | TokenKind::In | TokenKind::Theme
            | TokenKind::Use | TokenKind::Import => TokenCategory::Keyword,
            TokenKind::Header | TokenKind::Footer | TokenKind::Container | TokenKind::Sidebar
            | TokenKind::Section | TokenKind::Box | TokenKind::Text | TokenKind::Row
            | TokenKind::Column | TokenKind::Grid | TokenKind::Stack | TokenKind::Center
            | TokenKind::Spacer | TokenKind::Image | TokenKind::Button | TokenKind::Input
            | TokenKind::Card | TokenKind::Widget => TokenCategory::Keyword,
            TokenKind::Accordion | TokenKind::Bento | TokenKind::Breadcrumb | TokenKind::Hamburger
            | TokenKind::Kebab | TokenKind::Meatballs | TokenKind::Doner | TokenKind::Tabs
            | TokenKind::Pagination | TokenKind::LinkList | TokenKind::Nav | TokenKind::Password
            | TokenKind::Search | TokenKind::Checkbox | TokenKind::Radio | TokenKind::Dropdown
            | TokenKind::Combobox | TokenKind::Multiselect | TokenKind::DatePicker | TokenKind::Picker
            | TokenKind::Slider | TokenKind::Stepper | TokenKind::Toggle | TokenKind::Form
            | TokenKind::Modal | TokenKind::ConfirmDialog | TokenKind::Toast | TokenKind::Notification
            | TokenKind::Alert | TokenKind::MessageBox | TokenKind::Tooltip | TokenKind::Loader
            | TokenKind::ProgressBar | TokenKind::Badge | TokenKind::Icon | TokenKind::Tag
            | TokenKind::Comment | TokenKind::Feed | TokenKind::Carousel | TokenKind::Chart => TokenCategory::Keyword,
            TokenKind::Width | TokenKind::Height | TokenKind::Fill | TokenKind::Stroke
            | TokenKind::Radius | TokenKind::Padding | TokenKind::Gap | TokenKind::Grow
            | TokenKind::Shrink | TokenKind::Align | TokenKind::Justify | TokenKind::Direction
            | TokenKind::FontSize | TokenKind::FontWeight | TokenKind::Shadow => TokenCategory::Ident,
            TokenKind::Eof => TokenCategory::Eof,
            _ => TokenCategory::Delimiter,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

pub struct Lexer<'a> {
    source: &'a str,
    pos: usize,
    line: u32,
    column: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str, _path: Option<&'a str>) -> Self {
        Self {
            source,
            pos: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.source[self.pos..].chars().next()
    }

    fn peek2(&self) -> (Option<char>, Option<char>) {
        let mut it = self.source[self.pos..].chars();
        (it.next(), it.next())
    }

    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        self.pos += ch.len_utf8();
        Some(ch)
    }

    fn span_start(&self) -> (usize, u32, u32) {
        (self.pos, self.line, self.column)
    }

    fn make_span(&self, start: usize, start_line: u32, start_col: u32) -> Span {
        Span::new(start, self.pos, start_line, start_col)
    }

    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.peek() {
                Some(' ' | '\t' | '\r' | '\n') => {
                    self.advance();
                }
                Some('/') if self.peek2().1 == Some('/') => {
                    self.advance();
                    self.advance();
                    while self.peek().map(|c| c != '\n').unwrap_or(false) {
                        self.advance();
                    }
                }
                _ => break,
            }
        }
    }

    fn read_string(&mut self) -> Result<Token, NewtError> {
        let (start, line, col) = self.span_start();
        self.advance(); // opening "
        let mut parts: Vec<InterpPart> = Vec::new();
        let mut buf = String::new();
        let mut has_interp = false;

        while let Some(c) = self.peek() {
            if c == '"' {
                self.advance();
                if !has_interp {
                    return Ok(Token {
                        kind: TokenKind::String(buf),
                        span: self.make_span(start, line, col),
                    });
                }
                if !buf.is_empty() {
                    parts.push(InterpPart::Literal(buf));
                }
                return Ok(Token {
                    kind: TokenKind::InterpolatedString(parts),
                    span: self.make_span(start, line, col),
                });
            }
            if c == '\\' {
                self.advance();
                match self.advance() {
                    Some('n') => buf.push('\n'),
                    Some('t') => buf.push('\t'),
                    Some('r') => buf.push('\r'),
                    Some('"') => buf.push('"'),
                    Some('\\') => buf.push('\\'),
                    Some('{') => buf.push('{'),
                    Some('}') => buf.push('}'),
                    Some(o) => buf.push(o),
                    None => break,
                }
                continue;
            }
            if c == '{' {
                has_interp = true;
                self.advance(); // consume {
                if !buf.is_empty() {
                    parts.push(InterpPart::Literal(std::mem::take(&mut buf)));
                }
                // Collect expression source until matching }
                let mut depth = 1u32;
                let mut expr_src = String::new();
                while let Some(ec) = self.peek() {
                    if ec == '{' {
                        depth += 1;
                        expr_src.push(ec);
                        self.advance();
                    } else if ec == '}' {
                        depth -= 1;
                        if depth == 0 {
                            self.advance(); // consume closing }
                            break;
                        }
                        expr_src.push(ec);
                        self.advance();
                    } else if ec == '"' {
                        // String literal inside interpolation — consume it whole
                        expr_src.push(ec);
                        self.advance();
                        while let Some(sc) = self.peek() {
                            expr_src.push(sc);
                            self.advance();
                            if sc == '"' { break; }
                            if sc == '\\' {
                                if let Some(esc) = self.peek() {
                                    expr_src.push(esc);
                                    self.advance();
                                }
                            }
                        }
                    } else {
                        expr_src.push(ec);
                        self.advance();
                    }
                }
                if depth != 0 {
                    return Err(NewtError::lexer(
                        self.make_span(start, line, col),
                        "unterminated interpolation brace in string",
                    ));
                }
                parts.push(InterpPart::ExprSource(expr_src));
                continue;
            }
            buf.push(c);
            self.advance();
        }
        Err(NewtError::lexer(
            self.make_span(start, line, col),
            "unterminated string",
        ))
    }

    fn read_hex_color(&mut self) -> Result<Token, NewtError> {
        let (start, line, col) = self.span_start();
        self.advance(); // #
        let mut s = String::new();
        while let Some(c) = self.peek() {
            if c.is_ascii_hexdigit() {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        if s.len() == 6 {
            let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
            Ok(Token {
                kind: TokenKind::HexColor(r, g, b, 255),
                span: self.make_span(start, line, col),
            })
        } else if s.len() == 8 {
            let r = u8::from_str_radix(&s[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&s[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&s[4..6], 16).unwrap_or(0);
            let a = u8::from_str_radix(&s[6..8], 16).unwrap_or(255);
            Ok(Token {
                kind: TokenKind::HexColor(r, g, b, a),
                span: self.make_span(start, line, col),
            })
        } else {
            Err(NewtError::lexer(
                self.make_span(start, line, col),
                "hex color must be #RRGGBB or #RRGGBBAA",
            ))
        }
    }

    fn read_number(&mut self, first: char) -> Token {
        let (start, line, col) = self.span_start();
        self.advance(); // consume the first char (caller only peeked)
        let mut s = String::from(first);
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() || c == '.' {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let n: f64 = s.parse().unwrap_or(0.0);
        Token {
            kind: TokenKind::Number(n),
            span: self.make_span(start, line, col),
        }
    }

    fn read_ident_or_keyword(&mut self, first: char) -> Token {
        let (start, line, col) = self.span_start();
        self.advance(); // consume the first char (caller only peeked)
        let mut s = String::from(first);
        while let Some(c) = self.peek() {
            if c.is_ascii_alphanumeric() || c == '_' {
                s.push(c);
                self.advance();
            } else {
                break;
            }
        }
        let kind = match s.as_str() {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "let" => TokenKind::Let,
            "state" => TokenKind::State,
            "component" => TokenKind::Component,
            "screen" => TokenKind::Screen,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "theme" => TokenKind::Theme,
            "use" => TokenKind::Use,
            "import" => TokenKind::Import,
            "header" => TokenKind::Header,
            "footer" => TokenKind::Footer,
            "container" => TokenKind::Container,
            "sidebar" => TokenKind::Sidebar,
            "box" => TokenKind::Box,
            "text" => TokenKind::Text,
            "row" => TokenKind::Row,
            "column" => TokenKind::Column,
            "grid" => TokenKind::Grid,
            "stack" => TokenKind::Stack,
            "center" => TokenKind::Center,
            "spacer" => TokenKind::Spacer,
            "image" => TokenKind::Image,
            "button" => TokenKind::Button,
            "input" => TokenKind::Input,
            "card" => TokenKind::Card,
            "section" => TokenKind::Section,
            "widget" => TokenKind::Widget,
            "accordion" => TokenKind::Accordion,
            "bento" => TokenKind::Bento,
            "breadcrumb" => TokenKind::Breadcrumb,
            "hamburger" => TokenKind::Hamburger,
            "kebab" => TokenKind::Kebab,
            "meatballs" => TokenKind::Meatballs,
            "doner" => TokenKind::Doner,
            "tabs" => TokenKind::Tabs,
            "pagination" => TokenKind::Pagination,
            "linkList" => TokenKind::LinkList,
            "nav" => TokenKind::Nav,
            "password" => TokenKind::Password,
            "search" => TokenKind::Search,
            "checkbox" => TokenKind::Checkbox,
            "radio" => TokenKind::Radio,
            "dropdown" => TokenKind::Dropdown,
            "combobox" => TokenKind::Combobox,
            "multiselect" => TokenKind::Multiselect,
            "datePicker" => TokenKind::DatePicker,
            "picker" => TokenKind::Picker,
            "slider" => TokenKind::Slider,
            "stepper" => TokenKind::Stepper,
            "toggle" => TokenKind::Toggle,
            "form" => TokenKind::Form,
            "modal" => TokenKind::Modal,
            "confirmDialog" => TokenKind::ConfirmDialog,
            "toast" => TokenKind::Toast,
            "notification" => TokenKind::Notification,
            "alert" => TokenKind::Alert,
            "messageBox" => TokenKind::MessageBox,
            "tooltip" => TokenKind::Tooltip,
            "loader" => TokenKind::Loader,
            "progressBar" => TokenKind::ProgressBar,
            "badge" => TokenKind::Badge,
            "icon" => TokenKind::Icon,
            "tag" => TokenKind::Tag,
            "comment" => TokenKind::Comment,
            "feed" => TokenKind::Feed,
            "carousel" => TokenKind::Carousel,
            "chart" => TokenKind::Chart,
            "width" | "height" | "fill" | "stroke" | "radius" | "padding" | "gap"
            | "grow" | "shrink" | "align" | "justify" | "direction"
            | "fontSize" | "fontWeight" | "shadow" => {
                // Treat as identifier so "let padding = 24" works; parser uses Ident for prop names
                TokenKind::Ident(s)
            }
            _ => TokenKind::Ident(s),
        };
        Token {
            kind,
            span: self.make_span(start, line, col),
        }
    }

    pub fn next_token(&mut self) -> Result<Token, NewtError> {
        self.skip_whitespace_and_comments();
        let (start, line, col) = self.span_start();
        let c = match self.peek() {
            Some(c) => c,
            None => {
                return Ok(Token {
                    kind: TokenKind::Eof,
                    span: self.make_span(start, line, col),
                });
            }
        };

        // Two-char
        let (a, b) = self.peek2();
        if a == Some('-') && b == Some('>') {
            self.advance();
            self.advance();
            return Ok(Token {
                kind: TokenKind::Arrow,
                span: self.make_span(start, line, col),
            });
        }
        if a == Some('=') && b == Some('=') {
            self.advance();
            self.advance();
            return Ok(Token {
                kind: TokenKind::EqEq,
                span: self.make_span(start, line, col),
            });
        }
        if a == Some('!') && b == Some('=') {
            self.advance();
            self.advance();
            return Ok(Token {
                kind: TokenKind::NotEq,
                span: self.make_span(start, line, col),
            });
        }
        if a == Some('<') && b == Some('=') {
            self.advance();
            self.advance();
            return Ok(Token {
                kind: TokenKind::Le,
                span: self.make_span(start, line, col),
            });
        }
        if a == Some('>') && b == Some('=') {
            self.advance();
            self.advance();
            return Ok(Token {
                kind: TokenKind::Ge,
                span: self.make_span(start, line, col),
            });
        }
        if a == Some('&') && b == Some('&') {
            self.advance();
            self.advance();
            return Ok(Token {
                kind: TokenKind::And,
                span: self.make_span(start, line, col),
            });
        }
        if a == Some('|') && b == Some('|') {
            self.advance();
            self.advance();
            return Ok(Token {
                kind: TokenKind::Or,
                span: self.make_span(start, line, col),
            });
        }

        // Single char
        let kind = match c {
            '"' => return self.read_string(),
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            '[' => TokenKind::LeftBracket,
            ']' => TokenKind::RightBracket,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            ';' => TokenKind::Semicolon,
            ':' => TokenKind::Colon,
            '=' => TokenKind::Eq,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '<' => TokenKind::Lt,
            '>' => TokenKind::Gt,
            '!' => TokenKind::Not,
            _ if c.is_ascii_digit() => {
                let t = self.read_number(c);
                return Ok(t);
            }
            _ if c == '#' => {
                let t = self.read_hex_color()?;
                return Ok(t);
            }
            _ if c.is_ascii_alphabetic() || c == '_' => {
                let t = self.read_ident_or_keyword(c);
                return Ok(t);
            }
            _ => {
                return Err(NewtError::lexer(
                    self.make_span(start, line, col),
                    format!("unexpected character '{}'", c),
                ));
            }
        };
        self.advance();
        Ok(Token {
            kind,
            span: self.make_span(start, line, col),
        })
    }
}

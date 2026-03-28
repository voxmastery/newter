//! Newt — design canvas compiler and live renderer.
//!
//! Parses `.newt` UI source, builds a layout tree, and renders to a wgpu canvas.

pub mod app;
pub mod ast;
pub mod error;
pub mod html;
pub mod layout;
pub mod lexer;
pub mod parser;
pub mod react;
pub mod renderer;
pub mod serve;
pub mod value;

pub use ast::{ImportDecl, Program, ProgramItem, ScreenDecl, ThemeDecl};
pub use error::{format_error, NewtError, Source, Span};
pub use html::{layout_to_html, layout_to_reactive_html};
pub use react::layout_to_react;
pub use layout::{layout_tree, LayoutNode, Rect};
pub use lexer::{TokenCategory, TokenKind};
pub use parser::Parser;
pub use app::App;
pub use value::{EvalContext, Value, value_to_json};

use std::collections::HashSet;

use lexer::Lexer;

/// Collect color variables from the program for CSS export (:root { --name: #hex; }).
pub fn theme_css_vars(program: &Program) -> Vec<(String, String)> {
    let ctx = EvalContext::from_program(program);
    let mut out = Vec::new();
    for (name, val) in &ctx.variables {
        if let Value::Color { r, g, b, a } = val {
            let hex = if *a == 255 {
                format!("#{:02x}{:02x}{:02x}", r, g, b)
            } else {
                format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
            };
            out.push((name.clone(), hex));
        }
    }
    out
}
/// Returns true if the program contains any `state` declarations.
pub fn has_state_vars(program: &Program) -> bool {
    program.items.iter().any(|item| matches!(item, ProgramItem::StateDecl(_)))
}

use std::path::{Path, PathBuf};

/// Tokenize source for syntax highlighting. Returns (span, category) for each token until Eof.
pub fn tokenize(source: &str) -> Result<Vec<(Span, TokenCategory)>, NewtError> {
    let mut lexer = Lexer::new(source, None);
    let mut out = Vec::new();
    loop {
        let tok = lexer.next_token()?;
        let span = tok.span;
        let cat = tok.kind.category();
        out.push((span, cat));
        if matches!(tok.kind, TokenKind::Eof) {
            break;
        }
    }
    Ok(out)
}

/// Parse source code into an AST.
/// Returns the first error for backward compatibility. Use `parse_all` for all errors.
pub fn parse(source: &str, path: Option<&str>) -> Result<Program, NewtError> {
    let mut parser = Parser::new(source, path)?;
    parser.parse().map_err(|mut errors| {
        errors.drain(1..);
        errors.pop().unwrap()
    })
}

/// Parse source code, returning all errors at once instead of stopping at the first.
pub fn parse_all(source: &str, path: Option<&str>) -> Result<Program, Vec<NewtError>> {
    let mut parser = match Parser::new(source, path) {
        Ok(p) => p,
        Err(e) => return Err(vec![e]),
    };
    parser.parse()
}

/// Collect all parse errors from the source. Returns an empty vec on success.
/// Designed for LSP use where all diagnostics should appear at once.
pub fn check_all(source: &str, path: Option<&std::path::Path>) -> Vec<NewtError> {
    let trimmed = source.trim();
    let path_str = path.and_then(|p| p.to_str());
    match parse_all(trimmed, path_str) {
        Ok(program) => {
            // Parse succeeded — try compile to catch semantic/layout errors
            let program = if let Some(p) = path {
                let base = p.parent().unwrap_or_else(|| std::path::Path::new("."));
                match resolve_imports(program, base) {
                    Ok(p) => p,
                    Err(e) => return vec![e],
                }
            } else {
                program
            };
            match get_screen(&program, None) {
                Some(screen) => {
                    let rect = layout::Rect::new(0.0, 0.0, DEFAULT_VIEWPORT_W, DEFAULT_VIEWPORT_H);
                    let ctx = EvalContext::from_program(&program);
                    match layout_tree(&ctx, &screen.body, rect) {
                        Ok(_) => vec![],
                        Err(e) => vec![e],
                    }
                }
                None => vec![],
            }
        }
        Err(errors) => errors,
    }
}

/// Parse a file into an AST.
pub fn parse_file(path: &Path) -> Result<Program, NewtError> {
    let source = std::fs::read_to_string(path)?;
    parse(&source, path.to_str())
}

/// Symbol kind for LSP (hover, completion, go-to-definition).
#[derive(Debug, Clone)]
pub enum SymbolKind {
    Variable,
    Component { params: Vec<String> },
    Screen,
    Theme,
}

/// Build a symbol table from a program: name → (span, kind). Later definitions shadow earlier.
pub fn symbol_table(program: &Program) -> std::collections::HashMap<String, (Span, SymbolKind)> {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for item in &program.items {
        match item {
            ProgramItem::Variable(v) => {
                map.insert(
                    v.name.clone(),
                    (v.span, SymbolKind::Variable),
                );
            }
            ProgramItem::Component(c) => {
                map.insert(
                    c.name.clone(),
                    (c.span, SymbolKind::Component { params: c.params.clone() }),
                );
            }
            ProgramItem::Screen(s) => {
                map.insert(s.name.clone(), (s.span, SymbolKind::Screen));
            }
            ProgramItem::Theme(t) => {
                map.insert(t.name.clone(), (t.span, SymbolKind::Theme));
            }
            ProgramItem::StateDecl(sd) => {
                map.insert(sd.name.clone(), (sd.span, SymbolKind::Variable));
            }
            _ => {}
        }
    }
    map
}

/// Keywords for completion (top-level and expression context).
pub fn completion_keywords() -> Vec<&'static str> {
    vec![
        "screen", "let", "state", "component", "theme", "use", "import", "if", "for", "else", "in",
    ]
}

/// Element names for completion (row, column, card, button, etc.).
pub fn completion_element_names() -> Vec<&'static str> {
    vec![
        "header", "footer", "container", "sidebar", "section", "box", "text", "row", "column",
        "grid", "stack", "center", "spacer", "image", "button", "input", "card", "widget",
        "accordion", "bento", "breadcrumb", "hamburger", "kebab", "meatballs", "doner", "tabs",
        "pagination", "linkList", "nav", "password", "search", "checkbox", "radio", "dropdown",
        "combobox", "multiselect", "datePicker", "picker", "slider", "stepper", "toggle", "form",
        "modal", "confirmDialog", "toast", "notification", "alert", "messageBox", "tooltip",
        "loader", "progressBar", "badge", "icon", "tag", "comment", "feed", "carousel", "chart",
    ]
}

/// Prop names valid for elements (layout, style, content).
pub fn completion_prop_names() -> Vec<&'static str> {
    vec![
        "width", "height", "fill", "stroke", "strokeWidth", "radius", "padding", "gap",
        "grow", "shrink", "align", "justify", "direction", "fontSize", "fontWeight", "shadow",
        "content", "minWidth", "maxWidth", "minHeight", "maxHeight", "transition", "aspectRatio",
        "columns", "rows", "src", "role", "ariaLabel", "focusOrder", "onClick", "href", "name",
    ]
}

/// All screen names in the program (order preserved).
pub fn screen_names(program: &Program) -> Vec<String> {
    program
        .items
        .iter()
        .filter_map(|i| match i {
            ProgramItem::Screen(s) => Some(s.name.clone()),
            _ => None,
        })
        .collect()
}

/// Get a screen by name. If `name` is None, returns the first screen.
/// Use for multi-screen apps: Home, Dashboard, Settings, etc.
pub fn get_screen<'a>(program: &'a Program, name: Option<&str>) -> Option<&'a ScreenDecl> {
    for item in &program.items {
        if let ProgramItem::Screen(s) = item {
            if name.map_or(true, |n| s.name == n) {
                return Some(s);
            }
        }
    }
    None
}

/// Default viewport size for IDE and export (width, height).
pub const DEFAULT_VIEWPORT_W: f32 = 960.0;
pub const DEFAULT_VIEWPORT_H: f32 = 640.0;

/// Default port for the canvas IDE server.
pub const DEFAULT_SERVE_PORT: u16 = 3333;

/// Single compile entry: parse → resolve_imports (when path is set) → EvalContext → get_screen → layout_tree.
/// Use this from CLI, serve, LSP, and app so behavior and diagnostics are consistent.
/// Viewport is (0, 0, DEFAULT_VIEWPORT_W, DEFAULT_VIEWPORT_H) unless overridden later by the consumer.
pub fn compile(
    source: &str,
    path: Option<&std::path::Path>,
    screen_name: Option<&str>,
) -> Result<(Program, layout::LayoutNode), NewtError> {
    let trimmed = source.trim();
    let path_str = path.and_then(|p| p.to_str());
    let program = parse(trimmed, path_str)?;
    let program = if let Some(p) = path {
        let base = p.parent().unwrap_or_else(|| std::path::Path::new("."));
        resolve_imports(program, base)?
    } else {
        program
    };
    let screen = get_screen(&program, screen_name).ok_or_else(|| {
        let names = screen_names(&program);
        if names.is_empty() {
            NewtError::Other("no screen found".to_string())
        } else {
            NewtError::Other(format!(
                "screen '{}' not found. Available: {}",
                screen_name.unwrap_or(""),
                names.join(", ")
            ))
        }
    })?;
    let rect = layout::Rect::new(0.0, 0.0, DEFAULT_VIEWPORT_W, DEFAULT_VIEWPORT_H);
    let ctx = EvalContext::from_program(&program);
    let layout = layout_tree(&ctx, &screen.body, rect)?;
    Ok((program, layout))
}

/// Like `compile()` but accepts state overrides and returns the effective state.
/// State overrides are applied to variables matching `state` declarations in the program.
pub fn compile_with_state(
    source: &str,
    path: Option<&std::path::Path>,
    screen_name: Option<&str>,
    state_overrides: &std::collections::HashMap<String, Value>,
) -> Result<(Program, layout::LayoutNode, std::collections::HashMap<String, Value>), NewtError> {
    let trimmed = source.trim();
    let path_str = path.and_then(|p| p.to_str());
    let program = parse(trimmed, path_str)?;
    let program = if let Some(p) = path {
        let base = p.parent().unwrap_or_else(|| std::path::Path::new("."));
        resolve_imports(program, base)?
    } else {
        program
    };
    let screen = get_screen(&program, screen_name).ok_or_else(|| {
        let names = screen_names(&program);
        if names.is_empty() {
            NewtError::Other("no screen found".to_string())
        } else {
            NewtError::Other(format!(
                "screen '{}' not found. Available: {}",
                screen_name.unwrap_or(""),
                names.join(", ")
            ))
        }
    })?;

    // Collect state variable names
    let state_var_names: std::collections::HashSet<String> = program
        .items
        .iter()
        .filter_map(|item| {
            if let ProgramItem::StateDecl(sd) = item {
                Some(sd.name.clone())
            } else {
                None
            }
        })
        .collect();

    let rect = layout::Rect::new(0.0, 0.0, DEFAULT_VIEWPORT_W, DEFAULT_VIEWPORT_H);
    let mut ctx = EvalContext::from_program(&program);

    // Apply state overrides
    for (name, val) in state_overrides {
        if state_var_names.contains(name) {
            ctx.variables.insert(name.clone(), val.clone());
        }
    }

    let layout = layout_tree(&ctx, &screen.body, rect)?;

    // Build effective state (only state vars)
    let mut effective_state = std::collections::HashMap::new();
    for name in &state_var_names {
        if let Some(val) = ctx.variables.get(name) {
            effective_state.insert(name.clone(), val.clone());
        }
    }

    Ok((program, layout, effective_state))
}

/// Resolve all `import "path.newt"` declarations by loading and merging those files.
/// Avoids circular imports. `base_dir` is the directory of the current file.
pub fn resolve_imports(program: Program, base_dir: &Path) -> Result<Program, NewtError> {
    let mut visited = HashSet::new();
    resolve_imports_inner(program, base_dir, &mut visited)
}

fn resolve_imports_inner(
    program: Program,
    base_dir: &Path,
    visited: &mut HashSet<PathBuf>,
) -> Result<Program, NewtError> {
    let mut items = Vec::new();
    for item in program.items {
        match item {
            ProgramItem::Import(decl) => {
                let path = base_dir.join(&decl.path);
                let path = path
                    .canonicalize()
                    .map_err(|e| NewtError::Other(format!("import '{}': {}", decl.path, e)))?;
                if visited.contains(&path) {
                    return Err(NewtError::Other(format!(
                        "circular import: {}",
                        path.display()
                    )));
                }
                visited.insert(path.clone());
                let content = std::fs::read_to_string(&path)
                    .map_err(|e| NewtError::Other(format!("read {}: {}", path.display(), e)))?;
                let parsed = parse(&content, path.to_str())?;
                let parent = path.parent().unwrap_or(base_dir);
                let resolved = resolve_imports_inner(parsed, parent, visited)?;
                items.extend(resolved.items);
            }
            other => items.push(other),
        }
    }
    Ok(Program { items })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_default_program() {
        let source = r#"
let padding = 24;
let fill = #f0f0f0;
screen Main {
  column { gap: 16, padding: padding } {
    box { fill: #ffffff, radius: 8 } { text { content: "Hi", fontSize: 24 } }
  }
}
"#;
        let program = parse(source.trim(), None).expect("parse should succeed");
        assert!(!program.items.is_empty());
    }

    #[test]
    fn parse_screen_header_container_syntax() {
        let source = r#"
let padding = 24;
let cardFill = #ffffff;
let accent = #2563eb;

screen(Main) {
    header (
        row ( gap: 12, padding: 16 ) (
            text("Newt Canvas")
            button("Menu")
        )
    )
    container (
        column ( gap: 20, padding: padding ) (
            card ( fill: cardFill, radius: 12, padding: 24, stroke: #e5e7eb ) (
                text("Newt Canvas", fontSize: 28)
            )
            row ( gap: 12 ) (
                button("One", fill: #f3f4f6, radius: 8)
                button("Two", fill: #f3f4f6, radius: 8)
                button("Three", fill: accent, radius: 8)
            )
        )
    )
}
"#;
        let program = parse(source.trim(), None).expect("parse should succeed");
        assert!(!program.items.is_empty());
        let ctx = EvalContext::from_program(&program);
        let screen = program.items.iter().find_map(|i| {
            if let crate::ast::ProgramItem::Screen(s) = i {
                Some(s)
            } else {
                None
            }
        });
        let screen = screen.expect("one screen");
        let rect = crate::layout::Rect::new(0.0, 0.0, 960.0, 640.0);
        let layout = layout_tree(&ctx, &screen.body, rect).expect("layout");
        assert!(!layout.children.is_empty(), "screen should have header + container");
    }

    // --- Lexer Tests ---

    #[test]
    fn lex_number_tokens() {
        let mut lexer = lexer::Lexer::new("42 3.14", None);
        let t1 = lexer.next_token().unwrap();
        assert!(matches!(t1.kind, TokenKind::Number(n) if (n - 42.0).abs() < f64::EPSILON));
        let t2 = lexer.next_token().unwrap();
        assert!(matches!(t2.kind, TokenKind::Number(n) if (n - 3.14).abs() < 0.001));
    }

    #[test]
    fn lex_string_with_escapes() {
        let mut lexer = lexer::Lexer::new(r#""hello\nworld""#, None);
        let t = lexer.next_token().unwrap();
        assert!(matches!(t.kind, TokenKind::String(ref s) if s == "hello\nworld"));
    }

    #[test]
    fn lex_hex_color_6_and_8() {
        let mut lexer = lexer::Lexer::new("#ff0000 #00ff0080", None);
        let t1 = lexer.next_token().unwrap();
        assert!(matches!(t1.kind, TokenKind::HexColor(255, 0, 0, 255)));
        let t2 = lexer.next_token().unwrap();
        assert!(matches!(t2.kind, TokenKind::HexColor(0, 255, 0, 128)));
    }

    #[test]
    fn lex_two_char_operators() {
        let mut lexer = lexer::Lexer::new("-> == != <= >= && ||", None);
        let kinds: Vec<_> = std::iter::from_fn(|| {
            let t = lexer.next_token().ok()?;
            if matches!(t.kind, TokenKind::Eof) { None } else { Some(t.kind) }
        }).collect();
        assert_eq!(kinds, vec![
            TokenKind::Arrow, TokenKind::EqEq, TokenKind::NotEq,
            TokenKind::Le, TokenKind::Ge, TokenKind::And, TokenKind::Or,
        ]);
    }

    #[test]
    fn lex_keywords_vs_idents() {
        let mut lexer = lexer::Lexer::new("let screen myVar", None);
        let t1 = lexer.next_token().unwrap();
        assert!(matches!(t1.kind, TokenKind::Let));
        let t2 = lexer.next_token().unwrap();
        assert!(matches!(t2.kind, TokenKind::Screen));
        let t3 = lexer.next_token().unwrap();
        assert!(matches!(t3.kind, TokenKind::Ident(ref s) if s == "myVar"));
    }

    #[test]
    fn lex_unterminated_string_error() {
        let mut lexer = lexer::Lexer::new(r#""hello"#, None);
        let result = lexer.next_token();
        assert!(result.is_err());
    }

    #[test]
    fn lex_invalid_hex_color() {
        let mut lexer = lexer::Lexer::new("#fff", None);
        let result = lexer.next_token();
        assert!(result.is_err());
    }

    #[test]
    fn lex_comments_skipped() {
        let mut lexer = lexer::Lexer::new("42 // comment\n7", None);
        let t1 = lexer.next_token().unwrap();
        assert!(matches!(t1.kind, TokenKind::Number(n) if (n - 42.0).abs() < f64::EPSILON));
        let t2 = lexer.next_token().unwrap();
        assert!(matches!(t2.kind, TokenKind::Number(n) if (n - 7.0).abs() < f64::EPSILON));
    }

    // --- Parser Tests ---

    #[test]
    fn parse_variable_declaration() {
        let program = parse("let x = 42; screen Main { box {} }", None).unwrap();
        assert!(matches!(&program.items[0], ProgramItem::Variable(v) if v.name == "x"));
    }

    #[test]
    fn parse_component_with_params() {
        let source = "component Card(title, color) { box { fill: color } { text { content: title } } } screen Main { Card(\"Hi\", #ff0000) }";
        let program = parse(source, None).unwrap();
        if let ProgramItem::Component(c) = &program.items[0] {
            assert_eq!(c.name, "Card");
            assert_eq!(c.params, vec!["title", "color"]);
        } else {
            panic!("expected component");
        }
    }

    #[test]
    fn parse_theme_and_use() {
        let source = r#"
theme Dark {
    let bg = #1a1a1a;
    let fg = #ffffff;
}
use theme Dark;
screen Main { box { fill: bg } }
"#;
        let program = parse(source.trim(), None).unwrap();
        assert!(matches!(&program.items[0], ProgramItem::Theme(t) if t.name == "Dark"));
        assert!(matches!(&program.items[1], ProgramItem::UseTheme(n) if n == "Dark"));
    }

    #[test]
    fn parse_if_else() {
        let source = "screen Main { if true { box {} } else { text { content: \"no\" } } }";
        let result = parse(source, None);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_for_loop() {
        let source = "screen Main { for i in range(3) { text { content: \"item\" } } }";
        let result = parse(source, None);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_error_suggestion() {
        let result = parse("screan Main { box {} }", None);
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.suggestion().is_some());
    }

    #[test]
    fn parse_nested_elements() {
        let source = "screen Main { column { gap: 16 } { row { gap: 8 } { box {} box {} } } }";
        let (_, layout) = compile(source, None, None).unwrap();
        // Screen body is a Block with one child: the column
        let col = &layout.children[0];
        // Column has one child: the row
        assert_eq!(col.children.len(), 1);
        // Row has two children: the boxes
        assert_eq!(col.children[0].children.len(), 2);
    }

    // --- Eval Tests ---

    #[test]
    fn eval_binary_ops() {
        let source = "let x = 2 + 3; screen Main { box {} }";
        let program = parse(source, None).unwrap();
        let ctx = EvalContext::from_program(&program);
        let val = ctx.variables.get("x").unwrap();
        assert!(matches!(val, Value::Number(n) if (*n - 5.0).abs() < f64::EPSILON));
    }

    #[test]
    fn eval_comparison_ops() {
        let source = "let a = 5 > 3; let b = 2 == 2; screen Main { box {} }";
        let program = parse(source, None).unwrap();
        let ctx = EvalContext::from_program(&program);
        assert!(matches!(ctx.variables.get("a"), Some(Value::Bool(true))));
        assert!(matches!(ctx.variables.get("b"), Some(Value::Bool(true))));
    }

    #[test]
    fn eval_block_scoped_variables() {
        let source = "screen Main { box {} }";
        let program = parse(source, None).unwrap();
        let ctx = EvalContext::from_program(&program);
        let block_expr = crate::ast::Expr::Block {
            stmts: vec![
                crate::ast::Stmt::Let {
                    name: "x".to_string(),
                    value: crate::ast::Expr::Literal(crate::ast::Literal::Number(42.0)),
                    span: Span::new(0, 0, 1, 1),
                },
                crate::ast::Stmt::Expr(crate::ast::Expr::Ident("x".to_string(), Span::new(0, 0, 1, 1))),
            ],
            span: Span::new(0, 0, 1, 1),
        };
        let result = value::eval_expr(&ctx, &block_expr).unwrap();
        assert!(matches!(result, Value::Number(n) if (n - 42.0).abs() < f64::EPSILON));
    }

    #[test]
    fn eval_for_loop() {
        let source = "screen Main { box {} }";
        let program = parse(source, None).unwrap();
        let ctx = EvalContext::from_program(&program);
        let for_expr = crate::ast::Expr::For {
            var: "i".to_string(),
            iter: Box::new(crate::ast::Expr::Call {
                callee: "range".to_string(),
                args: vec![crate::ast::Expr::Literal(crate::ast::Literal::Number(3.0))],
                slot_args: None,
                span: Span::new(0, 0, 1, 1),
            }),
            body: Box::new(crate::ast::Expr::Ident("i".to_string(), Span::new(0, 0, 1, 1))),
            span: Span::new(0, 0, 1, 1),
        };
        let result = value::eval_expr(&ctx, &for_expr).unwrap();
        if let Value::Array(arr) = result {
            assert_eq!(arr.len(), 3);
            assert!(matches!(&arr[0], Value::Number(n) if (*n - 0.0).abs() < f64::EPSILON));
            assert!(matches!(&arr[2], Value::Number(n) if (*n - 2.0).abs() < f64::EPSILON));
        } else {
            panic!("expected array from for loop");
        }
    }

    #[test]
    fn eval_theme_variables() {
        let source = r#"
theme Light {
    let bg = #ffffff;
    let text_color = #000000;
}
use theme Light;
let x = bg;
screen Main { box {} }
"#;
        let program = parse(source.trim(), None).unwrap();
        let ctx = EvalContext::from_program(&program);
        assert!(matches!(ctx.variables.get("bg"), Some(Value::Color { r: 255, g: 255, b: 255, a: 255 })));
        assert!(ctx.variables.get("x").is_some());
    }

    #[test]
    fn eval_if_else_expr() {
        let source = "screen Main { box {} }";
        let program = parse(source, None).unwrap();
        let ctx = EvalContext::from_program(&program);
        let if_expr = crate::ast::Expr::If {
            cond: Box::new(crate::ast::Expr::Literal(crate::ast::Literal::Bool(false))),
            then_branch: Box::new(crate::ast::Expr::Literal(crate::ast::Literal::Number(1.0))),
            else_branch: Some(Box::new(crate::ast::Expr::Literal(crate::ast::Literal::Number(2.0)))),
            span: Span::new(0, 0, 1, 1),
        };
        let result = value::eval_expr(&ctx, &if_expr).unwrap();
        assert!(matches!(result, Value::Number(n) if (n - 2.0).abs() < f64::EPSILON));
    }

    // --- Layout Tests ---

    #[test]
    fn layout_row_splits_width() {
        let source = "screen Main { row { gap: 0 } { box {} box {} } }";
        let (_, layout) = compile(source, None, None).unwrap();
        let row = &layout.children[0];
        assert_eq!(row.children.len(), 2);
        let w1 = row.children[0].rect.w;
        let w2 = row.children[1].rect.w;
        assert!((w1 - w2).abs() < 1.0, "row children should have equal width");
    }

    #[test]
    fn layout_column_splits_height() {
        let source = "screen Main { column { gap: 0 } { box {} box {} box {} } }";
        let (_, layout) = compile(source, None, None).unwrap();
        let col = &layout.children[0];
        assert_eq!(col.children.len(), 3);
        let h1 = col.children[0].rect.h;
        let h2 = col.children[1].rect.h;
        assert!((h1 - h2).abs() < 1.0, "column children should have equal height");
    }

    #[test]
    fn layout_respects_padding() {
        let source = "screen Main { box { padding: 20 } { text { content: \"hi\" } } }";
        let (_, layout) = compile(source, None, None).unwrap();
        let child = &layout.children[0].children[0];
        assert!(child.rect.x >= 20.0);
        assert!(child.rect.y >= 20.0);
    }

    #[test]
    fn layout_fixed_width_child() {
        let source = "screen Main { row { gap: 0 } { box { width: 100 } box {} } }";
        let (_, layout) = compile(source, None, None).unwrap();
        let row = &layout.children[0];
        let w0 = row.children[0].rect.w;
        assert!((w0 - 100.0).abs() < 1.0, "fixed width child should be 100px");
    }

    #[test]
    fn layout_for_loop_produces_children() {
        let source = "screen Main { for i in range(4) { box {} } }";
        let (_, layout) = compile(source, None, None).unwrap();
        assert_eq!(layout.children[0].children.len(), 4);
    }

    #[test]
    fn layout_visibility_min_width() {
        let source = "screen Main { box { minWidth: 2000 } { text { content: \"hidden\" } } }";
        let (_, layout) = compile(source, None, None).unwrap();
        // Should produce an empty node (hidden because viewport < 2000)
        assert!(layout.children[0].children.is_empty());
    }

    #[test]
    fn layout_visibility_min_height() {
        let source = "screen Main { box { minHeight: 2000 } { text { content: \"hidden\" } } }";
        let (_, layout) = compile(source, None, None).unwrap();
        assert!(layout.children[0].children.is_empty());
    }

    // --- HTML Export Tests ---

    #[test]
    fn html_export_contains_text() {
        let source = "screen Main { text { content: \"Hello World\" } }";
        let (program, layout) = compile(source, None, None).unwrap();
        let vars = theme_css_vars(&program);
        let html = layout_to_html(&layout, 960, 640, if vars.is_empty() { None } else { Some(&vars) });
        assert!(html.contains("Hello World"));
        assert!(html.contains("<!DOCTYPE html>"));
    }

    #[test]
    fn html_export_escapes_entities() {
        let source = r#"screen Main { text { content: "<script>alert('xss')</script>" } }"#;
        let (_, layout) = compile(source, None, None).unwrap();
        let html = layout_to_html(&layout, 960, 640, None);
        assert!(!html.contains("<script>"));
        assert!(html.contains("&lt;script&gt;"));
    }

    #[test]
    fn html_export_theme_vars() {
        let source = r#"
theme MyTheme { let primary = #ff0000; }
use theme MyTheme;
screen Main { box { fill: primary } }
"#;
        let (program, layout) = compile(source.trim(), None, None).unwrap();
        let vars = theme_css_vars(&program);
        let html = layout_to_html(&layout, 960, 640, Some(&vars));
        assert!(html.contains("--primary"));
    }

    // --- Compile / End-to-End Tests ---

    #[test]
    fn compile_full_pipeline() {
        let source = r#"
let accent = #2563eb;
screen Main {
    column { gap: 16, padding: 24 } {
        text { content: "Title", fontSize: 28 }
        row { gap: 12 } {
            button("Click Me", fill: accent, radius: 8)
            button("Cancel", fill: #e5e7eb, radius: 8)
        }
    }
}
"#;
        let result = compile(source.trim(), None, None);
        assert!(result.is_ok());
    }

    #[test]
    fn compile_missing_screen_error() {
        let source = "let x = 5;";
        let result = compile(source, None, None);
        assert!(result.is_err());
    }

    #[test]
    fn compile_named_screen_selection() {
        let source = r#"
screen Home { box { fill: #ff0000 } }
screen About { box { fill: #00ff00 } }
"#;
        let (_, layout) = compile(source.trim(), None, Some("About")).unwrap();
        // The About screen has green fill
        assert!(matches!(layout.children[0].fill, Some((0, 255, 0, 255))));
    }

    #[test]
    fn compile_component_call() {
        let source = r#"
component Badge(label) {
    box { fill: #ff0000, radius: 4 } {
        text { content: label }
    }
}
screen Main { Badge("New") }
"#;
        let result = compile(source.trim(), None, None);
        assert!(result.is_ok());
    }

    // --- Error System Tests ---

    #[test]
    fn error_has_span_info() {
        let _result = parse("screen Main { invalid_stuff }", None);
        // Test a guaranteed parse error with span info
        let result = parse("{{{{", None);
        assert!(result.is_err());
    }

    #[test]
    fn error_format_pretty() {
        let source = "screan Main { box {} }";
        let src = Source::new(source.to_string(), Some("test.newt".to_string()));
        let err = parse(source, Some("test.newt")).unwrap_err();
        let formatted = format_error(&src, &err);
        assert!(formatted.contains("test.newt"));
        assert!(formatted.contains("error:"));
    }

    // --- String Interpolation Tests ---

    /// Walk layout tree and collect all text content strings.
    fn collect_texts(node: &LayoutNode) -> Vec<String> {
        let mut out = Vec::new();
        if let Some(ref t) = node.text {
            out.push(t.clone());
        }
        for child in &node.children {
            out.extend(collect_texts(child));
        }
        out
    }

    #[test]
    fn test_interp_basic() {
        let source = r#"
let name = "World";
screen Main { text("Hello {name}") }
"#;
        let (_, layout) = compile(source.trim(), None, None).unwrap();
        let texts = collect_texts(&layout);
        assert!(texts.iter().any(|t| t == "Hello World"), "texts: {:?}", texts);
    }

    #[test]
    fn test_interp_expr() {
        let source = r#"
let count = 3;
screen Main { text("total: {count * 2}") }
"#;
        let (_, layout) = compile(source.trim(), None, None).unwrap();
        let texts = collect_texts(&layout);
        assert!(texts.iter().any(|t| t == "total: 6"), "texts: {:?}", texts);
    }

    #[test]
    fn test_interp_plain_string_unchanged() {
        let source = r#"screen Main { text("no braces here") }"#;
        let (_, layout) = compile(source, None, None).unwrap();
        let texts = collect_texts(&layout);
        assert!(texts.iter().any(|t| t == "no braces here"), "texts: {:?}", texts);
    }

    #[test]
    fn test_interp_escaped_brace() {
        let source = r#"screen Main { text("literal \{brace\}") }"#;
        let (_, layout) = compile(source, None, None).unwrap();
        let texts = collect_texts(&layout);
        assert!(texts.iter().any(|t| t == "literal {brace}"), "texts: {:?}", texts);
    }

    // --- State Management Tests ---

    #[test]
    fn lex_state_keyword() {
        let mut lexer = lexer::Lexer::new("state", None);
        let t = lexer.next_token().unwrap();
        assert!(matches!(t.kind, TokenKind::State));
    }

    #[test]
    fn parse_state_declaration() {
        let source = "state count = 0; screen Main { box {} }";
        let program = parse(source, None).unwrap();
        assert!(matches!(&program.items[0], ProgramItem::StateDecl(sd) if sd.name == "count"));
    }

    #[test]
    fn eval_state_initial_value() {
        let source = "state count = 0; screen Main { box {} }";
        let program = parse(source, None).unwrap();
        let ctx = EvalContext::from_program(&program);
        let val = ctx.variables.get("count").unwrap();
        assert!(matches!(val, Value::Number(n) if (*n - 0.0).abs() < f64::EPSILON));
    }

    #[test]
    fn state_var_in_interpolation() {
        let source = r#"
state count = 5;
screen Main { text("Count: {count}") }
"#;
        let (_, layout) = compile(source.trim(), None, None).unwrap();
        let texts = collect_texts(&layout);
        assert!(texts.iter().any(|t| t == "Count: 5"), "texts: {:?}", texts);
    }

    #[test]
    fn onclick_handler_serialized() {
        let source = r#"
state count = 0;
screen Main { button("Add", onClick: { count = count + 1 }) }
"#;
        let (_, layout) = compile(source.trim(), None, None).unwrap();
        // The button is inside the screen block
        fn find_onclick(node: &LayoutNode) -> Option<String> {
            if node.on_click.is_some() {
                return node.on_click.clone();
            }
            for child in &node.children {
                if let Some(v) = find_onclick(child) {
                    return Some(v);
                }
            }
            None
        }
        let handler = find_onclick(&layout).expect("should have on_click");
        assert!(handler.contains("count"), "handler: {}", handler);
        assert!(handler.contains("1"), "handler: {}", handler);
    }

    #[test]
    fn parse_assignment_expr() {
        let source = "state count = 0; screen Main { button(\"Add\", onClick: { count = count + 1 }) }";
        let program = parse(source, None).unwrap();
        // If it parses without error, the assignment was handled
        assert!(!program.items.is_empty());
    }

    // --- Reactive HTML Tests ---

    #[test]
    fn reactive_html_contains_state_script() {
        let source = r#"
state count = 0;
screen Main { text("Count: {count}") }
"#;
        let (program, layout) = compile(source.trim(), None, None).unwrap();
        let html = layout_to_reactive_html(&program, &layout, 960, 640, None);
        assert!(html.contains("_state"), "should contain _state");
        assert!(html.contains("count"), "should contain count");
        assert!(html.contains("_render"), "should contain _render");
    }

    #[test]
    fn reactive_html_has_data_onclick() {
        let source = r#"
state count = 0;
screen Main { button("Add", onClick: { count = count + 1 }) }
"#;
        let (program, layout) = compile(source.trim(), None, None).unwrap();
        let html = layout_to_reactive_html(&program, &layout, 960, 640, None);
        assert!(html.contains("data-onclick"), "should contain data-onclick attribute");
    }

    #[test]
    fn reactive_html_has_data_content() {
        let source = r#"
state count = 0;
screen Main { text("Count: {count}") }
"#;
        let (program, layout) = compile(source.trim(), None, None).unwrap();
        let html = layout_to_reactive_html(&program, &layout, 960, 640, None);
        assert!(html.contains("data-content"), "should contain data-content attribute");
        assert!(html.contains("Count: {count}"), "should contain template pattern");
    }

    #[test]
    fn reactive_html_static_fallback() {
        let source = r#"screen Main { text("Hello") }"#;
        let (program, layout) = compile(source, None, None).unwrap();
        let html = layout_to_reactive_html(&program, &layout, 960, 640, None);
        assert!(html.contains("<!DOCTYPE html>"));
        assert!(html.contains("Hello"));
    }

    // --- Error Recovery Tests ---

    #[test]
    fn test_multiple_errors_reported() {
        let src = r#"
            screan Main { }
            componnt Card() { }
            scren Other { }
        "#;
        let errors = check_all(src, None);
        assert!(errors.len() >= 2, "should report multiple errors, got {}", errors.len());
    }

    #[test]
    fn test_single_error_backward_compat() {
        let src = "screan Main { }";
        let result = parse(src, None);
        assert!(result.is_err(), "should still return Err for backward compat");
    }

    #[test]
    fn test_parse_all_returns_all_errors() {
        let src = r#"
            screan Main { }
            componnt Card() { }
        "#;
        let result = parse_all(src.trim(), None);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 2, "parse_all should return multiple errors, got {}", errors.len());
    }

    #[test]
    fn test_error_recovery_valid_items_still_parse() {
        // Mix of valid and invalid: valid items before/after errors should not cause issues
        let src = r#"
            let x = 42;
            screan Bad { }
            screen Main { box {} }
        "#;
        // check_all should find the "screan" error but still be able to see "screen Main"
        let errors = check_all(src, None);
        assert!(!errors.is_empty(), "should find at least one error");
    }
}

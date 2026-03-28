//! WASM wrapper for the Newt compiler.
//!
//! Exposes the pure-Rust parts of the Newt compiler (parser, layout engine,
//! HTML/React/JSON exporters) to the browser via wasm-bindgen.
//!
//! The heavy native dependencies (wgpu, winit, tokio, axum) are excluded — this
//! crate re-includes only the source files that compile cleanly to wasm32.

// Re-include pure-Rust source modules from the main compiler crate via #[path].
// This avoids a full crate dependency on newter-compiler (which pulls in wgpu etc.).
#[path = "../../../newter-compiler/src/error.rs"]
pub mod error;
#[path = "../../../newter-compiler/src/ast.rs"]
pub mod ast;
#[path = "../../../newter-compiler/src/lexer.rs"]
pub mod lexer;
#[path = "../../../newter-compiler/src/parser.rs"]
pub mod parser;
#[path = "../../../newter-compiler/src/value.rs"]
pub mod value;
#[path = "../../../newter-compiler/src/layout.rs"]
pub mod layout;
#[path = "../../../newter-compiler/src/html.rs"]
pub mod html;
#[path = "../../../newter-compiler/src/react.rs"]
pub mod react;

use wasm_bindgen::prelude::*;

use ast::{Program, ProgramItem, ScreenDecl};
use error::NewtError;
use layout::Rect;
use value::EvalContext;

// ── Constants ──────────────────────────────────────────────────────────────

const DEFAULT_VIEWPORT_W: f32 = 960.0;
const DEFAULT_VIEWPORT_H: f32 = 640.0;

// ── Internal helpers (mirror lib.rs but without filesystem / import resolution) ─

fn parse_source(source: &str) -> Result<Program, NewtError> {
    let mut p = parser::Parser::new(source, None)?;
    p.parse().map_err(|mut errors| {
        errors.drain(1..);
        errors.pop().unwrap()
    })
}

fn get_screen<'a>(program: &'a Program, name: Option<&str>) -> Option<&'a ScreenDecl> {
    for item in &program.items {
        if let ProgramItem::Screen(s) = item {
            if name.map_or(true, |n| s.name == n) {
                return Some(s);
            }
        }
    }
    None
}

fn screen_names(program: &Program) -> Vec<String> {
    program
        .items
        .iter()
        .filter_map(|i| match i {
            ProgramItem::Screen(s) => Some(s.name.clone()),
            _ => None,
        })
        .collect()
}

/// Core compile pipeline: parse -> find screen -> layout.
/// Import resolution is skipped (no filesystem in WASM).
fn compile_source(
    source: &str,
    screen_name: Option<&str>,
) -> Result<(Program, layout::LayoutNode), NewtError> {
    let trimmed = source.trim();
    let program = parse_source(trimmed)?;
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
    let rect = Rect::new(0.0, 0.0, DEFAULT_VIEWPORT_W, DEFAULT_VIEWPORT_H);
    let ctx = EvalContext::from_program(&program);
    let root = layout::layout_tree(&ctx, &screen.body, rect)?;
    Ok((program, root))
}

fn err_to_js(e: NewtError) -> JsValue {
    JsValue::from_str(&e.to_string())
}

// ── Public wasm_bindgen API ────────────────────────────────────────────────

/// Compile Newt source to a standalone HTML page.
#[wasm_bindgen]
pub fn compile_to_html(source: &str) -> Result<String, JsValue> {
    let (program, root) = compile_source(source, None).map_err(err_to_js)?;
    // Collect theme CSS variables
    let ctx = EvalContext::from_program(&program);
    let mut css_vars = Vec::new();
    for (name, val) in &ctx.variables {
        if let value::Value::Color { r, g, b, a } = val {
            let hex = if *a == 255 {
                format!("#{:02x}{:02x}{:02x}", r, g, b)
            } else {
                format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
            };
            css_vars.push((name.clone(), hex));
        }
    }
    let html = html::layout_to_html(
        &root,
        DEFAULT_VIEWPORT_W as u32,
        DEFAULT_VIEWPORT_H as u32,
        Some(&css_vars),
    );
    Ok(html)
}

/// Compile Newt source to a React JSX component.
#[wasm_bindgen]
pub fn compile_to_react(source: &str) -> Result<String, JsValue> {
    let (program, root) = compile_source(source, None).map_err(err_to_js)?;
    let jsx = react::layout_to_react(&program, &root);
    Ok(jsx)
}

/// Compile Newt source and serialize the layout tree as JSON.
#[wasm_bindgen]
pub fn compile_to_json(source: &str) -> Result<String, JsValue> {
    let (_program, root) = compile_source(source, None).map_err(err_to_js)?;
    serde_json::to_string_pretty(&root).map_err(|e| JsValue::from_str(&e.to_string()))
}

/// Parse Newt source and return "ok" on success or the error message on failure.
#[wasm_bindgen]
pub fn check_syntax(source: &str) -> Result<String, JsValue> {
    match parse_source(source.trim()) {
        Ok(_) => Ok("ok".to_string()),
        Err(e) => {
            let src = error::Source::new(source.to_string(), None);
            Ok(error::format_error(&src, &e))
        }
    }
}

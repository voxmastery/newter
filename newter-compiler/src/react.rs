//! Generate React JSX from the Newt layout tree.
//!
//! Maps Newt elements to HTML/React equivalents with inline styles.
//! The output is a self-contained React component.

use crate::ast::ProgramItem;
use crate::layout::{LayoutKind, LayoutNode};
use crate::value::{eval_expr, EvalContext, Value};
use crate::Program;
use std::fmt::Write;

fn hex(r: u8, g: u8, b: u8, a: u8) -> String {
    if a == 255 {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        format!("rgba({}, {}, {}, {:.2})", r, g, b, a as f32 / 255.0)
    }
}

fn escape_jsx(s: &str) -> String {
    s.replace('{', "\\{").replace('}', "\\}")
}

/// Generate a complete React component from a Newt program.
pub fn layout_to_react(
    program: &Program,
    root: &LayoutNode,
) -> String {
    let mut out = String::new();

    // Collect state variables
    let mut state_vars: Vec<(String, String)> = Vec::new();
    let ctx = EvalContext::from_program(program);
    for item in &program.items {
        if let ProgramItem::StateDecl(sd) = item {
            let js_val = match eval_expr(&ctx, &sd.initial_value) {
                Ok(Value::Number(n)) => {
                    if n == (n as i64) as f64 {
                        format!("{}", n as i64)
                    } else {
                        format!("{}", n)
                    }
                }
                Ok(Value::String(s)) => format!("\"{}\"", s),
                Ok(Value::Bool(b)) => format!("{}", b),
                Ok(Value::Color { r, g, b, a }) => format!("\"{}\"", hex(r, g, b, a)),
                _ => "null".to_string(),
            };
            state_vars.push((sd.name.clone(), js_val));
        }
    }

    // Imports
    out.push_str("import React");
    if !state_vars.is_empty() {
        out.push_str(", { useState }");
    }
    out.push_str(" from 'react';\n\n");

    // Component
    out.push_str("export default function NewtApp() {\n");

    // State declarations
    for (name, val) in &state_vars {
        let capitalized = format!("{}{}", &name[..1].to_uppercase(), &name[1..]);
        writeln!(out, "  const [{name}, set{capitalized}] = useState({val});").unwrap();
    }
    if !state_vars.is_empty() {
        out.push('\n');
    }

    out.push_str("  return (\n");
    emit_jsx(&mut out, root, 2, &state_vars);
    out.push_str("  );\n");
    out.push_str("}\n");

    out
}

fn indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

fn emit_jsx(out: &mut String, n: &LayoutNode, depth: usize, state_vars: &[(String, String)]) {
    let tag = jsx_tag(n);
    let style = jsx_style(n);
    let mut extra_props = String::new();

    // Handle onClick for state mutations
    if let Some(ref onclick) = n.on_click {
        let handler = convert_onclick(onclick, state_vars);
        write!(extra_props, " onClick={{() => {{ {} }}}}", handler).unwrap();
    }

    // Self-closing tags for leaf elements
    let is_leaf = n.children.is_empty() && n.text.is_none();

    indent(out, depth);
    if is_leaf {
        writeln!(out, "<{tag} style={{{{{style}}}}}{extra_props} />").unwrap();
    } else if let Some(ref text) = n.text {
        // Text content with possible interpolation
        let content = convert_interpolation(text, state_vars);
        writeln!(out, "<{tag} style={{{{{style}}}}}{extra_props}>{content}</{tag}>").unwrap();
    } else {
        writeln!(out, "<{tag} style={{{{{style}}}}}{extra_props}>").unwrap();
        for child in &n.children {
            emit_jsx(out, child, depth + 1, state_vars);
        }
        indent(out, depth);
        writeln!(out, "</{tag}>").unwrap();
    }
}

fn jsx_tag(n: &LayoutNode) -> &'static str {
    match n.kind {
        LayoutKind::Button => "button",
        LayoutKind::Input => "input",
        LayoutKind::Image => "img",
        _ => "div",
    }
}

fn jsx_style(n: &LayoutNode) -> String {
    let mut parts: Vec<String> = Vec::new();

    // Layout
    match n.kind {
        LayoutKind::Row => {
            parts.push("display: 'flex'".into());
            parts.push("flexDirection: 'row'".into());
        }
        LayoutKind::Column => {
            parts.push("display: 'flex'".into());
            parts.push("flexDirection: 'column'".into());
        }
        LayoutKind::Center => {
            parts.push("display: 'flex'".into());
            parts.push("alignItems: 'center'".into());
            parts.push("justifyContent: 'center'".into());
        }
        LayoutKind::Grid => {
            parts.push("display: 'grid'".into());
        }
        LayoutKind::Stack => {
            parts.push("position: 'relative'".into());
        }
        _ => {}
    }

    // Gap (from rect spacing — approximate from children positions)
    // We encode gap in the layout, but for JSX we use the gap property
    // This is a simplification; the actual gap is computed during layout

    // Fill
    if let Some((r, g, b, a)) = n.fill {
        parts.push(format!("background: '{}'", hex(r, g, b, a)));
    }

    // Stroke
    if let Some((r, g, b, a)) = n.stroke {
        let w = n.stroke_width.unwrap_or(1.0).max(0.0);
        parts.push(format!("border: '{w}px solid {}'", hex(r, g, b, a)));
    }

    // Radius
    if n.radius > 0.0 {
        parts.push(format!("borderRadius: {}", n.radius as i32));
    }

    // Typography
    if n.font_size > 0.0 && n.font_size != 16.0 {
        parts.push(format!("fontSize: {}", n.font_size as i32));
    }
    if let Some(ref w) = n.font_weight {
        parts.push(format!("fontWeight: '{w}'"));
    }

    // Shadow
    if let Some(sh) = n.shadow {
        if sh > 0.0 {
            parts.push(format!(
                "boxShadow: '0 {}px {}px rgba(0,0,0,0.15)'",
                sh as i32,
                (sh * 1.5) as i32
            ));
        }
    }

    // Padding (approximated from the rect)
    parts.push("padding: 8".into());

    parts.join(", ")
}

/// Convert Newt onClick expressions to React state setters.
/// e.g. "count = count + 1" -> "setCount(count + 1)"
fn convert_onclick(expr: &str, state_vars: &[(String, String)]) -> String {
    let mut result = Vec::new();
    for stmt in expr.split(';') {
        let stmt = stmt.trim();
        if stmt.is_empty() {
            continue;
        }
        if let Some(eq_pos) = stmt.find('=') {
            let lhs = stmt[..eq_pos].trim();
            let rhs = stmt[eq_pos + 1..].trim();
            // Check if lhs is a state variable
            let is_state = state_vars.iter().any(|(name, _)| name == lhs);
            if is_state {
                let capitalized = format!("{}{}", &lhs[..1].to_uppercase(), &lhs[1..]);
                // Handle negation: !varName
                if rhs.starts_with('!') {
                    result.push(format!("set{}(!{})", capitalized, &rhs[1..]));
                } else {
                    result.push(format!("set{}({})", capitalized, rhs));
                }
            }
        }
    }
    result.join("; ")
}

/// Convert Newt string interpolation {expr} to JSX template literals.
/// e.g. "Count: {count}" -> `Count: ${count}`
fn convert_interpolation(text: &str, _state_vars: &[(String, String)]) -> String {
    if text.contains('{') {
        let converted = text
            .replace('{', "${")
            .replace("\\${", "{"); // Handle escaped braces
        format!("`{}`", converted)
    } else {
        text.to_string()
    }
}

//! Export layout to standalone HTML so you can open it in a browser (no GPU needed).

use crate::ast::ProgramItem;
use crate::layout::{LayoutKind, LayoutNode};
use crate::value::{eval_expr, EvalContext, Value};
use crate::Program;
use std::fmt::Write;

fn hex(r: u8, g: u8, b: u8, a: u8) -> String {
    if a == 255 {
        format!("#{:02x}{:02x}{:02x}", r, g, b)
    } else {
        format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
    }
}

fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// Optional theme variables to emit as CSS custom properties (:root { --name: value; }).
/// Values should be valid CSS (e.g. "#ffffff" or "24px").
pub fn layout_to_html(
    root: &LayoutNode,
    viewport_w: u32,
    viewport_h: u32,
    theme_css_vars: Option<&[(String, String)]>,
) -> String {
    let mut out = String::new();
    out.push_str("<!DOCTYPE html>\n<html><head><meta charset=\"utf-8\"><title>Newt</title><style>");
    if let Some(vars) = theme_css_vars {
        out.push_str(":root{");
        for (name, value) in vars.iter() {
            out.push_str(&format!("--{}:{};", name.replace(' ', "-"), value));
        }
        out.push_str("}");
    }
    out.push_str("*{margin:0;box-sizing:border-box;}");
    out.push_str("body{margin:0;background:#f9fafb;font-family:'Inter',system-ui,-apple-system,sans-serif;-webkit-font-smoothing:antialiased;}");
    out.push_str("@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');");
    out.push_str(".root{position:relative;width:100%;height:100vh;min-height:640px;}");
    out.push_str(".n{position:absolute;box-sizing:border-box;transition:transform 150ms ease,box-shadow 150ms ease,opacity 150ms ease;}");
    out.push_str(".n[data-on-click]:hover,.n[data-onclick]:hover{transform:translateY(-1px);box-shadow:0 4px 12px rgba(0,0,0,0.1);cursor:pointer;}");
    out.push_str("@keyframes fadeIn{from{opacity:0;transform:translateY(8px);}to{opacity:1;transform:translateY(0);}}");
    out.push_str(".n{animation:fadeIn 300ms ease-out both;}");
    out.push_str("</style></head><body><div class=\"root\" style=\"width:");
    out.push_str(&viewport_w.to_string());
    out.push_str("px;height:");
    out.push_str(&viewport_h.to_string());
    out.push_str("px;\">");
    emit_node(&mut out, root);
    out.push_str("</div></body></html>");
    out
}

fn emit_node(out: &mut String, n: &LayoutNode) {
    let r = &n.rect;
    let style = node_style(n);
    let tag = match n.kind {
        LayoutKind::Text if n.text.is_some() => "div",
        _ => "div",
    };
    let mut attrs = String::new();
    if let Some(ref role) = n.role {
        attrs.push_str(&format!(" role=\"{}\"", escape(role)));
    }
    if let Some(ref aria) = n.aria_label {
        attrs.push_str(&format!(" aria-label=\"{}\"", escape(aria)));
    }
    if let Some(o) = n.focus_order {
        attrs.push_str(&format!(" tabindex=\"{}\"", o));
    }
    if let Some(ref s) = n.on_click {
        attrs.push_str(&format!(" data-on-click=\"{}\"", escape(s)));
    }
    if let Some(ref s) = n.href {
        attrs.push_str(&format!(" data-href=\"{}\"", escape(s)));
    }
    if let Some(ref s) = n.name {
        attrs.push_str(&format!(" name=\"{}\"", escape(s)));
    }
    write!(
        out,
        "<{} class=\"n\" style=\"left:{}px;top:{}px;width:{}px;height:{}px;{}\"{}>",
        tag,
        r.x as i32,
        r.y as i32,
        r.w as i32,
        r.h as i32,
        style,
        attrs
    )
    .unwrap();

    if let Some(ref t) = n.text {
        write!(out, "{}", escape(t)).unwrap();
    }
    for (i, c) in n.children.iter().enumerate() {
        // Stagger entrance animation for children
        if i > 0 {
            let delay = i * 50; // 50ms between each child
            write!(out, "<div style=\"animation-delay:{}ms;\">", delay).unwrap();
        }
        emit_node(out, c);
        if i > 0 {
            out.push_str("</div>");
        }
    }
    write!(out, "</{}>", tag).unwrap();
}

fn node_style(n: &LayoutNode) -> String {
    let mut s = String::new();
    if let Some((r, g, b, a)) = n.fill {
        s.push_str(&format!("background:{};", hex(r, g, b, a)));
    }
    if let Some((r, g, b, a)) = n.stroke {
        let w = n.stroke_width.unwrap_or(1.0).max(0.0) as i32;
        s.push_str(&format!("border:{}px solid {};", w, hex(r, g, b, a)));
    } else {
        s.push_str("border:none;");
    }
    if n.radius > 0.0 {
        s.push_str(&format!("border-radius:{}px;", n.radius as i32));
    }
    if n.font_size > 0.0 {
        s.push_str(&format!("font-size:{}px;", n.font_size as i32));
    }
    if let Some(ref w) = n.font_weight {
        s.push_str(&format!("font-weight:{};", w));
    }
    if let Some(sh) = n.shadow {
        if sh > 0.0 {
            const SHADOW_SPREAD_MULTIPLIER: f32 = 1.5;
            s.push_str(&format!("box-shadow:0 {}px {}px rgba(0,0,0,0.15);", sh as i32, (sh * SHADOW_SPREAD_MULTIPLIER) as i32));
        }
    }
    if let Some(ms) = n.transition_ms {
        if ms > 0.0 {
            s.push_str(&format!("transition:all {}ms ease;", ms as i32));
        }
    }
    s.push_str("display:flex;align-items:center;padding:4px 8px;overflow:hidden;");
    s
}

/// Collect state variable names and their JS-literal initial values from the program.
fn collect_state_vars(program: &Program) -> Vec<(String, String)> {
    let ctx = EvalContext::from_program(program);
    let mut vars = Vec::new();
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
                Ok(Value::String(s)) => format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\"")),
                Ok(Value::Bool(b)) => format!("{}", b),
                Ok(Value::Color { r, g, b, a }) => format!("\"{}\"", hex(r, g, b, a)),
                _ => "null".to_string(),
            };
            vars.push((sd.name.clone(), js_val));
        }
    }
    vars
}

/// Generate HTML with an embedded JS runtime that makes state variables reactive.
/// Clicking elements with `data-onclick` handlers mutates state and re-renders.
pub fn layout_to_reactive_html(
    program: &Program,
    root: &LayoutNode,
    viewport_w: u32,
    viewport_h: u32,
    theme_css_vars: Option<&[(String, String)]>,
) -> String {
    let mut out = String::new();
    out.push_str("<!DOCTYPE html>\n<html><head><meta charset=\"utf-8\"><title>Newt</title><style>");
    if let Some(vars) = theme_css_vars {
        out.push_str(":root{");
        for (name, value) in vars.iter() {
            out.push_str(&format!("--{}:{};", name.replace(' ', "-"), value));
        }
        out.push_str("}");
    }
    out.push_str("*{margin:0;box-sizing:border-box;}");
    out.push_str("body{margin:0;background:#f9fafb;font-family:'Inter',system-ui,-apple-system,sans-serif;-webkit-font-smoothing:antialiased;}");
    out.push_str("@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');");
    out.push_str(".root{position:relative;width:100%;height:100vh;min-height:640px;}");
    out.push_str(".n{position:absolute;box-sizing:border-box;transition:transform 150ms ease,box-shadow 150ms ease,opacity 150ms ease;}");
    out.push_str("[data-onclick]{cursor:pointer;user-select:none;}");
    out.push_str("[data-onclick]:hover{transform:translateY(-1px);box-shadow:0 4px 12px rgba(0,0,0,0.1);}");
    out.push_str("@keyframes fadeIn{from{opacity:0;transform:translateY(8px);}to{opacity:1;transform:translateY(0);}}");
    out.push_str(".n{animation:fadeIn 300ms ease-out both;}");
    out.push_str("</style></head><body><div id=\"_newt_root\" class=\"root\" style=\"width:");
    out.push_str(&viewport_w.to_string());
    out.push_str("px;height:");
    out.push_str(&viewport_h.to_string());
    out.push_str("px;\">");
    emit_reactive_node(&mut out, root);
    out.push_str("</div>");

    // Emit the JS runtime
    let state_vars = collect_state_vars(program);
    out.push_str("<script>\n");
    // Build state object
    out.push_str("const _state={");
    for (i, (name, val)) in state_vars.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        write!(out, "{}:{}", name, val).unwrap();
    }
    out.push_str("};\n");
    out.push_str(REACTIVE_RUNTIME);
    out.push_str("</script>\n");

    out.push_str("</body></html>");
    out
}

fn emit_reactive_node(out: &mut String, n: &LayoutNode) {
    let r = &n.rect;
    let style = node_style(n);
    let mut attrs = String::new();
    if let Some(ref role) = n.role {
        attrs.push_str(&format!(" role=\"{}\"", escape(role)));
    }
    if let Some(ref aria) = n.aria_label {
        attrs.push_str(&format!(" aria-label=\"{}\"", escape(aria)));
    }
    if let Some(o) = n.focus_order {
        attrs.push_str(&format!(" tabindex=\"{}\"", o));
    }
    if let Some(ref s) = n.on_click {
        attrs.push_str(&format!(" data-onclick=\"{}\"", escape(s)));
    }
    if let Some(ref s) = n.content_template {
        attrs.push_str(&format!(" data-content=\"{}\"", escape(s)));
    }
    if let Some(ref s) = n.href {
        attrs.push_str(&format!(" data-href=\"{}\"", escape(s)));
    }
    if let Some(ref s) = n.name {
        attrs.push_str(&format!(" name=\"{}\"", escape(s)));
    }
    write!(
        out,
        "<div class=\"n\" style=\"left:{}px;top:{}px;width:{}px;height:{}px;{}\"{}>",
        r.x as i32,
        r.y as i32,
        r.w as i32,
        r.h as i32,
        style,
        attrs
    )
    .unwrap();

    if let Some(ref t) = n.text {
        write!(out, "{}", escape(t)).unwrap();
    }
    for c in &n.children {
        emit_reactive_node(out, c);
    }
    out.push_str("</div>");
}

const REACTIVE_RUNTIME: &str = r#"
function _applyOnClick(expr) {
    for (const stmt of expr.split(/;\s*/)) {
        if (!stmt.trim()) continue;
        const m = stmt.match(/^(\w+)\s*=\s*(.+)$/);
        if (m) {
            _state[m[1]] = _evalExpr(m[2].trim());
        }
    }
    _render();
}

function _evalExpr(expr) {
    var e = expr;
    for (var k in _state) {
        if (_state.hasOwnProperty(k)) {
            e = e.replace(new RegExp('\\b' + k + '\\b', 'g'), JSON.stringify(_state[k]));
        }
    }
    try { return Function('"use strict"; return (' + e + ')')(); }
    catch(_) { return expr; }
}

function _interpolate(tpl) {
    return tpl.replace(/\{([^}]+)\}/g, function(_, expr) {
        var v = _evalExpr(expr);
        if (typeof v === 'number') return Number.isInteger(v) ? v.toString() : v.toString();
        return String(v);
    });
}

function _render() {
    document.querySelectorAll('[data-content]').forEach(function(el) {
        el.textContent = _interpolate(el.dataset.content);
    });
    document.querySelectorAll('[data-onclick]').forEach(function(el) {
        el.onclick = function() { _applyOnClick(el.dataset.onclick); };
    });
}

document.addEventListener('DOMContentLoaded', _render);
"#;

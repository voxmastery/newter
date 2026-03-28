//! Flex-like layout engine: computes bounds for each element from the AST.

use crate::ast::*;
use crate::error::NewtError;
use crate::value::{eval_expr, EvalContext, Value};
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct LayoutNode {
    pub kind: LayoutKind,
    pub rect: Rect,
    pub fill: Option<(u8, u8, u8, u8)>,
    pub stroke: Option<(u8, u8, u8, u8)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stroke_width: Option<f32>,
    pub radius: f32,
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_template: Option<String>,
    pub font_size: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_weight: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transition_ms: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aria_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub focus_order: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "onClick")]
    pub on_click: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub href: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aspect_ratio: Option<f32>,
    pub children: Vec<LayoutNode>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum LayoutKind {
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
    Modal,
}

impl LayoutNode {
    fn empty(kind: LayoutKind, rect: Rect) -> Self {
        Self {
            kind,
            rect,
            fill: None,
            stroke: None,
            stroke_width: None,
            radius: 0.0,
            text: None,
            content_template: None,
            font_size: 16.0,
            font_weight: None,
            shadow: None,
            transition_ms: None,
            role: None,
            aria_label: None,
            focus_order: None,
            on_click: None,
            href: None,
            name: None,
            aspect_ratio: None,
            children: Vec::new(),
        }
    }
}

fn get_prop_number(ctx: &EvalContext, props: &[Prop], name: &str, default: f32) -> f32 {
    for p in props {
        let match_name = match &p.name {
            PropName::Ident(s) => s == name,
            PropName::Width => name == "width",
            PropName::Height => name == "height",
            PropName::Padding => name == "padding",
            PropName::Gap => name == "gap",
            PropName::Radius => name == "radius",
            PropName::FontSize => name == "fontSize",
            PropName::MinWidth => name == "minWidth",
            PropName::MaxWidth => name == "maxWidth",
            PropName::MinHeight => name == "minHeight",
            PropName::MaxHeight => name == "maxHeight",
            PropName::Shadow => name == "shadow",
            PropName::Transition => name == "transition",
            _ => false,
        };
        if !match_name {
            continue;
        }
        match &p.value {
            PropValue::Number(n) => return *n as f32,
            PropValue::Expr(e) => {
                if let Ok(Value::Number(n)) = eval_expr(ctx, e) {
                    return n as f32;
                }
            }
            _ => {}
        }
    }
    default
}

fn get_prop_color(ctx: &EvalContext, props: &[Prop], name: &str) -> Option<(u8, u8, u8, u8)> {
    for p in props {
        let match_name = match &p.name {
            PropName::Ident(s) => s == name,
            PropName::Fill => name == "fill",
            PropName::Stroke => name == "stroke",
            _ => false,
        };
        if !match_name {
            continue;
        }
        match &p.value {
            PropValue::Color { r, g, b, a } => return Some((*r, *g, *b, *a)),
            PropValue::Expr(e) => {
                if let Ok(Value::Color { r, g, b, a }) = eval_expr(ctx, e) {
                    return Some((r, g, b, a));
                }
            }
            _ => {}
        }
    }
    None
}

/// Parse grid track list like "80 1fr 340" into (value, is_fr) per track.
fn parse_grid_tracks(s: &str) -> Vec<(f32, bool)> {
    let mut out = Vec::new();
    for part in s.split_whitespace() {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }
        if part.ends_with("fr") {
            let v: f32 = part[..part.len() - 2].trim().parse().unwrap_or(1.0);
            out.push((v.max(0.0), true));
        } else if let Ok(v) = part.parse::<f32>() {
            out.push((v.max(0.0), false));
        }
    }
    if out.is_empty() {
        out.push((1.0, true));
    }
    out
}

fn get_child_width(ctx: &EvalContext, expr: &Expr) -> f32 {
    match expr {
        Expr::Element { props, .. } => get_prop_number(ctx, props, "width", 0.0),
        _ => 0.0,
    }
}

fn get_child_height(ctx: &EvalContext, expr: &Expr) -> f32 {
    match expr {
        Expr::Element { props, .. } => get_prop_number(ctx, props, "height", 0.0),
        _ => 0.0,
    }
}

/// Constrain rect to fit inside the given space with width/height = ratio (ratio > 0).
fn constrain_aspect_ratio(rect: Rect, ratio: f32) -> Rect {
    if ratio <= 0.0 {
        return rect;
    }
    let (w, h) = (rect.w, rect.h);
    let target_h = w / ratio;
    if target_h <= h && target_h > 0.0 {
        Rect::new(rect.x, rect.y, w, target_h)
    } else {
        let target_w = h * ratio;
        Rect::new(rect.x, rect.y, target_w.min(w), h)
    }
}

fn get_prop_string(ctx: &EvalContext, props: &[Prop], name: &str) -> Option<String> {
    for p in props {
        let match_name = match &p.name {
            PropName::Ident(s) => s == name,
            PropName::Content => name == "content",
            PropName::Role => name == "role",
            PropName::AriaLabel => name == "ariaLabel",
            _ => name == "onClick" || name == "href" || name == "name",
        };
        if !match_name {
            continue;
        }
        match &p.value {
            PropValue::String(s) => return Some(s.clone()),
            PropValue::Expr(e) => {
                if let Ok(Value::String(s)) = eval_expr(ctx, e) {
                    return Some(s);
                }
            }
            _ => {}
        }
    }
    None
}

fn expr_to_handler_string(expr: &Expr) -> String {
    match expr {
        Expr::Assignment { name, value, .. } => {
            format!("{} = {}", name, expr_to_handler_string(value))
        }
        Expr::Binary { left, op, right, .. } => {
            let op_str = match op {
                BinaryOp::Add => "+",
                BinaryOp::Sub => "-",
                BinaryOp::Mul => "*",
                BinaryOp::Div => "/",
                BinaryOp::Mod => "%",
                BinaryOp::Eq => "==",
                BinaryOp::Ne => "!=",
                BinaryOp::Lt => "<",
                BinaryOp::Le => "<=",
                BinaryOp::Gt => ">",
                BinaryOp::Ge => ">=",
                BinaryOp::And => "&&",
                BinaryOp::Or => "||",
            };
            format!(
                "{} {} {}",
                expr_to_handler_string(left),
                op_str,
                expr_to_handler_string(right)
            )
        }
        Expr::Ident(name, _) => name.clone(),
        Expr::Literal(Literal::Number(n)) => {
            if *n == (*n as i64) as f64 {
                format!("{}", *n as i64)
            } else {
                format!("{}", n)
            }
        }
        Expr::Literal(Literal::String(s)) => format!("\"{}\"", s),
        Expr::Literal(Literal::Bool(b)) => format!("{}", b),
        Expr::Block { stmts, .. } => {
            let parts: Vec<String> = stmts
                .iter()
                .filter_map(|s| match s {
                    Stmt::Expr(e) => Some(expr_to_handler_string(e)),
                    _ => None,
                })
                .collect();
            parts.join("; ")
        }
        Expr::Unary { op, inner, .. } => {
            let op_str = match op {
                UnaryOp::Not => "!",
                UnaryOp::Neg => "-",
            };
            format!("{}{}", op_str, expr_to_handler_string(inner))
        }
        _ => String::new(),
    }
}

fn get_prop_handler(props: &[Prop]) -> Option<String> {
    for p in props {
        let is_onclick = match &p.name {
            PropName::Ident(s) => s == "onClick",
            _ => false,
        };
        if !is_onclick {
            continue;
        }
        match &p.value {
            PropValue::String(s) => return Some(s.clone()),
            PropValue::Expr(e) => return Some(expr_to_handler_string(e)),
            _ => {}
        }
    }
    None
}

fn get_prop_content_template(props: &[Prop]) -> Option<String> {
    for p in props {
        let is_content = match &p.name {
            PropName::Content => true,
            PropName::Ident(s) => s == "content",
            _ => false,
        };
        if !is_content {
            continue;
        }
        if let PropValue::Expr(Expr::InterpolatedString { parts, .. }) = &p.value {
            let mut tpl = String::new();
            for seg in parts {
                match seg {
                    InterpSegment::Literal(s) => tpl.push_str(s),
                    InterpSegment::Expr(e) => {
                        tpl.push('{');
                        tpl.push_str(&expr_to_handler_string(e));
                        tpl.push('}');
                    }
                }
            }
            return Some(tpl);
        }
    }
    None
}

fn layout_kind_from_element(k: ElementKind) -> LayoutKind {
    use ElementKind::*;
    match k {
        Header | Footer | Container | Sidebar | Section | Widget => LayoutKind::Column,
        Accordion | Bento | Breadcrumb | Hamburger | Kebab | Meatballs | Doner
        | Tabs | Pagination | LinkList | Nav | Form | Feed | Carousel => LayoutKind::Column,
        Modal | Drawer | Popover => LayoutKind::Modal,
        Card | Box | ConfirmDialog | Toast | Notification | Alert | MessageBox
        | Tooltip | Loader | ProgressBar | Badge | Icon | Tag | Comment | Chart => LayoutKind::Box,
        Text => LayoutKind::Text,
        Row => LayoutKind::Row,
        Column => LayoutKind::Column,
        Grid => LayoutKind::Grid,
        Stack => LayoutKind::Stack,
        Center => LayoutKind::Center,
        Spacer | Separator => LayoutKind::Spacer,
        Image | Avatar | Skeleton | Rating => LayoutKind::Image,
        Button => LayoutKind::Button,
        Input | Password | Search | Select | Textarea | FileUpload | ColorPicker => LayoutKind::Input,
        Checkbox | Radio | Dropdown | Combobox | Multiselect | DatePicker | Picker
        | Slider | Stepper | Toggle => LayoutKind::Box,
        Table | Timeline | TreeView | CommandPalette => LayoutKind::Column,
        Splitter => LayoutKind::Row,
    }
}

pub fn layout_tree(ctx: &EvalContext, expr: &Expr, rect: Rect) -> Result<LayoutNode, NewtError> {
    layout_tree_with_viewport(ctx, expr, rect, rect)
}

fn layout_tree_with_viewport(
    ctx: &EvalContext,
    expr: &Expr,
    rect: Rect,
    viewport: Rect,
) -> Result<LayoutNode, NewtError> {
    match expr {
        Expr::Element { kind, props, children, .. } => {
            let layout_kind = layout_kind_from_element(*kind);
            let aspect_ratio = get_prop_number(ctx, props, "aspectRatio", 0.0);
            let rect = if aspect_ratio > 0.0 {
                constrain_aspect_ratio(rect, aspect_ratio)
            } else {
                rect
            };
            let padding = get_prop_number(ctx, props, "padding", 0.0);
            let gap = get_prop_number(ctx, props, "gap", 0.0);
            let fill = get_prop_color(ctx, props, "fill");
            let stroke = get_prop_color(ctx, props, "stroke");
            let stroke_width = get_prop_number(ctx, props, "strokeWidth", 1.0);
            let stroke_width = if stroke_width > 0.0 { Some(stroke_width) } else { None };
            let radius = get_prop_number(ctx, props, "radius", 0.0);
            let font_size = get_prop_number(ctx, props, "fontSize", 16.0);
            let text = get_prop_string(ctx, props, "content");
            let font_weight = get_prop_string(ctx, props, "fontWeight");
            let shadow = get_prop_number(ctx, props, "shadow", 0.0);
            let shadow = if shadow > 0.0 { Some(shadow) } else { None };
            let transition_ms = get_prop_number(ctx, props, "transition", 0.0);
            let transition_ms = if transition_ms > 0.0 { Some(transition_ms as i32 as f32) } else { None };
            let role = get_prop_string(ctx, props, "role");
            let aria_label = get_prop_string(ctx, props, "ariaLabel");
            let focus_order = get_prop_number(ctx, props, "focusOrder", f32::NAN);
            let focus_order = if !focus_order.is_nan() { Some(focus_order as i32) } else { None };
            let on_click = get_prop_handler(props);
            let content_template = get_prop_content_template(props);
            let href = get_prop_string(ctx, props, "href");
            let name = get_prop_string(ctx, props, "name");

            let min_w = get_prop_number(ctx, props, "minWidth", 0.0);
            let max_w = get_prop_number(ctx, props, "maxWidth", f32::MAX);
            let min_h = get_prop_number(ctx, props, "minHeight", 0.0);
            let max_h = get_prop_number(ctx, props, "maxHeight", f32::MAX);
            let visible = (min_w <= 0.0 || viewport.w >= min_w)
                && (max_w >= f32::MAX || viewport.w <= max_w)
                && (min_h <= 0.0 || viewport.h >= min_h)
                && (max_h >= f32::MAX || viewport.h <= max_h);
            if !visible {
                return Ok(LayoutNode::empty(layout_kind_from_element(*kind), rect));
            }

            let inner = Rect::new(
                rect.x + padding,
                rect.y + padding,
                (rect.w - 2.0 * padding).max(0.0),
                (rect.h - 2.0 * padding).max(0.0),
            );

            let child_nodes = match layout_kind {
                LayoutKind::Row => {
                    let mut nodes = Vec::new();
                    let total_children = children.len();
                    if total_children == 0 {
                    } else {
                        let mut fixed_w: f32 = 0.0;
                        let mut flexible_count = 0usize;
                        for child in children.iter() {
                            let cw = get_child_width(ctx, child);
                            if cw > 0.0 {
                                fixed_w += cw;
                            } else {
                                flexible_count += 1;
                            }
                        }
                        let total_gap = gap * (total_children as f32 - 1.0);
                        let remaining_w = (inner.w - total_gap - fixed_w).max(0.0);
                        let flexible_w = if flexible_count > 0 {
                            (remaining_w - gap * (flexible_count as f32 - 1.0)).max(0.0) / flexible_count as f32
                        } else {
                            0.0
                        };
                        let mut x = inner.x;
                        for child in children {
                            let cw = get_child_width(ctx, child);
                            let ch = get_child_height(ctx, child);
                            let child_w = if cw > 0.0 {
                                cw.min((inner.w - (x - inner.x)).max(0.0))
                            } else {
                                flexible_w
                            };
                            let child_h = if ch > 0.0 { ch.min(inner.h) } else { inner.h };
                            let child_rect = Rect::new(x, inner.y, child_w, child_h);
                            nodes.push(layout_tree_with_viewport(ctx, child, child_rect, viewport)?);
                            x += child_w + gap;
                        }
                    }
                    nodes
                }
                LayoutKind::Column => {
                    let mut nodes = Vec::new();
                    let total_children = children.len();
                    if total_children == 0 {
                    } else {
                        let mut fixed_h: f32 = 0.0;
                        let mut flexible_count = 0usize;
                        for child in children.iter() {
                            let ch = get_child_height(ctx, child);
                            if ch > 0.0 {
                                fixed_h += ch;
                            } else {
                                flexible_count += 1;
                            }
                        }
                        let total_gap = gap * (total_children as f32 - 1.0);
                        let remaining_h = (inner.h - total_gap - fixed_h).max(0.0);
                        let flexible_h = if flexible_count > 0 {
                            (remaining_h - gap * (flexible_count as f32 - 1.0)).max(0.0) / flexible_count as f32
                        } else {
                            0.0
                        };
                        let mut y = inner.y;
                        for child in children {
                            let cw = get_child_width(ctx, child);
                            let ch = get_child_height(ctx, child);
                            let child_w = if cw > 0.0 { cw.min(inner.w) } else { inner.w };
                            let child_h = if ch > 0.0 {
                                ch.min((inner.h - (y - inner.y)).max(0.0))
                            } else {
                                flexible_h
                            };
                            let child_rect = Rect::new(inner.x, y, child_w, child_h);
                            nodes.push(layout_tree_with_viewport(ctx, child, child_rect, viewport)?);
                            y += child_h + gap;
                        }
                    }
                    nodes
                }
                LayoutKind::Grid => {
                    let mut nodes = Vec::new();
                    let columns_str = get_prop_string(ctx, props, "columns").unwrap_or_else(|| "1fr".to_string());
                    let col_specs = parse_grid_tracks(&columns_str);
                    if col_specs.is_empty() || children.is_empty() {
                    } else {
                        let num_cols = col_specs.len();
                        let fixed_w: f32 = col_specs.iter().filter(|(_, fr)| !fr).map(|(v, _)| *v).sum();
                        let fr_total: f32 = col_specs.iter().filter(|(_, fr)| *fr).map(|(v, _)| *v).sum();
                        let remaining_w = (inner.w - fixed_w).max(0.0);
                        let col_widths: Vec<f32> = col_specs
                            .iter()
                            .map(|(v, fr)| if *fr { remaining_w * v / fr_total.max(1.0) } else { *v })
                            .collect();
                        let num_rows = (children.len() + num_cols - 1) / num_cols;
                        let row_h = if num_rows > 0 {
                            (inner.h / num_rows as f32).max(0.0)
                        } else {
                            0.0
                        };
                        let col_offsets: Vec<f32> = (0..num_cols)
                            .map(|i| inner.x + col_widths[..i].iter().sum::<f32>())
                            .collect();
                        for (i, child) in children.iter().enumerate() {
                            let row = i / num_cols;
                            let col = i % num_cols;
                            let x = col_offsets[col];
                            let child_rect = Rect::new(x, inner.y + row as f32 * row_h, col_widths[col], row_h);
                            nodes.push(layout_tree_with_viewport(ctx, child, child_rect, viewport)?);
                        }
                    }
                    nodes
                }
                LayoutKind::Modal => {
                    let mut nodes = Vec::new();
                    if children.is_empty() {
                    } else if children.len() == 1 {
                        nodes.push(layout_tree_with_viewport(ctx, &children[0], inner, viewport)?);
                    } else {
                        nodes.push(layout_tree_with_viewport(ctx, &children[0], inner, viewport)?);
                        let content_w = (inner.w * 0.8).min(400.0);
                        let content_h = (inner.h * 0.6).min(300.0);
                        let cx = inner.x + (inner.w - content_w) / 2.0;
                        let cy = inner.y + (inner.h - content_h) / 2.0;
                        let content_rect = Rect::new(cx, cy, content_w, content_h);
                        nodes.push(layout_tree_with_viewport(ctx, &children[1], content_rect, viewport)?);
                    }
                    nodes
                }
                LayoutKind::Stack | LayoutKind::Center | LayoutKind::Box | LayoutKind::Button | LayoutKind::Input => {
                    let mut nodes = Vec::new();
                    for child in children {
                        let r = if layout_kind == LayoutKind::Center && children.len() == 1 {
                            inner
                        } else {
                            inner
                        };
                        nodes.push(layout_tree_with_viewport(ctx, child, r, viewport)?);
                    }
                    nodes
                }
                LayoutKind::Text | LayoutKind::Spacer | LayoutKind::Image => Vec::new(),
            };

            Ok(LayoutNode {
                kind: layout_kind,
                rect,
                fill,
                stroke,
                stroke_width,
                radius,
                text,
                content_template,
                font_size,
                font_weight,
                shadow,
                transition_ms,
                role,
                aria_label,
                focus_order,
                on_click,
                href,
                name,
                aspect_ratio: if aspect_ratio > 0.0 { Some(aspect_ratio) } else { None },
                children: child_nodes,
            })
        }
        Expr::Call {
            callee,
            args,
            slot_args,
            span,
            ..
        } => {
            let comp = ctx
                .components
                .get(callee)
                .ok_or_else(|| NewtError::semantic(*span, format!("unknown component '{}'", callee)))?;
            let body = if let Some(ref slots) = slot_args {
                crate::ast::substitute_slots(&comp.body, slots)
            } else {
                comp.body.clone()
            };
            let mut new_ctx = EvalContext {
                variables: ctx.variables.clone(),
                components: ctx.components.clone(),
            };
            for (i, param) in comp.params.iter().enumerate() {
                if let Some(arg) = args.get(i) {
                    if let Ok(v) = eval_expr(ctx, arg) {
                        new_ctx.variables.insert(param.clone(), v);
                    }
                }
            }
            layout_tree_with_viewport(&new_ctx, &body, rect, viewport)
        }
        Expr::If {
            cond,
            then_branch,
            else_branch,
            ..
        } => {
            let cond_val = eval_expr(ctx, cond)?;
            let branch = if cond_val.as_bool().unwrap_or(false) {
                then_branch.as_ref()
            } else if let Some(eb) = else_branch {
                eb.as_ref()
            } else {
                return Ok(LayoutNode::empty(LayoutKind::Box, rect));
            };
            layout_tree_with_viewport(ctx, branch, rect, viewport)
        }
        Expr::For { var, iter, body, span, .. } => {
            let iter_val = eval_expr(ctx, iter)?;
            let arr = iter_val.as_array().map_err(|_| {
                NewtError::semantic(*span, "for loop expects an array or range(n)")
            })?;
            let mut child_nodes = Vec::new();
            let n = arr.len();
            if n == 0 {
                return Ok(LayoutNode::empty(LayoutKind::Column, rect));
            }
            let inner_h = rect.h.max(0.0) / n as f32;
            let mut y = rect.y;
            for val in arr.iter() {
                let mut new_ctx = EvalContext {
                    variables: ctx.variables.clone(),
                    components: ctx.components.clone(),
                };
                new_ctx.variables.insert(var.clone(), val.clone());
                let child_rect = Rect::new(rect.x, y, rect.w, inner_h);
                child_nodes.push(layout_tree_with_viewport(&new_ctx, body.as_ref(), child_rect, viewport)?);
                y += inner_h;
            }
            let mut node = LayoutNode::empty(LayoutKind::Column, rect);
            node.children = child_nodes;
            Ok(node)
        }
        Expr::Block { stmts, .. } => {
            // First pass: evaluate Let and StateDecl into a cloned context
            let mut block_ctx = EvalContext {
                variables: ctx.variables.clone(),
                components: ctx.components.clone(),
            };
            for s in stmts {
                match s {
                    Stmt::Let { name, value, .. } => {
                        if let Ok(val) = eval_expr(&block_ctx, value) {
                            block_ctx.variables.insert(name.clone(), val);
                        }
                    }
                    Stmt::StateDecl(sd) => {
                        if let Ok(val) = eval_expr(&block_ctx, &sd.initial_value) {
                            block_ctx.variables.insert(sd.name.clone(), val);
                        }
                    }
                    _ => {}
                }
            }
            // Second pass: layout Expr stmts using enriched context
            let mut child_nodes = Vec::new();
            let total = stmts.iter().filter(|s| matches!(s, Stmt::Expr(_))).count();
            if total == 0 {
                return Ok(LayoutNode::empty(LayoutKind::Box, rect));
            }
            let inner_h = rect.h.max(0.0) / total as f32;
            let mut y = rect.y;
            for s in stmts {
                if let Stmt::Expr(e) = s {
                    let child_rect = Rect::new(rect.x, y, rect.w, inner_h);
                    child_nodes.push(layout_tree_with_viewport(&block_ctx, e, child_rect, viewport)?);
                    y += inner_h;
                }
            }
            let mut node = LayoutNode::empty(LayoutKind::Column, rect);
            node.children = child_nodes;
            Ok(node)
        }
        _ => Ok(LayoutNode::empty(LayoutKind::Box, rect)),
    }
}

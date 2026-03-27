//! Runtime values for the Newt interpreter (used during layout).

use crate::ast::{ComponentDecl, Expr, Program, ProgramItem};
use crate::error::NewtError;
use std::collections::HashMap;

/// Maximum number of elements `range()` can produce to prevent runaway loops.
const MAX_RANGE_SIZE: i64 = 10_000;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Color { r: u8, g: u8, b: u8, a: u8 },
    Array(Vec<Value>),
}

impl Value {
    pub fn as_number(&self) -> Result<f64, NewtError> {
        match self {
            Value::Number(n) => Ok(*n),
            _ => Err(NewtError::Other("expected number".into())),
        }
    }

    pub fn as_string(&self) -> Result<&str, NewtError> {
        match self {
            Value::String(s) => Ok(s),
            _ => Err(NewtError::Other("expected string".into())),
        }
    }

    pub fn as_bool(&self) -> Result<bool, NewtError> {
        match self {
            Value::Bool(b) => Ok(*b),
            _ => Err(NewtError::Other("expected bool".into())),
        }
    }

    pub fn as_color(&self) -> Result<(u8, u8, u8, u8), NewtError> {
        match self {
            Value::Color { r, g, b, a } => Ok((*r, *g, *b, *a)),
            _ => Err(NewtError::Other("expected color".into())),
        }
    }

    pub fn as_array(&self) -> Result<&[Value], NewtError> {
        match self {
            Value::Array(a) => Ok(a.as_slice()),
            _ => Err(NewtError::Other("expected array".into())),
        }
    }
}

pub struct EvalContext {
    pub variables: HashMap<String, Value>,
    pub components: HashMap<String, ComponentDecl>,
}

impl EvalContext {
    pub fn from_program(program: &Program) -> Self {
        let mut components = HashMap::new();
        let mut themes: HashMap<String, HashMap<String, Value>> = HashMap::new();
        for item in &program.items {
            if let ProgramItem::Component(c) = item {
                components.insert(c.name.clone(), c.clone());
            }
        }
        let mut ctx = Self {
            variables: HashMap::new(),
            components,
        };
        for item in &program.items {
            match item {
                ProgramItem::Variable(v) => {
                    if let Ok(val) = eval_expr(&ctx, &v.value) {
                        ctx.variables.insert(v.name.clone(), val);
                    }
                }
                ProgramItem::Theme(t) => {
                    let mut theme_vars: HashMap<String, Value> = HashMap::new();
                    for v in &t.vars {
                        let mut theme_ctx = EvalContext {
                            variables: ctx.variables.clone(),
                            components: ctx.components.clone(),
                        };
                        for (k, val) in &theme_vars {
                            theme_ctx.variables.insert(k.clone(), val.clone());
                        }
                        if let Ok(val) = eval_expr(&theme_ctx, &v.value) {
                            theme_vars.insert(v.name.clone(), val);
                        }
                    }
                    themes.insert(t.name.clone(), theme_vars);
                }
                ProgramItem::UseTheme(name) => {
                    if let Some(theme_vars) = themes.get(name) {
                        for (k, v) in theme_vars {
                            ctx.variables.insert(k.clone(), v.clone());
                        }
                    }
                }
                ProgramItem::StateDecl(sd) => {
                    if let Ok(val) = eval_expr(&ctx, &sd.initial_value) {
                        ctx.variables.insert(sd.name.clone(), val);
                    }
                }
                _ => {}
            }
        }
        ctx
    }
}

/// Coerce any Value to a display string.
pub fn value_to_string(v: &Value) -> String {
    match v {
        Value::String(s) => s.clone(),
        Value::Number(n) => {
            if *n == (*n as i64) as f64 {
                format!("{}", *n as i64)
            } else {
                format!("{}", n)
            }
        }
        Value::Bool(b) => format!("{}", b),
        Value::Color { r, g, b, a } => {
            if *a == 255 {
                format!("#{:02x}{:02x}{:02x}", r, g, b)
            } else {
                format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a)
            }
        }
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(value_to_string).collect();
            format!("[{}]", items.join(", "))
        }
    }
}

pub fn value_to_json(v: &Value) -> serde_json::Value {
    match v {
        Value::Number(n) => {
            if *n == (*n as i64) as f64 {
                serde_json::Value::Number(serde_json::Number::from(*n as i64))
            } else {
                serde_json::json!(*n)
            }
        }
        Value::String(s) => serde_json::Value::String(s.clone()),
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Color { r, g, b, a } => {
            if *a == 255 {
                serde_json::json!(format!("#{:02x}{:02x}{:02x}", r, g, b))
            } else {
                serde_json::json!(format!("#{:02x}{:02x}{:02x}{:02x}", r, g, b, a))
            }
        }
        Value::Array(arr) => serde_json::Value::Array(arr.iter().map(value_to_json).collect()),
    }
}

pub fn json_to_value(v: &serde_json::Value) -> Option<Value> {
    match v {
        serde_json::Value::Number(n) => Some(Value::Number(n.as_f64()?)),
        serde_json::Value::String(s) => Some(Value::String(s.clone())),
        serde_json::Value::Bool(b) => Some(Value::Bool(*b)),
        _ => None,
    }
}

pub fn eval_expr(ctx: &EvalContext, expr: &Expr) -> Result<Value, NewtError> {
    use crate::ast::{BinaryOp, Expr, Literal, UnaryOp};
    match expr {
        Expr::Literal(lit) => match lit {
            Literal::Number(n) => Ok(Value::Number(*n)),
            Literal::String(s) => Ok(Value::String(s.clone())),
            Literal::Bool(b) => Ok(Value::Bool(*b)),
            Literal::Color { r, g, b, a } => Ok(Value::Color {
                r: *r,
                g: *g,
                b: *b,
                a: *a,
            }),
            Literal::Array(elems) => {
                let mut arr = Vec::with_capacity(elems.len());
                for e in elems {
                    arr.push(eval_expr(ctx, e)?);
                }
                Ok(Value::Array(arr))
            }
        },
        Expr::Ident(name, span) => {
            ctx.variables
                .get(name)
                .cloned()
                .ok_or_else(|| NewtError::semantic(*span, format!("undefined variable '{}'", name)))
        }
        Expr::Binary { left, op, right, .. } => {
            let l = eval_expr(ctx, left)?;
            let r = eval_expr(ctx, right)?;
            match op {
                BinaryOp::Add => Ok(Value::Number(l.as_number()? + r.as_number()?)),
                BinaryOp::Sub => Ok(Value::Number(l.as_number()? - r.as_number()?)),
                BinaryOp::Mul => Ok(Value::Number(l.as_number()? * r.as_number()?)),
                BinaryOp::Div => Ok(Value::Number(l.as_number()? / r.as_number()?)),
                BinaryOp::Mod => Ok(Value::Number(l.as_number()? % r.as_number()?)),
                BinaryOp::Eq => Ok(Value::Bool(match (&l, &r) {
                    (Value::Number(a), Value::Number(b)) => a == b,
                    (Value::Bool(a), Value::Bool(b)) => a == b,
                    (Value::String(a), Value::String(b)) => a == b,
                    _ => false,
                })),
                BinaryOp::Ne => Ok(Value::Bool(match (&l, &r) {
                    (Value::Number(a), Value::Number(b)) => a != b,
                    (Value::Bool(a), Value::Bool(b)) => a != b,
                    (Value::String(a), Value::String(b)) => a != b,
                    _ => true,
                })),
                BinaryOp::Lt => Ok(Value::Bool(l.as_number()? < r.as_number()?)),
                BinaryOp::Le => Ok(Value::Bool(l.as_number()? <= r.as_number()?)),
                BinaryOp::Gt => Ok(Value::Bool(l.as_number()? > r.as_number()?)),
                BinaryOp::Ge => Ok(Value::Bool(l.as_number()? >= r.as_number()?)),
                BinaryOp::And => Ok(Value::Bool(l.as_bool()? && r.as_bool()?)),
                BinaryOp::Or => Ok(Value::Bool(l.as_bool()? || r.as_bool()?)),
            }
        }
        Expr::Unary { op, inner, .. } => {
            let v = eval_expr(ctx, inner)?;
            match op {
                UnaryOp::Not => Ok(Value::Bool(!v.as_bool()?)),
                UnaryOp::Neg => Ok(Value::Number(-v.as_number()?)),
            }
        }
        Expr::Block { stmts, .. } => {
            let mut block_ctx = EvalContext {
                variables: ctx.variables.clone(),
                components: ctx.components.clone(),
            };
            let mut last = Value::Bool(false);
            for s in stmts {
                match s {
                    crate::ast::Stmt::Expr(e) => last = eval_expr(&block_ctx, e)?,
                    crate::ast::Stmt::Let { name, value, .. } => {
                        let val = eval_expr(&block_ctx, value)?;
                        block_ctx.variables.insert(name.clone(), val.clone());
                        last = val;
                    }
                    crate::ast::Stmt::StateDecl(sd) => {
                        let val = eval_expr(&block_ctx, &sd.initial_value)?;
                        block_ctx.variables.insert(sd.name.clone(), val.clone());
                        last = val;
                    }
                }
            }
            Ok(last)
        }
        Expr::If { cond, then_branch, else_branch, .. } => {
            if eval_expr(ctx, cond)?.as_bool()? {
                eval_expr(ctx, then_branch)
            } else if let Some(eb) = else_branch {
                eval_expr(ctx, eb)
            } else {
                Ok(Value::Bool(false))
            }
        }
        Expr::Call {
            callee,
            args,
            slot_args,
            span,
            ..
        } => {
            if callee == "range" && args.len() == 1 && slot_args.is_none() {
                let n = eval_expr(ctx, &args[0])?.as_number()? as i64;
                let n = n.max(0).min(MAX_RANGE_SIZE) as usize;
                return Ok(Value::Array(
                    (0..n).map(|i| Value::Number(i as f64)).collect(),
                ));
            }
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
            if slot_args.is_none() {
                for (i, param) in comp.params.iter().enumerate() {
                    if let Some(arg) = args.get(i) {
                        if let Ok(v) = eval_expr(ctx, arg) {
                            new_ctx.variables.insert(param.clone(), v);
                        }
                    }
                }
            }
            eval_expr(&new_ctx, &body)
        }
        Expr::For { var, iter, body, span, .. } => {
            let iter_val = eval_expr(ctx, iter)?;
            let arr = iter_val.as_array().map_err(|_| {
                NewtError::semantic(*span, "for loop expects an array or range(n)")
            })?;
            let results: Result<Vec<Value>, NewtError> = arr.iter().map(|val| {
                let mut loop_ctx = EvalContext {
                    variables: ctx.variables.clone(),
                    components: ctx.components.clone(),
                };
                loop_ctx.variables.insert(var.clone(), val.clone());
                eval_expr(&loop_ctx, body)
            }).collect();
            Ok(Value::Array(results?))
        }
        Expr::InterpolatedString { parts, .. } => {
            let mut result = String::new();
            for seg in parts {
                match seg {
                    crate::ast::InterpSegment::Literal(s) => result.push_str(s),
                    crate::ast::InterpSegment::Expr(e) => {
                        let val = eval_expr(ctx, e)?;
                        result.push_str(&value_to_string(&val));
                    }
                }
            }
            Ok(Value::String(result))
        }
        Expr::Assignment { value, .. } => {
            // At compile time, evaluate RHS only (actual mutation is runtime)
            eval_expr(ctx, value)
        }
        _ => Err(NewtError::Other("expression not evaluable to value".into())),
    }
}

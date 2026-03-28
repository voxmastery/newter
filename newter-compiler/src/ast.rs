//! Abstract Syntax Tree for the Newt UI language.

use crate::error::Span;
use std::collections::HashMap;

/// Root node: list of top-level items (variables, components, screens, themes, imports, use theme).
#[derive(Debug, Clone)]
pub struct Program {
    pub items: Vec<ProgramItem>,
}

#[derive(Debug, Clone)]
pub enum ProgramItem {
    Variable(VariableDecl),
    Component(ComponentDecl),
    Screen(ScreenDecl),
    Theme(ThemeDecl),
    Import(ImportDecl),
    UseTheme(String),
    StateDecl(StateVarDecl),
}

#[derive(Debug, Clone)]
pub struct ThemeDecl {
    pub name: String,
    pub vars: Vec<VariableDecl>,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ImportDecl {
    pub path: String,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct VariableDecl {
    pub name: String,
    pub value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct StateVarDecl {
    pub name: String,
    pub initial_value: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ComponentDecl {
    pub name: String,
    pub params: Vec<String>,
    pub body: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub struct ScreenDecl {
    pub name: String,
    pub body: Expr,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Ident(String, Span),
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        span: Span,
    },
    Unary {
        op: UnaryOp,
        inner: Box<Expr>,
        span: Span,
    },
    Call {
        callee: String,
        args: Vec<Expr>,
        /// When present, args are passed by slot name (e.g. sidebar: expr, main: expr).
        slot_args: Option<Vec<(String, Expr)>>,
        span: Span,
    },
    Element {
        kind: ElementKind,
        props: Vec<Prop>,
        children: Vec<Expr>,
        span: Span,
    },
    Block {
        stmts: Vec<Stmt>,
        span: Span,
    },
    If {
        cond: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
        span: Span,
    },
    For {
        var: String,
        iter: Box<Expr>,
        body: Box<Expr>,
        span: Span,
    },
    InterpolatedString {
        parts: Vec<InterpSegment>,
        span: Span,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
        span: Span,
    },
}

/// A segment of an interpolated string in the AST.
#[derive(Debug, Clone)]
pub enum InterpSegment {
    /// Literal text.
    Literal(String),
    /// A parsed expression to be evaluated and coerced to string.
    Expr(Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Color { r: u8, g: u8, b: u8, a: u8 },
    Array(Vec<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
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
    Icon,
    Tag,
    Comment,
    Feed,
    Carousel,
    Chart,

    // New elements (v0.2)
    Table,
    Avatar,
    Skeleton,
    Drawer,
    Select,
    Textarea,
    Popover,
    Separator,
    Timeline,
    Rating,
    FileUpload,
    ColorPicker,
    TreeView,
    CommandPalette,
    Splitter,
}

#[derive(Debug, Clone)]
pub struct Prop {
    pub name: PropName,
    pub value: PropValue,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum PropName {
    Ident(String),
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
    Src, // image src
    Content, // text content
    MinWidth,
    MaxWidth,
    MinHeight,
    MaxHeight,
    Transition,
    Role,
    AriaLabel,
    FocusOrder,
}

#[derive(Debug, Clone)]
pub enum PropValue {
    Expr(Expr),
    Number(f64),
    String(String),
    Color { r: u8, g: u8, b: u8, a: u8 },
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Expr(Expr),
    Let { name: String, value: Expr, span: Span },
    StateDecl(StateVarDecl),
}

impl ElementKind {
    pub fn from_token_kind(t: &crate::lexer::TokenKind) -> Option<Self> {
        use crate::lexer::TokenKind;
        match t {
            TokenKind::Header => Some(ElementKind::Header),
            TokenKind::Footer => Some(ElementKind::Footer),
            TokenKind::Container => Some(ElementKind::Container),
            TokenKind::Sidebar => Some(ElementKind::Sidebar),
            TokenKind::Section => Some(ElementKind::Section),
            TokenKind::Box => Some(ElementKind::Box),
            TokenKind::Text => Some(ElementKind::Text),
            TokenKind::Row => Some(ElementKind::Row),
            TokenKind::Column => Some(ElementKind::Column),
            TokenKind::Grid => Some(ElementKind::Grid),
            TokenKind::Stack => Some(ElementKind::Stack),
            TokenKind::Center => Some(ElementKind::Center),
            TokenKind::Spacer => Some(ElementKind::Spacer),
            TokenKind::Image => Some(ElementKind::Image),
            TokenKind::Button => Some(ElementKind::Button),
            TokenKind::Input => Some(ElementKind::Input),
            TokenKind::Card => Some(ElementKind::Card),
            TokenKind::Widget => Some(ElementKind::Widget),
            TokenKind::Accordion => Some(ElementKind::Accordion),
            TokenKind::Bento => Some(ElementKind::Bento),
            TokenKind::Breadcrumb => Some(ElementKind::Breadcrumb),
            TokenKind::Hamburger => Some(ElementKind::Hamburger),
            TokenKind::Kebab => Some(ElementKind::Kebab),
            TokenKind::Meatballs => Some(ElementKind::Meatballs),
            TokenKind::Doner => Some(ElementKind::Doner),
            TokenKind::Tabs => Some(ElementKind::Tabs),
            TokenKind::Pagination => Some(ElementKind::Pagination),
            TokenKind::LinkList => Some(ElementKind::LinkList),
            TokenKind::Nav => Some(ElementKind::Nav),
            TokenKind::Password => Some(ElementKind::Password),
            TokenKind::Search => Some(ElementKind::Search),
            TokenKind::Checkbox => Some(ElementKind::Checkbox),
            TokenKind::Radio => Some(ElementKind::Radio),
            TokenKind::Dropdown => Some(ElementKind::Dropdown),
            TokenKind::Combobox => Some(ElementKind::Combobox),
            TokenKind::Multiselect => Some(ElementKind::Multiselect),
            TokenKind::DatePicker => Some(ElementKind::DatePicker),
            TokenKind::Picker => Some(ElementKind::Picker),
            TokenKind::Slider => Some(ElementKind::Slider),
            TokenKind::Stepper => Some(ElementKind::Stepper),
            TokenKind::Toggle => Some(ElementKind::Toggle),
            TokenKind::Form => Some(ElementKind::Form),
            TokenKind::Modal => Some(ElementKind::Modal),
            TokenKind::ConfirmDialog => Some(ElementKind::ConfirmDialog),
            TokenKind::Toast => Some(ElementKind::Toast),
            TokenKind::Notification => Some(ElementKind::Notification),
            TokenKind::Alert => Some(ElementKind::Alert),
            TokenKind::MessageBox => Some(ElementKind::MessageBox),
            TokenKind::Tooltip => Some(ElementKind::Tooltip),
            TokenKind::Loader => Some(ElementKind::Loader),
            TokenKind::ProgressBar => Some(ElementKind::ProgressBar),
            TokenKind::Badge => Some(ElementKind::Badge),
            TokenKind::Icon => Some(ElementKind::Icon),
            TokenKind::Tag => Some(ElementKind::Tag),
            TokenKind::Comment => Some(ElementKind::Comment),
            TokenKind::Feed => Some(ElementKind::Feed),
            TokenKind::Carousel => Some(ElementKind::Carousel),
            TokenKind::Chart => Some(ElementKind::Chart),
            TokenKind::Table => Some(ElementKind::Table),
            TokenKind::Avatar => Some(ElementKind::Avatar),
            TokenKind::Skeleton => Some(ElementKind::Skeleton),
            TokenKind::Drawer => Some(ElementKind::Drawer),
            TokenKind::Select => Some(ElementKind::Select),
            TokenKind::Textarea => Some(ElementKind::Textarea),
            TokenKind::Popover => Some(ElementKind::Popover),
            TokenKind::Separator => Some(ElementKind::Separator),
            TokenKind::Timeline => Some(ElementKind::Timeline),
            TokenKind::Rating => Some(ElementKind::Rating),
            TokenKind::FileUpload => Some(ElementKind::FileUpload),
            TokenKind::ColorPicker => Some(ElementKind::ColorPicker),
            TokenKind::TreeView => Some(ElementKind::TreeView),
            TokenKind::CommandPalette => Some(ElementKind::CommandPalette),
            TokenKind::Splitter => Some(ElementKind::Splitter),
            _ => None,
        }
    }
}

/// Replace Ident(slot_name) in expr with the corresponding slot expression when using named slots.
pub fn substitute_slots(expr: &Expr, slot_args: &[(String, Expr)]) -> Expr {
    let map: HashMap<_, _> = slot_args.iter().cloned().collect();
    substitute_slots_inner(expr, &map)
}

fn substitute_slots_inner(expr: &Expr, map: &HashMap<String, Expr>) -> Expr {
    match expr {
        Expr::Ident(name, span) => {
            if let Some(slot_expr) = map.get(name) {
                substitute_slots_inner(slot_expr, map)
            } else {
                Expr::Ident(name.clone(), *span)
            }
        }
        Expr::Element {
            kind,
            props,
            children,
            span,
        } => Expr::Element {
            kind: *kind,
            props: props.clone(),
            children: children
                .iter()
                .map(|e| substitute_slots_inner(e, map))
                .collect(),
            span: *span,
        },
        Expr::Call {
            callee,
            args,
            slot_args: sa,
            span,
        } => Expr::Call {
            callee: callee.clone(),
            args: args.iter().map(|e| substitute_slots_inner(e, map)).collect(),
            slot_args: sa.clone(),
            span: *span,
        },
        Expr::Block { stmts, span } => Expr::Block {
            stmts: stmts
                .iter()
                .map(|s| match s {
                    Stmt::Expr(e) => Stmt::Expr(substitute_slots_inner(e, map)),
                    Stmt::StateDecl(sd) => Stmt::StateDecl(StateVarDecl {
                        name: sd.name.clone(),
                        initial_value: substitute_slots_inner(&sd.initial_value, map),
                        span: sd.span,
                    }),
                    other => other.clone(),
                })
                .collect(),
            span: *span,
        },
        Expr::If {
            cond,
            then_branch,
            else_branch,
            span,
        } => Expr::If {
            cond: Box::new(substitute_slots_inner(cond, map)),
            then_branch: Box::new(substitute_slots_inner(then_branch, map)),
            else_branch: else_branch
                .as_ref()
                .map(|e| Box::new(substitute_slots_inner(e, map))),
            span: *span,
        },
        Expr::For {
            var,
            iter,
            body,
            span,
        } => Expr::For {
            var: var.clone(),
            iter: Box::new(substitute_slots_inner(iter, map)),
            body: Box::new(substitute_slots_inner(body, map)),
            span: *span,
        },
        Expr::InterpolatedString { parts, span } => Expr::InterpolatedString {
            parts: parts
                .iter()
                .map(|seg| match seg {
                    InterpSegment::Literal(s) => InterpSegment::Literal(s.clone()),
                    InterpSegment::Expr(e) => {
                        InterpSegment::Expr(Box::new(substitute_slots_inner(e, map)))
                    }
                })
                .collect(),
            span: *span,
        },
        Expr::Assignment { name, value, span } => Expr::Assignment {
            name: name.clone(),
            value: Box::new(substitute_slots_inner(value, map)),
            span: *span,
        },
        other => other.clone(),
    }
}

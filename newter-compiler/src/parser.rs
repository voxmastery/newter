//! Recursive descent parser for the Newt UI language.

use crate::ast::*;
use crate::error::{NewtError, Span};
use crate::lexer::{Lexer, Token, TokenKind};

pub struct Parser<'a> {
    tokens: Vec<Token>,
    index: usize,
    errors: Vec<NewtError>,
    _marker: std::marker::PhantomData<&'a str>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, path: Option<&'a str>) -> Result<Self, NewtError> {
        let mut lexer = Lexer::new(source, path);
        let mut tokens = Vec::new();
        loop {
            let t = lexer.next_token()?;
            let is_eof = matches!(t.kind, TokenKind::Eof);
            tokens.push(t);
            if is_eof {
                break;
            }
        }
        Ok(Self {
            tokens,
            index: 0,
            errors: Vec::new(),
            _marker: std::marker::PhantomData,
        })
    }

    fn current(&self) -> &Token {
        self.tokens.get(self.index).unwrap_or_else(|| self.tokens.last().unwrap())
    }

    fn advance(&mut self) -> Token {
        let t = self.current().clone();
        if self.index < self.tokens.len() {
            self.index += 1;
        }
        t
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token, NewtError> {
        let cur = self.current();
        if std::mem::discriminant(&cur.kind) == std::mem::discriminant(&kind) {
            return Ok(self.advance());
        }
        Err(NewtError::parse(
            cur.span,
            format!("expected {}, got {}", kind, cur.kind),
        ))
    }

    fn expect_ident(&mut self) -> Result<String, NewtError> {
        let cur = self.current();
        match &cur.kind {
            TokenKind::Ident(s) => {
                let name = s.clone();
                self.advance();
                Ok(name)
            }
            _ => Err(NewtError::parse(
                cur.span,
                format!("expected identifier, got {}", cur.kind),
            )),
        }
    }

    fn at(&self, kind: TokenKind) -> bool {
        std::mem::discriminant(&self.current().kind) == std::mem::discriminant(&kind)
    }

    fn at_ident(&self) -> bool {
        matches!(self.current().kind, TokenKind::Ident(_))
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }

    fn at_ident_then_colon(&self) -> bool {
        if !self.at_ident() {
            return false;
        }
        self.peek_next()
            .map(|t| std::mem::discriminant(&t.kind) == std::mem::discriminant(&TokenKind::Colon))
            .unwrap_or(false)
    }

    pub fn parse(&mut self) -> Result<Program, Vec<NewtError>> {
        let mut items = Vec::new();
        while !self.at(TokenKind::Eof) {
            match self.parse_top_level_item() {
                Ok(item) => items.push(item),
                Err(e) => {
                    self.errors.push(e);
                    self.synchronize();
                }
            }
        }
        if self.errors.is_empty() {
            Ok(Program { items })
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }

    fn parse_top_level_item(&mut self) -> Result<ProgramItem, NewtError> {
        if self.at(TokenKind::Import) {
            Ok(ProgramItem::Import(self.parse_import()?))
        } else if self.at(TokenKind::Theme) {
            Ok(ProgramItem::Theme(self.parse_theme()?))
        } else if self.at(TokenKind::Use) {
            Ok(ProgramItem::UseTheme(self.parse_use_theme()?))
        } else if self.at(TokenKind::Let) {
            Ok(ProgramItem::Variable(self.parse_variable()?))
        } else if self.at(TokenKind::Component) {
            Ok(ProgramItem::Component(self.parse_component()?))
        } else if self.at(TokenKind::Screen) {
            Ok(ProgramItem::Screen(self.parse_screen()?))
        } else if self.at(TokenKind::State) {
            Ok(ProgramItem::StateDecl(self.parse_state_decl()?))
        } else {
            let cur = self.current();
            let msg = format!(
                "expected import, theme, use theme, let, state, component, or screen at top level, got {}",
                cur.kind
            );
            let err = if let TokenKind::Ident(ref s) = cur.kind {
                if let Some(suggestion) = closest_top_level_keyword(s) {
                    NewtError::parse_with_suggestion(cur.span, msg, format!("did you mean `{}`?", suggestion))
                } else {
                    NewtError::parse(cur.span, msg)
                }
            } else {
                NewtError::parse(cur.span, msg)
            };
            Err(err)
        }
    }

    /// Skip tokens until we reach a safe restart point for top-level parsing.
    /// Tracks brace depth so we skip entire `{ ... }` blocks.
    fn synchronize(&mut self) {
        let mut depth = 0i32;
        loop {
            match self.current().kind {
                TokenKind::Eof => break,
                TokenKind::LeftBrace => { depth += 1; self.advance(); }
                TokenKind::RightBrace => {
                    if depth > 0 {
                        depth -= 1;
                        self.advance();
                        if depth == 0 {
                            // Finished skipping a balanced block — ready for next top-level item
                            break;
                        }
                    } else {
                        self.advance();
                    }
                }
                TokenKind::Semicolon if depth == 0 => { self.advance(); break; }
                TokenKind::Let | TokenKind::Component | TokenKind::Screen
                | TokenKind::Theme | TokenKind::Import | TokenKind::Use
                | TokenKind::State if depth == 0 => break,
                _ => { self.advance(); }
            }
        }
    }

    fn parse_import(&mut self) -> Result<ImportDecl, NewtError> {
        let span = self.current().span;
        self.expect(TokenKind::Import)?;
        let path = match &self.current().kind {
            TokenKind::String(s) => s.clone(),
            _ => return Err(NewtError::parse(self.current().span, "import expects a string path")),
        };
        self.advance();
        self.expect(TokenKind::Semicolon)?;
        Ok(ImportDecl { path, span })
    }

    fn parse_theme(&mut self) -> Result<ThemeDecl, NewtError> {
        let span = self.current().span;
        self.expect(TokenKind::Theme)?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::LeftBrace)?;
        let mut vars = Vec::new();
        while !self.at(TokenKind::RightBrace) && !self.at(TokenKind::Eof) {
            if self.at(TokenKind::Let) {
                vars.push(self.parse_variable()?);
            } else {
                return Err(NewtError::parse(
                    self.current().span,
                    "theme body may only contain let declarations",
                ));
            }
        }
        self.expect(TokenKind::RightBrace)?;
        Ok(ThemeDecl { name, vars, span })
    }

    fn parse_use_theme(&mut self) -> Result<String, NewtError> {
        self.expect(TokenKind::Use)?;
        self.expect(TokenKind::Theme)?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(name)
    }

    fn parse_variable(&mut self) -> Result<VariableDecl, NewtError> {
        let span = self.current().span;
        self.expect(TokenKind::Let)?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::Eq)?;
        let value = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(VariableDecl { name, value, span })
    }

    fn parse_state_decl(&mut self) -> Result<StateVarDecl, NewtError> {
        let span = self.current().span;
        self.expect(TokenKind::State)?;
        let name = self.expect_ident()?;
        self.expect(TokenKind::Eq)?;
        let initial_value = self.parse_expr()?;
        self.expect(TokenKind::Semicolon)?;
        Ok(StateVarDecl { name, initial_value, span })
    }

    fn parse_component(&mut self) -> Result<ComponentDecl, NewtError> {
        let span = self.current().span;
        self.expect(TokenKind::Component)?;
        let name = self.expect_ident()?;
        let params = if self.at(TokenKind::LeftParen) {
            self.advance();
            let mut p = Vec::new();
            if !self.at(TokenKind::RightParen) {
                p.push(self.expect_ident()?);
                while self.at(TokenKind::Comma) {
                    self.advance();
                    p.push(self.expect_ident()?);
                }
            }
            self.expect(TokenKind::RightParen)?;
            p
        } else {
            Vec::new()
        };
        self.expect(TokenKind::LeftBrace)?;
        let body = self.parse_expr()?;
        self.expect(TokenKind::RightBrace)?;
        Ok(ComponentDecl { name, params, body, span })
    }

    /// Parse block body (after opening {): exprs until }, return Block. No leading {.
    fn parse_block_body(&mut self, span: Span) -> Result<Expr, NewtError> {
        let mut stmts = Vec::new();
        while !self.at(TokenKind::RightBrace) && !self.at(TokenKind::Eof) {
            if self.at(TokenKind::Let) {
                self.advance();
                let name = self.expect_ident()?;
                self.expect(TokenKind::Eq)?;
                let value = self.parse_expr()?;
                self.expect(TokenKind::Semicolon)?;
                stmts.push(Stmt::Let {
                    name,
                    value,
                    span: self.current().span,
                });
            } else if self.at(TokenKind::State) {
                let sd = self.parse_state_decl()?;
                stmts.push(Stmt::StateDecl(sd));
            } else {
                stmts.push(Stmt::Expr(self.parse_expr()?));
                if self.at(TokenKind::Semicolon) {
                    self.advance();
                }
            }
        }
        Ok(Expr::Block { stmts, span })
    }

    fn parse_screen(&mut self) -> Result<ScreenDecl, NewtError> {
        let span = self.current().span;
        self.expect(TokenKind::Screen)?;
        let name = if self.at(TokenKind::LeftParen) {
            self.advance();
            let n = self.expect_ident()?;
            self.expect(TokenKind::RightParen)?;
            n
        } else if self.at_ident() {
            self.expect_ident()?
        } else {
            "Main".to_string()
        };
        self.expect(TokenKind::LeftBrace)?;
        let body = self.parse_block_body(span)?;
        self.expect(TokenKind::RightBrace)?;
        Ok(ScreenDecl { name, body, span })
    }

    fn parse_expr(&mut self) -> Result<Expr, NewtError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, NewtError> {
        let mut left = self.parse_and()?;
        while self.at(TokenKind::Or) {
            let span = self.current().span;
            self.advance();
            let right = self.parse_and()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::Or,
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, NewtError> {
        let mut left = self.parse_equality()?;
        while self.at(TokenKind::And) {
            let span = self.current().span;
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::Binary {
                left: Box::new(left),
                op: BinaryOp::And,
                right: Box::new(right),
                span,
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, NewtError> {
        let mut left = self.parse_comparison()?;
        loop {
            let span = self.current().span;
            if self.at(TokenKind::EqEq) {
                self.advance();
                left = Expr::Binary {
                    left: Box::new(left),
                    op: BinaryOp::Eq,
                    right: Box::new(self.parse_comparison()?),
                    span,
                };
            } else if self.at(TokenKind::NotEq) {
                self.advance();
                left = Expr::Binary {
                    left: Box::new(left),
                    op: BinaryOp::Ne,
                    right: Box::new(self.parse_comparison()?),
                    span,
                };
            } else {
                break;
            }
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, NewtError> {
        let mut left = self.parse_term()?;
        loop {
            let span = self.current().span;
            let op = if self.at(TokenKind::Lt) {
                self.advance();
                BinaryOp::Lt
            } else if self.at(TokenKind::Le) {
                self.advance();
                BinaryOp::Le
            } else if self.at(TokenKind::Gt) {
                self.advance();
                BinaryOp::Gt
            } else if self.at(TokenKind::Ge) {
                self.advance();
                BinaryOp::Ge
            } else {
                break;
            };
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(self.parse_term()?),
                span,
            };
        }
        Ok(left)
    }

    fn parse_term(&mut self) -> Result<Expr, NewtError> {
        let mut left = self.parse_factor()?;
        loop {
            let span = self.current().span;
            let op = if self.at(TokenKind::Plus) {
                self.advance();
                BinaryOp::Add
            } else if self.at(TokenKind::Minus) {
                self.advance();
                BinaryOp::Sub
            } else {
                break;
            };
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(self.parse_factor()?),
                span,
            };
        }
        Ok(left)
    }

    fn parse_factor(&mut self) -> Result<Expr, NewtError> {
        let mut left = self.parse_unary()?;
        loop {
            let span = self.current().span;
            let op = if self.at(TokenKind::Star) {
                self.advance();
                BinaryOp::Mul
            } else if self.at(TokenKind::Slash) {
                self.advance();
                BinaryOp::Div
            } else if self.at(TokenKind::Percent) {
                self.advance();
                BinaryOp::Mod
            } else {
                break;
            };
            left = Expr::Binary {
                left: Box::new(left),
                op,
                right: Box::new(self.parse_unary()?),
                span,
            };
        }
        Ok(left)
    }

    fn parse_unary(&mut self) -> Result<Expr, NewtError> {
        let span = self.current().span;
        if self.at(TokenKind::Not) {
            self.advance();
            let inner = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Not,
                inner: Box::new(inner),
                span,
            });
        }
        if self.at(TokenKind::Minus) {
            self.advance();
            let inner = self.parse_unary()?;
            return Ok(Expr::Unary {
                op: UnaryOp::Neg,
                inner: Box::new(inner),
                span,
            });
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expr, NewtError> {
        let span = self.current().span;
        match &self.current().kind {
            TokenKind::Number(n) => {
                let v = *n;
                self.advance();
                return Ok(Expr::Literal(Literal::Number(v)));
            }
            TokenKind::String(s) => {
                let v = s.clone();
                self.advance();
                return Ok(Expr::Literal(Literal::String(v)));
            }
            TokenKind::InterpolatedString(parts) => {
                let parts = parts.clone();
                self.advance();
                return self.build_interp_expr(parts, span);
            }
            TokenKind::True => {
                self.advance();
                return Ok(Expr::Literal(Literal::Bool(true)));
            }
            TokenKind::False => {
                self.advance();
                return Ok(Expr::Literal(Literal::Bool(false)));
            }
            TokenKind::HexColor(r, g, b, a) => {
                let (r, g, b, a) = (*r, *g, *b, *a);
                self.advance();
                return Ok(Expr::Literal(Literal::Color { r, g, b, a }));
            }
            TokenKind::LeftBracket => {
                self.advance();
                let mut elems = Vec::new();
                while !self.at(TokenKind::RightBracket) && !self.at(TokenKind::Eof) {
                    elems.push(self.parse_expr()?);
                    if self.at(TokenKind::Comma) {
                        self.advance();
                    }
                }
                self.expect(TokenKind::RightBracket)?;
                return Ok(Expr::Literal(Literal::Array(elems)));
            }
            TokenKind::LeftBrace => {
                self.advance();
                let mut stmts = Vec::new();
                while !self.at(TokenKind::RightBrace) && !self.at(TokenKind::Eof) {
                    if self.at(TokenKind::Let) {
                        self.advance();
                        let name = self.expect_ident()?;
                        self.expect(TokenKind::Eq)?;
                        let value = self.parse_expr()?;
                        self.expect(TokenKind::Semicolon)?;
                        stmts.push(Stmt::Let {
                            name,
                            value,
                            span: self.current().span,
                        });
                    } else if self.at(TokenKind::State) {
                        let sd = self.parse_state_decl()?;
                        stmts.push(Stmt::StateDecl(sd));
                    } else {
                        stmts.push(Stmt::Expr(self.parse_expr()?));
                        if self.at(TokenKind::Semicolon) {
                            self.advance();
                        }
                    }
                }
                self.expect(TokenKind::RightBrace)?;
                return Ok(Expr::Block { stmts, span });
            }
            TokenKind::If => {
                self.advance();
                let cond = Box::new(self.parse_expr()?);
                self.expect(TokenKind::LeftBrace)?;
                let then_branch = Box::new(self.parse_expr()?);
                self.expect(TokenKind::RightBrace)?;
                let else_branch = if self.at(TokenKind::Else) {
                    self.advance();
                    self.expect(TokenKind::LeftBrace)?;
                    let e = self.parse_expr()?;
                    self.expect(TokenKind::RightBrace)?;
                    Some(Box::new(e))
                } else {
                    None
                };
                return Ok(Expr::If {
                    cond,
                    then_branch,
                    else_branch,
                    span,
                });
            }
            TokenKind::For => {
                self.advance();
                let var = self.expect_ident()?;
                self.expect(TokenKind::In)?;
                let iter = Box::new(self.parse_expr()?);
                self.expect(TokenKind::LeftBrace)?;
                let body = Box::new(self.parse_expr()?);
                self.expect(TokenKind::RightBrace)?;
                return Ok(Expr::For { var, iter, body, span });
            }
            TokenKind::Ident(_) => {
                let name = self.expect_ident()?;
                if self.at(TokenKind::LeftParen) {
                    self.advance();
                    let (args, slot_args) = if !self.at(TokenKind::RightParen) && self.at_ident_then_colon() {
                        let mut slot_args = Vec::new();
                        while !self.at(TokenKind::RightParen) && !self.at(TokenKind::Eof) {
                            let slot_name = self.expect_ident()?;
                            self.expect(TokenKind::Colon)?;
                            let e = self.parse_expr()?;
                            slot_args.push((slot_name, e));
                            if self.at(TokenKind::Comma) {
                                self.advance();
                            }
                        }
                        (Vec::new(), Some(slot_args))
                    } else {
                        let mut args = Vec::new();
                        if !self.at(TokenKind::RightParen) {
                            args.push(self.parse_expr()?);
                            while self.at(TokenKind::Comma) {
                                self.advance();
                                args.push(self.parse_expr()?);
                            }
                        }
                        (args, None)
                    };
                    self.expect(TokenKind::RightParen)?;
                    return Ok(Expr::Call {
                        callee: name,
                        args,
                        slot_args,
                        span,
                    });
                }
                if let Some(kind) = ElementKind::from_token_kind(&TokenKind::Ident(name.clone())) {
                    // We already advanced for ident; element keyword is the ident
                    return self.parse_element_props_and_children(kind, span);
                }
                if self.at(TokenKind::Eq) {
                    self.advance();
                    let value = self.parse_expr()?;
                    return Ok(Expr::Assignment { name, value: Box::new(value), span });
                }
                return Ok(Expr::Ident(name, span));
            }
            _ => {}
        }

        // Element keywords: box, text, row, column, ...
        if let Some(kind) = ElementKind::from_token_kind(&self.current().kind) {
            let span = self.current().span;
            self.advance();
            return self.parse_element_props_and_children(kind, span);
        }

        Err(NewtError::parse(
            self.current().span,
            format!("expected expression, got {}", self.current().kind),
        ))
    }

    fn parse_element_props_and_children(&mut self, kind: ElementKind, span: Span) -> Result<Expr, NewtError> {
        let mut props = Vec::new();
        let mut children = Vec::new();

        // First group: ( ... ) or { ... }
        if self.at(TokenKind::LeftParen) {
            self.advance();
            if matches!(self.current().kind, TokenKind::String(_))
                && (kind == ElementKind::Text || kind == ElementKind::Button)
            {
                let s = match &self.current().kind {
                    TokenKind::String(x) => x.clone(),
                    _ => unreachable!(),
                };
                self.advance();
                props.push(Prop {
                    name: PropName::Content,
                    value: PropValue::String(s),
                    span: self.current().span,
                });
                while self.at(TokenKind::Comma) {
                    self.advance();
                    props.push(self.parse_prop()?);
                }
                self.expect(TokenKind::RightParen)?;
            } else if matches!(self.current().kind, TokenKind::InterpolatedString(_))
                && (kind == ElementKind::Text || kind == ElementKind::Button)
            {
                let (parts, ispan) = match &self.current().kind {
                    TokenKind::InterpolatedString(p) => (p.clone(), self.current().span),
                    _ => unreachable!(),
                };
                self.advance();
                let expr = self.build_interp_expr(parts, ispan)?;
                props.push(Prop {
                    name: PropName::Content,
                    value: PropValue::Expr(expr),
                    span: ispan,
                });
                while self.at(TokenKind::Comma) {
                    self.advance();
                    props.push(self.parse_prop()?);
                }
                self.expect(TokenKind::RightParen)?;
            } else if self.at_ident_then_colon() || self.is_prop_keyword() {
                while !self.at(TokenKind::RightParen) && !self.at(TokenKind::Eof) {
                    props.push(self.parse_prop()?);
                    if self.at(TokenKind::Comma) {
                        self.advance();
                    }
                }
                self.expect(TokenKind::RightParen)?;
            } else {
                while !self.at(TokenKind::RightParen) && !self.at(TokenKind::Eof) {
                    children.push(self.parse_expr()?);
                    if self.at(TokenKind::Comma) {
                        self.advance();
                    }
                }
                self.expect(TokenKind::RightParen)?;
            }
        }

        // Second group: ( children ) if we had ( props ) first
        if self.at(TokenKind::LeftParen) {
            self.advance();
            while !self.at(TokenKind::RightParen) && !self.at(TokenKind::Eof) {
                children.push(self.parse_expr()?);
                if self.at(TokenKind::Comma) {
                    self.advance();
                }
            }
            self.expect(TokenKind::RightParen)?;
        }

        // Brace group(s): { props and/or children } or first { props } then { children }
        if self.at(TokenKind::LeftBrace) {
            self.advance();
            let mut in_first_brace = true;
            while !self.at(TokenKind::RightBrace) && !self.at(TokenKind::Eof) {
                if in_first_brace && (self.at_ident() || self.is_prop_keyword()) {
                    props.push(self.parse_prop()?);
                    if self.at(TokenKind::Comma) {
                        self.advance();
                    }
                } else {
                    in_first_brace = false;
                    children.push(self.parse_expr()?);
                    if self.at(TokenKind::Comma) {
                        self.advance();
                    }
                }
            }
            self.expect(TokenKind::RightBrace)?;
            // Second brace group = children only (legacy: column { props } { children })
            if self.at(TokenKind::LeftBrace) {
                self.advance();
                while !self.at(TokenKind::RightBrace) && !self.at(TokenKind::Eof) {
                    children.push(self.parse_expr()?);
                    if self.at(TokenKind::Comma) {
                        self.advance();
                    }
                }
                self.expect(TokenKind::RightBrace)?;
            }
        }

        Ok(Expr::Element {
            kind,
            props,
            children,
            span,
        })
    }

    /// Parse interpolated string parts into an `Expr::InterpolatedString`.
    fn build_interp_expr(
        &self,
        parts: Vec<crate::lexer::InterpPart>,
        span: Span,
    ) -> Result<Expr, NewtError> {
        use crate::lexer::InterpPart;
        let mut segments = Vec::with_capacity(parts.len());
        for part in parts {
            match part {
                InterpPart::Literal(s) => {
                    segments.push(InterpSegment::Literal(s));
                }
                InterpPart::ExprSource(src) => {
                    let mut sub_parser = Parser::new(&src, None)?;
                    let expr = sub_parser.parse_expr()?;
                    segments.push(InterpSegment::Expr(Box::new(expr)));
                }
            }
        }
        Ok(Expr::InterpolatedString { parts: segments, span })
    }

    fn is_prop_keyword(&self) -> bool {
        matches!(
            self.current().kind,
            TokenKind::Width
                | TokenKind::Height
                | TokenKind::Fill
                | TokenKind::Stroke
                | TokenKind::Radius
                | TokenKind::Padding
                | TokenKind::Gap
                | TokenKind::Grow
                | TokenKind::Shrink
                | TokenKind::Align
                | TokenKind::Justify
                | TokenKind::Direction
                | TokenKind::FontSize
                | TokenKind::FontWeight
                | TokenKind::Shadow
        )
    }

    fn parse_prop(&mut self) -> Result<Prop, NewtError> {
        let span = self.current().span;
        let name = match &self.current().kind {
            TokenKind::Ident(s) => {
                let n = s.clone();
                self.advance();
                PropName::Ident(n)
            }
            TokenKind::Width => {
                self.advance();
                PropName::Width
            }
            TokenKind::Height => {
                self.advance();
                PropName::Height
            }
            TokenKind::Fill => {
                self.advance();
                PropName::Fill
            }
            TokenKind::Stroke => {
                self.advance();
                PropName::Stroke
            }
            TokenKind::Radius => {
                self.advance();
                PropName::Radius
            }
            TokenKind::Padding => {
                self.advance();
                PropName::Padding
            }
            TokenKind::Gap => {
                self.advance();
                PropName::Gap
            }
            TokenKind::Grow => {
                self.advance();
                PropName::Grow
            }
            TokenKind::Shrink => {
                self.advance();
                PropName::Shrink
            }
            TokenKind::Align => {
                self.advance();
                PropName::Align
            }
            TokenKind::Justify => {
                self.advance();
                PropName::Justify
            }
            TokenKind::Direction => {
                self.advance();
                PropName::Direction
            }
            TokenKind::FontSize => {
                self.advance();
                PropName::FontSize
            }
            TokenKind::FontWeight => {
                self.advance();
                PropName::FontWeight
            }
            TokenKind::Shadow => {
                self.advance();
                PropName::Shadow
            }
            _ => {
                return Err(NewtError::parse(
                    span,
                    "expected property name",
                ));
            }
        };
        self.expect(TokenKind::Colon)?;
        let value = self.parse_prop_value()?;
        Ok(Prop { name, value, span })
    }

    fn parse_prop_value(&mut self) -> Result<PropValue, NewtError> {
        match &self.current().kind {
            TokenKind::Number(n) => {
                let v = *n;
                self.advance();
                Ok(PropValue::Number(v))
            }
            TokenKind::String(s) => {
                let v = s.clone();
                self.advance();
                Ok(PropValue::String(v))
            }
            TokenKind::InterpolatedString(parts) => {
                let parts = parts.clone();
                let span = self.current().span;
                self.advance();
                let expr = self.build_interp_expr(parts, span)?;
                Ok(PropValue::Expr(expr))
            }
            TokenKind::HexColor(r, g, b, a) => {
                let (r, g, b, a) = (*r, *g, *b, *a);
                self.advance();
                Ok(PropValue::Color { r, g, b, a })
            }
            _ => {
                let e = self.parse_expr()?;
                Ok(PropValue::Expr(e))
            }
        }
    }
}

/// Top-level keywords that can start a program item.
const TOP_LEVEL_KEYWORDS: &[&str] = &["import", "theme", "use", "let", "state", "component", "screen"];

fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut prev = (0..=b.len()).collect::<Vec<_>>();
    for (i, ca) in a.iter().enumerate() {
        let mut curr = vec![i + 1];
        for (j, cb) in b.iter().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            curr.push((prev[j] + cost).min(prev[j + 1] + 1).min(curr[j] + 1));
        }
        prev = curr;
    }
    prev[b.len()]
}

fn closest_top_level_keyword(ident: &str) -> Option<String> {
    let ident_lower = ident.to_lowercase();
    TOP_LEVEL_KEYWORDS
        .iter()
        .min_by_key(|kw| levenshtein(&ident_lower, kw))
        .filter(|kw| levenshtein(&ident_lower, kw) <= 2)
        .map(|s| (*s).to_string())
}

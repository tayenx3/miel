//! Parser - Syntactic Analysis

// todo: ADD TESTS!!!

pub mod ast;

use crate::lexer::token::{Token, TokenKind};
use crate::common::{Span, Context, Diag, Label};
use ast::*;

pub struct Parser<'p> {
    ctx: &'p Context<'p>,
    tokens: &'p [Token],
    pos: usize,
    next_node_id: usize,
}

impl<'p> Parser<'p> {
    pub fn new(ctx: &'p Context<'p>, tokens: &'p [Token]) -> Self {
        Self {
            ctx, tokens,
            pos: 0usize,
            next_node_id: 0usize,
        }
    }

    #[inline]
    fn create_node(&mut self, kind: NodeKind, span: Span) -> Node {
        let id = NodeId(self.next_node_id);
        self.next_node_id += 1;
        Node { id, kind, span }
    }
    #[inline]
    fn advance(&mut self) { self.pos += 1 }
    #[inline]
    fn expect(&mut self, expected: TokenKind) -> Result<&Token, Diag> {
        match self.tokens.get(self.pos) {
            Some(tok) if tok.kind == expected => {
                self.advance();
                Ok(tok)
            },
            Some(other) => Err(Diag::error()
                .with_message(format!("Unexpected `{}`", other.format(self.ctx.rodeo)))
                .with_labels(vec![
                    Label::primary(self.ctx.source_id, other.span.start..other.span.end)
                        .with_message(format!(
                            "expected `{}`, found `{}`",
                            expected.format(self.ctx.rodeo),
                            other.format(self.ctx.rodeo)
                        ))
                ])),
            None => {
                let span = self.tokens.last()
                    .map(|tok| tok.span.splat_to_end())
                    .unwrap_or(Span::empty(self.ctx.source_id));
                Err(Diag::error()
                    .with_message("Unexpected end of input")
                    .with_labels(vec![
                        Label::primary(self.ctx.source_id, span.start..span.end)
                            .with_message(format!(
                                "expected `{}`, found end of input",
                                expected.format(self.ctx.rodeo)
                            ))
                    ]))
            },
        }
    }
    #[inline]
    fn expect_any_without_advance(&mut self, expected: &str) -> Result<&Token, Diag> {
        self.tokens.get(self.pos)
            .ok_or_else(|| {
                let span = self.tokens.last()
                    .map(|tok| tok.span.splat_to_end())
                    .unwrap_or(Span::empty(self.ctx.source_id));
                Diag::error()
                    .with_message("Unexpected end of input")
                    .with_labels(vec![
                        Label::primary(self.ctx.source_id, span.start..span.end)
                            .with_message(format!("expected {expected}, found end of input"))
                    ])
            })
    }
    #[inline]
    fn expect_ident(&mut self) -> Result<(lasso::Spur, Span), Diag> {
        match self.tokens.get(self.pos) {
            Some(tok) => if let TokenKind::Identifier(name) = tok.kind {
                self.advance();
                Ok((name, tok.span))
            } else {
                Err(Diag::error()
                    .with_message(format!("Unexpected `{}`", tok.format(self.ctx.rodeo)))
                    .with_labels(vec![
                        Label::primary(self.ctx.source_id, tok.span.start..tok.span.end)
                            .with_message(format!(
                                "expected identifier, found `{}`",
                                tok.format(self.ctx.rodeo)
                            ))
                    ]))
            },
            None => {
                let span = self.tokens.last()
                    .map(|tok| tok.span.splat_to_end())
                    .unwrap_or(Span::empty(self.ctx.source_id));
                Err(Diag::error()
                    .with_message("Unexpected end of input")
                    .with_labels(vec![
                        Label::primary(self.ctx.source_id, span.start..span.end)
                            .with_message("expected identifier, found end of input")
                    ]))
            },
        }
    }

    pub fn parse(&mut self) -> Result<Ast, Diag> {
        let mut nodes = Vec::new();

        while self.tokens.get(self.pos).is_some() {
            nodes.push(self.parse_statement()?);
        }

        Ok(Ast(nodes.into()))
    }

    fn parse_statement(&mut self) -> Result<Node, Diag> {
        let prev_pos = self.pos;
        let prev_node_id = self.next_node_id;
        
        match self.try_parse_decl() {
            Ok(decl) => return Ok(decl),
            Err((d, likelihood)) => if likelihood {
                return Err(d);
            }
        }
        self.pos = prev_pos;
        self.next_node_id = prev_node_id;
        
        let node = self.parse_expression(0)?;
        let span = node.span.concat(&self.expect(TokenKind::Semicolon)?.span);
        Ok(self.create_node(NodeKind::Semi(Box::new(node)), span))
    }

    fn parse_expression(&mut self, min_bp: usize) -> Result<Node, Diag> {
        let prev_pos = self.pos;
        let prev_node_id = self.next_node_id;
        
        match self.try_parse_mutation() {
            Ok(decl) => return Ok(decl),
            Err((d, likelihood)) => if likelihood {
                return Err(d);
            }
        }
        self.pos = prev_pos;
        self.next_node_id = prev_node_id;
        
        let mut lhs = self.parse_primary()?;
        while let Some(tok) = self.tokens.get(self.pos) {
            match &tok.kind {
                TokenKind::Operator(op) if op.can_infix() => {
                    let bp = {
                        let bp = op.binding_power();
                        if bp < min_bp { break }
                        bp + 1
                    };
                    self.advance();
                    let rhs = self.parse_expression(bp)?;
                    let span = lhs.span.concat(&rhs.span);
                    lhs = self.create_node(
                        NodeKind::BinaryOp {
                            op: (*op, tok.span),
                            lhs: Box::new(lhs),
                            rhs: Box::new(rhs)
                        },
                        span
                    );
                },
                TokenKind::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    while let Some(tok) = self.tokens.get(self.pos) {
                        if tok.kind == TokenKind::RParen { break }
                        args.push(self.parse_expression(0)?);
                        if self.expect(TokenKind::Comma).is_err() { break }
                    }
                    let span = tok.span.concat(&self.expect(TokenKind::RParen)?.span);
                    lhs = self.create_node(
                        NodeKind::Call {
                            callee: Box::new(lhs),
                            args,
                        },
                        span
                    );
                },
                _ => break,
            }
        }
        Ok(lhs)
    }

    fn parse_primary(&mut self) -> Result<Node, Diag> {
        let tok = *self.expect_any_without_advance("expression")?;
        match &tok.kind {
            TokenKind::IntLit(i) => {
                self.advance();
                Ok(self.create_node(
                    NodeKind::IntLit(*i),
                    tok.span
                ))
            },
            TokenKind::FloatLit(i) => {
                self.advance();
                Ok(self.create_node(
                    NodeKind::FloatLit(*i),
                    tok.span
                ))
            },
            TokenKind::BoolLit(i) => {
                self.advance();
                Ok(self.create_node(
                    NodeKind::BoolLit(*i),
                    tok.span
                ))
            },
            TokenKind::StringLit(i) => {
                self.advance();
                Ok(self.create_node(
                    NodeKind::StringLit(self.ctx.rodeo.resolve(i).to_string()),
                    tok.span
                ))
            },
            TokenKind::Identifier(i) => {
                self.advance();
                Ok(self.create_node(
                    NodeKind::Identifier(*i),
                    tok.span
                ))
            },
            TokenKind::LParen => self.parse_paren(),
            TokenKind::LCurly => self.parse_block(),
            TokenKind::Operator(op) if op.can_prefix() => {
                self.advance();
                let operand = self.parse_expression(0)?;
                let span = tok.span.concat(&operand.span);
                Ok(self.create_node(
                    NodeKind::UnaryOp {
                        op: (*op, tok.span),
                        operand: Box::new(operand)
                    },
                    span
                ))
            },
            TokenKind::KwCallable => self.parse_callable(),
            TokenKind::KwNil => {
                self.advance();
                Ok(self.create_node(
                    NodeKind::Nil,
                    tok.span
                ))
            },
            TokenKind::KwIf => self.parse_if(),
            TokenKind::KwWhile => self.parse_while(),
            TokenKind::KwReturn => self.parse_return(),
            TokenKind::KwBreak => {
                self.advance();
                Ok(self.create_node(
                    NodeKind::Break,
                    tok.span
                ))
            },
            TokenKind::KwContinue => {
                self.advance();
                Ok(self.create_node(
                    NodeKind::Continue,
                    tok.span
                ))
            },
            other => Err(Diag::error()
                .with_message(format!("Unexpected `{}`", other.format(self.ctx.rodeo)))
                .with_labels(vec![
                    Label::primary(self.ctx.source_id, tok.span.start..tok.span.end)
                        .with_message(format!(
                            "expected expression, found `{}`",
                            other.format(self.ctx.rodeo)
                        ))
                ]))
        }
    }

    fn parse_paren(&mut self) -> Result<Node, Diag> {
        let mut span = self.expect(TokenKind::LParen)?.span;
        let mut items = Vec::new();
        let mut is_tuple = false;
        while self.tokens.get(self.pos).is_some() {
            items.push(self.parse_expression(0)?);
            if self.expect(TokenKind::Comma).is_ok() {
                is_tuple = true;
            } else {
                break;
            }
        }
        span = span.concat(&self.expect(TokenKind::RParen)?.span);
        if is_tuple {
            Ok(self.create_node(
                NodeKind::Tuple(items),
                span,
            ))
        } else {
            // just `items[0]` throws an error because of borrow issues
            Ok(items.into_iter().next().unwrap())
        }
    }

    fn parse_block(&mut self) -> Result<Node, Diag> {
        let mut span = self.expect(TokenKind::LCurly)?.span;
        let mut stmts = Vec::new();
        while let Some(tok) = self.tokens.get(self.pos) {
            if tok.kind == TokenKind::RCurly { break }
            let expr = self.parse_expression(0)?;
            if let Ok(Token { span: semi_span, .. }) = self.expect(TokenKind::Semicolon) {
                let span = expr.span.concat(semi_span);
                stmts.push(self.create_node(
                    NodeKind::Semi(Box::new(expr)),
                    span
                ));
            } else {
                stmts.push(expr);
                break;
            }
        }
        span = span.concat(&self.expect(TokenKind::RCurly)?.span);
        Ok(self.create_node(
            NodeKind::Block(stmts),
            span,
        ))
    }

    fn parse_callable(&mut self) -> Result<Node, Diag> {
        let mut span = self.expect(TokenKind::KwCallable)?.span;
        self.expect(TokenKind::LParen)?;
        let mut params = Vec::new();
        while let Some(tok) = self.tokens.get(self.pos) {
            if tok.kind == TokenKind::RParen { break }
            params.push(self.parse_param()?);
            if self.expect(TokenKind::Comma).is_err() { break }
        }
        span = span.concat(&self.expect(TokenKind::RParen)?.span);
        let ret_ty = if let Some(Token { kind: TokenKind::Colon, .. }) = self.tokens.get(self.pos) {
            self.advance();
            let ty = self.parse_type()?;
            span = span.concat(&ty.span);
            Some(ty)
        } else {
            None
        };
        let sig_span = span;
        let mut body_span = self.expect(TokenKind::LCurly)?.span;
        let mut body = Vec::new();
        while let Some(tok) = self.tokens.get(self.pos) {
            if tok.kind == TokenKind::RCurly { break }
            let expr = self.parse_expression(0)?;
            if let Ok(Token { span: semi_span, .. }) = self.expect(TokenKind::Semicolon) {
                let span = expr.span.concat(semi_span);
                body.push(self.create_node(
                    NodeKind::Semi(Box::new(expr)),
                    span
                ));
            } else {
                body.push(expr);
                break;
            }
        }
        body_span = body_span.concat(&self.expect(TokenKind::RCurly)?.span);
        span = span.concat(&body_span);

        Ok(self.create_node(
            NodeKind::Callable { params, ret_ty, sig_span, body: (body, body_span) },
            span
        ))
    }

    fn parse_type(&mut self) -> Result<ParsedType, Diag> {
        let tok = *self.expect_any_without_advance("type")?;
        match &tok.kind {
            TokenKind::Identifier(i) => {
                self.advance();
                Ok(ParsedType {
                    kind: ParsedTypeKind::Identifier(*i),
                    span: tok.span
                })
            },
            TokenKind::LParen => self.parse_type_paren(),
            TokenKind::KwNil => {
                self.advance();
                Ok(ParsedType {
                    kind: ParsedTypeKind::Nil,
                    span: tok.span
                })
            },
            other => Err(Diag::error()
                .with_message(format!("Unexpected `{}`", other.format(self.ctx.rodeo)))
                .with_labels(vec![
                    Label::primary(self.ctx.source_id, tok.span.start..tok.span.end)
                        .with_message(format!(
                            "expected type, found `{}`",
                            other.format(self.ctx.rodeo)
                        ))
                ]))
        }
    }

    fn parse_type_paren(&mut self) -> Result<ParsedType, Diag> {
        let mut span = self.expect(TokenKind::LParen)?.span;
        let mut items = Vec::new();
        let mut is_tuple = false;
        while self.tokens.get(self.pos).is_some() {
            items.push(self.parse_type()?);
            if self.expect(TokenKind::Comma).is_ok() {
                is_tuple = true;
            } else {
                break;
            }
        }
        span = span.concat(&self.expect(TokenKind::RParen)?.span);
        if is_tuple {
            Ok(ParsedType {
                kind: ParsedTypeKind::Tuple(items),
                span,
            })
        } else {
            // just `items[0]` throws an error because of borrow issues
            Ok(items.into_iter().next().unwrap())
        }
    }

    fn parse_param(&mut self) -> Result<Param, Diag> {
        let (name, mut span) = self.expect_ident()?;
        self.expect(TokenKind::Colon)?;
        let ty = self.parse_type()?;
        span = span.concat(&ty.span);
        Ok(Param { name, ty, span })
    }

    // the bool in the error result is the likelihood of it being a incorrect mutation instead of a different expression
    fn try_parse_mutation(&mut self) -> Result<Node, (Diag, bool)> {
        // later: parse tuple items, array indices and struct fields
        let (name, mut span) = self.expect_ident().map_err(|err| (err, false))?;
        if let Some(Token { kind: TokenKind::Reassign(op), span: op_span }) = self.tokens.get(self.pos) {
            self.advance();
            let expr = self.parse_expression(0).map_err(|err| (err, true))?;
            span = span.concat(&expr.span);
            Ok(self.create_node(
                NodeKind::Mutation {
                    name,
                    op: (*op, *op_span),
                    expr: Box::new(expr)
                },
                span
            ))
        } else {
            // error isn't read anyway
            Err((Diag::error(), false))
        }
    }

    // the bool in the error result is the likelihood of it being a incorrect variable declaration instead of a different expression
    fn try_parse_decl(&mut self) -> Result<Node, (Diag, bool)> {
        let (name, mut span) = self.expect_ident().map_err(|err| (err, false))?;
        if self.expect(TokenKind::Colon).is_ok() {
            let ty = self.parse_type().map_err(|err| (err, true))?;
            if self.expect(TokenKind::CColon).is_ok() {
                let expr = self.parse_expression(0).map_err(|err| (err, true))?;
                span = span.concat(&expr.span);
                Ok(self.create_node(
                    NodeKind::TypedConstDecl {
                        name, ty,
                        expr: Box::new(expr)
                    },
                    span
                ))
            } else {
                self.expect(TokenKind::Reassign(crate::common::ReassignmentOp::Assign))
                    .map_err(|err| (err, true))?;
                let expr = self.parse_expression(0).map_err(|err| (err, true))?;
                span = span.concat(&expr.span);
                Ok(self.create_node(
                    NodeKind::TypedVarDecl { name, ty, expr: Box::new(expr) },
                    span
                ))
            }
        } else if self.expect(TokenKind::CColon).is_ok() {
            let expr = self.parse_expression(0).map_err(|err| (err, true))?;
            span = span.concat(&expr.span);
            Ok(self.create_node(
                NodeKind::ShortConstDecl {
                    name,
                    expr: Box::new(expr)
                },
                span
            ))
        } else {
            self.expect(TokenKind::Walrus).map_err(|err| (err, false))?;
            let expr = self.parse_expression(0).map_err(|err| (err, true))?;
            span = span.concat(&expr.span);
            Ok(self.create_node(
                NodeKind::ShortVarDecl { name, expr: Box::new(expr) },
                span
            ))
        }
    }

    fn parse_if(&mut self) -> Result<Node, Diag> {
        let mut span = self.expect(TokenKind::KwIf)?.span;
        let cond = Box::new(self.parse_expression(0)?);
        self.expect(TokenKind::KwThen)?;
        let then = Box::new(self.parse_expression(0)?);
        let else_ = if self.expect(TokenKind::KwElse).is_ok() {
            let expr = self.parse_expression(0)?;
            span = span.concat(&expr.span);
            Some(Box::new(expr))
        } else {
            span = span.concat(&then.span);
            None
        };
        Ok(self.create_node(
            NodeKind::If { cond, then, else_ },
            span
        ))
    }
    
    fn parse_while(&mut self) -> Result<Node, Diag> {
        let mut span = self.expect(TokenKind::KwWhile)?.span;
        let cond = Box::new(self.parse_expression(0)?);
        self.expect(TokenKind::KwDo)?;
        let body = Box::new(self.parse_expression(0)?);
        span = span.concat(&body.span);
        Ok(self.create_node(
            NodeKind::While { cond, body },
            span
        ))
    }

    fn parse_return(&mut self) -> Result<Node, Diag> {
        let mut span = self.expect(TokenKind::KwReturn)?.span;
        let prev_pos = self.pos;
        if let Ok(expr) = self.parse_expression(0) {
            span = span.concat(&expr.span);
            Ok(self.create_node(
                NodeKind::Return(Some(Box::new(expr))),
                span
            ))
        } else {
            self.pos = prev_pos;
            Ok(self.create_node(
                NodeKind::Return(None),
                span
            ))
        }
    }
}
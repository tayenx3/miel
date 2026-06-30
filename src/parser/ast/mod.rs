//! AST - Abstract Syntax Tree

pub mod visit;

use crate::common::{Operator, ReassignmentOp, Span};
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq)]
pub struct Ast(pub Arc<[Node]>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Param {
    pub name: lasso::Spur,
    pub ty: ParsedType,
    pub span: Span
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    IntLit(lasso::Spur), FloatLit(lasso::Spur),
    BoolLit(bool), StringLit(String),
    Identifier(lasso::Spur),
    Nil,
    BinaryOp {
        op: (Operator, Span),
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    UnaryOp {
        op: (Operator, Span),
        operand: Box<Node>,
    },
    Tuple(Vec<Node>),
    Block(Vec<Node>),
    Semi(Box<Node>),

    Callable {
        params: Vec<Param>,
        ret_ty: Option<ParsedType>,
        sig_span: Span,
        body: (Vec<Node>, Span)
    },
    Call {
        callee: Box<Node>,
        args: Vec<Node>
    },

    ShortVarDecl {
        name: lasso::Spur,
        expr: Box<Node>,
    },
    TypedVarDecl {
        name: lasso::Spur,
        ty: ParsedType,
        expr: Box<Node>,
    },
    Mutation {
        name: lasso::Spur,
        op: (ReassignmentOp, Span),
        expr: Box<Node>,
    },

    ShortConstDecl {
        name: lasso::Spur,
        expr: Box<Node>,
    },
    TypedConstDecl {
        name: lasso::Spur,
        ty: ParsedType,
        expr: Box<Node>,
    },

    If {
        cond: Box<Node>,
        then: Box<Node>,
        else_: Option<Box<Node>>,
    },
    While {
        cond: Box<Node>,
        body: Box<Node>,
    },
    Return(Option<Box<Node>>),
    Break, Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub span: Span,
}

impl Node {
    pub fn accept<'ast, V: visit::Visitor<'ast>>(&'ast self, visitor: &mut V) -> V::Result {
        match &self.kind {
            NodeKind::IntLit(i) => visitor.visit_int_lit(self, i),
            NodeKind::FloatLit(i) => visitor.visit_float_lit(self, i),
            NodeKind::BoolLit(i) => visitor.visit_bool_lit(self, *i),
            NodeKind::StringLit(i) => visitor.visit_string_lit(self, i),
            NodeKind::Identifier(i) => visitor.visit_identifier(self, i),
            NodeKind::Nil => visitor.visit_nil(self),
            NodeKind::BinaryOp {
                op: (op, op_span),
                lhs, rhs
            } => visitor.visit_binary_op(op, op_span, lhs, rhs),
            NodeKind::UnaryOp {
                op: (op, op_span),
                operand
            } => visitor.visit_unary_op(op, op_span, operand),
            NodeKind::Tuple(items) => visitor.visit_tuple(self, items),
            NodeKind::Block(stmts) => visitor.visit_block(self, stmts),
            NodeKind::Semi(stmt) => visitor.visit_semi(self, stmt),
            NodeKind::Callable {
                params, ret_ty,
                sig_span,
                body: (body, body_span)
            } => visitor.visit_callable(self, params, ret_ty.as_ref(), sig_span, body, body_span),
            NodeKind::Call { callee, args } => visitor.visit_call(self, callee, args),
            NodeKind::ShortVarDecl { name, expr } => visitor.visit_short_var_decl(self, name, expr),
            NodeKind::TypedVarDecl { name, ty, expr } => visitor.visit_typed_var_decl(self, name, ty, expr),
            NodeKind::Mutation {
                name, op: (op, op_span),
                expr
            } => visitor.visit_mutation(self, name, op, op_span, expr),
            NodeKind::ShortConstDecl { name, expr } => visitor.visit_short_const_decl(self, name, expr),
            NodeKind::TypedConstDecl { name, ty, expr } => visitor.visit_typed_const_decl(self, name, ty, expr),
            NodeKind::If { cond, then, else_ } => visitor.visit_if(self, cond, then, else_.as_deref()),
            NodeKind::While { cond, body } => visitor.visit_while(self, cond, body),
            NodeKind::Return(i) => visitor.visit_return(self, i.as_deref()),
            NodeKind::Break => visitor.visit_break(self),
            NodeKind::Continue => visitor.visit_continue(self),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParsedTypeKind {
    Identifier(lasso::Spur),
    Nil,
    Tuple(Vec<ParsedType>),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ParsedType {
    pub kind: ParsedTypeKind,
    pub span: Span,
}
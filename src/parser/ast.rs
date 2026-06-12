use crate::common::{Operator, Span};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConstKind {
    Proc, Func
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    IntLit(i64), FloatLit(f64),
    StringLit(String),
    Identifier(lasso::Spur),
    Nil,
    BinaryOp {
        op: Operator,
        lhs: Box<Node>,
        rhs: Box<Node>,
    },
    UnaryOp {
        op: Operator,
        operand: Box<Node>,
    },
    Tuple(Vec<Node>),

    Callable {
        params: Vec<Param>,
        ret_ty: Option<ParsedType>,
        body: Vec<Node>
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

    ShortConstDecl {
        name: (Option<ConstKind>, lasso::Spur),
        expr: Box<Node>,
    },
    ConstDecl {
        name: (Option<ConstKind>, lasso::Spur),
        ty: ParsedType,
        expr: Box<Node>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub id: NodeId,
    pub kind: NodeKind,
    pub span: Span,
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
//! AST Visitor API

use crate::common::{Operator, ReassignmentOp, Span};
use super::{Node, NodeKind, Param, ParsedType};

pub trait Visitor<'ast> {
    type Result;

    fn visit_int_lit(&mut self, node: &'ast Node, value: &'ast lasso::Spur) -> Self::Result;
    fn visit_float_lit(&mut self, node: &'ast Node, value: &'ast lasso::Spur) -> Self::Result;
    fn visit_bool_lit(&mut self, node: &'ast Node, value: bool) -> Self::Result;
    fn visit_string_lit(&mut self, node: &'ast Node, value: &'ast str) -> Self::Result;
    fn visit_identifier(&mut self, node: &'ast Node, value: &'ast lasso::Spur) -> Self::Result;
    fn visit_nil(&mut self, node: &'ast Node) -> Self::Result;
    fn visit_binary_op(&mut self, op: &'ast Operator, op_span: &'ast Span, lhs: &'ast Node, rhs: &'ast Node) -> Self::Result;
    fn visit_unary_op(&mut self, op: &'ast Operator, op_span: &'ast Span, operand: &'ast Node) -> Self::Result;
    fn visit_tuple(&mut self, node: &'ast Node, items: &'ast [Node]) -> Self::Result;
    fn visit_block(&mut self, node: &'ast Node, stmts: &'ast [Node]) -> Self::Result;
    fn visit_semi(&mut self, node: &'ast Node, stmt: &'ast Node) -> Self::Result;
    fn visit_callable(
        &mut self, node: &'ast Node, params: &'ast [Param], ret_ty: Option<&'ast ParsedType>,
        sig_span: &Span, body: &'ast [Node], body_span: &'ast Span
    ) -> Self::Result;
    fn visit_call(&mut self, node: &'ast Node, callee: &'ast Node, args: &'ast [Node]) -> Self::Result;
    fn visit_short_var_decl(&mut self, node: &'ast Node, name: &'ast lasso::Spur, expr: &'ast Node) -> Self::Result;
    fn visit_typed_var_decl(&mut self, node: &'ast Node, name: &'ast lasso::Spur, ty: &'ast ParsedType, expr: &'ast Node) -> Self::Result;
    fn visit_mutation(
        &mut self, node: &'ast Node, name: &'ast lasso::Spur, op: &'ast ReassignmentOp,
        op_span: &'ast Span, expr: &'ast Node
    ) -> Self::Result;
    fn visit_short_const_decl(&mut self, node: &'ast Node, name: &'ast lasso::Spur, expr: &'ast Node) -> Self::Result;
    fn visit_typed_const_decl(&mut self, node: &'ast Node, name: &'ast lasso::Spur, ty: &'ast ParsedType, expr: &'ast Node) -> Self::Result;
    fn visit_if(&mut self, node: &'ast Node, cond: &'ast Node, then: &'ast Node, else_: Option<&'ast Node>) -> Self::Result;
    fn visit_while(&mut self, node: &'ast Node, cond: &'ast Node, body: &'ast Node) -> Self::Result;
    fn visit_return(&mut self, node: &'ast Node, value: Option<&'ast Node>) -> Self::Result;
    fn visit_break(&mut self, node: &'ast Node) -> Self::Result;
    fn visit_continue(&mut self, node: &'ast Node) -> Self::Result;
}
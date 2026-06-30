use std::str::FromStr;
use num_bigint::BigInt;
use crate::parser::ast::visit::Visitor;
use crate::parser::ast::*;
use crate::common::{Diag, Label, Operator, ReassignmentOp, Span};
use super::super::errors;
use super::super::scope::{Symbol, SymbolMap};
use super::super::types::{Type, TypeManager};
use super::ConstValue;

pub struct ConstEvaluator<'co> {
    rodeo: &'co lasso::Rodeo,
    scope: &'co mut Vec<SymbolMap>,
    types: &'co mut TypeManager,
}

impl<'co> ConstEvaluator<'co> {
    pub fn new(rodeo: &'co lasso::Rodeo, scope: &'co mut Vec<SymbolMap>, types: &'co mut TypeManager) -> Self {
        Self { rodeo, scope, types }
    }
    
    fn invalid_const_expr_err(span: &Span, s: &str) -> Diag {
        let span = span.as_ref();
        Diag::error()
            .with_message("Unexpected non-constant expression")
            .with_labels(vec![
                Label::primary(span.source_id, span.start..span.end)
                    .with_message(s)
            ])
    }
}

impl<'ast, 'co> Visitor<'ast> for ConstEvaluator<'co> {
    type Result = Result<ConstValue, Vec<Diag>>;

    fn visit_int_lit(&mut self, node: &'ast Node, value: &'ast lasso::Spur) -> Self::Result {
        let lit = self.rodeo.resolve(value);
        let val = BigInt::from_str(lit)
            .map_err(|err| vec![Diag::error()
                .with_message("Invalid literal")
                .with_labels(vec![
                    Label::primary(node.span.source_id, node.span.start..node.span.end)
                        .with_message(&err.to_string())
                ])])?;
        let ty_id = self.types.create_type(Type::AmbiguousInt);
        self.types.assign_node_type(node.id, ty_id);
        Ok(ConstValue::Int(val))
    }

    fn visit_float_lit(&mut self, node: &'ast Node, value: &'ast lasso::Spur) -> Self::Result {
        let lit = self.rodeo.resolve(value);
        let val = f64::from_str(lit)
            .map_err(|err| vec![Diag::error()
                .with_message("Invalid literal")
                .with_labels(vec![
                    Label::primary(node.span.source_id, node.span.start..node.span.end)
                        .with_message(&err.to_string())
                ])])?;
        let ty_id = self.types.create_type(Type::AmbiguousFloat);
        self.types.assign_node_type(node.id, ty_id);
        Ok(ConstValue::Float(val))
    }

    fn visit_bool_lit(&mut self, node: &'ast Node, value: bool) -> Self::Result {
        self.types.assign_node_type(node.id, self.types.predef_types.bool_id);
        Ok(ConstValue::Bool(value))
    }

    fn visit_string_lit(&mut self, _node: &'ast Node, value: &'ast str) -> Self::Result {
        todo!("strings")
    }

    fn visit_identifier(&mut self, node: &'ast Node, value: &'ast lasso::Spur) -> Self::Result {
        for map in self.scope.iter().rev() {
            if let Some(Symbol { ty, val, .. }) = map.find_symbol(value) {
                if let Some(v) = val {
                    self.types.assign_node_type(node.id, *ty);
                    return Ok(v.clone());
                } else {
                    return Err(vec![Self::invalid_const_expr_err(
                        &node.span,
                        &format!("`{}` is not a constant", self.rodeo.resolve(value))
                    )]);
                }
            }
        }
        Err(vec![errors::unknown_ident(
            &node.span,
            self.rodeo.resolve(value),
            &[],
        )])
    }

    fn visit_nil(&mut self, _node: &'ast Node) -> Self::Result {
        Ok(ConstValue::Nil)
    }

    fn visit_binary_op(&mut self, op: &'ast Operator, op_span: &'ast Span, lhs: &'ast Node, rhs: &'ast Node) -> Self::Result {
        todo!()
    }

    fn visit_unary_op(&mut self, op: &'ast Operator, op_span: &'ast Span, operand: &'ast Node) -> Self::Result {
        todo!()
    }

    fn visit_tuple(&mut self, _node: &'ast Node, items: &'ast [Node]) -> Self::Result {
        let mut item_vals = Vec::new();
        let mut errors = Vec::new();
        for item in items {
            match item.accept(self) {
                Ok(val) => if val == ConstValue::Never {
                    if errors.is_empty() {
                        return Ok(val);
                    } else {
                        return Err(errors);
                    }
                } else {
                    item_vals.push(val);
                },
                Err(err) => errors.extend(err),
            }
        }
        if errors.is_empty() {
            Ok(ConstValue::Tuple(item_vals.into()))
        } else {
            Err(errors)
        }
    }

    fn visit_block(&mut self, _node: &'ast Node, stmts: &'ast [Node]) -> Self::Result {
        let mut final_val = ConstValue::Nil;
        let mut errors = Vec::new();
        for stmt in stmts {
            match stmt.accept(self) {
                Ok(val) => {
                    if val == ConstValue::Never { break }
                    final_val = val;
                },
                Err(err) => errors.extend(err),
            }
        }
        if errors.is_empty() {
            Ok(final_val)
        } else {
            Err(errors)
        }
    }

    fn visit_semi(&mut self, _node: &'ast Node, stmt: &'ast Node) -> Self::Result {
        if stmt.accept(self)? == ConstValue::Never {
            Ok(ConstValue::Never)
        } else {
            Ok(ConstValue::Nil)
        }
    }

    fn visit_callable(
        &mut self, node: &'ast Node, params: &'ast [Param], ret_ty: Option<&'ast ParsedType>,
        sig_span: &Span, body: &'ast [Node], body_span: &'ast Span
    ) -> Self::Result {
        todo!()
    }
    
    fn visit_if(&mut self, _node: &'ast Node, cond: &'ast Node, then: &'ast Node, else_: Option<&'ast Node>) -> Self::Result {
        todo!()
    }

    fn visit_call(&mut self, node: &'ast Node, _callee: &'ast Node, _args: &'ast [Node]) -> Self::Result {
        Err(vec![Self::invalid_const_expr_err(&node.span, "expected constant expression")])
    }

    fn visit_short_var_decl(&mut self, node: &'ast Node, _name: &'ast lasso::Spur, expr: &'ast Node) -> Self::Result {
        Err(vec![Self::invalid_const_expr_err(&node.span, "expected constant expression")])
    }

    fn visit_typed_var_decl(&mut self, node: &'ast Node, _name: &'ast lasso::Spur, ty: &'ast ParsedType, expr: &'ast Node) -> Self::Result {
        Err(vec![Self::invalid_const_expr_err(&node.span, "expected constant expression")])
    }

    fn visit_mutation(
        &mut self, node: &'ast Node, _name: &'ast lasso::Spur, _op: &'ast ReassignmentOp,
        _op_span: &'ast Span, _expr: &'ast Node
    ) -> Self::Result {
        Err(vec![Self::invalid_const_expr_err(&node.span, "expected constant expression")])
    }

    fn visit_short_const_decl(&mut self, node: &'ast Node, _name: &'ast lasso::Spur, _expr: &'ast Node) -> Self::Result {
        Err(vec![Self::invalid_const_expr_err(&node.span, "expected constant expression")])
    }

    fn visit_typed_const_decl(&mut self, node: &'ast Node, _name: &'ast lasso::Spur, _ty: &'ast ParsedType, _expr: &'ast Node) -> Self::Result {
        Err(vec![Self::invalid_const_expr_err(&node.span, "expected constant expression")])
    }

    fn visit_while(&mut self, node: &'ast Node, _cond: &'ast Node, _body: &'ast Node) -> Self::Result {
        Err(vec![Self::invalid_const_expr_err(&node.span, "expected constant expression")])
    }

    fn visit_return(&mut self, node: &'ast Node, _value: Option<&'ast Node>) -> Self::Result {
        Err(vec![Self::invalid_const_expr_err(&node.span, "expected constant expression")])
    }

    fn visit_break(&mut self, node: &'ast Node) -> Self::Result {
        Err(vec![Self::invalid_const_expr_err(&node.span, "expected constant expression")])
    }

    fn visit_continue(&mut self, node: &'ast Node) -> Self::Result {
        Err(vec![Self::invalid_const_expr_err(&node.span, "expected constant expression")])
    }
}
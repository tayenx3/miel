use crate::common::{Diag, Label};
use crate::parser::ast::visit::Visitor;
use crate::parser::ast::{Ast, Node, NodeKind};
use super::super::scope::SymbolMap;
use super::super::types::TypeManager;
use super::super::errors;
use super::eval::ConstEvaluator;

pub struct ConstantCollector<'cc> {
    rodeo: &'cc lasso::Rodeo,
    scope: &'cc mut Vec<SymbolMap>,
    types: &'cc mut TypeManager,
}

impl<'cc> ConstantCollector<'cc> {
    pub fn new(rodeo: &'cc lasso::Rodeo, scope: &'cc mut Vec<SymbolMap>, types: &'cc mut TypeManager) -> Self {
        Self { rodeo, scope, types }
    }

    pub fn collect_constants(&mut self, items: &[Node]) -> Result<(), Vec<Diag>> {
        let mut errors = Vec::new();
        for item in items {
            if let Err(errs) = self.collect_constant(item) {
                errors.extend(errs);
            }
        }
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn collect_constant(&mut self, item: &Node) -> Result<(), Vec<Diag>> {
        match &item.kind {
            NodeKind::Semi(stmt) => self.collect_constant(stmt),
            NodeKind::ShortConstDecl { name, expr } => {
                let val = expr.accept(&mut ConstEvaluator::new(
                    self.rodeo,
                    self.scope,
                    self.types,
                ))?;
                let ty = self.types.get_node_type(&expr.id).copied().unwrap();
                self.scope.last_mut().unwrap()
                    .define_const_symbol(*name, ty, item.span, val);
                Ok(())
            },
            NodeKind::TypedConstDecl { name, ty, expr } => {
                let mut errors = Vec::new();
                let annotated_ty = match self.types.resolve_type(ty) {
                    Ok(ty) => Some(ty),
                    Err(errs) => {
                        errors.extend(errs);
                        None
                    }
                };
                let val = match expr.accept(&mut ConstEvaluator::new(
                    self.rodeo,
                    self.scope,
                    self.types,
                )) {
                    Ok(val) => Some(val),
                    Err(errs) => {
                        errors.extend(errs);
                        None
                    }
                };
                if !errors.is_empty() {
                    return Err(errors);
                }
                let annotated_ty = annotated_ty.unwrap();
                let init_ty = self.types.get_node_type(&expr.id).copied().unwrap();
                let final_ty = if self.types.can_coerce(&init_ty, &annotated_ty) {
                    self.types.coerce_type(init_ty, annotated_ty);
                    annotated_ty
                } else {
                    return Err(vec![errors::type_mismatch(
                        &expr.span,
                        &self.types.get_type(&annotated_ty).unwrap().format(&*self.types),
                        &self.types.get_type(&init_ty).unwrap().format(&*self.types),
                        &["annotated type and initializer type must be of the same type"],
                    )]);
                };
                self.scope.last_mut().unwrap()
                    .define_const_symbol(*name, final_ty, item.span, val.unwrap());
                Ok(())
            },
            _ => Ok(())
        }
    }
}
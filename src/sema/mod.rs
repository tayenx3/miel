pub mod symbol;
pub mod ty;

use std::collections::HashMap;
use symbol::*;
use ty::*;
use crate::parser::ast::{Ast, Node, NodeId, NodeKind, ParsedType, ParsedTypeKind};
use crate::common::{ContextMut, Diag, Label};

const CANDIDATE_SCORE_THRESHOLD: f64 = 0.7;

// the Vec<Vec<SymbolMap>> was created so that child functions cannot access the variables of the parent function,
// only constants from the root-level scope
// 
// each Vec<SymbolMap> represents a function's scope stack
pub struct SemaChecker<'sch> {
    ctx: &'sch ContextMut<'sch>,
    root_scope: ConstSymbolMap,
    scope: Vec<Vec<SymbolMap>>,
    ty_pool: TypePool,
    type_map: HashMap<NodeId, TypeId>,
    type_registry: HashMap<lasso::Spur, TypeId>,
}

impl<'sch> SemaChecker<'sch> {
    pub fn new(ctx: &'sch mut ContextMut<'sch>) -> Self {
        let ty_pool = TypePool::new();
        let mut type_registry = HashMap::new();
        
        type_registry.insert(ctx.rodeo.get_or_intern("int"), ty_pool.int_id);
        type_registry.insert(ctx.rodeo.get_or_intern("uint"), ty_pool.uint_id);
        type_registry.insert(ctx.rodeo.get_or_intern("i8"), ty_pool.i8_id);
        type_registry.insert(ctx.rodeo.get_or_intern("i16"), ty_pool.i16_id);
        type_registry.insert(ctx.rodeo.get_or_intern("i32"), ty_pool.i32_id);
        type_registry.insert(ctx.rodeo.get_or_intern("i64"), ty_pool.i64_id);
        type_registry.insert(ctx.rodeo.get_or_intern("u8"), ty_pool.u8_id);
        type_registry.insert(ctx.rodeo.get_or_intern("u16"), ty_pool.u16_id);
        type_registry.insert(ctx.rodeo.get_or_intern("u32"), ty_pool.u32_id);
        type_registry.insert(ctx.rodeo.get_or_intern("u64"), ty_pool.u64_id);
        type_registry.insert(ctx.rodeo.get_or_intern("float"), ty_pool.float_id);
        type_registry.insert(ctx.rodeo.get_or_intern("f32"), ty_pool.f32_id);
        type_registry.insert(ctx.rodeo.get_or_intern("f64"), ty_pool.f64_id);
        type_registry.insert(ctx.rodeo.get_or_intern("nil"), ty_pool.nil_id);
        
        Self {
            ctx,
            root_scope: ConstSymbolMap::new(),
            scope: Vec::new(),
            ty_pool,
            type_map: HashMap::new(),
            type_registry
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Result<(), Vec<Diag>> {
        let mut errors = Vec::new();
        for item in ast.0.iter() {
            if let Err(errs) = self.check_root_item(item) {
                errors.extend(errs);
            }
        }
        if !errors.is_empty() { return Err(errors) }
        for item in ast.0.iter() {
            if let Err(errs) = self.collect_function(item) {
                errors.extend(errs);
            }
        }
        Ok(())
    }

    fn resolve_type(&mut self, p_ty: &ParsedType) -> Result<TypeId, Vec<Diag>> {
        match &p_ty.kind {
            ParsedTypeKind::Identifier(i) => {
                let mut candidate = None;
                let mut last_score = 0.0;
                for (name, ty) in &self.type_registry {
                    if name == i {
                        return Ok(*ty);
                    } else {
                        let score = strsim::jaro_winkler(
                            self.ctx.rodeo.resolve(i),
                            self.ctx.rodeo.resolve(name)
                        );
                        if score > CANDIDATE_SCORE_THRESHOLD && score > last_score {
                            last_score = score;
                            candidate = Some(name);
                        }
                    }
                }
                if let Some(c) = candidate {
                    Err(vec![Diag::error()
                        .with_message("Unknown type")
                        .with_labels(vec![
                            Label::primary(self.ctx.source_id, p_ty.span.start..p_ty.span.end)
                                .with_message(format!("`{}` not in scope", self.ctx.rodeo.resolve(i)))
                        ]).with_notes(vec![
                            format!("Did you mean `{}`?", self.ctx.rodeo.resolve(c))
                        ])])
                } else {
                    Err(vec![Diag::error()
                        .with_message("Unknown type")
                        .with_labels(vec![
                            Label::primary(self.ctx.source_id, p_ty.span.start..p_ty.span.end)
                                .with_message(format!("`{}` not in scope", self.ctx.rodeo.resolve(i)))
                        ])])
                }
            },
            ParsedTypeKind::Nil => Ok(self.ty_pool.nil_id),
            ParsedTypeKind::Tuple(items) => {
                let mut errors = Vec::new();
                let mut item_tys = Vec::new();
                for i in items {
                    match self.resolve_type(i) {
                        Ok(ty) => item_tys.push(ty),
                        Err(err) => errors.extend(err),
                    }
                }
                if !errors.is_empty() {
                    return Err(errors);
                }
                Ok(self.ty_pool.create_type(Type::Tuple(item_tys)))
            },
        }
    }

    fn check_root_item(&mut self, item: &Node) -> Result<(), Vec<Diag>> {
        match &item.kind {
            NodeKind::ShortConstDecl { name, expr } => {
                let mut errors = Vec::new();
                if let Ok((_, _, defined_at)) = self.root_scope.find_symbol(name, &self.ctx.as_ctx()) {
                    errors.push(Diag::error()
                        .with_message("Already defined constant")
                        .with_labels(vec![
                            Label::primary(self.ctx.source_id, item.span.start..item.span.end)
                                .with_message(format!(
                                    "`{}` already defined",
                                    self.ctx.rodeo.resolve(name)
                                )),
                            Label::secondary(self.ctx.source_id, defined_at.start..defined_at.end)
                                .with_message(format!(
                                    "`{}` was defined here",
                                    self.ctx.rodeo.resolve(name)
                                )),
                        ]));
                }
                match self.check_node_const(expr) {
                    Ok((init_ty_id, init_val)) => {
                        if !errors.is_empty() { return Err(errors) }
                        self.root_scope.define_symbol(*name, init_ty_id, init_val, item.span)
                    },
                    Err(errs) => {
                        errors.extend(errs);
                        return Err(errors);
                    }
                }
                Ok(())
            },
            NodeKind::ConstDecl { name, ty, expr } => {
                let mut errors = Vec::new();
                let resolved_ty = match self.resolve_type(ty) {
                    Ok(o) => o,
                    Err(err) => {
                        errors.extend(err);
                        self.ty_pool.nil_id
                    },
                };
                let (init_ty_id, init_val) = match self.check_node_const(expr) {
                    Ok(o) => o,
                    Err(err) => {
                        errors.extend(err);
                        (self.ty_pool.nil_id, ConstValue::Nil)
                    },
                };
                if !errors.is_empty() {
                    return Err(errors);
                }

                if self.ty_pool.get_type(&init_ty_id)
                    .unwrap()
                    .is_coerceable_into(
                        &self.ty_pool.get_type(&resolved_ty)
                            .unwrap()
                    )
                {
                    self.ty_pool.coerce_type(
                        &init_ty_id,
                        self.ty_pool.get_type(&resolved_ty).unwrap().clone()
                    );
                } else {
                    return Err(vec![Diag::error()
                        .with_message("Type mismatch")
                        .with_labels(vec![
                            Label::primary(self.ctx.source_id, item.span.start..item.span.end)
                                .with_message(format!(
                                    "expected `{}`, found `{}`",
                                    self.ty_pool.get_type(&resolved_ty)
                                        .unwrap().format(&self.ty_pool),
                                    self.ty_pool.get_type(&init_ty_id)
                                        .unwrap().format(&self.ty_pool)
                                ))
                        ])]);
                }

                self.root_scope.define_symbol(*name, resolved_ty, init_val, item.span);
                Ok(())
            },
            _ => Err(vec![Diag::error()
                .with_message("Expected root-level item")
                .with_labels(vec![
                    Label::primary(self.ctx.source_id, item.span.start..item.span.end)
                        .with_message("invalid root-level item")
                ])])
        }
    }
    
    fn collect_function(&mut self, item: &Node) -> Result<(), Vec<Diag>> {
        match &item.kind {
            NodeKind::ShortConstDecl { name, expr }
            | NodeKind::ConstDecl { name, expr, .. } => {
                if let NodeKind::Callable { params, body, .. } = &expr.kind {
                    let mut errors = Vec::new();
                    if let Ok(Type::Callable { params: param_tys, .. })
                        = self.root_scope.find_symbol(name, &self.ctx.as_ctx())
                            .map(|x| self.ty_pool.get_type(x.0).unwrap())
                    {
                        let mut smap = SymbolMap::new();
                        for (p, ty) in params.iter().zip(param_tys) {
                            smap.define_symbol(p.name, *ty);
                        }
                        self.scope.push(vec![smap]);
                    }
                    for stmt in body {
                        if let Err(errs) = self.check_node(stmt) {
                            errors.extend(errs);
                        }
                    }
                    if !errors.is_empty() { return Err(errors) }
                }
                Ok(())
            },
            _ => Err(vec![Diag::error()
                .with_message("Expected root-level item")
                .with_labels(vec![
                    Label::primary(self.ctx.source_id, item.span.start..item.span.end)
                        .with_message("invalid root-level item")
                ])])
        }
    }

    fn check_node(&mut self, node: &Node) -> Result<(), Vec<Diag>> {
        todo!()
    }
    
    fn check_node_const(&mut self, node: &Node) -> Result<(TypeId, ConstValue), Vec<Diag>> {
        match &node.kind {
            NodeKind::IntLit(i) => {
                let t_id = self.ty_pool.create_type(Type::AmbiguousInt);
                self.type_map.insert(node.id, t_id);
                Ok((t_id, ConstValue::Int(*i)))
            },
            NodeKind::FloatLit(i) => {
                let t_id = self.ty_pool.create_type(Type::AmbiguousFloat);
                self.type_map.insert(node.id, t_id);
                Ok((t_id, ConstValue::Float(*i)))
            },
            NodeKind::StringLit(_) => todo!("strings"),
            NodeKind::Identifier(i) => {
                let (t_id, val) = match self.root_scope.find_symbol(i, &self.ctx.as_ctx()) {
                    Ok((ty, val, _)) => (*ty, val.clone()),
                    Err(Some(candidate)) => return Err(vec![Diag::error()
                        .with_message("Unknown identifier")
                        .with_labels(vec![
                            Label::primary(self.ctx.source_id, node.span.start..node.span.end)
                                .with_message(format!("`{}` not in scope", self.ctx.rodeo.resolve(i)))
                        ]).with_notes(vec![
                            format!("Did you mean `{}`?", self.ctx.rodeo.resolve(candidate))
                        ])]),
                    Err(None) => return Err(vec![Diag::error()
                        .with_message("Unknown identifier")
                        .with_labels(vec![
                            Label::primary(self.ctx.source_id, node.span.start..node.span.end)
                                .with_message(format!("`{}` not in scope", self.ctx.rodeo.resolve(i)))
                        ])]),
                };
                self.type_map.insert(node.id, t_id);
                Ok((t_id, val))
            },
            NodeKind::Nil => {
                let t_id = self.ty_pool.nil_id;
                self.type_map.insert(node.id, t_id);
                Ok((t_id, ConstValue::Nil))
            },
            NodeKind::BinaryOp { op, lhs, rhs } => {
                let mut errors = Vec::new();
                let (lty, lval) = match self.check_node_const(lhs) {
                    Ok(o) => o,
                    Err(errs) => {
                        errors.extend(errs);
                        (self.ty_pool.nil_id, ConstValue::Nil)
                    },
                };
                let (rty, rval) = match self.check_node_const(rhs) {
                    Ok(o) => o,
                    Err(errs) => {
                        errors.extend(errs);
                        (self.ty_pool.nil_id, ConstValue::Nil)
                    },
                };
                if !errors.is_empty() { return Err(errors); }
                if let Some(ty) = op.infix_output_ty(&lty, &rty, &mut self.ty_pool) {
                    self.type_map.insert(node.id, ty);
                    let result = op.eval_infix(&lval, &rval).unwrap();
                    Ok((ty, result))
                } else {
                    Err(vec![Diag::error()
                        .with_message("Type mismatch")
                        .with_labels(vec![
                            Label::primary(self.ctx.source_id, node.span.start..node.span.end)
                                .with_message(format!(
                                    "cannot do `{op}` infix operation on types `{}` and `{}`",
                                    self.ty_pool.get_type(&lty)
                                        .unwrap().format(&self.ty_pool),
                                    self.ty_pool.get_type(&rty)
                                        .unwrap().format(&self.ty_pool),
                                ))
                        ])])
                }
            },
            NodeKind::UnaryOp { op, operand } => {
                let (oty, oval) = self.check_node_const(operand)?;
                if let Some(ty) = op.prefix_output_ty(&oty, &self.ty_pool) {
                    self.type_map.insert(node.id, ty);
                    let result = op.eval_prefix(&oval).unwrap();
                    Ok((ty, result))
                } else {
                    Err(vec![Diag::error()
                        .with_message("Type mismatch")
                        .with_labels(vec![
                            Label::primary(self.ctx.source_id, node.span.start..node.span.end)
                                .with_message(format!(
                                    "cannot do `{op}` prefix operation on types `{}`",
                                    self.ty_pool.get_type(&oty)
                                        .unwrap().format(&self.ty_pool),
                                ))
                        ])])
                }
            },
            NodeKind::Tuple(items) => {
                let mut errors = Vec::new();
                let mut item_tys = Vec::new();
                let mut item_vals = Vec::new();
                for i in items {
                    match self.check_node_const(i) {
                        Ok((ty, val)) => {
                            item_tys.push(ty);
                            item_vals.push(val);
                        },
                        Err(errs) => errors.extend(errs),
                    }
                }
                if !errors.is_empty() { return Err(errors) }
                Ok((
                    self.ty_pool.create_type(Type::Tuple(item_tys)),
                    ConstValue::Tuple(item_vals)
                ))
            },
            NodeKind::Callable { params, ret_ty, body } => {
                let mut errors = Vec::new();
                let mut param_tys = Vec::new();
                for p in params {
                    match self.resolve_type(&p.ty) {
                        Ok(ty) => param_tys.push(ty),
                        Err(errs) => errors.extend(errs),
                    }
                }
                let ret_t = match ret_ty.as_ref()
                    .map(|pty| self.resolve_type(pty))
                    .unwrap_or(Ok(self.ty_pool.nil_id))
                {
                    Ok(t) => t,
                    Err(errs) => {
                        errors.extend(errs);
                        self.ty_pool.nil_id
                    }
                };
                if !errors.is_empty() { return Err(errors) }
                let value = ConstValue::Callable {
                    params: params.iter().zip(param_tys.iter())
                        .map(|(p, ty)| (p.name, *ty))
                        .collect(),
                    ret_ty: ret_t,
                    body: body.clone()
                };
                let ty_id = self.ty_pool.create_type(Type::Callable {
                    params: param_tys,
                    ret_ty: ret_t
                });
                Ok((ty_id, value))
            },
            _ => Err(vec![Diag::error()
                .with_message("Unexpected non-constant expression")
                .with_labels(vec![
                    Label::primary(self.ctx.source_id, node.span.start..node.span.end)
                        .with_message("expected constant expression")
                ])])
        }
    }
}
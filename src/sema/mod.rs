// For all who enter
// this ill-begotten realm,
// 
// thou shalt not pass
// and thou shalt not return.
// 
// Thou shalt be stuffed in this purgatory
// 
// 
// forever
// in eternity.

pub mod symbol;
pub mod ty;

use std::collections::HashMap;
use symbol::*;
use ty::*;
use colored::Colorize;
use crate::parser::ast::{Ast, Node, NodeId, NodeKind, ParsedType, ParsedTypeKind};
use crate::common::{ContextMut, Diag, Label, Span};

const CANDIDATE_SCORE_THRESHOLD: f64 = 0.7;

// the Vec<Vec<SymbolMap>> was created so that child functions cannot access the variables of the parent function,
// only constants from the root-level scope
// 
// each Vec<SymbolMap> represents a function's scope stack
pub struct SemaChecker<'sch> {
    ctx: &'sch ContextMut<'sch>,
    scope: Vec<Vec<SymbolMap>>,
    ty_pool: TypePool,
    type_map: HashMap<NodeId, TypeId>,
    type_registry: HashMap<lasso::Spur, TypeId>,
    constants: HashMap<NodeId, ConstId>,
    next_constant_id: usize,
}

impl<'sch> SemaChecker<'sch> {
    pub fn new(ctx: &'sch mut ContextMut<'sch>) -> Self {
        let ty_pool = TypePool::new();
        let mut type_registry = HashMap::new();
        
        type_registry.insert(ctx.rodeo.get_or_intern("int"), ty_pool.predef_types.int_id);
        type_registry.insert(ctx.rodeo.get_or_intern("uint"), ty_pool.predef_types.uint_id);
        type_registry.insert(ctx.rodeo.get_or_intern("i8"), ty_pool.predef_types.i8_id);
        type_registry.insert(ctx.rodeo.get_or_intern("i16"), ty_pool.predef_types.i16_id);
        type_registry.insert(ctx.rodeo.get_or_intern("i32"), ty_pool.predef_types.i32_id);
        type_registry.insert(ctx.rodeo.get_or_intern("i64"), ty_pool.predef_types.i64_id);
        type_registry.insert(ctx.rodeo.get_or_intern("u8"), ty_pool.predef_types.u8_id);
        type_registry.insert(ctx.rodeo.get_or_intern("u16"), ty_pool.predef_types.u16_id);
        type_registry.insert(ctx.rodeo.get_or_intern("u32"), ty_pool.predef_types.u32_id);
        type_registry.insert(ctx.rodeo.get_or_intern("u64"), ty_pool.predef_types.u64_id);
        type_registry.insert(ctx.rodeo.get_or_intern("float"), ty_pool.predef_types.float_id);
        type_registry.insert(ctx.rodeo.get_or_intern("f32"), ty_pool.predef_types.f32_id);
        type_registry.insert(ctx.rodeo.get_or_intern("f64"), ty_pool.predef_types.f64_id);
        type_registry.insert(ctx.rodeo.get_or_intern("nil"), ty_pool.predef_types.nil_id);
        
        Self {
            ctx,
            scope: vec![vec![SymbolMap::new()]],
            ty_pool,
            type_map: HashMap::new(),
            type_registry,
            constants: HashMap::new(),
            next_constant_id: 0usize,
        }
    }

    pub fn check(&mut self, ast: &Ast) -> Result<(), Vec<Diag>> {
        let mut errors = Vec::new();
        for item in ast.0.iter() {
            match &item.kind {
                NodeKind::ShortConstDecl { .. }
                | NodeKind::ConstDecl { .. } => {
                    if let Err(errs) = self.collect_constant(item) {
                        errors.extend(errs);
                    }
                },
                _ => errors.push(Diag::error()
                    .with_message("Expected root-level item")
                    .with_labels(vec![
                        Label::primary(item.span.source_id, item.span.start..item.span.end)
                            .with_message("invalid root-level item")
                    ]))
            }
        }
        if !errors.is_empty() { return Err(errors) }
        for item in ast.0.iter() {
            if let Err(errs) = self.check_function(item) {
                errors.extend(errs);
            }
        }
        if !errors.is_empty() { return Err(errors) }
        Ok(())
    }

    fn find_ident(&self, i: &lasso::Spur, span: Span) -> Result<(&TypeId, Option<&ConstValue>, &Span), Diag> {
        let candidate = {
            let mut candidate = None;
            for scope in self.scope.last().unwrap().iter().rev() {
                match scope.find_symbol(i, &self.ctx.as_ctx()) {
                    Ok(v) => {
                        return Ok(v);
                    },
                    Err(Some((c, score))) => if candidate
                        .map(|x: (_, f64)| score > x.1)
                        .unwrap_or(true)
                    {
                        candidate = Some((c, score));
                    },
                    Err(None) => continue,
                }
            }
            candidate
        };
        match (self.scope.first().unwrap().first().unwrap().find_symbol(i, &self.ctx.as_ctx()), candidate) {
            (Ok(v), _) => Ok(v),
            (Err(Some((candidate, a_score))), Some((c, b_score))) => {
                // everyday i pray for the ability to name better variables
                let ri = self.ctx.rodeo.resolve(i);
                let a = self.ctx.rodeo.resolve(candidate);
                let b = self.ctx.rodeo.resolve(c);
                if a_score > b_score {
                    Err(Diag::error()
                        .with_message("Unknown identifier")
                        .with_labels(vec![
                            Label::primary(span.source_id, span.start..span.end)
                                .with_message(format!("`{ri}` not in scope"))
                        ]).with_notes(vec![
                            format!(
                                "{}: Did you mean `{a}`?",
                                "note".blue().bold().underline()
                            )
                        ]))
                } else {
                    Err(Diag::error()
                        .with_message("Unknown identifier")
                        .with_labels(vec![
                            Label::primary(span.source_id, span.start..span.end)
                                .with_message(format!("`{ri}` not in scope"))
                        ]).with_notes(vec![
                            format!(
                                "{}: Did you mean `{b}`?",
                                "note".blue().bold().underline()
                            )
                        ]))
                }
            },
            (Err(Some((candidate, _))), None)
            | (Err(None), Some((candidate, _))) => {
                Err(Diag::error()
                    .with_message("Unknown identifier")
                    .with_labels(vec![
                        Label::primary(span.source_id, span.start..span.end)
                            .with_message(format!("`{}` not in scope", self.ctx.rodeo.resolve(i)))
                    ]).with_notes(vec![
                        format!(
                            "{}: Did you mean `{}`?",
                            "note".blue().bold().underline(),
                            self.ctx.rodeo.resolve(candidate),
                        )
                    ]))
            },
            (Err(None), None) => Err(Diag::error()
                .with_message("Unknown identifier")
                .with_labels(vec![
                    Label::primary(span.source_id, span.start..span.end)
                        .with_message(format!("`{}` not in scope", self.ctx.rodeo.resolve(i)))
                ])),
        }
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
                            Label::primary(p_ty.span.source_id, p_ty.span.start..p_ty.span.end)
                                .with_message(format!("`{}` not in scope", self.ctx.rodeo.resolve(i)))
                        ]).with_notes(vec![
                            format!(
                                "{}: Did you mean `{}`?",
                                "note".blue().bold().underline(),
                                self.ctx.rodeo.resolve(c)
                            )
                        ])])
                } else {
                    Err(vec![Diag::error()
                        .with_message("Unknown type")
                        .with_labels(vec![
                            Label::primary(p_ty.span.source_id, p_ty.span.start..p_ty.span.end)
                                .with_message(format!("`{}` not in scope", self.ctx.rodeo.resolve(i)))
                        ])])
                }
            },
            ParsedTypeKind::Nil => Ok(self.ty_pool.predef_types.nil_id),
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

    fn collect_constant(&mut self, item: &Node) -> Result<(), Vec<Diag>> {
        match &item.kind {
            NodeKind::ShortConstDecl { name, expr } => {
                let mut errors = Vec::new();
                if let Ok((_, _, defined_at)) = self.scope.first().unwrap()
                    .first().unwrap()
                    .find_symbol(name, &self.ctx.as_ctx())
                {
                    errors.push(Diag::error()
                        .with_message("Already defined constant")
                        .with_labels(vec![
                            Label::primary(item.span.source_id, item.span.start..item.span.end)
                                .with_message(format!(
                                    "`{}` redefined here",
                                    self.ctx.rodeo.resolve(name)
                                )),
                            Label::secondary(defined_at.source_id, defined_at.start..defined_at.end)
                                .with_message(format!(
                                    "`{}` was defined here",
                                    self.ctx.rodeo.resolve(name)
                                )),
                        ]).with_notes(vec![
                            format!(
                                "{}: Constants cannot be redefined in the same scope",
                                "note".blue().bold().underline()
                            )
                        ]));
                }
                match self.check_node_const(expr) {
                    Ok((init_ty_id, init_val)) => {
                        if !errors.is_empty() {
                            return Err(errors);
                        }
                        if !errors.is_empty() { return Err(errors) }
                        self.scope.last_mut().unwrap()
                            .last_mut().unwrap()
                            .define_constant(*name, init_ty_id, init_val, item.span);
                        let const_id = ConstId(self.next_constant_id);
                        self.next_constant_id += 1;
                        self.constants.insert(item.id, const_id);
                        Ok(())
                    },
                    Err(errs) => {
                        errors.extend(errs);
                        Err(errors)
                    }
                }
            },
            NodeKind::ConstDecl { name, ty, expr } => {
                let mut errors = Vec::new();
                if let Ok((_, _, defined_at)) = self.scope.first().unwrap()
                    .first().unwrap()
                    .find_symbol(name, &self.ctx.as_ctx())
                {
                    errors.push(Diag::error()
                        .with_message("Already defined constant")
                        .with_labels(vec![
                            Label::primary(item.span.source_id, item.span.start..item.span.end)
                                .with_message(format!(
                                    "`{}` redefined here",
                                    self.ctx.rodeo.resolve(name)
                                )),
                            Label::secondary(defined_at.source_id, defined_at.start..defined_at.end)
                                .with_message(format!(
                                    "`{}` was defined here",
                                    self.ctx.rodeo.resolve(name)
                                )),
                        ]).with_notes(vec![
                            format!(
                                "{}: Constants cannot be redefined in the same scope",
                                "note".blue().bold().underline()
                            )
                        ]));
                }
                let resolved_ty = match self.resolve_type(ty) {
                    Ok(o) => o,
                    Err(err) => {
                        errors.extend(err);
                        self.ty_pool.predef_types.nil_id
                    },
                };
                let (init_ty_id, init_val) = match self.check_node_const(expr) {
                    Ok(o) => o,
                    Err(err) => {
                        errors.extend(err);
                        (self.ty_pool.predef_types.nil_id, ConstValue::Nil)
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
                            Label::primary(item.span.source_id, item.span.start..item.span.end)
                                .with_message(format!(
                                    "expected `{}`, found `{}`",
                                    self.ty_pool.get_type(&resolved_ty)
                                        .unwrap().format(&self.ty_pool),
                                    self.ty_pool.get_type(&init_ty_id)
                                        .unwrap().format(&self.ty_pool)
                                ))
                        ])]);
                }

                self.scope.last_mut().unwrap()
                    .last_mut().unwrap()
                    .define_constant(*name, resolved_ty, init_val, item.span);
                let const_id = ConstId(self.next_constant_id);
                self.next_constant_id += 1;
                self.constants.insert(item.id, const_id);
                Ok(())
            },
            _ => Ok(())
        }
    }

    fn check_function(&mut self, item: &Node) -> Result<(), Vec<Diag>> {
        match &item.kind {
            NodeKind::ShortConstDecl { name, expr }
            | NodeKind::ConstDecl { name, expr, .. } => {
                if let NodeKind::Callable { params, sig_span, body: (body, body_span), .. } = &expr.kind {
                    let mut errors = Vec::new();
                    if let Ok(Type::Callable { params: param_tys, .. })
                        = self.scope.last().unwrap().last().unwrap().find_symbol(name, &self.ctx.as_ctx())
                            .map(|x| self.ty_pool.get_type(x.0).unwrap())
                    {
                        let mut smap = SymbolMap::new();
                        for (p, ty) in params.iter().zip(param_tys) {
                            smap.define_symbol(p.name, *ty, p.span);
                        }
                        self.scope.push(vec![smap]);
                    }
                    for stmt in body {
                        if let Err(errs) = self.collect_constant(stmt) {
                            errors.extend(errs);
                        }
                    }
                    if !errors.is_empty() { return Err(errors) }
                    for stmt in body {
                        if let Err(errs) = self.check_function(stmt) {
                            errors.extend(errs);
                        }
                    }
                    if !errors.is_empty() { return Err(errors) }
                    let mut last_ty = self.ty_pool.predef_types.nil_id;
                    for stmt in body {
                        if let Err(errs) = self.check_node(stmt) {
                            errors.extend(errs);
                        } else {
                            last_ty = self.type_map[&stmt.id];
                        }
                    }
                    if !errors.is_empty() { return Err(errors) }
                    self.scope.pop();
                    if let Ok(Type::Callable { ret_ty, .. })
                        = self.scope.last().unwrap().last().unwrap().find_symbol(name, &self.ctx.as_ctx())
                            .map(|x| self.ty_pool.get_type(x.0).unwrap())
                    {
                        let body_ty = self.ty_pool.get_type(&last_ty).unwrap();
                        let ret_ty = self.ty_pool.get_type(ret_ty).unwrap();
                        if body_ty.is_coerceable_into(ret_ty) {
                            self.ty_pool.coerce_type(&last_ty, ret_ty.clone());
                        } else {
                            return Err(vec![Diag::error()
                                .with_message("Type mismatch")
                                .with_labels(vec![
                                    Label::primary(
                                        body_span.source_id,
                                        body_span.start..body_span.end
                                    ).with_message(format!(
                                        "expected `{}`, found `{}`",
                                        ret_ty.format(&self.ty_pool),
                                        body_ty.format(&self.ty_pool)
                                    )),
                                    Label::secondary(
                                        sig_span.source_id,
                                        sig_span.start..sig_span.end
                                    ).with_message("signature was defined here")
                                ])
                            ]);
                        }
                    }
                }
                Ok(())
            },
            _ => Ok(())
        }
    }

    fn check_node(&mut self, node: &Node) -> Result<(), Vec<Diag>> {
        match &node.kind {
            NodeKind::IntLit(_) => {
                let t_id = self.ty_pool.create_type(Type::AmbiguousInt);
                self.type_map.insert(node.id, t_id);
                Ok(())
            },
            NodeKind::FloatLit(_) => {
                let t_id = self.ty_pool.create_type(Type::AmbiguousFloat);
                self.type_map.insert(node.id, t_id);
                Ok(())
            },
            NodeKind::StringLit(_) => todo!("strings"),
            NodeKind::Identifier(i) => {
                self.type_map.insert(
                    node.id,
                    *self.find_ident(i, node.span).map_err(|err| vec![err])?.0
                );
                Ok(())
            },
            NodeKind::Nil => {
                self.type_map.insert(node.id, self.ty_pool.predef_types.nil_id);
                Ok(())
            },
            NodeKind::BinaryOp { op: (op, op_span), lhs, rhs } => {
                let mut errors = Vec::new();
                if let Err(errs) = self.check_node(lhs) {
                    errors.extend(errs);
                }
                if let Err(errs) = self.check_node(rhs) {
                    errors.extend(errs);
                }
                if !errors.is_empty() { return Err(errors) }
                let lty = &self.type_map[&lhs.id];
                let rty = &self.type_map[&rhs.id];
                if let Some(ty) = op.infix_output_ty(&lty, &rty, &mut self.ty_pool) {
                    self.type_map.insert(node.id, ty);
                    Ok(())
                } else {
                    let lty_fmt = self.ty_pool.get_type(&lty)
                        .unwrap().format(&self.ty_pool);
                    let rty_fmt = self.ty_pool.get_type(&rty)
                        .unwrap().format(&self.ty_pool);
                    Err(vec![Diag::error()
                        .with_message("Type mismatch")
                        .with_labels(vec![
                            Label::primary(op_span.source_id, op_span.start..op_span.end)
                                .with_message(format!("cannot do `{op}` infix operation on types `{lty_fmt}` and `{rty_fmt}`")),
                            Label::secondary(lhs.span.source_id, lhs.span.start..lhs.span.end)
                                .with_message(format!("this has type `{lty_fmt}`")),
                            Label::secondary(rhs.span.source_id, rhs.span.start..rhs.span.end)
                                .with_message(format!("this has type `{rty_fmt}`")),
                        ])])
                }
            },
            NodeKind::UnaryOp { op, operand } => {
                self.check_node(operand)?;
                let oty = &self.type_map[&operand.id];
                if let Some(ty) = op.prefix_output_ty(&oty, &self.ty_pool) {
                    self.type_map.insert(node.id, ty);
                    Ok(())
                } else {
                    let oty_fmt = self.ty_pool.get_type(&oty)
                        .unwrap().format(&self.ty_pool);
                    Err(vec![Diag::error()
                        .with_message("Type mismatch")
                        .with_labels(vec![
                            Label::primary(node.span.source_id, node.span.start..node.span.end)
                                .with_message(format!("cannot do `{op}` prefix operation on types `{oty_fmt}`")),
                            Label::secondary(operand.span.source_id, operand.span.start..operand.span.end)
                                .with_message(format!("this has type `{oty_fmt}`")),
                        ])])
                }
            },
            NodeKind::Tuple(items) => {
                let mut errors = Vec::new();
                let mut item_tys = Vec::new();
                for i in items {
                    match self.check_node(i) {
                        Ok(()) => item_tys.push(self.type_map[&i.id]),
                        Err(errs) => errors.extend(errs),
                    }
                }
                if !errors.is_empty() { return Err(errors) }
                self.type_map.insert(node.id, self.ty_pool.create_type(Type::Tuple(item_tys)));
                Ok(())
            },
            NodeKind::Block(items) => {
                let mut errors = Vec::new();
                let mut last_ty = self.ty_pool.predef_types.nil_id;
                self.scope.last_mut().unwrap().push(SymbolMap::new());
                for i in items {
                    match self.check_node(i) {
                        Ok(()) => last_ty = self.type_map[&i.id],
                        Err(errs) => errors.extend(errs),
                    }
                }
                self.scope.last_mut().unwrap().pop();
                if !errors.is_empty() { return Err(errors) }
                self.type_map.insert(node.id, last_ty);
                Ok(())
            },
            NodeKind::Callable { .. } => Ok(()),
            NodeKind::Call { callee, args } => {
                let mut errors = Vec::new();
                if let Err(errs) = self.check_node(callee) {
                    errors.extend(errs);
                }
                let mut arg_tys = Vec::new();
                for arg in args {
                    match self.check_node(arg) {
                        Ok(()) => arg_tys.push(self.type_map.get(&arg.id).copied().unwrap()),
                        Err(errs) => errors.extend(errs),
                    }
                }
                if !errors.is_empty() { return Err(errors) }
                let callee_ty = self.ty_pool.get_type(&self.type_map[&callee.id]).unwrap();
                let mut coerce = HashMap::new();
                if let Type::Callable { params, ret_ty } = callee_ty {
                    let a_len = args.len();
                    let p_len = params.len();
                    if a_len != p_len {
                        return Err(vec![Diag::error()
                            .with_message("Invalid argument count")
                            .with_labels(vec![
                                Label::primary(node.span.source_id, node.span.start..node.span.end)
                                    .with_message(format!("expected {p_len} arguments, found {a_len}"))
                            ])]);
                    }

                    for (idx, (arg, param)) in arg_tys.iter().zip(params).enumerate() {
                        let aty = self.ty_pool.get_type(arg).unwrap();
                        let pty = self.ty_pool.get_type(param).unwrap();
                        if aty.is_coerceable_into(pty) {
                            coerce.insert(arg, pty.clone());
                        } else {
                            errors.push(Diag::error()
                                .with_message("Type mismatch")
                                .with_labels(vec![
                                    Label::primary(
                                        args[idx].span.source_id,
                                        args[idx].span.start..args[idx].span.end
                                    ).with_message(format!(
                                        "parameter #{} expected `{}`, found `{}`",
                                        idx + 1,
                                        pty.format(&self.ty_pool),
                                        aty.format(&self.ty_pool)
                                    ))
                                ]));
                        }
                    }
                    if !errors.is_empty() { return Err(errors) }
                    self.type_map.insert(node.id, *ret_ty);
                } else {
                    return Err(vec![Diag::error()
                        .with_message("Calling a non-callable")
                        .with_labels(vec![
                            Label::primary(callee.span.source_id, callee.span.start..callee.span.end)
                                .with_message(format!(
                                    "type `{}` is not callable",
                                    callee_ty.format(&self.ty_pool)
                                ))
                        ])]);
                }
                for (from, to) in coerce {
                    self.ty_pool.coerce_type(from, to);
                }
                Ok(())
            },
            NodeKind::ShortVarDecl { name, expr } => {
                self.check_node(expr)?;
                let ty = self.type_map[&expr.id];
                self.scope.last_mut().unwrap().last_mut().unwrap()
                    .define_symbol(*name, ty, node.span);
                self.type_map.insert(node.id, ty);
                Ok(())
            },
            NodeKind::TypedVarDecl { name, ty, expr } => {
                let mut errors = Vec::new();
                let resolved_ty = match self.resolve_type(ty) {
                    Ok(ty) => ty,
                    Err(errs) => {
                        errors.extend(errs);
                        self.ty_pool.predef_types.nil_id
                    },
                };
                if let Err(errs) = self.check_node(expr) {
                    errors.extend(errs);
                }
                if !errors.is_empty() { return Err(errors) }
                let init_ty = self.type_map[&expr.id];

                let resolved_ty_data = self.ty_pool.get_type(&resolved_ty).unwrap();
                let init_ty_data = self.ty_pool.get_type(&init_ty).unwrap();
                if init_ty_data.is_coerceable_into(resolved_ty_data) {
                    self.ty_pool.coerce_type(&init_ty, resolved_ty_data.clone());
                } else {
                    return Err(vec![Diag::error()
                        .with_message("Type mismatch")
                        .with_labels(vec![
                            Label::primary(expr.span.source_id, expr.span.start..expr.span.end)
                                .with_message(format!(
                                    "expected `{}`, found `{}`",
                                    resolved_ty_data.format(&self.ty_pool),
                                    init_ty_data.format(&self.ty_pool)
                                ))
                        ])]);
                }
                
                self.scope.last_mut().unwrap().last_mut().unwrap()
                    .define_symbol(*name, init_ty, node.span);
                self.type_map.insert(node.id, init_ty);
                Ok(())
            },
            NodeKind::Mutation { name, op: (op, op_span), expr } => {
                let mut errors = Vec::new();
                if let Err(errs) = self.check_node(expr) {
                    errors.extend(errs);
                }
                match self.find_ident(name, node.span) {
                    Ok((ty_id, val, defined_at)) => {
                        let ty_id = *ty_id;
                        let defined_at = *defined_at;
                        if val.is_some() {
                            let rname = self.ctx.rodeo.resolve(name);
                            errors.push(
                                Diag::error()
                                    .with_message("Mutation of constant")
                                    .with_labels(vec![
                                        Label::primary(node.span.source_id, node.span.start..node.span.end)
                                            .with_message(format!(
                                                "cannot mutate constant `{}`",
                                                rname,
                                            )),
                                        Label::secondary(defined_at.source_id, defined_at.start..defined_at.end)
                                            .with_message(format!(
                                                "`{}` defined here",
                                                rname,
                                            )),
                                    ]).with_notes(vec![format!(
                                        "{}: Constants cannot be mutated",
                                        "note".blue().bold().underline(),
                                    )])
                            );
                        }
                        let expr_ty_id = self.type_map[&expr.id];
                        if op.validate_reassignment(&ty_id, &expr_ty_id, &mut self.ty_pool) {
                            self.type_map.insert(node.id, ty_id);
                            return Ok(());
                        } else {
                            errors.push(
                                Diag::error()
                                    .with_message("Type mismatch")
                                    .with_labels(vec![
                                        Label::primary(op_span.source_id, op_span.start..op_span.end)
                                            .with_message(format!(
                                                "cannot apply `{}` reassignment to type `{}` by type `{}`",
                                                op,
                                                self.ty_pool.get_type(&ty_id).unwrap().format(&self.ty_pool),
                                                self.ty_pool.get_type(&expr_ty_id).unwrap().format(&self.ty_pool),
                                            )),
                                        Label::secondary(expr.span.source_id, expr.span.start..expr.span.end)
                                            .with_message(format!(
                                                "this has type `{}`",
                                                self.ty_pool.get_type(&expr_ty_id).unwrap().format(&self.ty_pool),
                                            )),
                                        Label::secondary(defined_at.source_id, defined_at.start..defined_at.end)
                                            .with_message(format!(
                                                "`{}` defined here",
                                                self.ctx.rodeo.resolve(name),
                                            )),
                                    ])
                            );
                        }
                    },
                    Err(err) => errors.push(err)
                }
                
                Err(errors)
            },
            NodeKind::ShortConstDecl { .. }
            | NodeKind::ConstDecl { .. } => Ok(()),
            NodeKind::If { cond, then, else_ } => {
                let mut errors = Vec::new();
                if let Err(errs) = self.check_node(cond) {
                    errors.extend(errs);
                } else {
                    let cond_ty = self.ty_pool.get_type(&self.type_map[&cond.id]).unwrap();
                    if *cond_ty != Type::Bool {
                        errors.push(
                            Diag::error()
                                .with_message("Type mismatch")
                                .with_labels(vec![
                                    Label::primary(cond.span.source_id, cond.span.start..cond.span.end)
                                        .with_message(format!(
                                            "expected `bool`, found `{}`",
                                            cond_ty.format(&self.ty_pool)
                                        ))
                                ]).with_notes(vec![format!(
                                    "{}: `if` condition must be of type `bool`",
                                    "note".blue().bold().underline(),
                                )])
                        );
                    }
                }
                self.scope.last_mut().unwrap().push(SymbolMap::new());
                if let Err(errs) = self.check_node(then) {
                    errors.extend(errs);
                }
                self.scope.last_mut().unwrap()
                    .last_mut().unwrap()
                    .clear();
                if let Some(expr) = else_ {
                    if let Err(errs) = self.check_node(expr) {
                        errors.extend(errs);
                    }
                }
                self.scope.last_mut().unwrap().pop();
                if !errors.is_empty() {
                    return Err(errors);
                }
                
                let then_ty = self.ty_pool.get_type(&self.type_map[&then.id]).unwrap();
                if let Some(expr) = &else_ {
                    let else_ty = self.ty_pool.get_type(&self.type_map[&expr.id]).unwrap();
                    if then_ty.is_coerceable_into(else_ty) {
                        self.ty_pool.coerce_type(&self.type_map[&then.id], else_ty.clone());
                    } else if else_ty.is_coerceable_into(then_ty) {
                        self.ty_pool.coerce_type(&self.type_map[&expr.id], then_ty.clone());
                    } else {
                        errors.push(
                            Diag::error()
                                .with_message("Type mismatch")
                                .with_labels(vec![
                                    Label::primary(expr.span.source_id, expr.span.start..expr.span.end)
                                        .with_message(format!(
                                            "expected `{}`, found `{}`",
                                            then_ty.format(&self.ty_pool),
                                            else_ty.format(&self.ty_pool)
                                        )),
                                    Label::secondary(then.span.source_id, then.span.start..then.span.end)
                                        .with_message(format!(
                                            "this has type `{}`",
                                            then_ty.format(&self.ty_pool)
                                        ))
                                ]).with_notes(vec![format!(
                                    "{}: Clauses must return the same type or one clause returning `nil`",
                                    "note".blue().bold().underline(),
                                )])
                        );
                    }
                } else if *then_ty != Type::Nil {
                    errors.push(
                        Diag::error()
                            .with_message("Missing clause")
                            .with_labels(vec![
                                Label::primary(node.span.source_id, node.span.start..node.span.end)
                                    .with_message(format!(
                                        "expected `else` clause of type `{}`",
                                        then_ty.format(&self.ty_pool)
                                    ))
                            ]).with_notes(vec![format!(
                                "{}: Clauses must return the same type or one clause returning `nil`",
                                "note".blue().bold().underline(),
                            )])
                    );
                }
                if !errors.is_empty() { return Err(errors) }
                self.type_map.insert(node.id, self.type_map[&then.id]);
                Ok(())
            },
            NodeKind::While { cond, body } => {
                let mut errors = Vec::new();
                if let Err(errs) = self.check_node(cond) {
                    errors.extend(errs);
                } else {
                    let cond_ty = self.ty_pool.get_type(&self.type_map[&cond.id]).unwrap();
                    if *cond_ty != Type::Bool {
                        errors.push(
                            Diag::error()
                                .with_message("Type mismatch")
                                .with_labels(vec![
                                    Label::primary(cond.span.source_id, cond.span.start..cond.span.end)
                                        .with_message(format!(
                                            "expected `bool`, found `{}`",
                                            cond_ty.format(&self.ty_pool)
                                        ))
                                ]).with_notes(vec![format!(
                                    "{}: `while` condition must be of type `bool`",
                                    "note".blue().bold().underline(),
                                )])
                        );
                    }
                }
                self.scope.last_mut().unwrap().push(SymbolMap::new());
                if let Err(errs) = self.check_node(body) {
                    errors.extend(errs);
                }
                self.scope.last_mut().unwrap().pop();
                if !errors.is_empty() { return Err(errors) }
                self.type_map.insert(node.id, self.ty_pool.predef_types.nil_id);
                Ok(())
            },
        }
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
                let (ty, val, defined_at) = self.find_ident(i, node.span).map_err(|err| vec![err])?;
                if let Some(val) = val.cloned() {
                    let ty = *ty;
                    self.type_map.insert(node.id, ty);
                    Ok((ty, val))
                } else {
                    let ri = self.ctx.rodeo.resolve(i);
                    Err(vec![Diag::error()
                        .with_message("Unexpected non-constant expression")
                        .with_labels(vec![
                            Label::primary(node.span.source_id, node.span.start..node.span.end)
                                .with_message("expected constant expression"),
                            Label::secondary(defined_at.source_id, defined_at.start..defined_at.end)
                                .with_message(format!("`{}` defined here", ri)),
                        ]).with_notes(vec![format!(
                            "{}: Identifier `{}` is non-constant",
                            "note".blue().bold().underline(),
                            ri,
                        )])])
                }
            },
            NodeKind::Nil => {
                let t_id = self.ty_pool.predef_types.nil_id;
                self.type_map.insert(node.id, t_id);
                Ok((t_id, ConstValue::Nil))
            },
            NodeKind::BinaryOp { op: (op, op_span), lhs, rhs } => {
                let mut errors = Vec::new();
                let (lty, lval) = match self.check_node_const(lhs) {
                    Ok(o) => o,
                    Err(errs) => {
                        errors.extend(errs);
                        (self.ty_pool.predef_types.nil_id, ConstValue::Nil)
                    },
                };
                let (rty, rval) = match self.check_node_const(rhs) {
                    Ok(o) => o,
                    Err(errs) => {
                        errors.extend(errs);
                        (self.ty_pool.predef_types.nil_id, ConstValue::Nil)
                    },
                };
                if !errors.is_empty() { return Err(errors); }
                if let Some(ty) = op.infix_output_ty(&lty, &rty, &mut self.ty_pool) {
                    self.type_map.insert(node.id, ty);
                    let result = op.eval_infix(&lval, &rval).unwrap();
                    Ok((ty, result))
                } else {
                    let lty_fmt = self.ty_pool.get_type(&lty)
                        .unwrap().format(&self.ty_pool);
                    let rty_fmt = self.ty_pool.get_type(&rty)
                        .unwrap().format(&self.ty_pool);
                    Err(vec![Diag::error()
                        .with_message("Type mismatch")
                        .with_labels(vec![
                            Label::primary(op_span.source_id, op_span.start..op_span.end)
                                .with_message(format!("cannot do `{op}` infix operation on types `{lty_fmt}` and `{rty_fmt}`")),
                            Label::secondary(lhs.span.source_id, lhs.span.start..lhs.span.end)
                                .with_message(format!("this has type `{lty_fmt}`")),
                            Label::secondary(rhs.span.source_id, rhs.span.start..rhs.span.end)
                                .with_message(format!("this has type `{rty_fmt}`")),
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
                    let oty_fmt = self.ty_pool.get_type(&oty)
                        .unwrap().format(&self.ty_pool);
                    Err(vec![Diag::error()
                        .with_message("Type mismatch")
                        .with_labels(vec![
                            Label::primary(node.span.source_id, node.span.start..node.span.end)
                                .with_message(format!("cannot do `{op}` prefix operation on types `{oty_fmt}`")),
                            Label::secondary(operand.span.source_id, operand.span.start..operand.span.end)
                                .with_message(format!("this has type`{oty_fmt}`")),
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
                let ty_id = self.ty_pool.create_type(Type::Tuple(item_tys));
                self.type_map.insert(node.id, ty_id);
                Ok((
                    ty_id,
                    ConstValue::Tuple(item_vals)
                ))
            },
            NodeKind::Callable { params, ret_ty, body, .. } => {
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
                    .unwrap_or(Ok(self.ty_pool.predef_types.nil_id))
                {
                    Ok(t) => t,
                    Err(errs) => {
                        errors.extend(errs);
                        self.ty_pool.predef_types.nil_id
                    }
                };
                if !errors.is_empty() { return Err(errors) }
                let value = ConstValue::Callable {
                    params: params.iter().zip(param_tys.iter())
                        .map(|(p, ty)| (p.name, *ty))
                        .collect(),
                    ret_ty: ret_t,
                    body: body.0.clone()
                };
                let ty_id = self.ty_pool.create_type(Type::Callable {
                    params: param_tys,
                    ret_ty: ret_t
                });
                self.type_map.insert(node.id, ty_id);
                Ok((ty_id, value))
            },
            // todo: check for calls to constant functions
            _ => Err(vec![Diag::error()
                .with_message("Unexpected non-constant expression")
                .with_labels(vec![
                    Label::primary(node.span.source_id, node.span.start..node.span.end)
                        .with_message("expected constant expression")
                ])])
        }
    }
}
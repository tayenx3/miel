use std::collections::HashMap;
use crate::common::{Context, Span};
use crate::parser::ast::Node;

use super::ty::TypeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScopeContext {
    Loop, Normal
}

#[derive(Debug)]
pub struct SymbolMap {
    types: HashMap<lasso::Spur, TypeId>,
    constants: HashMap<lasso::Spur, ConstValue>,
    defined_at: HashMap<lasso::Spur, Span>,
    ctx: ScopeContext,
}

impl SymbolMap {
    pub fn new(ctx: ScopeContext) -> Self {
        Self {
            types: HashMap::new(),
            constants: HashMap::new(),
            defined_at: HashMap::new(),
            ctx
        }
    }

    pub fn is_loop(&self) -> bool {
        matches!(self.ctx, ScopeContext::Loop)
    }

    pub fn clear(&mut self) {
        self.types.clear();
        self.constants.clear();
        self.defined_at.clear();
    }
    
    pub fn define_symbol(&mut self, name: lasso::Spur, ty: TypeId, defined_at: Span) {
        self.types.insert(name, ty);
        self.defined_at.insert(name, defined_at);
    }
    
    pub fn define_constant(&mut self, name: lasso::Spur, ty: TypeId, val: ConstValue, defined_at: Span) {
        self.types.insert(name, ty);
        self.constants.insert(name, val);
        self.defined_at.insert(name, defined_at);
    }
    
    pub fn find_symbol(&self, name: &lasso::Spur, ctx: &Context) -> Result<(&TypeId, Option<&ConstValue>, &Span), Option<(&lasso::Spur, f64)>> {
        let mut ty = None;
        let mut candidate = None;
        let mut last_score = 0.0;
        for (sname, sty) in &self.types {
            if sname == name {
                ty = Some(sty);
                break;
            } else {
                let score = strsim::jaro_winkler(
                    ctx.rodeo.resolve(name),
                    ctx.rodeo.resolve(sname)
                );
                if score > super::CANDIDATE_SCORE_THRESHOLD && score > last_score {
                    last_score = score;
                    candidate = Some((sname, last_score));
                }
            }
        }
        match ty {
            Some(t) => Ok((
                t,
                self.constants.get(name),
                &self.defined_at[name],
            )),
            None => Err(candidate)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstId(pub usize);

#[derive(Debug, Clone)]
#[allow(unused)]
pub enum ConstValue {
    Int(i64),
    Float(f64),
    Bool(bool),
    Nil,
    Tuple(Vec<ConstValue>),
    Callable {
        params: Vec<(lasso::Spur, TypeId)>,
        ret_ty: TypeId,
        body: Vec<Node>
    }
}

use std::collections::HashMap;
use crate::common::{Context, Span};
use crate::parser::ast::Node;

use super::ty::TypeId;

pub struct SymbolMap {
    types: HashMap<lasso::Spur, TypeId>,
}

impl SymbolMap {
    pub fn new() -> Self {
        Self {
            types: HashMap::new()
        }
    }
    
    pub fn define_symbol(&mut self, name: lasso::Spur, ty: TypeId) {
        self.types.insert(name, ty);
    }
}

#[derive(Debug, Clone)]
pub enum ConstValue {
    Int(i64),
    Float(f64),
    Nil,
    Tuple(Vec<ConstValue>),
    Callable {
        params: Vec<(lasso::Spur, TypeId)>,
        ret_ty: TypeId,
        body: Vec<Node>
    }
}

pub struct ConstSymbolMap {
    types: HashMap<lasso::Spur, TypeId>,
    values: HashMap<lasso::Spur, ConstValue>,
    defined_at: HashMap<lasso::Spur, Span>
}

impl ConstSymbolMap {
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            values: HashMap::new(),
            defined_at: HashMap::new(),
        }
    }

    pub fn define_symbol(&mut self, name: lasso::Spur, ty: TypeId, value: ConstValue, defined_at: Span) {
        self.types.insert(name, ty);
        self.values.insert(name, value);
        self.defined_at.insert(name, defined_at);
    }

    pub fn find_symbol(&self, name: &lasso::Spur, ctx: &Context) -> Result<(&TypeId, &ConstValue, &Span), Option<&lasso::Spur>> {
        // this already checks if `name` is in scope or not
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
                    candidate = Some(sname);
                }
            }
        }
        match ty {
            Some(t) => Ok((
                t,
                &self.values[name],
                &self.defined_at[name],
            )),
            None => Err(candidate)
        }
    }
}
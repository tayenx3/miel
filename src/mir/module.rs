use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use crate::common::Context;
use super::callable::*;
use super::inst::MirConstId;
use super::builder::MirFunctionBuilder;

pub struct MirModule<'a> {
    pub ctx: Context<'a>,
    name: Arc<str>,
    callables: HashMap<MirCallableId, MirCallable>,
    next_callable_id: usize,
    constants: HashMap<MirConstId, MirCallable>,
    constant_names: HashMap<lasso::Spur, MirConstId>,
    next_const_id: usize,
}

impl<'a> MirModule<'a> {
    pub fn new<S: Into<Arc<str>>>(name: S, ctx: Context<'a>) -> Self {
        Self {
            ctx, name: name.into(),
            callables: HashMap::new(),
            next_callable_id: 0usize,
            constants: HashMap::new(),
            constant_names: HashMap::new(),
            next_const_id: 0usize,
        }
    }

    pub fn define_callable(&mut self, fb: MirFunctionBuilder) -> MirCallableId {
        let id = MirCallableId(self.next_const_id);
        self.next_const_id += 1;
        self.callables.insert(id, MirCallable {
            sig: fb.sig,
            blocks: fb.blocks.into_values().collect(),
        });
        id
    }

    pub fn define_constant(&mut self, name: lasso::Spur, c: MirCallable) -> Option<MirConstId> {
        let id = MirConstId(self.next_const_id);
        self.next_const_id += 1;
        self.constants.insert(id, c);
        if self.constant_names.contains_key(&name) {
            return None;
        }
        self.constant_names.insert(name, id);
        Some(id)
    }
}

impl<'a> fmt::Debug for MirModule<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(module {}", self.name)?;
        write!(f, "\n    (callables")?;
        for (ca_id, callable) in &self.callables {
            write!(f, "\n        ({:?} {})", ca_id, callable.fmt(self.ctx.rodeo, 2))?;
        }
        write!(f, ")\n    (constants")?;
        for (co_id, cc) in &self.constants {
            write!(
                f,
                "\n        ({} {})",
                self.ctx.rodeo.resolve(
                    self.constant_names.iter()
                        .find(|(_, id)| **id == *co_id)
                        .unwrap().0
                ),
                cc.fmt(self.ctx.rodeo, 2),
            )?;
        }
        write!(f, "))")
    }
}
use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use crate::common::Context;
use super::callable::*;
use super::inst::IrConstId;
use super::builder::IrFunctionBuilder;

pub struct IrModule<'a> {
    pub ctx: Context<'a>,
    name: Arc<str>,
    callables: HashMap<IrCallableId, IrCallable>,
    next_callable_id: usize,
    constants: HashMap<IrConstId, IrCallable>,
    constant_names: HashMap<lasso::Spur, IrConstId>,
    next_const_id: usize,
}

impl<'a> IrModule<'a> {
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

    pub fn define_callable(&mut self, fb: IrFunctionBuilder) -> IrCallableId {
        let id = IrCallableId(self.next_const_id);
        self.next_const_id += 1;
        self.callables.insert(id, IrCallable {
            sig: fb.sig,
            blocks: fb.blocks.into_iter()
                .map(|(_, b)| b)
                .collect(),
        });
        id
    }

    pub fn define_constant(&mut self, name: lasso::Spur, c: IrCallable) -> Option<IrConstId> {
        let id = IrConstId(self.next_const_id);
        self.next_const_id += 1;
        self.constants.insert(id, c);
        if self.constant_names.contains_key(&name) {
            return None;
        }
        self.constant_names.insert(name, id);
        Some(id)
    }
}

impl<'a> fmt::Debug for IrModule<'a> {
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
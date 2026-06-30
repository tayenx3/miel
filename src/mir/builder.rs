use std::collections::HashMap;
use crate::common::Span;
use super::ty::MirType;
use super::inst::*;
use super::block::*;
use super::callable::MirSignature;

pub struct MirFunctionBuilder {
    pub sig: MirSignature,
    pub blocks: HashMap<MirBlockId, MirBlock>,
    pub next_block_id: usize,
    pub next_register_id: usize,
    pub current_block: Option<MirBlockId>,
}

impl MirFunctionBuilder {
    pub fn new(sig: MirSignature) -> Self {
        Self {
            sig,
            blocks: HashMap::new(),
            next_block_id: 0usize,
            next_register_id: 0usize,
            current_block: None,
        }
    }

    pub fn create_block(&mut self, params: Vec<MirType>) -> MirBlockId {
        let id = MirBlockId(self.next_block_id);
        self.next_block_id += 1;
        self.blocks.insert(id, MirBlock {
            id,
            params: params.into_iter().map(|ty| {
                let reg = MirValue(self.next_register_id);
                self.next_register_id += 1;
                (reg, ty)
            }).collect(),
            insts: Vec::new(),
            terminal: None,
        });
        id
    }

    pub fn switch_to_block(&mut self, block: MirBlockId) {
        self.current_block = Some(block);
    }

    pub fn ib(&mut self, span: Span) -> MirInstBuilder<'_> {
        MirInstBuilder { builder: self, span }
    }
}

pub struct MirInstBuilder<'b> {
    builder: &'b mut MirFunctionBuilder,
    span: Span
}

impl<'b> MirInstBuilder<'b> {
    pub fn set(&mut self, ty: MirType, expr: MirExpr) {
        debug_assert!(self.builder.current_block.is_some(), "no block selected");
        if let Some(block) = self.builder.current_block.as_ref() {
            debug_assert!(self.builder.blocks[block].terminal.is_none());
            let dst = MirValue(self.builder.next_register_id);
            self.builder.next_register_id += 1;
            self.builder.blocks.get_mut(block)
                .unwrap().insts
                .push(MirInst {
                    kind: MirInstKind::Set { dst, ty, expr },
                    span: self.span,
                });
        }
    }

    pub fn ret(&mut self, v: MirValue) {
        debug_assert!(self.builder.current_block.is_some(), "no block selected");
        if let Some(block) = self.builder.current_block.as_ref() {
            debug_assert!(self.builder.blocks[block].terminal.is_none());
            self.builder.blocks.get_mut(block).unwrap()
                .terminal = Some(MirTerminal {
                    kind: MirTerminalKind::Ret(v),
                    span: self.span,
                });
        }
    }
}
use std::collections::HashMap;
use crate::common::Span;
use super::ty::IrType;
use super::inst::*;
use super::block::*;
use super::callable::IrSignature;

pub struct IrFunctionBuilder {
    pub sig: IrSignature,
    pub blocks: HashMap<IrBlockId, IrBlock>,
    pub next_block_id: usize,
    pub next_register_id: usize,
    pub current_block: Option<IrBlockId>,
}

impl IrFunctionBuilder {
    pub fn new(sig: IrSignature) -> Self {
        Self {
            sig,
            blocks: HashMap::new(),
            next_block_id: 0usize,
            next_register_id: 0usize,
            current_block: None,
        }
    }

    pub fn create_block(&mut self, params: Vec<IrType>) -> IrBlockId {
        let id = IrBlockId(self.next_block_id);
        self.next_block_id += 1;
        self.blocks.insert(id, IrBlock {
            id,
            params: params.into_iter().map(|ty| {
                let reg = IrValue(self.next_register_id);
                self.next_register_id += 1;
                (reg, ty)
            }).collect(),
            insts: Vec::new(),
            terminal: None,
        });
        id
    }

    pub fn switch_to_block(&mut self, block: IrBlockId) {
        self.current_block = Some(block);
    }

    pub fn ib(&mut self, span: Span) -> IrInstBuilder<'_> {
        IrInstBuilder { builder: self, span }
    }
}

pub struct IrInstBuilder<'b> {
    builder: &'b mut IrFunctionBuilder,
    span: Span
}

impl<'b> IrInstBuilder<'b> {
    pub fn set(&mut self, expr: IrExpr) {
        debug_assert!(self.builder.current_block.is_some(), "no block selected");
        if let Some(block) = self.builder.current_block.as_ref() {
            debug_assert!(self.builder.blocks[block].terminal.is_none());
            let dst = IrValue(self.builder.next_register_id);
            self.builder.next_register_id += 1;
            self.builder.blocks.get_mut(block)
                .unwrap().insts
                .push(IrInst {
                    kind: IrInstKind::Set { dst, expr },
                    span: self.span,
                });
        }
    }

    pub fn ret(&mut self, v: IrValue) {
        debug_assert!(self.builder.current_block.is_some(), "no block selected");
        if let Some(block) = self.builder.current_block.as_ref() {
            debug_assert!(self.builder.blocks[block].terminal.is_none());
            self.builder.blocks.get_mut(block).unwrap()
                .terminal = Some(IrTerminal {
                    kind: IrTerminalKind::Ret(v),
                    span: self.span,
                });
        }
    }
}
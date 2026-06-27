use std::fmt;
use super::ty::IrType;
use super::inst::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrBlockId(pub usize);

impl fmt::Debug for IrBlockId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "b{}", self.0)
    }
}

#[derive(Clone, PartialEq)]
pub struct IrBlock {
    pub id: IrBlockId,
    pub params: Vec<(IrValue, IrType)>,
    pub insts: Vec<IrInst>,
    pub terminal: Option<IrTerminal>,
}

impl IrBlock {
    pub fn fmt(&self, rodeo: &lasso::Rodeo, tabs: usize) -> String {
        let mut f = format!("(block {:?} (", self.id);
        for (idx, (pr, pt)) in self.params.iter().enumerate() {
            if idx > 0 {
                f.push(' ');
            }
            f.push_str(&format!("({pr:?} {pt:?})"));
        }
        f.push_str(")");
        for inst in &self.insts {
            f.push_str(&format!("\n{}{}", "    ".repeat(tabs + 1), inst.fmt(rodeo, tabs + 1)));
        }
        if let Some(t) = self.terminal.as_ref() {
            f.push_str(&format!("\n{}{}", "    ".repeat(tabs + 1), t.fmt()));
        }
        f.push(')');
        f
    }
}
use std::fmt;
use super::ty::IrType;
use super::block::IrBlock;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrCallableId(pub usize);

impl fmt::Debug for IrCallableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ca{}", self.0)
    }
}

#[derive(Clone, PartialEq)]
pub struct IrSignature {
    pub params: Vec<IrType>,
    pub return_: IrType,
}

impl fmt::Debug for IrSignature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(")?;
        for (idx, param) in self.params.iter().enumerate() {
            if idx > 0 {
                write!(f, " ")?;
            }
            write!(f, "{param:?}")?;
        }
        write!(f, ") {:?}", self.return_)
    }
}

#[derive(Clone, PartialEq)]
pub struct IrCallable {
    pub sig: IrSignature,
    pub blocks: Vec<IrBlock>,
}

impl IrCallable {
    pub fn fmt(&self, rodeo: &lasso::Rodeo, tabs: usize) -> String {
        let mut f = format!("(callable {:?}",  self.sig);
        for block in &self.blocks {
            f.push_str(&format!("\n{}{}", "    ".repeat(tabs + 1), block.fmt(rodeo, tabs + 1)));
        }
        f.push(')');
        f
    }
}
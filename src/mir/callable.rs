use std::fmt;
use super::ty::MirType;
use super::block::MirBlock;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MirCallableId(pub usize);

impl fmt::Debug for MirCallableId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ca{}", self.0)
    }
}

#[derive(Clone, PartialEq)]
pub struct MirSignature {
    pub params: Vec<MirType>,
    pub return_: MirType,
}

impl fmt::Debug for MirSignature {
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
pub struct MirCallable {
    pub sig: MirSignature,
    pub blocks: Vec<MirBlock>,
}

impl MirCallable {
    pub fn fmt(&self, rodeo: &lasso::Rodeo, tabs: usize) -> String {
        let mut f = format!("(callable {:?}",  self.sig);
        for block in &self.blocks {
            f.push_str(&format!("\n{}{}", "    ".repeat(tabs + 1), block.fmt(rodeo, tabs + 1)));
        }
        f.push(')');
        f
    }
}
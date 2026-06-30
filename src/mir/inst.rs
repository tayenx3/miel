use std::fmt;
use crate::common::Span;
use super::callable::*;
use super::ty::MirType;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MirConstId(pub usize);

impl fmt::Debug for MirConstId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "co{}", self.0)
    }
}

#[derive(Clone, PartialEq)]
pub enum MirConstValue {
    Int(lasso::Spur),
    Float(lasso::Spur),
    Bool(bool),
    Nil,
    Tuple(Vec<MirConstValue>),
    Callable(MirCallableId),
}

impl MirConstValue {
    pub fn fmt(&self, rodeo: &lasso::Rodeo, tabs: usize) -> String {
        match self {
            Self::Int(i) => format!("(int {})", rodeo.resolve(i)),
            Self::Float(i) => format!("(float {})", rodeo.resolve(i)),
            Self::Bool(i) => format!("(bool {i})"),
            Self::Nil => "nil".to_string(),
            Self::Tuple(items) => {
                let mut output = "(tuple".to_string();
                for (idx, item) in items.iter().enumerate() {
                    if idx > 0 {
                        output.push('\n');
                    }
                    output.push_str(&item.fmt(rodeo, tabs + 1));
                }
                output.push(')');
                output
            },
            Self::Callable(c) => format!("{c:?}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct MirValue(pub usize);

impl fmt::Debug for MirValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

#[derive(Clone, PartialEq)]
pub enum MirExpr {
    Const(MirConstValue),
    IAdd(MirValue, MirValue),
    ISub(MirValue, MirValue),
    IMul(MirValue, MirValue),
    SDiv(MirValue, MirValue),
    UDiv(MirValue, MirValue),
    SRem(MirValue, MirValue),
    URem(MirValue, MirValue),
}

impl MirExpr {
    pub fn fmt(&self, rodeo: &lasso::Rodeo, tabs: usize) -> String {
        match self {
            Self::Const(c) => format!("(const {})", c.fmt(rodeo, tabs + 1)),
            Self::IAdd(l, r) => format!("(iadd {l:?} {r:?})"),
            Self::ISub(l, r) => format!("(isub {l:?} {r:?})"),
            Self::IMul(l, r) => format!("(imul {l:?} {r:?})"),
            Self::SDiv(l, r) => format!("(sdiv {l:?} {r:?})"),
            Self::UDiv(l, r) => format!("(udiv {l:?} {r:?})"),
            Self::SRem(l, r) => format!("(srem {l:?} {r:?})"),
            Self::URem(l, r) => format!("(urem {l:?} {r:?})"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum MirInstKind {
    Set {
        dst: MirValue,
        ty: MirType,
        expr: MirExpr,
    },
}

impl MirInstKind {
    pub fn fmt(&self, rodeo: &lasso::Rodeo, tabs: usize) -> String {
        match self {
            Self::Set { dst, ty, expr } => format!("(= {ty:?} {dst:?} {})", expr.fmt(rodeo, tabs + 1)),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct MirInst {
    pub kind: MirInstKind,
    pub span: Span,
}

impl MirInst {
    pub fn fmt(&self, rodeo: &lasso::Rodeo, tabs: usize) -> String {
        self.kind.fmt(rodeo, tabs + 1)
    }
}

#[derive(Clone, PartialEq)]
pub enum MirTerminalKind {
    Ret(MirValue),
}

impl MirTerminalKind {
    pub fn fmt(&self) -> String {
        match self {
            Self::Ret(v) => format!("(ret {v:?})"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct MirTerminal {
    pub kind: MirTerminalKind,
    pub span: Span,
}

impl MirTerminal {
    pub fn fmt(&self) -> String {
        self.kind.fmt()
    }
}
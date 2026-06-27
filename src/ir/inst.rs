use std::fmt;
use crate::common::Span;
use super::callable::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct IrConstId(pub usize);

impl fmt::Debug for IrConstId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "co{}", self.0)
    }
}

#[derive(Clone, PartialEq)]
pub enum IrConstValue {
    Int(lasso::Spur),
    Float(lasso::Spur),
    Bool(bool),
    Nil,
    Tuple(Vec<IrConstValue>),
    Callable(IrCallableId),
}

impl IrConstValue {
    pub fn fmt(&self, rodeo: &lasso::Rodeo, tabs: usize) -> String {
        match self {
            Self::Int(i) => format!("(int {})", rodeo.resolve(i)),
            Self::Float(i) => format!("(float {})", rodeo.resolve(i)),
            Self::Bool(i) => format!("(bool {i})"),
            Self::Nil => "nil".to_string(),
            Self::Tuple(items) => {
                let mut output = format!("(tuple");
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
pub struct IrValue(pub usize);

impl fmt::Debug for IrValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "v{}", self.0)
    }
}

#[derive(Clone, PartialEq)]
pub enum IrExpr {
    Const(IrConstValue),
    Add(IrValue, IrValue),
    Sub(IrValue, IrValue),
    Mul(IrValue, IrValue),
    Div(IrValue, IrValue),
    Rem(IrValue, IrValue),
}

impl IrExpr {
    pub fn fmt(&self, rodeo: &lasso::Rodeo, tabs: usize) -> String {
        match self {
            Self::Const(c) => format!("(const {})", c.fmt(rodeo, tabs + 1)),
            Self::Add(l, r) => format!("(add {l:?} {r:?})"),
            Self::Sub(l, r) => format!("(sub {l:?} {r:?})"),
            Self::Mul(l, r) => format!("(mul {l:?} {r:?})"),
            Self::Div(l, r) => format!("(div {l:?} {r:?})"),
            Self::Rem(l, r) => format!("(rem {l:?} {r:?})"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum IrInstKind {
    Set {
        dst: IrValue,
        expr: IrExpr,
    },
}

impl IrInstKind {
    pub fn fmt(&self, rodeo: &lasso::Rodeo, tabs: usize) -> String {
        match self {
            Self::Set { dst, expr } => format!("(= {dst:?} {})", expr.fmt(rodeo, tabs + 1)),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct IrInst {
    pub kind: IrInstKind,
    pub span: Span,
}

impl IrInst {
    pub fn fmt(&self, rodeo: &lasso::Rodeo, tabs: usize) -> String {
        format!("{}", self.kind.fmt(rodeo, tabs + 1))
    }
}

#[derive(Clone, PartialEq)]
pub enum IrTerminalKind {
    Ret(IrValue),
}

impl IrTerminalKind {
    pub fn fmt(&self) -> String {
        match self {
            Self::Ret(v) => format!("(ret {v:?})"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct IrTerminal {
    pub kind: IrTerminalKind,
    pub span: Span,
}

impl IrTerminal {
    pub fn fmt(&self) -> String {
        format!("{}", self.kind.fmt())
    }
}
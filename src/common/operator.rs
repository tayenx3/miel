use std::fmt;
use crate::sema::{symbol::ConstValue, ty::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
    Eq, Ne, Gt, Lt, Ge, Le,
    Pipe, Ampersand, Caret, Bang,
    KwOr, KwAnd, KwXor, KwNot,
}

impl Operator {
    #[inline]
    pub const fn can_infix(&self) -> bool {
        !matches!(self, Self::Bang | Self::KwNot)
    }

    #[inline]
    pub const fn can_prefix(&self) -> bool {
        matches!(self, Self::Plus | Self::Minus)
    }

    #[inline]
    pub const fn binding_power(&self) -> usize {
        match self {
            Self::KwOr | Self::KwXor => 10,
            Self::KwAnd => 20,
            Self::Eq | Self::Ne => 30,
            Self::Gt | Self::Lt | Self::Ge | Self::Le => 40,
            Self::Pipe | Self::Caret => 50,
            Self::Ampersand => 60,
            Self::Plus | Self::Minus => 70,
            Self::Star | Self::Slash | Self::Modulo => 80,
            _ => 0, // prefix op
        }
    }

    // comparisons might be inconsistent with true evaluations with floats
    pub fn eval_infix(&self, lhs: &ConstValue, rhs: &ConstValue) -> Option<ConstValue> {
        match self {
            Self::Plus => match (lhs, rhs) {
                (ConstValue::Int(l), ConstValue::Int(r)) => Some(ConstValue::Int(l + r)),
                (ConstValue::Float(l), ConstValue::Float(r)) => Some(ConstValue::Float(l + r)),
                _ => None
            },
            Self::Minus => match (lhs, rhs) {
                (ConstValue::Int(l), ConstValue::Int(r)) => Some(ConstValue::Int(l - r)),
                (ConstValue::Float(l), ConstValue::Float(r)) => Some(ConstValue::Float(l - r)),
                _ => None
            },
            Self::Star => match (lhs, rhs) {
                (ConstValue::Int(l), ConstValue::Int(r)) => Some(ConstValue::Int(l * r)),
                (ConstValue::Float(l), ConstValue::Float(r)) => Some(ConstValue::Float(l * r)),
                _ => None
            },
            Self::Slash => match (lhs, rhs) {
                (ConstValue::Int(l), ConstValue::Int(r)) => Some(ConstValue::Int(l / r)),
                (ConstValue::Float(l), ConstValue::Float(r)) => Some(ConstValue::Float(l / r)),
                _ => None
            },
            Self::Modulo => match (lhs, rhs) {
                (ConstValue::Int(l), ConstValue::Int(r)) => Some(ConstValue::Int(l % r)),
                (ConstValue::Float(l), ConstValue::Float(r)) => Some(ConstValue::Float(l % r)),
                _ => None
            },
            Self::Eq => match (lhs, rhs) {
                (ConstValue::Int(l), ConstValue::Int(r)) => Some(ConstValue::Bool(l == r)),
                (ConstValue::Float(l), ConstValue::Float(r)) => Some(ConstValue::Bool(l == r)),
                (ConstValue::Nil, ConstValue::Nil) => Some(ConstValue::Bool(true)),
                (ConstValue::Tuple(l), ConstValue::Tuple(r)) => compare_tuples_eq(l, r).map(|b| ConstValue::Bool(b)),
                _ => None
            },
            Self::Ne => match (lhs, rhs) {
                (ConstValue::Int(l), ConstValue::Int(r)) => Some(ConstValue::Bool(l == r)),
                (ConstValue::Float(l), ConstValue::Float(r)) => Some(ConstValue::Bool(l == r)),
                (ConstValue::Nil, ConstValue::Nil) => Some(ConstValue::Bool(true)),
                (ConstValue::Tuple(l), ConstValue::Tuple(r)) => compare_tuples_ne(l, r).map(|b| ConstValue::Bool(b)),
                _ => None
            },
            Self::Gt => match (lhs, rhs) {
                (ConstValue::Int(l), ConstValue::Int(r)) => Some(ConstValue::Bool(l > r)),
                (ConstValue::Float(l), ConstValue::Float(r)) => Some(ConstValue::Bool(l > r)),
                _ => None
            },
            Self::Lt => match (lhs, rhs) {
                (ConstValue::Int(l), ConstValue::Int(r)) => Some(ConstValue::Bool(l < r)),
                (ConstValue::Float(l), ConstValue::Float(r)) => Some(ConstValue::Bool(l < r)),
                _ => None
            },
            Self::Ge => match (lhs, rhs) {
                (ConstValue::Int(l), ConstValue::Int(r)) => Some(ConstValue::Bool(l >= r)),
                (ConstValue::Float(l), ConstValue::Float(r)) => Some(ConstValue::Bool(l >= r)),
                _ => None
            },
            Self::Le => match (lhs, rhs) {
                (ConstValue::Int(l), ConstValue::Int(r)) => Some(ConstValue::Bool(l <= r)),
                (ConstValue::Float(l), ConstValue::Float(r)) => Some(ConstValue::Bool(l <= r)),
                _ => None
            },
            Self::Pipe => match (lhs, rhs) {
                (ConstValue::Int(l), ConstValue::Int(r)) => Some(ConstValue::Int(l | r)),
                (ConstValue::Bool(l), ConstValue::Bool(r)) => Some(ConstValue::Bool(l | r)),
                _ => None
            },
            Self::Ampersand => match (lhs, rhs) {
                (ConstValue::Int(l), ConstValue::Int(r)) => Some(ConstValue::Int(l & r)),
                (ConstValue::Bool(l), ConstValue::Bool(r)) => Some(ConstValue::Bool(l & r)),
                _ => None
            },
            Self::Caret => match (lhs, rhs) {
                (ConstValue::Int(l), ConstValue::Int(r)) => Some(ConstValue::Int(l ^ r)),
                (ConstValue::Bool(l), ConstValue::Bool(r)) => Some(ConstValue::Bool(l ^ r)),
                _ => None
            },
            Self::KwOr => match (lhs, rhs) {
                (ConstValue::Bool(l), ConstValue::Bool(r)) => Some(ConstValue::Bool(*l || *r)),
                _ => None
            },
            Self::KwAnd => match (lhs, rhs) {
                (ConstValue::Bool(l), ConstValue::Bool(r)) => Some(ConstValue::Bool(*l && *r)),
                _ => None
            },
            Self::KwXor => match (lhs, rhs) {
                (ConstValue::Bool(l), ConstValue::Bool(r)) => Some(ConstValue::Bool(*l ^ *r)),
                _ => None
            },
            _ => None
        }
    }
    
    pub fn eval_prefix(&self, operand: &ConstValue) -> Option<ConstValue> {
        match self {
            Self::Plus => match operand {
                ConstValue::Int(o) => Some(ConstValue::Int(*o)),
                ConstValue::Float(o) => Some(ConstValue::Float(*o)),
                _ => None
            },
            Self::Minus => match operand {
                ConstValue::Int(o) => Some(ConstValue::Int(-(*o))),
                ConstValue::Float(o) => Some(ConstValue::Float(-(*o))),
                _ => None
            },
            Self::Bang => match operand {
                ConstValue::Int(o) => Some(ConstValue::Int(!o)),
                ConstValue::Bool(o) => Some(ConstValue::Bool(!o)),
                _ => None
            },
            Self::KwNot => match operand {
                ConstValue::Bool(o) => Some(ConstValue::Bool(!o)),
                _ => None
            },
            _ => None
        }
    }

    pub fn infix_output_ty(&self, lhs: &TypeId, rhs: &TypeId, type_pool: &mut TypePool) -> Option<TypeId> {
        let lty = type_pool.get_type(lhs)?;
        let rty = type_pool.get_type(rhs)?;
        match self {
            Self::Plus | Self::Minus | Self::Star
            | Self::Slash | Self::Modulo => if lty.is_numeric() && rty.is_numeric() {
                if lty == rty {
                    Some(*lhs)
                } else if lty.is_coerceable_into(rty) {
                    type_pool.coerce_type(lhs, rty.clone());
                    Some(*lhs)
                } else if rty.is_coerceable_into(lty) {
                    type_pool.coerce_type(rhs, lty.clone());
                    Some(*lhs)
                } else {
                    None
                }
            } else {
                None
            },
            Self::Eq | Self::Ne => if matches!(lty, Type::Callable { .. }) {
                None
            } else if lty == rty {
                Some(type_pool.predef_types.bool_id)
            } else if lty.is_coerceable_into(rty) {
                type_pool.coerce_type(lhs, rty.clone());
                Some(type_pool.predef_types.bool_id)
            } else if rty.is_coerceable_into(lty) {
                type_pool.coerce_type(rhs, lty.clone());
                Some(type_pool.predef_types.bool_id)
            } else {
                None
            },
            Self::Gt | Self::Lt | Self::Ge | Self::Le => if lty.is_numeric() && rty.is_numeric() {
                if lty == rty {
                    Some(type_pool.predef_types.bool_id)
                } else if lty.is_coerceable_into(rty) {
                    type_pool.coerce_type(lhs, rty.clone());
                    Some(type_pool.predef_types.bool_id)
                } else if rty.is_coerceable_into(lty) {
                    type_pool.coerce_type(rhs, lty.clone());
                    Some(type_pool.predef_types.bool_id)
                } else {
                    None
                }
            } else {
                None
            },
            Self::Pipe | Self::Ampersand | Self::Caret => if lty.is_int() && rty.is_int() {
                if lty == rty {
                    Some(*lhs)
                } else if lty.is_coerceable_into(rty) {
                    type_pool.coerce_type(lhs, rty.clone());
                    Some(*lhs)
                } else if rty.is_coerceable_into(lty) {
                    type_pool.coerce_type(rhs, lty.clone());
                    Some(*lhs)
                } else {
                    None
                }
            } else if matches!((lty, rty), (Type::Bool, Type::Bool)) {
                Some(*lhs)
            } else {
                None
            },
            Self::KwOr | Self::KwAnd | Self::KwXor => if matches!((lty, rty), (Type::Bool, Type::Bool)) {
                Some(*lhs)
            } else {
                None
            },
            _ => None
        }
    }
    
    pub fn prefix_output_ty(&self, operand: &TypeId, type_pool: &TypePool) -> Option<TypeId> {
        let oty = type_pool.get_type(operand)?;
        match self {
            Self::Plus | Self::Minus => if oty.is_numeric() {
                Some(*operand)
            } else {
                None
            },
            Self::Bang => if oty.is_int() || *oty == Type::Bool {
                Some(*operand)
            } else {
                None
            },
            Self::KwNot => if *oty == Type::Bool {
                Some(*operand)
            } else {
                None
            },
            _ => None
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Star => write!(f, "*"),
            Self::Slash => write!(f, "/"),
            Self::Modulo => write!(f, "%"),
            Self::Eq => write!(f, "=="),
            Self::Ne => write!(f, "!="),
            Self::Gt => write!(f, ">"),
            Self::Lt => write!(f, "<"),
            Self::Ge => write!(f, ">="),
            Self::Le => write!(f, "<="),
            Self::Pipe => write!(f, "|"),
            Self::Ampersand => write!(f, "&"),
            Self::Caret => write!(f, "^"),
            Self::Bang => write!(f, "!"),
            Self::KwOr => write!(f, "or"),
            Self::KwAnd => write!(f, "and"),
            Self::KwXor => write!(f, "xor"),
            Self::KwNot => write!(f, "not"),
        }
    }
}

fn compare_tuples_eq(l: &[ConstValue], r: &[ConstValue]) -> Option<bool> {
    for (lval, rval) in l.iter().zip(r) {
        match (lval, rval) {
            (ConstValue::Int(l), ConstValue::Int(r)) => if l != r {
                return Some(false);
            },
            (ConstValue::Float(l), ConstValue::Float(r)) => if l != r {
                return Some(false);
            },
            (ConstValue::Nil, ConstValue::Nil) => continue,
            (ConstValue::Tuple(l), ConstValue::Tuple(r)) => if !compare_tuples_eq(l, r)? {
                return Some(false);
            },
            _ => return None,
        }
    }
    Some(true)
}

fn compare_tuples_ne(l: &[ConstValue], r: &[ConstValue]) -> Option<bool> {
    for (lval, rval) in l.iter().zip(r) {
        match (lval, rval) {
            (ConstValue::Int(l), ConstValue::Int(r)) => if l != r {
                return Some(true);
            },
            (ConstValue::Float(l), ConstValue::Float(r)) => if l != r {
                return Some(true);
            },
            (ConstValue::Nil, ConstValue::Nil) => continue,
            (ConstValue::Tuple(l), ConstValue::Tuple(r)) => if compare_tuples_ne(l, r)? {
                return Some(true);
            },
            _ => return None,
        }
    }
    Some(false)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReassignmentOp {
    Assign,
    PlusEq, MinusEq, StarEq, SlashEq, ModuloEq,
    PipeEq, AmpersandEq, CaretEq,
}

impl ReassignmentOp {
    pub fn validate_reassignment(&self, original: &TypeId, expr: &TypeId, type_pool: &mut TypePool) -> bool {
        let oty = type_pool.get_type(original).unwrap();
        let ety = type_pool.get_type(expr).unwrap();
        match self {
            Self::Assign => if oty == ety {
                true
            } else if oty.is_coerceable_into(ety) {
                type_pool.coerce_type(original, ety.clone());
                true
            } else if ety.is_coerceable_into(oty) {
                type_pool.coerce_type(expr, oty.clone());
                true
            } else {
                false
            },
            Self::PlusEq | Self::MinusEq | Self::StarEq
            | Self::SlashEq | Self::ModuloEq => if oty.is_numeric() && ety.is_numeric() {
                if oty == ety {
                    true
                } else if oty.is_coerceable_into(ety) {
                    type_pool.coerce_type(original, ety.clone());
                    true
                } else if ety.is_coerceable_into(oty) {
                    type_pool.coerce_type(expr, oty.clone());
                    true
                } else {
                    false
                }
            } else {
                false
            },
            Self::PipeEq | Self::AmpersandEq | Self::CaretEq => if oty.is_int() && ety.is_int() {
                if oty == ety {
                    true
                } else if oty.is_coerceable_into(ety) {
                    type_pool.coerce_type(original, ety.clone());
                    true
                } else if ety.is_coerceable_into(oty) {
                    type_pool.coerce_type(expr, oty.clone());
                    true
                } else {
                    false
                }
            } else {
                false
            },
        }
    }
}

impl fmt::Display for ReassignmentOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Assign => write!(f, "="),
            Self::PlusEq => write!(f, "+="),
            Self::MinusEq => write!(f, "-="),
            Self::StarEq => write!(f, "*="),
            Self::SlashEq => write!(f, "/="),
            Self::ModuloEq => write!(f, "%="),
            Self::PipeEq => write!(f, "|="),
            Self::AmpersandEq => write!(f, "&="),
            Self::CaretEq => write!(f, "^="),
        }
    }
}
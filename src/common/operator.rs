use std::fmt;
use crate::sema::{symbol::ConstValue, ty::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Plus,
    Minus,
    Star,
    Slash,
    Modulo,
}

impl Operator {
    #[inline]
    pub const fn can_infix(&self) -> bool {
        true // there's no purely infix ops rn
    }

    #[inline]
    pub const fn can_prefix(&self) -> bool {
        matches!(self, Self::Plus | Self::Minus)
    }

    #[inline]
    pub const fn binding_power(&self) -> usize {
        match self {
            Self::Plus | Self::Minus => 20,
            Self::Star | Self::Slash | Self::Modulo => 30,
        }
    }

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
            }
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
        }
    }
}

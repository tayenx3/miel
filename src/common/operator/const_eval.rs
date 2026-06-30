use num_bigint::BigInt;
use super::Operator;
use crate::sema::constants::ConstValue;

pub fn eval_int(op: &Operator, lhs: &BigInt, rhs: &BigInt) -> Option<ConstValue> {
    match op {
        Operator::Plus => Some(ConstValue::Int(lhs + rhs)),
        Operator::Minus => Some(ConstValue::Int(lhs - rhs)),
        Operator::Star => Some(ConstValue::Int(lhs * rhs)),
        Operator::Slash => Some(ConstValue::Int(lhs / rhs)),
        Operator::Modulo => Some(ConstValue::Int(lhs % rhs)),
        Operator::Eq => Some(ConstValue::Bool(lhs == rhs)),
        Operator::Ne => Some(ConstValue::Bool(lhs != rhs)),
        Operator::Gt => Some(ConstValue::Bool(lhs > rhs)),
        Operator::Lt => Some(ConstValue::Bool(lhs < rhs)),
        Operator::Ge => Some(ConstValue::Bool(lhs >= rhs)),
        Operator::Le => Some(ConstValue::Bool(lhs <= rhs)),
        Operator::Pipe => Some(ConstValue::Int(lhs | rhs)),
        Operator::Ampersand => Some(ConstValue::Int(lhs & rhs)),
        Operator::Caret => Some(ConstValue::Int(lhs ^ rhs)),
        _ => None,
    }
}

pub fn eval_float(op: &Operator, lhs: &f64, rhs: &f64) -> Option<ConstValue> {
    match op {
        Operator::Plus => Some(ConstValue::Float(lhs + rhs)),
        Operator::Minus => Some(ConstValue::Float(lhs - rhs)),
        Operator::Star => Some(ConstValue::Float(lhs * rhs)),
        Operator::Slash => Some(ConstValue::Float(lhs / rhs)),
        Operator::Modulo => Some(ConstValue::Float(lhs % rhs)),
        Operator::Eq => Some(ConstValue::Bool(lhs == rhs)),
        Operator::Ne => Some(ConstValue::Bool(lhs != rhs)),
        Operator::Gt => Some(ConstValue::Bool(lhs > rhs)),
        Operator::Lt => Some(ConstValue::Bool(lhs < rhs)),
        Operator::Ge => Some(ConstValue::Bool(lhs >= rhs)),
        Operator::Le => Some(ConstValue::Bool(lhs <= rhs)),
        _ => None,
    }
}

pub fn eval_bool(op: &Operator, lhs: &bool, rhs: &bool) -> Option<ConstValue> {
    match op {
        Operator::Pipe => Some(ConstValue::Bool(lhs | rhs)),
        Operator::Ampersand => Some(ConstValue::Bool(lhs & rhs)),
        Operator::Caret | Operator::KwXor => Some(ConstValue::Bool(lhs ^ rhs)),
        Operator::KwOr => Some(ConstValue::Bool(*lhs || *rhs)),
        Operator::KwAnd => Some(ConstValue::Bool(*lhs && *rhs)),
        _ => None,
    }
}
pub mod const_eval;

use std::fmt;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Plus = 0,
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

// fn compare_tuples_eq(l: &[ConstValue], r: &[ConstValue]) -> Option<bool> {
//     for (lval, rval) in l.iter().zip(r) {
//         match (lval, rval) {
//             (ConstValue::Int(l), ConstValue::Int(r)) => if l != r {
//                 return Some(false);
//             },
//             (ConstValue::Float(l), ConstValue::Float(r)) => if l != r {
//                 return Some(false);
//             },
//             (ConstValue::Nil, ConstValue::Nil) => continue,
//             (ConstValue::Tuple(l), ConstValue::Tuple(r)) => if !compare_tuples_eq(l, r)? {
//                 return Some(false);
//             },
//             _ => return None,
//         }
//     }
//     Some(true)
// }

// fn compare_tuples_ne(l: &[ConstValue], r: &[ConstValue]) -> Option<bool> {
//     for (lval, rval) in l.iter().zip(r) {
//         match (lval, rval) {
//             (ConstValue::Int(l), ConstValue::Int(r)) => if l != r {
//                 return Some(true);
//             },
//             (ConstValue::Float(l), ConstValue::Float(r)) => if l != r {
//                 return Some(true);
//             },
//             (ConstValue::Nil, ConstValue::Nil) => continue,
//             (ConstValue::Tuple(l), ConstValue::Tuple(r)) => if compare_tuples_ne(l, r)? {
//                 return Some(true);
//             },
//             _ => return None,
//         }
//     }
//     Some(false)
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReassignmentOp {
    Assign,
    PlusEq, MinusEq, StarEq, SlashEq, ModuloEq,
    PipeEq, AmpersandEq, CaretEq,
}

impl ReassignmentOp {
    // pub fn validate_reassignment(&self, original: &TypeId, expr: &TypeId, type_pool: &mut TypePool) -> bool {
    //     let oty = type_pool.get_type(original).unwrap();
    //     let ety = type_pool.get_type(expr).unwrap();
    //     match self {
    //         Self::Assign => if oty == ety {
    //             true
    //         } else if oty.is_coerceable_into(ety) {
    //             type_pool.coerce_type(original, ety.clone());
    //             true
    //         } else if ety.is_coerceable_into(oty) {
    //             type_pool.coerce_type(expr, oty.clone());
    //             true
    //         } else {
    //             false
    //         },
    //         Self::PlusEq | Self::MinusEq | Self::StarEq
    //         | Self::SlashEq | Self::ModuloEq => if oty.is_numeric() && ety.is_numeric() {
    //             if oty == ety {
    //                 true
    //             } else if oty.is_coerceable_into(ety) {
    //                 type_pool.coerce_type(original, ety.clone());
    //                 true
    //             } else if ety.is_coerceable_into(oty) {
    //                 type_pool.coerce_type(expr, oty.clone());
    //                 true
    //             } else {
    //                 false
    //             }
    //         } else {
    //             false
    //         },
    //         Self::PipeEq | Self::AmpersandEq | Self::CaretEq => if oty.is_int() && ety.is_int() {
    //             if oty == ety {
    //                 true
    //             } else if oty.is_coerceable_into(ety) {
    //                 type_pool.coerce_type(original, ety.clone());
    //                 true
    //             } else if ety.is_coerceable_into(oty) {
    //                 type_pool.coerce_type(expr, oty.clone());
    //                 true
    //             } else {
    //                 false
    //             }
    //         } else {
    //             false
    //         },
    //     }
    // }
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
pub mod eval;
pub mod collect;

use std::sync::Arc;
use num_bigint::BigInt;
use crate::parser::ast::Node;
use crate::sema::types::TypeId;
use crate::common::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstValue {
    Int(BigInt), Float(f64),
    Bool(bool), Never, Nil,
    Tuple(Arc<[ConstValue]>),
    Callable {
        params: Arc<[(lasso::Spur, TypeId)]>,
        ret_ty: TypeId,
        body: Vec<Node>,
        body_span: Span,
    }
}


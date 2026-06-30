pub mod types;
pub mod constants;
pub mod scope;
pub mod errors;

use types::{TypeManager, TypeId, Type};
use constants::collect::ConstantCollector;
use scope::ScopeManager;
use crate::common::Diag;
use crate::parser::ast::{Ast, Node, NodeId, NodeKind, ParsedType, ParsedTypeKind};
use crate::parser::ast::visit::Visitor;

pub struct SemaChecker {
    types: TypeManager,
    scope: ScopeManager,
}

impl SemaChecker {
    pub fn new(rodeo: &mut lasso::Rodeo) -> Self {
        Self {
            types: TypeManager::new(rodeo),
            scope: ScopeManager::new(),
        }
    }

    pub fn check(&mut self, ast: &Ast) {}
}